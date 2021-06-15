use {crate::regex, regex::Match};

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
///
/// Please try to use the existing implementation for string slices instead of writing your own
pub trait ExtractMention {
    /// Get the mentions contained in the value
    fn mentions(&self) -> Vec<Mention<'_>>;
}

impl<T> ExtractMention for T
where
    T: AsRef<str>,
{
    fn mentions(&self) -> Vec<Mention<'_>> {
        // Regex101 link (for explaination of the regex): https://regex101.com/r/pyTTsW/1
        let mention_regex = regex!(r#"(?:^|\W)@([\w\-]+)(?:@([\w\.\-]+[[:alnum:]]+))?"#);

        mention_regex
            .captures_iter(self.as_ref())
            .map(|capture| {
                let username = capture.get(1).unwrap().as_str();
                let domain = capture.get(2).as_ref().map(Match::as_str);

                Mention::new(username, domain)
            })
            .collect()
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
