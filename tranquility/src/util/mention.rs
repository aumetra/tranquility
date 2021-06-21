use {
    crate::{
        consts::regex::MENTION, database::Actor as DbActor, error::Error, regex, state::ArcState,
        well_known::webfinger,
    },
    async_trait::async_trait,
    regex::{Captures, Match},
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

#[async_trait]
/// Trait for formatting mentions

pub trait FormatMention {
    /// Format the mentions to links
    async fn format_mentions(&mut self, state: ArcState) -> Vec<Tag>;
}

#[async_trait]
impl FormatMention for Object {
    async fn format_mentions(&mut self, state: ArcState) -> Vec<Tag> {
        let tags = self.content.format_mentions(state).await;
        self.tag = tags.clone();

        tags
    }
}

#[async_trait]
impl FormatMention for String {
    async fn format_mentions(&mut self, state: ArcState) -> Vec<Tag> {
        let handle = Handle::current();
        let text = self.clone();

        // We have to do those moves (async -> sync -> async) because we can't run futures to completion inside a synchronous closure without blocking
        // and we can't access our database without an async executor because SQLx is async-only
        //
        // That's why we use `spawn_blocking` to be allowed to block and then use the handle to the runtime we created earlier
        // to spawn a future onto the already existing runtime for the networking/database interactions and block until the future has resolved
        let format_result = tokio::task::spawn_blocking(move || {
            let mut tags = Vec::new();

            let output = MENTION_REGEX.replace_all(text.as_str(), |capture: &Captures<'_>| {
                let username = capture.name("username").unwrap().as_str();
                let domain = capture.name("domain").as_ref().map(Match::as_str);

                // Block until the future has resolved
                // This is fine because we are inside the `spawn_blocking` context where blocking is allowed
                let actor_result: Result<Actor, Error> = handle.block_on(async {
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

                let mention = capture.get(0).unwrap().as_str().to_string();
                if let Ok(actor) = actor_result {
                    // Create a new ActivityPub tag object
                    tags.push(Tag {
                        r#type: "Mention".into(),
                        name: mention.clone(),
                        href: actor.id.clone(),
                    });

                    format!(r#"<a href="{}">{}</a>"#, actor.id, mention)
                } else {
                    mention
                }
            });

            (output.to_string(), tags)
        })
        .await
        // Log the error and move on
        // The user will most likely delete and redraft when the mentions don't work
        .map_err(|err| error!(error = ?err));

        if let Ok((content, tags)) = format_result {
            *self = content;

            tags
        } else {
            Vec::new()
        }
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
