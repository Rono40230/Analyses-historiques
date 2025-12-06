# 📋 Plan d'Action: Amélioration Modale IAnalyse Statistique

**Objectif Principal**: Transformer la modale "IAnalyse Statistique" pour afficher des analyses Straddle utiles basées sur les 25 archives existantes (4 Volatilité + 20 Métriques Rétrospectives + 1 Heatmap).

**Date de Démarrage**: 6 décembre 2025  
**Status**: 🟡 En Cours

---

## 🎯 PHASE 1: Infrastructure & Parsing (URGENT - Commence Maintenant)

### **Étape 1.1: Explorer Structure des Archives** ⭐ PRIORITÉ 0
**Objectif**: Comprendre exactement la structure JSON de chaque type d'archive.

**Tâches**:
- [ ] Examiner une archive "Volatilité brute" (structure complète)
- [ ] Examiner une archive "Métriques Rétrospectives" (structure complète)
- [ ] Examiner l'archive "Heatmap" (structure + données de corrélation)
- [ ] Documenter les champs clés pour chaque type :
  - Volatilité: `peakDelay`, `decayTimeout`, `peakAtr`, `confidence`, `eventCount`, `pair`, `eventType`
  - Rétrospectives: (même structure + extras?)
  - Heatmap: `pairsArray`, `eventsArray`, `impactMatrix`

**Fichiers à Consulter**:
- Où sont stockées les archives ? (localStorage, fichier JSON, base données?)
- Comment sont-elles chargées dans la modale actuelle ?
- `src/components/GlobalAnalysisModal.vue` ou `IanalyseStatistique.vue`
- `src/composables/useGlobalAnalysis.ts`

**Livrable**: Document avec structures JSON de chaque archive type

✅ **STATUS**: TERMINÉ - Voir `docs/ARCHIVE_STRUCTURES.md`

**Résumé Découvertes**:
- Archive stockée en BD : `id`, `title`, `archive_type`, `data_json` (String JSON)
- Type 1 (Volatilité): Simple metrics (symbol, confidence, volatility)
- Type 2 (Rétrospectives): Complet avec peakDelay, decayTimeout, confidence
- Type 3 (Heatmap): Corrélations pair × événement + impact scores
- Approche parsing: Normaliser tous les types en `NormalizedArchive`

---

### **Étape 1.2: Créer Composable `useArchiveStatistics.ts`** ⭐ PRIORITÉ 1
**Objectif**: Centraliser toute la logique de parsing et calcul statistique.

**Tâches**:
- [x] Créer `src/composables/useArchiveStatistics.ts` (296 lignes)
- [x] Implémenter fonction : `loadAllArchives()` 
- [x] Implémenter fonction : `parseArchiveByType(archive)`
- [x] Implémenter fonction : `groupArchivesByEvent(archives)`
- [x] Implémenter fonction : `groupArchivesByPair(archives)`
- [x] Créer interfaces TypeScript pour tous les types

**Status**: ✅ TERMINÉ

---

### **Étape 1.3: Implémenter Calculs Statistiques** ⭐ PRIORITÉ 2
**Objectif**: Créer fonctions de calcul pour chaque bloc de stats.

**Tâches**:
- [x] Créer `src/composables/useArchiveCalculations.ts` (165 lignes)
- [x] Fonction : `calculateEventStatistics(archives)` ✓
- [x] Fonction : `calculatePairStatistics(archives)` ✓
- [x] Fonction : `calculateTradabilityScore(eventStats)` ✓
- [x] Fonction : `calculateOptimalStraddleParams(eventStats)` ✓
- [x] Fonction : `extractHeatmapData(archives)` ✓
- [x] Fonction : `generateAdvice(eventStats, pairStats)` ✓
- [x] Intégrer calculs dans `useArchiveStatistics.ts` avec computed values ✓

**Status**: ✅ TERMINÉ

---

## 🎨 PHASE 2: Composants & Affichage (IMPORTANT)

### **Étape 2.1: Créer Composant `EventAnalysisBlock.vue`** ⭐ PRIORITÉ 3
**Objectif**: Afficher "Types d'Événements Tradables" avec score tradabilité.

**Tâches**:
- [ ] Créer `src/components/analysis/EventAnalysisBlock.vue` (max 200 lignes)
- [ ] Props : `eventStatistics: Record<string, EventStats>`
- [ ] Afficher pour chaque événement :
  ```
  🟢 NFP - OPTIMAL
  ├─ Vol: 45p ATR | Pic: T+3.2min
  ├─ Confiance: 92% | Analyses: 6
  ├─ Straddle SL: 67.5p | TP: 135p
  └─ [📋 Détails] [📈 Graphique]
  ```
- [ ] Couleurs dynamiques basées sur score:
  - 🟢 OPTIMAL: score >= 80%
  - 🟡 BON: score 60-79%
  - 🔴 RISQUÉ: score < 60%
- [ ] Click sur bloc → Affiche tooltips avec détails Straddle
- [ ] Trier par tradabilityScore DESC

**Fichier**: `src/components/analysis/EventAnalysisBlock.vue`  
**Taille Max**: 200 lignes

---

### **Étape 2.2: Créer Composant `PairAnalysisBlock.vue`** ⭐ PRIORITÉ 3
**Objectif**: Afficher "Performance Straddle par Paire".

**Tâches**:
- [ ] Créer `src/components/analysis/PairAnalysisBlock.vue` (max 200 lignes)
- [ ] Props : `pairStatistics: Record<string, PairStats>`
- [ ] Afficher pour chaque paire :
  ```
  EURUSD
  ├─ Confiance: 82% (12 analyses)
  ├─ Vol moyenne: 28 pips ATR
  ├─ Sensibilité:
  │  ├─ NFP: 92%
  │  ├─ CPI: 78%
  │  └─ BCE: 88%
  └─ Verdict: 🟢 TRÈS BON
  ```
- [ ] Trier par avgConfidence DESC
- [ ] Couleurs basées sur avgConfidence

**Fichier**: `src/components/analysis/PairAnalysisBlock.vue`  
**Taille Max**: 200 lignes

---

### **Étape 2.3: Créer Composant `TimingAnalysisBlock.vue`** ⭐ PRIORITÉ 3
**Objectif**: Afficher "Fenêtres Temporelles Optimales" avec stratégie Straddle.

**Tâches**:
- [ ] Créer `src/components/analysis/TimingAnalysisBlock.vue` (max 250 lignes)
- [ ] Props : `eventStatistics: Record<string, EventStats>`, `pairVariances: ?`
- [ ] Afficher pour chaque événement :
  ```
  NFP (Non-Farm Payroll)
  ├─ 📍 CHRONOLOGIE
  │  ├─ T-90sec: Pré-annonce (spreads serrés)
  │  ├─ T0: ANNONCE (spike)
  │  ├─ T+3.2min: PEAK ← PRENDRE TP 50%
  │  ├─ T+9min: Demi-vie
  │  └─ T+18.5min: STABILISATION ← EXIT OBLIGATOIRE
  │
  ├─ 🎯 STRADDLE SETUP
  │  ├─ Placement: T-60sec
  │  ├─ SL: 67.5p | TP: 135p
  │  └─ Gain estimé: 2.5R
  │
  └─ 📈 VARIANCE PAR PAIRE
     ├─ EURUSD: T+3.0min (± 0.6min)
     ├─ GBPUSD: T+3.4min (± 0.8min)
     └─ USDJPY: T+2.8min (± 0.4min)
  ```
- [ ] Visualisation timeline (SVG simple ou CSS)
- [ ] Tooltips sur chaque phase

**Fichier**: `src/components/analysis/TimingAnalysisBlock.vue`  
**Taille Max**: 250 lignes

---

### **Étape 2.4: Créer Composant `AdviceBlock.vue`** ⭐ PRIORITÉ 4
**Objectif**: Afficher "Conseils Dynamiques Straddle".

**Tâches**:
- [ ] Créer `src/components/analysis/AdviceBlock.vue` (max 150 lignes)
- [ ] Props : `allStatistics: { events, pairs, heatmap }`
- [ ] Générer 4 sections :
  ```
  🎯 MEILLEURS SETUPS
  - NFP (vendredi): EURUSD 92% ✓
  - BCE (jeudi): EURUSD 88% ✓
  
  ⚠️ À ÉVITER
  - Inflation (62% confiance)
  - Multiple events same hour
  
  💰 OPTIMISATION
  - Meilleur ratio: NFP/EURUSD (1:3.2)
  - Plus rapide: USDJPY (T+2.8min)
  
  🔔 ALERTES
  - "NFP demain 8h30: EURUSD ready"
  ```
- [ ] Algorithme : Sélectionner setups avec confiance > 75% AND count >= 3
- [ ] Générer dynamiquement selon les données

**Fichier**: `src/components/analysis/AdviceBlock.vue`  
**Taille Max**: 150 lignes

---

### **Étape 2.5: Créer Composant `GlobalStatsBlock.vue`** ⭐ PRIORITÉ 4
**Objectif**: Afficher "Statistiques Globales".

**Tâches**:
- [ ] Créer `src/components/analysis/GlobalStatsBlock.vue` (max 120 lignes)
- [ ] Props : `allArchives: Archive[]`
- [ ] Afficher :
  ```
  📊 RÉSUMÉ DES 25 ARCHIVES
  ├─ Total événements: 32
  ├─ Total paires: 6
  ├─ Confiance moyenne: 79.3%
  └─ Win rate estimé: 71%
  
  MÉTRIQUES STRADDLE
  ├─ ATR moyen: 38 pips
  ├─ TP/SL ratio: 1:2.3
  ├─ Durée moyenne: 16.2 min
  └─ Recommandation: "Excellent setup"
  ```

**Fichier**: `src/components/analysis/GlobalStatsBlock.vue`  
**Taille Max**: 120 lignes

---

### **Étape 2.6: Refactoriser GlobalAnalysisModal.vue** ⭐ PRIORITÉ 5
**Objectif**: Intégrer tous les nouveaux composants dans la modale.

**Tâches**:
- [ ] Ouvrir `src/components/GlobalAnalysisModal.vue`
- [ ] Remplacer les 3 blocs "Cette analyse nécessite..." par:
  - `<EventAnalysisBlock />`
  - `<PairAnalysisBlock />`
  - `<TimingAnalysisBlock />`
- [ ] Ajouter tab system :
  ```
  [Événements] [Paires] [Timing] [Conseils] [Stats Globales]
  ```
- [ ] Charger archives au montage via `useArchiveStatistics()`
- [ ] Passer statistiques calculées à chaque composant
- [ ] Garder bouton "Appliquer les filtres" pour navigation

**Fichier**: `src/components/GlobalAnalysisModal.vue`

---

## 🧪 PHASE 3: Tests & Validation (IMPORTANT)

### **Étape 3.1: Test Parsing Archives** ⭐ PRIORITÉ 6
**Objectif**: Vérifier que les archives sont correctement parsées.

**Tâches**:
- [ ] Créer tests unitaires pour `useArchiveStatistics.ts`
- [ ] Tester : `loadAllArchives()` retourne 25 archives
- [ ] Tester : Chaque archive a champs unifiés (pair, eventType, etc.)
- [ ] Tester : `groupArchivesByEvent()` retourne ~32 événements
- [ ] Tester : `groupArchivesByPair()` retourne ~6 paires

**Fichier**: `src/composables/__tests__/useArchiveStatistics.test.ts`

---

### **Étape 3.2: Test Calculs Statistiques** ⭐ PRIORITÉ 6
**Objectif**: Vérifier que les calculs sont corrects.

**Tâches**:
- [ ] Tester : `calculateEventStatistics()` retourne EventStats[] correct
- [ ] Vérifier : Moyennes sont correctes
- [ ] Vérifier : Variance est calculée (ou score est realistique)
- [ ] Tester : `calculateTradabilityScore()` retourne 0-100
- [ ] Tester : Score OPTIMAL >= 80, BON 60-79, RISQUÉ < 60

---

### **Étape 3.3: Test Affichage Modal** ⭐ PRIORITÉ 6
**Objectif**: Vérifier que la modale affiche correctement les données.

**Tâches**:
- [ ] Ouvrir modale → Doit afficher tous les blocs (pas de messages "nécessite...")
- [ ] Bloc Événements → Afficher 32 événements triés par score
- [ ] Bloc Paires → Afficher 6 paires triés par confiance
- [ ] Bloc Timing → Afficher timeline + setup Straddle pour chaque
- [ ] Bloc Conseils → Afficher conseils pertinents (confiance > 75%)
- [ ] Bloc Stats → Afficher statistiques globales correctes

**Fichier**: Manuel (testing dans l'app)

---

## 📦 Livrables et Commits

### **Commit 1: Infrastructure Parsing**
```
feat(archive-stats): ajouter composable useArchiveStatistics
- Implémenter loadAllArchives()
- Implémenter parseArchiveByType()
- Implémenter groupArchivesByEvent()
- Implémenter groupArchivesByPair()
```

### **Commit 2: Calculs Statistiques**
```
feat(archive-stats): implémenter calculs statistiques
- calculateEventStatistics()
- calculatePairStatistics()
- calculateTradabilityScore()
- calculateOptimalStradleParams()
- extractHeatmapImpacts()
```

### **Commit 3: Composants Analyse**
```
feat(components): ajouter composants analyse Straddle
- EventAnalysisBlock.vue
- PairAnalysisBlock.vue
- TimingAnalysisBlock.vue
- AdviceBlock.vue (optionnel)
- GlobalStatsBlock.vue (optionnel)
```

### **Commit 4: Refactoring Modal**
```
refactor(global-analysis): intégrer nouveaux blocs analyse
- Remplacer messages "nécessite archives..."
- Ajouter tab system
- Charger et passer données statistiques
- Tester affichage complet
```

---

## 🎯 Métriques de Succès

- ✅ Les 25 archives sont toutes chargées (pas uniquement 4)
- ✅ Les 3 blocs "Cette analyse nécessite..." sont remplacés par des données réelles
- ✅ NFP affiche score tradabilité (ex: 92% OPTIMAL)
- ✅ EURUSD affiche confiance 82% + sensibilité par événement
- ✅ NFP affiche timeline + setup Straddle (SL/TP/Placement)
- ✅ Aucun warning/erreur dans console
- ✅ Tous les tests passent (> 80% coverage)

---

## 📅 Estimations Temps

| Phase | Étape | Durée | Cumul |
|-------|-------|-------|-------|
| 1 | 1.1 | 30 min | 30 min |
| 1 | 1.2 | 45 min | 1h15 |
| 1 | 1.3 | 45 min | 2h00 |
| 2 | 2.1 | 60 min | 3h00 |
| 2 | 2.2 | 45 min | 3h45 |
| 2 | 2.3 | 60 min | 4h45 |
| 2 | 2.4 | 30 min | 5h15 |
| 2 | 2.5 | 30 min | 5h45 |
| 2 | 2.6 | 60 min | 6h45 |
| 3 | Tests | 45 min | 7h30 |

---

## ✅ Status de Progression

- [x] Plan rédigé
- [x] **⚡ Étape 1.1: Explorer structures archives** ✅
- [x] **⚡ Étape 1.2: Composable useArchiveStatistics** ✅
- [x] **⚡ Étape 1.3: Calculs statistiques** ✅
- [ ] Étape 2.1: Composant EventAnalysisBlock
- [ ] Étape 2.2-2.6: Autres composants & Modal
- [ ] Étape 3.1-3.3: Tests

---

## 🚀 PHASE 1 TERMINÉE

**Livrables**:
✅ `docs/ARCHIVE_STRUCTURES.md` - Documentation structures archives  
✅ `src/composables/useArchiveStatistics.ts` - Parser + groupement (296 lignes)  
✅ `src/composables/useArchiveCalculations.ts` - Calculs statistiques (165 lignes)  
✅ Commande Tauri: `list_all_archives`  
✅ Compilation réussie (Fedora + TypeScript)

**Prêt pour PHASE 2 ?**
