// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        task -> Text,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
