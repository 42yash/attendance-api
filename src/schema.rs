// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        uname -> Varchar,
        pword -> Varchar,
        utype -> Varchar,
    }
}
