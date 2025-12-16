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

L'application est organisée en 5 onglets principaux suivant le flux de travail logique d'un trader.

### 1. 🔥 Heatmap de Corrélation (Vue d'ensemble)
*C'est le point de départ pour identifier les opportunités de la semaine.*
*   **Fonctionnement :** Affiche une matrice visuelle (Événements x Paires).
*   **Lecture :**
    *   🟥 **Rouge** : Impact violent, forte volatilité.
    *   🟩 **Vert** : Impact faible ou nul.
*   **Action :** Cliquez sur une case "chaude" pour voir les détails de l'impact historique de cet événement sur cette paire.

### 2. 📊 Volatilité Brute (Analyse Technique)
*Pour analyser le comportement d'une paire indépendamment des news.*
*   **Fonctionnement :** Sélectionnez une paire (ex: EURUSD). L'appli analyse chaque heure de la journée sur l'historique complet.
*   **Résultat :**
    *   **Tableau Horaire :** Montre la volatilité moyenne, le bruit et les mouvements pour chaque heure (00h-23h).
    *   **Recommandation :** Identifie les "Golden Hours" (meilleures heures pour trader) et les zones de danger.

### 3. 📊 Corrélation Événementielle (Analyse Fondamentale)
*Le cœur du système pour le News Trading.*
*   **Workflow :**
    1.  Choisissez un événement (ex: "Non-Farm Employment Change").
    2.  L'appli charge toutes les occurrences passées de cet événement.
    3.  Elle superpose les graphiques M1 (1 minute) pour montrer la "signature" moyenne de l'événement.
*   **Métriques Clés :**
    *   **Pic de volatilité :** Combien de minutes après l'annonce le mouvement max se produit-il ?
    *   **Directionnalité :** Est-ce que ça part tout droit ou est-ce que ça hésite ?

### 4. 🧪 Backtest (Simulation)
*Pour valider votre stratégie avant de risquer un centime.*
*   **Workflow :**
    1.  Configurez vos paramètres (Offset, SL, TP, Trailing Stop).
    2.  Lancez la simulation sur l'historique.
    3.  L'appli "rejoue" chaque événement passé tick par tick.
*   **Résultat :**
    *   **Win Rate :** Taux de réussite théorique.
    *   **Drawdown :** Pire perte historique.
    *   **Equity Curve :** Courbe de progression du capital.

### 5. 🗄️ Archives & Exports
*Pour sauvegarder et partager votre travail.*
*   **Archives :** Sauvegardez vos analyses prometteuses pour les retrouver plus tard.
*   **Exports PDF :** Générez des rapports professionnels :
    *   *Fiche Paramètres :* Les réglages exacts à copier dans votre plateforme de trading.
    *   *Blacklist :* Les événements à bannir absolument.
    *   *Rapport de Backtest :* Preuve de performance de la stratégie.

---

## 📥 Importation de Données

Pour fonctionner, l'application a besoin de carburant (données) :
1.  **Données de Prix (Bougies) :** Fichiers CSV exportés depuis MT4/MT5 ou Dukascopy (Format OHLCV).
2.  **Calendrier Économique :** Fichier CSV contenant l'historique des annonces économiques (Date, Heure, Impact, Devise).

*L'onglet "Importer" permet de charger, nettoyer et stocker ces données dans la base locale sécurisée.*

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
