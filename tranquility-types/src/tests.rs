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

#[test]
fn decode_create_activity_url() {
    let _activity: crate::activitypub::Activity = serde_json::from_str(CREATE_ACTIVTY).unwrap();
}

#[test]
fn decode_create_activity_object() {
    let _activity: crate::activitypub::Activity =
        serde_json::from_str(CREATE_ACTIVTY_OBJECT).unwrap();
}
