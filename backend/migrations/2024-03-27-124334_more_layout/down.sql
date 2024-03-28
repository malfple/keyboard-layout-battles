-- This file should undo anything in `up.sql`
DELETE FROM layout_tab
WHERE id >= 33;

ALTER TABLE layout_tab AUTO_INCREMENT=33;
