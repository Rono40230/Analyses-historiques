# Tâches en cours

## 📅 Futur : Unification des Logiques de Trading
- [ ] **Harmoniser Volatilité Brute avec Correlation de la volatilité**
    - Importer la logique de `bidi_calculator.rs` (Noise Ratio, SL adaptatif) vers `straddle_simulator.rs`.
    - Remplacer le SL fixe (1:1) par un SL basé sur le Noise Ratio.
    - Importer le calcul de Timeout basé sur la décroissance.
    - Objectif : Rendre l'onglet "Volatilité Brute" aussi opérationnel que le Correlation.
