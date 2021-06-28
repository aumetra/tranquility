use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
/// Struct representing an [ActivityStreams tag](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tag)
pub struct Tag {
    pub r#type: String,
    /// Format: @\<username\>@\<instance\>
    pub name: String,
    pub href: String,
}
