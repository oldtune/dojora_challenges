// @generated automatically by Diesel CLI.

diesel::table! {
    challenge (id) {
        id -> Uuid,
        title -> Text,
        description -> Nullable<Text>,
    }
}
