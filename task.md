# 📋 Plan de Correction : Normalisation et Cohérence des Métriques

Ce document détaille les étapes nécessaires pour corriger les incohérences de calcul (Max Spike, échelles des indices, décalage graphique) identifiées lors de l'analyse des captures.

## 🎯 Objectif
Unifier la gestion des unités (Pips/Points) à travers tout le système pour garantir des métriques réalistes et une cohérence parfaite entre les tableaux et les graphiques.

---

## 🚀 Phase 1 : Correction du Moteur de Normalisation (Priorité Haute)
*L'objectif est de s'assurer que le système identifie correctement chaque actif et sa valeur de "point".*

- [x] **Améliorer `AssetProperties` (`models/asset_class.rs`)**
    - [x] Affiner la détection des indices (US30, NAS100, DAX, etc.) pour utiliser une valeur de point de `1.0`.
    - [x] Ajouter une logique de détection du nombre de décimales pour s'adapter aux différents formats de fichiers CSV (MetaTrader vs TradingView).
    - [x] Valider les conventions pour l'Or (XAU) et l'Argent (XAG).

- [x] **Sécuriser le calcul du "Max Spike" (`services/metrics/distribution.rs`)**
    - [x] Remplacer le maximum absolu (sensible aux erreurs de données) par un percentile élevé (ex: 95e ou 98e percentile).
    - [x] Ajouter un filtre pour ignorer les bougies aberrantes (ex: True Range > 500% de la moyenne locale).

---

## 🏗️ Phase 2 : Unification de la Chaîne de Calcul (Priorité Moyenne)
*L'objectif est de supprimer les "bypass" de normalisation pour que tous les modules parlent la même langue.*

- [x] **Refactoriser `Stats15MinCalculator` (`services/volatility/stats_15min.rs`)**
    - [x] Intégrer `AssetProperties` dès le début du calcul.
    - [x] Normaliser l'ATR, le Range et le Max Spike immédiatement après le calcul brut.
    - [x] Supprimer la dépendance à `get_point_value` (obsolète) au profit de `AssetProperties`.

- [x] **Synchroniser le Graphique (`services/volatility/quarterly_aggregator.rs`)**
    - [x] Appliquer la normalisation au `volatility_profile` (données minute par minute).
    - [x] S'assurer que les valeurs envoyées au frontend pour le graphique sont en points/pips et non en prix brut.

---

## 🎨 Phase 3 : Cohérence de l'Interface (Priorité Basse)
*L'objectif est d'afficher clairement les unités à l'utilisateur.*

- [x] **Mise à jour de l'affichage UI**
    - [x] Utiliser l'étiquette d'unité (`unit`) renvoyée par le backend ("pips" ou "pts") dans les en-têtes de colonnes.
    - [x] Harmoniser le nombre de décimales affichées selon le type d'actif (ex: 1 décimale pour le Forex, 0 pour les Indices).

---

## ✅ Critères de Validation
1. [x] **BTCUSD :** Le Max Spike dans le tableau doit être cohérent avec le sommet du graphique (environ 50-100 pts, pas 11 000).
2. [x] **Indices :** L'ATR du Nasdaq doit afficher des valeurs réalistes (ex: 20-50 pts) et non des centaines de milliers.
3. [x] **Forex :** L'ATR doit être clairement identifiable en pips (ex: 8.5 pips).
4. [x] **Zéro Régression :** Les calculs de score de confiance et de paramètres Straddle doivent rester fonctionnels.
