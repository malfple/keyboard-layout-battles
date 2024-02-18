// @generated automatically by Diesel CLI.

diesel::table! {
    battle_history_tab (id) {
        id -> Unsigned<Bigint>,
        layout_id_1 -> Unsigned<Bigint>,
        layout_id_2 -> Unsigned<Bigint>,
        #[max_length = 64]
        base_layout_data -> Varchar,
        user_id_typer -> Unsigned<Bigint>,
        layout_1_rating -> Integer,
        layout_2_rating -> Integer,
        rating_1_gain -> Integer,
        rating_2_gain -> Integer,
        result_data -> Json,
        is_personal -> Bool,
        time_created -> Bigint,
    }
}

diesel::table! {
    battle_tab (id) {
        #[max_length = 32]
        id -> Varchar,
        layout_id_1 -> Unsigned<Bigint>,
        layout_id_2 -> Unsigned<Bigint>,
        #[max_length = 64]
        base_layout_data -> Varchar,
        user_id_typer -> Nullable<Unsigned<Bigint>>,
        content_data -> Json,
        is_personal -> Bool,
        time_created -> Bigint,
        time_modified -> Bigint,
    }
}

diesel::table! {
    layout_tab (id) {
        id -> Unsigned<Bigint>,
        sequence_id -> Nullable<Unsigned<Bigint>>,
        #[max_length = 32]
        name -> Varchar,
        #[max_length = 64]
        layout_data -> Varchar,
        #[max_length = 256]
        description -> Nullable<Varchar>,
        rating -> Integer,
        rating_comfort -> Integer,
        rating_data -> Nullable<Json>,
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
        #[max_length = 64]
        layout_data -> Varchar,
        time_created -> Bigint,
        time_modified -> Bigint,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    battle_history_tab,
    battle_tab,
    layout_tab,
    user_tab,
);
