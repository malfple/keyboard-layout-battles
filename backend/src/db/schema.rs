// @generated automatically by Diesel CLI.

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
