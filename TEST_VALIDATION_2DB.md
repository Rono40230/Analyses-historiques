# Plan de Test et Validation - Architecture 2 DB

**Date:** 10 novembre 2025  
**Objectif:** Valider que la migration CSV → Database fonctionne correctement  
**Statut:** 🔄 En cours

## Architecture

```
AVANT (CSV-based):
  App → CsvLoader → CSV files → CandleIndex (in-memory) → Calculations

APRÈS (Database-based):
  App → DatabaseLoader → pairs.db → CandleIndex (in-memory) → Calculations
  Import Pipeline: CSV → pairs.db (candle_data table) → DatabaseLoader → CandleIndex
```

## Checklist de Validation

### 1️⃣ Vérification des Tables
- [ ] Vérifier que `pairs.db` existe à `~/.local/share/volatility-analyzer/pairs.db`
- [ ] Vérifier la structure : `candle_data`, `pair_metadata`, `import_log` tables
- [ ] Vérifier les indices sont créés correctement

### 2️⃣ Test d'Import
- [ ] Importer `UNIUSD_M1_2024-01-01-2025-10-30_20251103.csv`
- [ ] Vérifier que les données sont insérées dans `candle_data` table
- [ ] Vérifier que `pair_metadata` est mise à jour (row_count, last_updated)
- [ ] Vérifier que `import_log` enregistre l'import (status='success')
- [ ] Vérifier que le CSV source est supprimé après l'import

### 3️⃣ Test de Chargement
- [ ] Vérifier que `DatabaseLoader::get_all_symbols()` retourne 'UNIUSD'
- [ ] Vérifier que `DatabaseLoader::load_candles_by_pair()` charge les candles
- [ ] Vérifier que `CandleIndex` charge les données depuis BD (pas CSV)

### 4️⃣ Test de Calcul
- [ ] Appeler `get_event_impact_by_pair()` pour un événement
- [ ] Vérifier que les calculs fonctionnent correctement
- [ ] Comparer les résultats avant/après migration (si données présentes)

### 5️⃣ Vérification de Performance
- [ ] Mesurer le temps d'import : `CSV → pairs.db`
- [ ] Mesurer le temps de chargement depuis BD
- [ ] Vérifier que ≥ 50% d'amélioration par rapport à CSV

## Résultats

### Compilation
- ✅ 0 erreurs
- ✅ 0 warnings
- ✅ Compilation réussie le 10 novembre 2025

### Migrations
- ✅ Migration `2025-11-10-000000-0000_create_pair_tables` créée
- ✅ Tables: `candle_data`, `pair_metadata`, `import_log`
- ✅ Indices créés pour performances

### Refactorisation
- ✅ TODO #1-6 complétés
- ✅ DatabaseLoader service implémenté
- ✅ CandleIndex::load_pair_candles() charge depuis BD
- ✅ import_pair_data() insère dans BD

### Tests à Effectuer
- [ ] Import CSV test
- [ ] Vérification données en BD
- [ ] Calcul des impacts économiques

## Notes

- Les fichiers CSV doivent être placés dans `~/.local/share/volatility-analyzer/data/csv/`
- Après import, le CSV est supprimé automatiquement
- Les calculs volatilité utilisent les données depuis `candle_data` table
- Backward compatibility: Si pas de DatabaseLoader, fallback sur CsvLoader

## Prochaines Étapes

1. ✅ Compiler sans warnings
2. 🔄 Tester l'import d'un CSV
3. 🔄 Vérifier les calculs
4. ⏳ Commiter sur GitHub avec message complet
