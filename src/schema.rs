// @generated automatically by Diesel CLI.

diesel::table! {
    definitions (id) {
        id -> Integer,
        word_id -> Integer,
        definition -> Text,
        definition_header -> Text,
    }
}

diesel::table! {
    words (id) {
        id -> Integer,
        word -> Text,
    }
}

diesel::joinable!(definitions -> words (word_id));

diesel::allow_tables_to_appear_in_same_query!(
    definitions,
    words,
);
