-- This file should undo anything in `up.sql`

ALTER TABLE battle_history_tab
DROP INDEX idx_layout_id_1_id;

ALTER TABLE battle_history_tab
DROP INDEX idx_layout_id_2_id;

ALTER TABLE battle_history_tab
DROP INDEX idx_user_id_typer_id;
