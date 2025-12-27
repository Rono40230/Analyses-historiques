# 🛠️ Plan d'Amélioration Bidi (Audit Volatilité)

Ce document liste les tâches correctives suite à l'audit de logique financière du 25/12/2025.
L'objectif est de rendre les paramètres générés sûrs et rentables pour le robot de trading Bidi.

## 🔴 Priorité 1 : Réalisme Financier (Critique)
*Ces correctifs sont indispensables pour que les backtests reflètent la réalité du marché.*

- [x] **Simuler le Spread & Slippage** (`straddle_simulator.rs`)
    - [x] Ajouter un paramètre `simulated_spread` (ex: 3-5 pips fixes pour news).
    - [x] Ajouter un paramètre `simulated_slippage` (ex: 10% de l'ATR ou fixe).
    - [x] Déduire ces coûts du P&L de chaque trade simulé.
    - [x] Comptabiliser le spread *deux fois* en cas de Whipsaw.

- [x] **Correction de la Détection Whipsaw** (`straddle_simulator.rs`)
    - [x] Définir un Whipsaw non seulement par la touche des deux bornes, mais aussi par la perte nette (Spread x2).
    - [x] Pénaliser fortement le score de fiabilité si le ratio Whipsaw > 20%.

## 🟠 Priorité 2 : Intelligence des Données
*Pour éviter de diluer les statistiques avec du bruit.*

- [ ] **Filtrage par Déviation** (`global_analyzer_event_analysis.rs`)
    - [x] **Parsing des Données**: Extraire Actual/Forecast/Previous des fichiers CSV/Excel et les stocker en DB.
    - [x] Ne pas mélanger les événements "neutres" (Actual = Forecast) avec les surprises.
    - [ ] Ajouter un filtre : Analyser uniquement si `|Actual - Forecast| > Threshold`.
    - [ ] Séparer les stats : "Impact si Surprise" vs "Impact Global".

- [ ] **Correction de la Directionnalité** (`global_analyzer_straddle_calc.rs`)
    - [ ] Ne plus utiliser la volatilité brute pour définir un mouvement directionnel.
    - [ ] Implémenter le ratio `Body / Range` (Taille du corps / Taille totale).
    - [ ] Exclure les bougies "Doji" (haute volatilité mais clôture proche de l'ouverture) des succès directionnels.

## 🟡 Priorité 3 : Affinement des Paramètres (Bidi V5)
*Pour générer des paramètres dynamiques et non arbitraires.*

- [ ] **Formules Linéaires vs Seuils Fixes** (`straddle_parameter_service.rs`)
    - [ ] Remplacer les paliers (`if noise > 2.5`) par une formule continue.
    - [ ] Formule proposée : `Offset = ATR * (1.5 + (NoiseRatio * 0.5))`.
    - [ ] Formule proposée : `SL = ATR * (2.0 + (NoiseRatio * 0.8))`.

- [ ] **Suppression du Biais "Look-ahead"** (`straddle_simulator.rs`)
    - [ ] Pour l'optimisation rétroactive, ne pas utiliser les mèches de l'événement *courant* pour calculer son offset idéal.
    - [ ] Utiliser une moyenne glissante des 5 derniers événements similaires.

- [ ] **Définition du Hard TP**
    - [ ] Ajouter un calcul de TP fixe (ex: 2x le risque ou 80% de l'ATR moyen historique).
    - [ ] Ne pas se reposer uniquement sur le Trailing Stop pour les news.

## 🔵 Priorité 4 : UX & Visualisation
*Pour aider le trader à prendre la décision.*

- [ ] **Indicateur de Confiance**
    - [ ] Afficher un score de confiance (0-100%) basé sur la taille de l'échantillon et la régularité des réactions passées.
    - [ ] Alerter si l'échantillon est trop faible (< 5 événements).

- [ ] **Visualisation du Spread Impact**
    - [ ] Afficher graphiquement la zone de prix "mangée" par le spread théorique sur les graphiques de backtest.
