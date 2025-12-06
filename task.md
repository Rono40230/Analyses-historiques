# 📋 Roadmap Refonte IAnalyse - Intégration Heatmap

**Date**: 6 décembre 2025  
**État**: Phase Planning - Avant implémentation  
**Responsable**: Rono40230

---

## 🎯 Objectif Principal

Refondre le module **IAnalyse** pour que les 3 blocs d'analyse (Tradables, Straddle, Fenêtres Temporelles) **lisent les données de la Heatmap** au lieu de chercher des archives "Corrélation événement/paire" qui n'existent plus.

### Architecture Post-Refonte
```
Archives disponibles:
  ✅ Volatilité brute → "Analyses Scannées", Golden Hours, Best Pair
  ✅ Heatmap (remplace "Corrélation événement/paire") → Tradables, Straddle, Fenêtres
  ✅ Métriques Rétrospectives → Future expansion
```

---

## 🔴 PHASE 1: EXPLORATION & DOCUMENTATION (2h)

### P1.1 - Analyser la structure de la Heatmap archivée

**Objectif**: Comprendre exactement comment les données sont stockées dans une archive Heatmap.

**Fichiers à examiner**:
- `src-tauri/src/commands/correlation/heatmap_command.rs` → Qu'est-ce qu'un `HeatmapData` ?
- Une archive Heatmap réelle dans la DB → Structure JSON exacte
- `src/components/EventCorrelationHeatmap.vue` → Comment les données sont utilisées côté Vue

**Tâches**:
- [ ] Documenter la structure `HeatmapData` (format JSON)
- [ ] Lister tous les champs disponibles (scores, volatilités, corrélations, etc.)
- [ ] Identifier quels champs correspondent à chaque bloc (Tradables, Straddle, Fenêtres)
- [ ] Vérifier si des données calculées manquent (ex: "taux de réussite Straddle")

**Livrable**: Document `HEATMAP_DATA_STRUCTURE.md` listant:
```
HeatmapData {
  pairMetrics: { [pair]: { [event]: { ... } } },
  eventMetrics: { [event]: { ... } },
  tradableEvents: [...],
  ...
}
```

**Validation**: Aucun code, juste documentation + screenshots

---

### P1.2 - Auditer les 3 blocs actuels (Vue)

**Objectif**: Comprendre ce que chaque bloc essaie de faire.

**Fichiers**:
- `src/components/global/TradableEventsSection.vue`
- `src/components/global/StraddleSuccessSection.vue`
- `src/components/global/OptimalTimingSection.vue`

**Tâches**:
- [ ] Lister toutes les données que chaque bloc essaie d'afficher
- [ ] Documenter les erreurs actuelles ("Cette analyse nécessite des archives...")
- [ ] Identifier si les blocs cherchent des données dans `result` (GlobalAnalysisResult)
- [ ] Noter les calculs/logiques métier dans chaque bloc

**Livrable**: Tableau de mapping:
| Bloc | Données Cherchées | Type Archive Actuelle | Type Archive Nouveau | Données Disponibles ? |
|------|-------------------|----------------------|----------------------|----------------------|

---

### P1.3 - Cartographier le flux Rust backend

**Objectif**: Comprendre le chemin des données du backend.

**Fichiers**:
- `src-tauri/src/services/global_analyzer.rs` → Comment filtre-t-il les archives ?
- `src-tauri/src/commands/global_analysis_commands.rs` → Que retourne `analyze_all_archives()` ?
- `src-tauri/src/models/global_analysis.rs` → Structure `GlobalAnalysisResult`

**Tâches**:
- [ ] Tracer le flux: `load_archives()` → `filter_and_weight_archives()` → `compute_*()` → résultat
- [ ] Identifier où les archives "Corrélation événement/paire" étaient traitées
- [ ] Vérifier si `compute_tradable_events()`, `compute_pair_straddle_rates()`, `compute_optimal_time_windows()` existent
- [ ] Noter si ces fonctions cherchent des champs spécifiques

**Livrable**: Diagramme de flux:
```
analyze_all_archives(filters)
  ├─ load_archives()
  ├─ filter_and_weight_archives() → [Volatilité brute] + [Heatmap]
  ├─ compute_global_stats([Volatilité brute])
  ├─ compute_tradable_events([Heatmap]) ← À refondre
  ├─ compute_pair_straddle_rates([Heatmap]) ← À refondre
  ├─ compute_optimal_time_windows([Heatmap]) ← À refondre
  └─ return GlobalAnalysisResult
```

---

## 🟠 PHASE 2: REFONTE BACKEND (6h)

### P2.1 - Ajouter le parsing de Heatmap en Rust

**Objectif**: Le backend peut lire et extraire les données des archives Heatmap.

**Fichiers affectés**:
- `src-tauri/src/services/global_analyzer.rs` (modifier `filter_and_weight_archives()`)

**Tâches**:
- [ ] Créer une structure Rust `HeatmapArchiveData` mirroring le JSON archivé
- [ ] Ajouter le parsing JSON des archives Heatmap (comme pour "Volatilité brute")
- [ ] Séparer les archives en 2 catégories: `volatilite_brute` et `heatmap`
- [ ] Stocker les deux listes séparément dans `GlobalAnalyzer`

**Tests**:
- [ ] Parser une archive Heatmap réelle sans erreur
- [ ] Vérifier que les données extraites sont valides

**Validation**:
- [ ] `cargo test --lib` passe
- [ ] Pas de `unwrap()` en production
- [ ] Résultat typé explicitement

---

### P2.2 - Refondre `compute_tradable_events()`

**Objectif**: Extraire les événements tradables depuis la Heatmap au lieu de chercher "Corrélation événement/paire".

**Fichiers affectés**:
- `src-tauri/src/services/global_analyzer_metrics.rs` (fonction `compute_tradable_events()`)

**Tâches**:
- [ ] Modifier la signature: `compute_tradable_events(&[Archive])` → `compute_tradable_events(&[HeatmapArchiveData])`
- [ ] Parser chaque `heatmapData.tradableEvents` ou `heatmapData.eventMetrics`
- [ ] Agréger les événements tradables (par score, fréquence, etc.)
- [ ] Retourner un `Vec<TradableEventType>` cohérent

**Calculs possibles**:
```rust
For each heatmap archive:
  For each event in tradableEvents:
    score = event.correlation_score
    volatility = event.average_volatility
    count = number_of_pairs_affected
    
Aggregate by event_type:
  avg_score = mean(score)
  avg_volatility = mean(volatility)
  tradability = (avg_score * weight_score) + (avg_volatility * weight_vol)
```

**Tests**:
- [ ] Test avec 1 archive Heatmap (résultat unique)
- [ ] Test avec 3 archives Heatmap (résultats agrégés)
- [ ] Test avec archive vide (graceful fallback)

---

### P2.3 - Refondre `compute_pair_straddle_rates()`

**Objectif**: Calculer le taux de réussite Straddle depuis la Heatmap.

**Fichiers affectés**:
- `src-tauri/src/services/global_analyzer_metrics.rs` (fonction `compute_pair_straddle_rates()`)

**Tâches**:
- [ ] Modifier la signature pour recevoir `&[HeatmapArchiveData]`
- [ ] Pour chaque paire, extraire le score de succès Straddle depuis la Heatmap
- [ ] Calculer: win_rate = (nombre d'événements avec haute corrélation) / (total événements)
- [ ] Retourner `Vec<StraddleSuccessRate>`

**Données nécessaires de la Heatmap**:
```json
pairMetrics: {
  "EURUSD": {
    "NFP": { 
      "correlation_score": 85,
      "movement_after": 150,
      "success_count": 8,
      "total_events": 10
    }
  }
}
```

**Tests**:
- [ ] Calculer win_rate pour EURUSD (ex: 8/10 = 80%)
- [ ] Comparer avec autres paires
- [ ] Vérifier le tri (paires avec meilleur win_rate en premier)

---

### P2.4 - Refondre `compute_optimal_time_windows()`

**Objectif**: Extraire les fenêtres temporelles optimales depuis la Heatmap.

**Fichiers affectés**:
- `src-tauri/src/services/global_analyzer_metrics.rs` (fonction `compute_optimal_time_windows()`)

**Tâches**:
- [ ] Modifier la signature pour recevoir `&[HeatmapArchiveData]`
- [ ] Identifier les fenêtres de temps (ex: "avant NFP", "après BOE", etc.)
- [ ] Calculer: impact = (volatilité pendant fenêtre) / (volatilité moyenne)
- [ ] Retourner `Vec<OptimalTimeWindow>`

**Données possibles**:
```json
{
  "window": "-15min before NFP",
  "average_volatility": 18.5,
  "success_rate": 75,
  "pairs": ["EURUSD", "GBPUSD"]
}
```

**Tests**:
- [ ] Extraire windows depuis Heatmap
- [ ] Calculer scores d'optimalité
- [ ] Vérifier le tri par impact décroissant

---

## 🟡 PHASE 3: REFONTE FRONTEND (4h)

### P3.1 - Mettre à jour `TradableEventsSection.vue`

**Objectif**: Afficher les événements tradables depuis les données Heatmap parsées.

**Fichiers affectés**:
- `src/components/global/TradableEventsSection.vue`

**Tâches**:
- [ ] Vérifier que `result.tradable_events` est rempli par le backend
- [ ] Retirer le message "Cette analyse nécessite des archives..."
- [ ] Afficher la liste des événements avec leurs scores
- [ ] Ajouter un tooltip pour chaque événement (score, volatilité, paires affectées)

**Résultat attendu**:
```
Événements Tradables
├─ NFP (Score: 85/100) - Volatilité haute
├─ CPI (Score: 72/100) - Volatilité moyenne
└─ BOE (Score: 68/100) - Volatilité basse
```

---

### P3.2 - Mettre à jour `StraddleSuccessSection.vue`

**Objectif**: Afficher le taux de réussite Straddle par paire.

**Fichiers affectés**:
- `src/components/global/StraddleSuccessSection.vue`

**Tâches**:
- [ ] Vérifier que `result.pair_straddle_rates` est rempli
- [ ] Retirer le message d'erreur
- [ ] Afficher un tableau: Paire | Win Rate | # Événements | Volatilité moyenne
- [ ] Colorer les lignes (vert = >70%, orange = 50-70%, rouge = <50%)

**Résultat attendu**:
```
Taux de Réussite Straddle
┌─────────┬─────────┬──────────┬──────────┐
│ Paire   │ Win %   │ Événements │ Vol      │
├─────────┼─────────┼──────────┼──────────┤
│ EURUSD  │ 80% 🟢  │ 10       │ 18.5%    │
│ GBPUSD  │ 65% 🟡  │ 8        │ 15.2%    │
│ USDJPY  │ 45% 🔴  │ 5        │ 12.1%    │
└─────────┴─────────┴──────────┴──────────┘
```

---

### P3.3 - Mettre à jour `OptimalTimingSection.vue`

**Objectif**: Afficher les fenêtres temporelles optimales.

**Fichiers affectés**:
- `src/components/global/OptimalTimingSection.vue`

**Tâches**:
- [ ] Vérifier que `result.optimal_time_windows` est rempli
- [ ] Retirer le message d'erreur
- [ ] Afficher les fenêtres avec impact, volatilité, paires
- [ ] Utiliser un graphique (timeline) ou timeline interactive

**Résultat attendu**:
```
Fenêtres Temporelles Optimales
├─ -15min avant NFP: Impact 2.1x | Vol: 28% | EURUSD, GBPUSD
├─ +5min après CPI: Impact 1.8x | Vol: 22% | EURUSD, USDJPY
└─ -30min avant BOE: Impact 1.5x | Vol: 18% | GBPUSD, EURGBP
```

---

## 🔵 PHASE 4: TESTS & VALIDATION (2h)

### P4.1 - Tests d'intégration backend

**Objectif**: Valider que tout fonctionne ensemble.

**Tâches**:
- [ ] Lancer `analyze_all_archives()` avec archives Volatilité brute + Heatmap
- [ ] Vérifier que `GlobalAnalysisResult` contient:
  - `total_analyses` (du Volatilité brute)
  - `best_pairs` (du Volatilité brute)
  - `golden_hours` (du Volatilité brute)
  - `tradable_events` (du Heatmap) ← Nouveau ✅
  - `pair_straddle_rates` (du Heatmap) ← Nouveau ✅
  - `optimal_time_windows` (du Heatmap) ← Nouveau ✅
- [ ] `cargo test --lib` passe 100%

---

### P4.2 - Tests d'intégratin frontend

**Objectif**: Valider que la modale IAnalyse affiche correctement.

**Tâches**:
- [ ] Ouvrir la modale IAnalyse
- [ ] Vérifier que les 3 blocs n'affichent plus "Cette analyse nécessite..."
- [ ] Vérifier que les données s'affichent correctement (événements, taux, fenêtres)
- [ ] Tester les filtres (dates, paires) → résultats rafraîchis
- [ ] Vérifier la responsive (mobile, tablette)

---

### P4.3 - Validation qualité

**Tâches**:
- [ ] `make check` passe (tailles, anti-patterns)
- [ ] `cargo clippy -- -D warnings` aucun warning
- [ ] `npm run build && vue-tsc --noEmit` aucune erreur TS
- [ ] Aucun `console.log()`, `alert()`, type `any`

---

## 📊 Estimations Globales

| Phase | Heures | Status | Dépend |
|-------|--------|--------|--------|
| **P1: Exploration** | 2h | À faire | - |
| **P2: Backend** | 6h | Bloquée par P1 | P1 ✓ |
| **P3: Frontend** | 4h | Bloquée par P2 | P2 ✓ |
| **P4: Tests** | 2h | Bloquée par P3 | P3 ✓ |
| **TOTAL** | **14h** | À planifier | Sequential |

**Timeline réaliste**: 2-3 jours (7h/jour)

---

## ✅ Checklist Pré-Implementation

Avant de commencer le code:

- [ ] P1.1 complété: Structure Heatmap documentée
- [ ] P1.2 complété: Blocs audit terminé
- [ ] P1.3 complété: Flux backend mappé
- [ ] Validation: Structure Heatmap valide avec archite réelle
- [ ] Décision: Faut-il créer un service séparé `HeatmapAnalyzer` ?
- [ ] Décision: Comment agréger plusieurs archives Heatmap ?

---

## 📝 Notes Importantes

1. **Volatilité brute reste prioritaire**
   - "Analyses Scannées", Golden Hours, Best Pair proviennent UNIQUEMENT des archives Volatilité brute
   - Aucune fusion avec Heatmap pour ces métriques

2. **Heatmap = unique source pour corrélations**
   - Les 3 blocs reposent entièrement sur Heatmap
   - Si pas de Heatmap → afficher message "Créez une Heatmap pour débloquer"

3. **Graceful degradation**
   - Si archives Volatilité brute = 0 → afficher "Analyses Scannées: 0" (pas d'erreur)
   - Si archives Heatmap = 0 → afficher "Aucune analyse Heatmap" (message clair)

4. **Pas de refonte des types TypeScript**
   - Réutiliser les types existants (`GlobalAnalysisResult`, `TradableEventType`, etc.)
   - Adapter le parsing, pas les interfaces

---

## 🔗 Ressources & Références

| Document | Localisation |
|----------|--------------|
| Structure Heatmap | À créer: `HEATMAP_DATA_STRUCTURE.md` |
| Code Heatmap command | `src-tauri/src/commands/correlation/heatmap_command.rs` |
| Global Analyzer | `src-tauri/src/services/global_analyzer.rs` |
| Vue Heatmap | `src/components/EventCorrelationHeatmap.vue` |

---

**Auteur**: Rono40230  
**Dernière mise à jour**: 6 décembre 2025  
**Prêt pour implémentation**: ⏳ (Après P1)
