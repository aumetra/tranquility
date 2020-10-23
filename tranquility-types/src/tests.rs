const CREATE_ACTIVTY: &str = r#"
{
    "@context": ["https://www.w3.org/ns/activitystreams"],
    "cc": [
        "https://b.example.com/users/test",
        "https://a.example.com/users/test/followers"
    ],
    "id": "https://a.example.com/activities/2h2HRj2bRQ0z4Y6-HDYIO0xfuxROSYo-WQ99b5SV9w6SqWH-5WTLCjVoGC4nz1R",
    "to": [
        "https://www.w3.org/ns/activitystreams#Public"
    ],
    "type": "Create",
    "actor": "https://a.example.com/users/test",
    "object": "https://a.example.com/objects/1VWqVfdrnwVre6q-pTmlbIDpIkqe0ci-49TAPGIRMNXIozC-ohEOFqMx8pvD5ut",
    "context": "https://b.example.com/contexts/21706f7d-928a-4643-b3fd-581f6c1ea83e",
    "published": "2020-10-20T17:33:21.634Z",
    "context_id": 1
}
"#;

const CREATE_ACTIVTY_OBJECT: &str = r#"
{
    "@context": ["https://www.w3.org/ns/activitystreams"],
    "cc": [
        "https://b.example.com/users/test",
        "https://a.example.com/users/test/followers"
    ],
    "id": "https://a.example.com/activities/2h2HRj2bRQ0z4Y6-HDYIO0xfuxROSYo-WQ99b5SV9w6SqWH-5WTLCjVoGC4nz1R",
    "to": [
        "https://www.w3.org/ns/activitystreams#Public"
    ],
    "type": "Create",
    "actor": "https://a.example.com/users/test",
    "object": {
        "cc": [
            "https://b.example.com/users/test",
            "https://a.example.com/users/test/followers"
        ],
        "id": "https://a.example.com/objects/1VWqVfdrnwVre6q-pTmlbIDpIkqe0ci-49TAPGIRMNXIozC-ohEOFqMx8pvD5ut",
        "to": [
            "https://www.w3.org/ns/activitystreams#Public"
        ],
        "bcc":[
        ],
        "bto": [
        ],
        "type": "Note",
        "actor": "https://a.example.com/users/test",
        "content": "@test@b.example.com test",
        "context": "https://b.example.com/contexts/21706f7d-928a-4643-b3fd-581f6c1ea83e",
        "summary": "",
        "@context": [
            "https://www.w3.org/ns/activitystreams",
            "https://w3id.org/security/v1",
            {
                "Hashtag": "as:Hashtag"
            }
        ],
        "inReplyTo": null,
        "published": "2020-10-20T17:33:21.634Z",
        "sensitive": false,
        "context_id": 1,
        "attributedTo": "https://a.example.com/users/test",
        "conversation": "https://b.example.com/contexts/21706f7d-928a-4643-b3fd-581f6c1ea83e"
    },
    "context": "https://b.example.com/contexts/21706f7d-928a-4643-b3fd-581f6c1ea83e",
    "published": "2020-10-20T17:33:21.634Z",
    "context_id": 1
}
"#;

const MASTODON_WEBFINGER_RESOURCE: &str = r#"
{
    "subject": "acct:Gargron@mastodon.social",
    "aliases": [
      "https://mastodon.social/@Gargron",
      "https://mastodon.social/users/Gargron"
    ],
    "links": [
      {
        "rel": "http://webfinger.net/rel/profile-page",
        "type": "text/html",
        "href": "https://mastodon.social/@Gargron"
      },
      {
        "rel": "self",
        "type": "application/activity+json",
        "href": "https://mastodon.social/users/Gargron"
      },
      {
        "rel": "http://ostatus.org/schema/1.0/subscribe",
        "template": "https://mastodon.social/authorize_interaction?uri={uri}"
      }
    ]
  }
"#;

const PLEROMA_ACTOR: &str = r#"
{
    "@context": ["https://www.w3.org/ns/activitystreams", "https://lain.com/schemas/litepub-0.1.jsonld", {
            "@language": "und"
        }
    ],
    "attachment": [],
    "capabilities": {
        "acceptsChatMessages": true
    },
    "discoverable": false,
    "endpoints": {
        "oauthAuthorizationEndpoint": "https://lain.com/oauth/authorize",
        "oauthRegistrationEndpoint": "https://lain.com/api/v1/apps",
        "oauthTokenEndpoint": "https://lain.com/oauth/token",
        "sharedInbox": "https://lain.com/inbox",
        "uploadMedia": "https://lain.com/api/ap/upload_media"
    },
    "followers": "https://lain.com/users/lain/followers",
    "following": "https://lain.com/users/lain/following",
    "icon": {
        "type": "Image",
        "url": "https://lain.com/media/2c67ab791a9c8df78b003eb09a931b0693a15992697b0f2a2cb2a8e0dee4ec28.png"
    },
    "id": "https://lain.com/users/lain",
    "image": {
        "type": "Image",
        "url": "https://lain.com/media/2398f2000f757c0707c9b43dae9868271766a73e314b1d4d945adcfeee7d674f.jpg"
    },
    "inbox": "https://lain.com/users/lain/inbox",
    "manuallyApprovesFollowers": false,
    "name": "bodhisattva\n\nbodhisattva",
    "outbox": "https://lain.com/users/lain/outbox",
    "preferredUsername": "lain",
    "publicKey": {
        "id": "https://lain.com/users/lain#main-key",
        "owner": "https://lain.com/users/lain",
        "publicKeyPem": "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAsnyIJ/uSrx/bVfsLmB0b\nmIiX5lBIuGJqSa7/br3Xl/zcLennthAhl3seSAC1EXtkhVJfN+2NVP8GajiKtVnl\nNTIyi1fzH+w9s2YtH9DD5onz8lpy2Aaq5ax+eA7L+4TvYjpvQigmqBwayh0hxPGZ\nGzMfuAh5BTtHsd7hWkeyVzTGb1s9bAtsJukCoygLYnzbZNuAxRIXgvQ1I3QDkLXU\nu6Gykg67yN/TOrUfBWs5rZLRqbM5Re5AD0VipCA7n1iLYDewDCoeFsaIlW7Qw3mK\nA4Anw8UouNOrFu2+YcKGSMLk8GfiFYsjUrROjv/wuRQMQXRb5R71IwsRk1y+sYka\nSwIDAQAB\n-----END PUBLIC KEY-----\n\n"
    },
    "summary": "No more hiding",
    "tag": [],
    "type": "Person",
    "url": "https://lain.com/users/lain"
}
"#;

const RFC_JRD: &str = r#"
{
    "subject":"http://blog.example.com/article/id/314",
    "expires":"2010-01-30T09:30:00Z",

    "aliases":[
        "http://blog.example.com/cool_new_thing",
        "http://blog.example.com/steve/article/7"
    ],

    "properties": {
        "http://blgx.example.net/ns/version":"1.3",
        "http://blgx.example.net/ns/ext":null
    },

    "links": [
        {
            "rel":"author",
            "type":"text/html",
            "href":"http://blog.example.com/author/steve",
            "titles": {
                "default":"About the Author",
                "en-us":"Author Information"
            },
            "properties": {
                "http://example.com/role":"editor"
            }
        },
        {
            "rel":"author",
            "href":"http://example.com/author/john",
            "titles": {
                "default":"The other author"
            }
        },
        {
            "rel":"copyright",
            "template":"http://example.com/copyright?id={uri}"
        }
    ]
}
"#;

#[test]
fn decode_actor() {
    let _actor: crate::activitypub::Actor = serde_json::from_str(PLEROMA_ACTOR).unwrap();
}

#[test]
fn decode_create_activity_url() {
    let _activity: crate::activitypub::Activity = serde_json::from_str(CREATE_ACTIVTY).unwrap();
}

#[test]
fn decode_create_activity_object() {
    let _activity: crate::activitypub::Activity =
        serde_json::from_str(CREATE_ACTIVTY_OBJECT).unwrap();
}

#[test]
fn decode_rfc_jrd() {
    let _jrd: crate::webfinger::Resource = serde_json::from_str(RFC_JRD).unwrap();
}

#[test]
fn decode_mastodon_webfinger_resource() {
    let _resource: crate::webfinger::Resource =
        serde_json::from_str(MASTODON_WEBFINGER_RESOURCE).unwrap();
}
