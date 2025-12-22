# 📊 Analyses Historiques - Volatility Analyzer

## 🎯 À quoi sert cette application ?

**Analyses Historiques** est un outil d'aide à la décision conçu spécifiquement pour le **News Trading** (trading d'annonces économiques) sur le Forex et les Indices.

Son objectif unique est de **paramétrer scientifiquement** une stratégie de type **Straddle** (achat et vente simultanés) en analysant le comportement passé des marchés. Au lieu de deviner des paramètres au hasard (Stop Loss, Take Profit, distance d'entrée), l'application analyse des années d'historique pour vous donner les valeurs optimales basées sur la volatilité réelle.

Elle répond à la question fondamentale : *"Comment cet actif réagit-il habituellement à cet événement économique précis ?"*

---

## ❓ À quelles questions répond-elle ?

L'application permet de répondre précisément aux questions suivantes avant chaque annonce économique :

1.  **Faut-il trader cet événement ?**
    *   *Est-ce que ça bouge assez ?* (Volatilité suffisante)
    *   *Est-ce que c'est propre ?* (Ratio de bruit faible, peu de mèches)
    *   *Est-ce que c'est dangereux ?* (Risque de "Whipsaw" / faux départ)

2.  **Comment paramétrer mon robot (Straddle) ?**
    *   *À quelle distance placer mes ordres ?* (Offset optimal)
    *   *Où mettre mon Stop Loss pour ne pas être sorti par le bruit ?*
    *   *Combien de temps garder la position ?* (Durée de l'impulsion)

3.  **Quelle est la meilleure paire ?**
    *   *Sur quel actif l'impact est-il le plus fort et le plus directionnel ?* (Comparaison EURUSD vs GBPUSD vs GOLD...)

4.  **Quelle est la performance attendue ?**
    *   *Si j'avais tradé cet événement les 5 dernières années avec ces paramètres, quel aurait été mon résultat ?* (Backtest)

---

## 🛠️ Workflow par Onglet

L'application est organisée en 6 onglets principaux suivant le flux de travail logique d'un trader.

### 1. 📅 Planning (Feuille de Route Hebdomadaire)
*C'est votre tableau de bord opérationnel pour la semaine à venir.*
*   **Fonctionnement :** Affiche le calendrier économique de la semaine en cours (synchronisé depuis Forex Factory ou importé manuellement).
*   **Cartes Événements :** Chaque événement futur est affiché avec :
    *   Un badge d'impact (High/Medium).
    *   Un badge indiquant le nombre d'occurrences historiques disponibles (📚).
*   **Workflow d'Analyse :**
    1.  Sélectionnez la paire à trader directement sur la carte de l'événement.
    2.  Cliquez sur le bouton **"📊 Analyser"**.
    3.  Une fenêtre s'ouvre avec l'analyse historique complète et les paramètres optimaux (Offset, SL, TP).
    4.  Si une stratégie "Simultanée" (Double Straddle) est possible, les paramètres additionnels s'affichent.

### 2. 📥 Calendrier (Import Hub)
*Le centre de gestion de vos données.*
*   **Import Calendrier :** Deux modes d'importation :
    *   **Historique Général :** Importez un gros fichier CSV (ex: 2018-2024) pour nourrir les statistiques.
    *   **Planning Hebdo :** Cochez la case "Planning Hebdo" pour importer le fichier de la semaine en cours. Cela remplace automatiquement l'ancien planning sans toucher à votre historique général.
*   **Import Paires :** Importez vos données OHLC (M1) pour permettre les calculs de volatilité.

### 3. 🔥 Heatmap de Corrélation
*Pour identifier les opportunités en un coup d'œil.*
*   **Fonctionnement :** Affiche une matrice visuelle (Événements x Paires).
*   **Lecture :**
    *   🟥 **Rouge** : Impact violent, forte volatilité.
    *   🟩 **Vert** : Impact faible ou nul.
*   **Action :** Cliquez sur une case pour lancer une analyse détaillée.

### 4. 📊 Volatilité (Analyse Technique)
*Pour analyser le comportement structurel d'une paire.*
*   **Fonctionnement :** Sélectionnez une paire (ex: EURUSD). L'appli analyse chaque heure de la journée.
*   **Résultat :**
    *   **Tableau Horaire :** Volatilité moyenne, bruit, mouvements pour chaque heure.
    *   **Analyse Bidi :** Cliquez sur une heure pour voir l'analyse bidirectionnelle détaillée (probabilités de mouvement haussier vs baissier).

### 5. 🧪 Backtest & Archives
*Pour valider et sauvegarder.*
*   **Archives :** Retrouvez toutes vos analyses sauvegardées depuis l'onglet Planning ou Volatilité.
*   **Backtest :** Rejouez les événements passés tick par tick avec vos paramètres (Offset, SL, TP) pour vérifier la robustesse de la stratégie (Win Rate, Drawdown, Equity Curve).

### 6. 🖨️ Exports
*Pour générer vos fiches de trading.*
*   Générez des rapports PDF professionnels incluant :
    *   Les paramètres de trading (Entrée, SL, TP).
    *   Les statistiques de volatilité.
    *   Le classement des meilleures opportunités.

---

## 📥 Importation de Données

Pour fonctionner, l'application a besoin de deux types de données :

1.  **Données de Prix (Bougies M1) :**
    *   Fichiers CSV exportés depuis MT4/MT5 ou Dukascopy.
    *   Format attendu : Date, Open, High, Low, Close, Volume.
    *   *Astuce :* L'application nettoie et convertit automatiquement les formats courants.

2.  **Calendriers Économiques (CSV) :**
    *   **Source recommandée :** Forex Factory.
    *   **Format :** Date, Time, Currency, Impact, Event Name.
    *   *Workflow :* Importez un gros historique une fois pour toutes, puis mettez à jour le "Planning Hebdo" chaque semaine.

---

## 🚀 Installation & Démarrage

### Prérequis
- **Node.js** (v18+)
- **Rust** (v1.70+)
- **Tauri CLI**

### Commandes
```bash
# Installation des dépendances
npm install

# Lancement en mode développement
npm run tauri dev

# Compilation pour production
npm run tauri build
```

---

## 🛡️ Confidentialité
Cette application fonctionne **100% en local**. Aucune donnée (ni vos CSV, ni vos analyses) n'est envoyée sur un serveur externe. Tout est stocké dans une base de données SQLite sur votre machine.
