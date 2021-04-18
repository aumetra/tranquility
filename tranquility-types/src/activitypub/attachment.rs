use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
/// Struct representing an [ActivityStreams attachment](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-attachment)
pub struct Attachment {
    pub r#type: String,
    pub url: String,
}
