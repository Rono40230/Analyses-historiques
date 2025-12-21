# 📋 Tâches : Unification des Modales d'Analyse (Corrélation vs Volatilité)

Ce plan vise à aligner la modale d'analyse de l'onglet "Corrélation" (`RetroactiveAnalysisResultsViewer`) sur la logique et l'esthétique mises à jour de la modale "Volatilité Brute" (`MetricsAnalysisModal`).

## 🚀 Priorité 1 : Refactoring Frontend (Composants & Affichage)

L'objectif est d'éliminer la duplication de code et d'assurer un affichage cohérent (UnitDisplay).

- [ ] **Remplacement des Panneaux de Direction**
    - Remplacer `RetroAnalysisDirectionalPanel.vue` par `StraddleDirectionalCard.vue`.
    - Adapter les props passées par `RetroAnalysisResults.vue` pour correspondre à celles attendues par `StraddleDirectionalCard`.
    - S'assurer que `UnitDisplay` est bien utilisé pour le formatage (points vs pips).

- [ ] **Remplacement des Panneaux Simultanés**
    - Remplacer `RetroAnalysisSimultaneousPanelFinal.vue` par `StraddleSimultaneousCard.vue`.
    - Vérifier l'affichage du "SL Recovery" et du "Trailing Stop".

- [ ] **Harmonisation de l'Affichage Temporel**
    - Corriger l'affichage du "Moment de placement" dans la vue Corrélation.
    - Gérer les cas "T0", "(Début)" comme dans la nouvelle modale, au lieu d'un "min avant" statique.

## ⚙️ Priorité 2 : Révision de la Logique Backend (Calculs)

Les métriques affichées dans la vue Corrélation doivent provenir de la même logique "StraddleService" que la vue Volatilité.

- [ ] **Audit de la commande `analyze_retrospective`** (ou équivalent utilisé pour la corrélation)
    - Vérifier si elle appelle `StraddleService` ou si elle utilise une ancienne logique ad-hoc.

- [ ] **Mise à jour des Formules Straddle**
    - **Offset** : Doit inclure la logique "Percentile 95 des mèches" + "Marge de sécurité".
    - **Stop Loss** : Doit respecter le ratio risque/récompense défini dans le nouveau service.
    - **Trailing Stop** : Doit utiliser le coefficient dynamique basé sur le Noise Ratio.

- [ ] **Propagation des Données**
    - S'assurer que l'objet JSON renvoyé au frontend contient bien les champs nécessaires (`offset_optimal`, `sl_adjusted`, etc.) pour nourrir les nouveaux composants.

## 🎨 Priorité 3 : Nettoyage & Détails

- [ ] **Suppression du Code Mort**
    - Une fois la migration faite, supprimer `RetroAnalysisDirectionalPanel.vue` et `RetroAnalysisSimultaneousPanelFinal.vue`.
    - Nettoyer les imports inutilisés dans `RetroAnalysisResults.vue`.

- [ ] **Vérification Visuelle**
    - Tester l'affichage sur une paire en JPY (formatage pips) et une paire standard (points).
    - Vérifier l'alignement des graphiques SVG de visualisation.
