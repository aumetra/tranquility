use {
    crate::{
        consts::regex::MENTION, database::Actor as DbActor, error::Error, regex, state::ArcState,
        well_known::webfinger,
    },
    async_trait::async_trait,
    regex::{Captures, Match},
    std::{mem, sync::Arc},
    tokio::runtime::Handle,
    tranquility_types::activitypub::{Actor, Object, Tag},
};

regex!(MENTION_REGEX = MENTION);

/// Struct representing a mention
///
/// If it's a remote mention (mentions a user from a different instance), a `domain` value is present
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Mention<'a> {
    pub username: &'a str,
    pub domain: Option<&'a str>,
}

impl<'a> Mention<'a> {
    fn new(username: &'a str, domain: Option<&'a str>) -> Self {
        Self { username, domain }
    }
}

/// Trait for getting mentions
pub trait ExtractMention {
    /// Get the mentions contained in the value
    fn mentions(&self) -> Vec<Mention<'_>>;
}

impl<T> ExtractMention for T
where
    T: AsRef<str>,
{
    fn mentions(&self) -> Vec<Mention<'_>> {
        MENTION_REGEX
            .captures_iter(self.as_ref())
            .map(|capture| {
                let username = capture.name("username").unwrap().as_str();
                let domain = capture.name("domain").as_ref().map(Match::as_str);

                Mention::new(username, domain)
            })
            .collect()
    }
}

#[inline]
/// Format the username and the domain into the mention format
fn format_mention(username: &str, domain: Option<&str>) -> String {
    if let Some(domain) = domain {
        format!("@{}@{}", username, domain)
    } else {
        format!("@{}", username)
    }
}

#[async_trait]
/// Trait for formatting mentions

pub trait FormatMention {
    /// Format the mentions to links
    async fn format_mentions(&mut self, state: ArcState) -> Vec<Tag>;
}

#[async_trait]
impl FormatMention for Object {
    async fn format_mentions(&mut self, state: ArcState) -> Vec<Tag> {
        self.content.format_mentions(state).await
    }
}

#[async_trait]
impl FormatMention for String {
    async fn format_mentions(&mut self, state: ArcState) -> Vec<Tag> {
        let handle = Handle::current();

        // Safety:
        // This transmute is necessary as the `spawn_blocking` function requires the closure to have a static lifetime
        // The data that is being formatted might not have a static lifetime
        // This is fine though because the task gets awaited and therefore should get joined before the value can even be dropped
        //
        // (maybe clone this instead of transmuting)
        let this = unsafe { mem::transmute::<_, &'static mut String>(self) };

        // We have to do those moves (async -> sync -> async) because we can't run futures to completion inside a synchronous closure without blocking
        // and we can't access our database without an async executor because SQLx is async-only
        //
        // That's why we use `spawn_blocking` to be allowed to block and then use the handle to the runtime we created earlier
        // to spawn a future onto the already existing runtime for the networking/database interactions and block until the future has resolved
        tokio::task::spawn_blocking(move || {
            let mut tags = Vec::new();

            let output = MENTION_REGEX.replace_all(this.as_str(), |capture: &Captures<'_>| {
                let state = Arc::clone(&state);
                let username = capture.name("username").unwrap().as_str();
                let domain = capture.name("domain").as_ref().map(Match::as_str);

                // Block until the future has resolved
                // This is fine because we are inside the `spawn_blocking` context where blocking is allowed
                let actor_result: Result<Actor, Error> = handle.block_on(async move {
                    let actor = if let Some(domain) = domain {
                        let (actor, _db_actor) =
                            webfinger::fetch_actor(&state, username, domain).await?;

                        actor
                    } else {
                        let db_actor = DbActor::by_username_local(&state.db_pool, username).await?;
                        let actor: Actor = serde_json::from_value(db_actor.actor)?;

                        actor
                    };

                    Ok(actor)
                });

                let mention = format_mention(username, domain);

                if let Ok(actor) = actor_result {
                    // Create a new ActivityPub tag object
                    let tag = Tag {
                        r#type: "Mention".into(),
                        name: mention.clone(),
                        href: actor.id.clone(),
                    };
                    tags.push(tag);

                    format!(r#"<a href="{}">{}</a>"#, actor.id, mention)
                } else {
                    mention
                }
            });
            *this = output.to_string();

            tags
        })
        .await
        .map_err(|err| error!(error = ?err))
        .unwrap_or_default()
    }
}

#[cfg(test)]
mod test {
    use super::{ExtractMention, Mention};

    const LOCAL_MENTION: &str = "@alice";
    const MULTIPLE_MENTIONS: &str = "@alice @bob@example.com\n@carol@the.third.example.com\t@dave@fourth.example.com hello@example.com";
    const REMOTE_MENTION: &str = "@bob@example.com";

    #[test]
    fn local_mention() {
        let mentions = LOCAL_MENTION.mentions();

        assert_eq!(mentions, [Mention::new("alice", None)])
    }

    #[test]
    fn multiple_mentions() {
        let mentions = MULTIPLE_MENTIONS.mentions();

        assert_eq!(
            mentions,
            [
                Mention::new("alice", None),
                Mention::new("bob", Some("example.com")),
                Mention::new("carol", Some("the.third.example.com")),
                Mention::new("dave", Some("fourth.example.com")),
            ]
        )
    }

    #[test]
    fn remote_mention() {
        let mentions = REMOTE_MENTION.mentions();

        assert_eq!(mentions, [Mention::new("bob", Some("example.com"))])
    }
}
