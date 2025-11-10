-- Rollback: Supprimer toutes les tables de pairs.db
DROP TABLE IF EXISTS import_log;
DROP TABLE IF EXISTS pair_metadata;
DROP TABLE IF EXISTS candle_data;
