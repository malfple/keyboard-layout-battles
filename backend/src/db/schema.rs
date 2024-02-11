// @generated automatically by Diesel CLI.

diesel::table! {
    layout_tab (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 32]
        name -> Varchar,
        #[max_length = 256]
        layout_data -> Varchar,
        #[max_length = 256]
        description -> Nullable<Varchar>,
        rating -> Integer,
        rating_comfort -> Integer,
        rating_data -> Nullable<Json>,
        is_active -> Bool,
        time_created -> Bigint,
        time_modified -> Bigint,
    }
}

diesel::table! {
    user_tab (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 32]
        username -> Varchar,
        #[max_length = 64]
        password -> Varchar,
        #[max_length = 255]
        layout_data -> Varchar,
        time_created -> Bigint,
        time_modified -> Bigint,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    layout_tab,
    user_tab,
);
