-- Migration rollback

ALTER TABLE calendar_events 
DROP COLUMN calendar_import_id;

DROP INDEX IF EXISTS idx_calendar_imports_is_active;

DROP TABLE IF EXISTS calendar_imports;
