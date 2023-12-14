// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        priority -> Nullable<Text>,
        completed -> Nullable<Bool>,
        created_at -> Timestamptz,
    }
}
