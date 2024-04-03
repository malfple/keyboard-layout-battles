-- Your SQL goes here

CREATE INDEX idx_layout_id_1_id
ON battle_history_tab (layout_id_1, id);

CREATE INDEX idx_layout_id_2_id
ON battle_history_tab (layout_id_2, id);

CREATE INDEX idx_user_id_typer_id
ON battle_history_tab (user_id_typer, id);
