# Tâches du Projet Analyses Historiques

## 🚀 FEAT-01 : Backtest Événementiel (Simulation de Performance) <!-- id: 6 -->
Objectif : Valider la "Recette" (Paramètres Straddle) en la rejouant sur les événements passés pour estimer la rentabilité et le risque.

### 1. Backend : Moteur de Simulation (Rust)
- [x] **Architecture du Service**
    - Créer le module `src-tauri/src/services/backtest/`
    - Définir les structures de données :
        - `StrategyMode` (Enum : `Directionnel` | `Simultane`)
        - `BacktestConfig` (Paramètres : Offset, SL, TP, Timeout, Spread)
        - `TradeResult` (Date, Pips, Duration, Outcome: TP/SL/Timeout)
        - `BacktestSummary` (WinRate, TotalPips, MaxDrawdown, ProfitFactor)

- [x] **Implémentation du Moteur (`engine.rs`)**
    - Logique de chargement des bougies (T-5min à T+60min pour chaque événement).
    - **Simulation Mode Directionnel (Breakout)** :
        - Ordres OCO (Buy Stop / Sell Stop).
        - Gestion du Whipsaw (si SL touché -> Perte).
    - **Simulation Mode Simultané (Recovery)** :
        - Ordres initiaux.
        - Si SL touché -> Activation du trade inverse avec `stop_loss_recovery`.
        - Gestion de la "Double Perte" (Pire scénario).

- [x] **Commande Tauri**
    - Créer `src-tauri/src/commands/backtest.rs`.
    - Exposer `run_backtest_command(pair, event_type, params, mode)`.

### 2. Frontend : Interface de Simulation (Vue.js)
- [x] **Composant d'Affichage (`BacktestResultsPanel.vue`)**
    - Design "Accordéon" qui s'ouvre sous les paramètres.
    - Affichage des KPIs (Win Rate, Gain Total, etc.).
    - Liste déroulante des trades individuels (Date | Résultat | Détail).

- [x] **Intégration dans `BacktestView.vue`** (Adaptation: Vue dédiée créée)
    - Création d'une vue dédiée pour le Backtest.
    - Configuration via `BacktestConfigPanel.vue`.
    - Affichage des résultats via `BacktestResultsPanel.vue`.
    - Ajout de l'onglet "Backtest" dans `App.vue`.

### 3. Export & Archivage
- [ ] **Adaptation de `ArchiveModal.vue`**
    - Supporter le type d'archive "Backtest".
    - Permettre de sauvegarder le JSON complet des résultats.
- [ ] **Visualisation des Archives**
    - Permettre de rouvrir un Backtest archivé dans le `BacktestResultsPanel` (mode lecture seule).

---

## ✅ Tâches terminées (Historique)
- [x] **FIX-01 à FIX-04** : Correctifs critiques (ATR, Timezone, Corrélation, Normalisation).
- [x] **LOGIC-01** : Gestion du Spread & Whipsaw.
- [x] **LOGIC-02** : Paramètres Dynamiques.
- [x] **LOGIC-03** : Harmonisation de la Corrélation (BidiCalculator utilise StraddleParameterService).
