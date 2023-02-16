// @generated automatically by Diesel CLI.

diesel::table! {
    matches (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        player_one_name -> Text,
        player_two_name -> Text,
        created_date -> Timestamp,
    }
}
