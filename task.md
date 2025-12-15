# Tâches du Projet Analyses Historiques

## 🚨 CORRECTIFS CRITIQUES (PRIORITÉ ABSOLUE)
- [x] **FIX-01 : Calcul ATR & Max Spike** <!-- id: 0 -->
    - `src-tauri/src/services/volatility/hourly_stats.rs`
    - Problème : L'ATR lisse trop les pics de volatilité (News).
    - Action : Remplacer `atr_values.last()` par `mean(&atr_values)`.
    - Action : Ajouter le calcul de `max_true_range` (le plus grand range M1 brut de l'heure) pour capturer l'explosivité réelle.
- [x] **FIX-02 : Timezone & UTC** <!-- id: 1 -->
    - `src-tauri/src/services/volatility/hourly_stats.rs`
    - Problème : `PARIS_OFFSET_HOURS` hardcodé (+1).
    - Action : Passer tout le backend en UTC strict.
- [x] **FIX-03 : Fenêtre de Corrélation Précise** <!-- id: 2 -->
    - `src-tauri/src/services/event_correlation.rs`
    - Problème : Fenêtre -2h/+2h trop large.
    - Action : Réduire à `-10 min` / `+30 min`.
    - Action : Utiliser le `max_true_range` dans cette fenêtre pour mesurer l'impact.
- [x] **FIX-04 : Normalisation Pips/Points** <!-- id: 8 -->
    - `src-tauri/src/models/` & `src-tauri/src/services/`
    - Problème : Incohérence des unités entre Forex (5 digits), JPY (3 digits), Indices et Crypto.
    - Action : Créer un `PipValueNormalizer` qui détecte automatiquement la classe d'actif.
    - Action : Standardiser l'affichage (Forex=Pips, Indices=Points, Crypto=$) tout en gardant la précision interne.

## 🛠 AMÉLIORATIONS LOGIQUES (STRADDLE V2)
- [ ] **LOGIC-01 : Gestion du Spread & Whipsaw** <!-- id: 3 -->
    - `src-tauri/src/services/straddle_parameter_service.rs`
    - Action : Ajouter une "Marge de Sécurité Spread" configurable (ex: +3 pips).
    - Action : Détecter les "Dojis Géants" (High Volatility + Low Body) et recommander `RISKY`.
- [ ] **LOGIC-02 : Paramètres Dynamiques** <!-- id: 4 -->
    - `src-tauri/src/services/straddle_parameter_service.rs`
    - Action : Timeout basé sur la durée de retour au calme.
    - Action : Seuils relatifs (`target = 2.0 * average_atr`) pour le `BestQuarterFinder`.
- [ ] **LOGIC-03 : Analyse Conditionnelle** <!-- id: 5 -->
    - Permettre de filtrer les stats horaires : "Seulement les jours avec Event High Impact".

## 🚀 FONCTIONNALITÉS (PHASE SUIVANTE)
- [ ] **FEAT-01 : Backtest Événementiel** <!-- id: 6 -->
    - "Comment a réagi l'EURUSD aux 10 derniers NFP ?"
- [ ] **FEAT-02 : Export PDF** <!-- id: 7 -->
    - Rapport propre pour le trader.

## 📝 Tâches en cours
- [ ] Aucune tâche active pour le moment.

## ✅ Tâches terminées
- [x] Audit initial du code et des formules.
