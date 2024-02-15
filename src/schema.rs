#[macro_use]
use diesel::table;

table! {
    folders (id) {
        id -> Integer,
        name -> Text,
        created_at -> Timestamp,
    }
}

table! {
    files (id) {
        id -> Integer,
        name -> Text,
        format -> Text,
        created_at -> Timestamp,
        content -> Text,
        folder_id -> Integer,
    }
}
