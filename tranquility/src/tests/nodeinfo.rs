use {super::test_state, std::sync::Arc};

const NODEINFO_21_SCHEMA: &str = r#"
{
    "$schema": "http://json-schema.org/draft-04/schema#",
    "id": "http://nodeinfo.diaspora.software/ns/schema/2.1#",
    "description": "NodeInfo schema version 2.1.",
    "type": "object",
    "additionalProperties": false,
    "required": [
      "version",
      "software",
      "protocols",
      "services",
      "openRegistrations",
      "usage",
      "metadata"
    ],
    "properties": {
      "version": {
        "description": "The schema version, must be 2.1.",
        "enum": [
          "2.1"
        ]
      },
      "software": {
        "description": "Metadata about server software in use.",
        "type": "object",
        "additionalProperties": false,
        "required": [
          "name",
          "version"
        ],
        "properties": {
          "name": {
            "description": "The canonical name of this server software.",
            "type": "string",
            "pattern": "^[a-z0-9-]+$"
          },
          "version": {
            "description": "The version of this server software.",
            "type": "string"
          },
          "repository": {
            "description": "The url of the source code repository of this server software.",
            "type": "string"
          },
          "homepage": {
            "description": "The url of the homepage of this server software.",
            "type": "string"
          }
        }
      },
      "protocols": {
        "description": "The protocols supported on this server.",
        "type": "array",
        "minItems": 1,
        "items": {
          "enum": [
            "activitypub",
            "buddycloud",
            "dfrn",
            "diaspora",
            "libertree",
            "ostatus",
            "pumpio",
            "tent",
            "xmpp",
            "zot"
          ]
        }
      },
      "services": {
        "description": "The third party sites this server can connect to via their application API.",
        "type": "object",
        "additionalProperties": false,
        "required": [
          "inbound",
          "outbound"
        ],
        "properties": {
          "inbound": {
            "description": "The third party sites this server can retrieve messages from for combined display with regular traffic.",
            "type": "array",
            "minItems": 0,
            "items": {
              "enum": [
                "atom1.0",
                "gnusocial",
                "imap",
                "pnut",
                "pop3",
                "pumpio",
                "rss2.0",
                "twitter"
              ]
            }
          },
          "outbound": {
            "description": "The third party sites this server can publish messages to on the behalf of a user.",
            "type": "array",
            "minItems": 0,
            "items": {
              "enum": [
                "atom1.0",
                "blogger",
                "buddycloud",
                "diaspora",
                "dreamwidth",
                "drupal",
                "facebook",
                "friendica",
                "gnusocial",
                "google",
                "insanejournal",
                "libertree",
                "linkedin",
                "livejournal",
                "mediagoblin",
                "myspace",
                "pinterest",
                "pnut",
                "posterous",
                "pumpio",
                "redmatrix",
                "rss2.0",
                "smtp",
                "tent",
                "tumblr",
                "twitter",
                "wordpress",
                "xmpp"
              ]
            }
          }
        }
      },
      "openRegistrations": {
        "description": "Whether this server allows open self-registration.",
        "type": "boolean"
      },
      "usage": {
        "description": "Usage statistics for this server.",
        "type": "object",
        "additionalProperties": false,
        "required": [
          "users"
        ],
        "properties": {
          "users": {
            "description": "statistics about the users of this server.",
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "total": {
                "description": "The total amount of on this server registered users.",
                "type": "integer",
                "minimum": 0
              },
              "activeHalfyear": {
                "description": "The amount of users that signed in at least once in the last 180 days.",
                "type": "integer",
                "minimum": 0
              },
              "activeMonth": {
                "description": "The amount of users that signed in at least once in the last 30 days.",
                "type": "integer",
                "minimum": 0
              }
            }
          },
          "localPosts": {
            "description": "The amount of posts that were made by users that are registered on this server.",
            "type": "integer",
            "minimum": 0
          },
          "localComments": {
            "description": "The amount of comments that were made by users that are registered on this server.",
            "type": "integer",
            "minimum": 0
          }
        }
      },
      "metadata": {
        "description": "Free form key value pairs for software specific values. Clients should not rely on any specific key present.",
        "type": "object",
        "minProperties": 0,
        "additionalProperties": true
      }
    }
  }"#;

#[tokio::test]
async fn nodeinfo() {
    let state = Arc::new(test_state().await);

    let well_known = crate::well_known::routes(&state);

    let response = warp::test::request()
        .path("/.well-known/nodeinfo/2.1")
        .reply(&well_known)
        .await;
    assert!(response.status().is_success());

    let response_body = response.body();
    let response_body =
        serde_json::from_slice(response_body).expect("Couldn't parse nodeinfo entity");

    let schema = serde_json::from_str(NODEINFO_21_SCHEMA).expect("Couldn't parse nodeinfo schema");

    assert!(jsonschema::is_valid(&schema, &response_body))
}
