table! {
    activities (id) {
        id -> Uuid,
        owner_id -> Uuid,
        data -> Jsonb,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    actors (id) {
        id -> Uuid,
        username -> Text,
        email -> Nullable<Text>,
        password_hash -> Nullable<Text>,
        actor -> Jsonb,
        url -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

joinable!(activities -> actors (owner_id));

allow_tables_to_appear_in_same_query!(activities, actors,);
