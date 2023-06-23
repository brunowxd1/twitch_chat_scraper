// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Int4,
        user_id -> Int4,
        comment -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        is_sub -> Bool,
        is_partner -> Bool,
        is_mod -> Bool,
        is_vip -> Bool,
        is_admin -> Bool,
        is_broadcaster -> Bool,
    }
}

diesel::joinable!(comments -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    users,
);
