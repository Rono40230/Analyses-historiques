CREATE TABLE archives (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  archive_type TEXT NOT NULL, -- "METRICS", "EVENT_IMPACT", "PAIR_IMPACT", "HEATMAP"
  period_start TEXT NOT NULL,
  period_end TEXT NOT NULL,
  comment TEXT,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  data_json TEXT NOT NULL -- Stockage du résultat complet en JSON
);
