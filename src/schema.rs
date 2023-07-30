// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Uuid,
        title -> Varchar,
        body -> Text,
        #[max_length = 20]
        status -> Varchar,
        sender_id -> Uuid,
        recipient_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        surename -> Varchar,
        email -> Varchar,
        rule -> Varchar,
        password -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    users,
);
