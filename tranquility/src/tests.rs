use crate::activitypub::FollowActivity;

const FOLLOW_ACTIVITY: &str = r#"
{
    "cc": ["https://www.w3.org/ns/activitystreams#Public"],
    "id": "https://a.example.com/activities/8dcc256a-8c3f-49ee-ab22-bb51c9082260",
    "to": ["https://b.example.com/users/test"],
    "type": "Follow",
    "actor": "https://a.example.com/users/test",
    "state": "pending",
    "object": "https://b.example.com/users/test",
    "context": "https://a.example.com/contexts/9c3b4420-dd74-454b-8124-c4759b849f3a",
    "published": "2019-08-20T14:02:09.995388Z",
    "context_id": 8
}
"#;

#[test]
fn decode_follow_activity() {
    let follow_activity: FollowActivity = serde_json::from_str(FOLLOW_ACTIVITY).unwrap();

    assert_eq!(follow_activity.activity.r#type, "Follow");
    assert!(follow_activity.approved);
}
