# 🗺️ ROADMAP - Analyses Historiques Volatilité

## 🎯 Objectif Global
Transformer l'application en outil d'aide à la décision pour optimiser les paramètres d'un robot de trading Straddle sur le Forex, en analysant les patterns de volatilité déclenchés par les événements économiques passés.

---

## 📊 PHASE 0 : État des lieux (EN COURS)

### Objectif
Documenter exhaustivement toutes les fonctionnalités existantes, métriques calculées et analyses disponibles.

### Tâches
- [ ] Inventaire complet des métriques calculées actuellement
- [ ] Liste des analyses disponibles par onglet
- [ ] Identification des données manquantes pour l'optimisation Straddle
- [ ] Cartographie de l'architecture des services/commandes
- [ ] Documentation des structures de données existantes

### Livrable
- Document `ETAT_DES_LIEUX.md` avec analyse complète de l'existant

---

## 🔍 PHASE 1 : Métriques Événementielles Avancées

### 1.1 - Durée de volatilité post-événement
**Problème :** Actuellement on ne sait pas combien de temps dure le mouvement après un événement.

**Solution :**
- Calculer la durée moyenne du pic de volatilité (en minutes)
- Identifier le temps moyen avant retour à la normale
- Mesurer la "demi-vie" de la volatilité (temps pour revenir à 50% de l'ATR normal)

**Métriques à ajouter :**
```rust
pub struct EventVolatilityDuration {
    pub event_type: String,
    pub avg_peak_duration_minutes: f64,
    pub avg_return_to_normal_minutes: f64,
    pub volatility_half_life_minutes: f64,
    pub max_observed_duration: i32,
    pub min_observed_duration: i32,
}
```

**Impact paramètres EA :**
- `TradeExpiration` optimal par événement
- `UseEventBasedExpiration = true`

---

### 1.2 - Ratio gagnant/perdant & détection fausses cassures
**Problème :** Tous les événements ne génèrent pas de mouvements directionnels exploitables.

**Solution :**
- Calculer le % d'événements ayant généré un mouvement > seuil dans UNE direction
- Détecter les aller-retours (whipsaw) : mouvement > X pips puis retour complet
- Score de "qualité du mouvement" (0-10) par type d'événement

**Métriques à ajouter :**
```rust
pub struct EventMovementQuality {
    pub event_type: String,
    pub directional_move_rate: f64,        // % mouvements clairs (>70% = bon)
    pub whipsaw_rate: f64,                 // % fausses cassures
    pub avg_pips_moved: f64,               // Mouvement moyen en pips
    pub success_rate: f64,                 // % événements "tradables"
    pub quality_score: f64,                // Score 0-10
    pub sample_size: i32,
}
```

**Impact paramètres EA :**
- `MinHistoricalSuccessRate` : Filtrer événements < 65%
- `AvoidFakeEvents = true` pour whipsaw_rate > 30%
- `EventBlacklist` : Liste des événements à éviter

---

### 1.3 - Fenêtre d'entrée optimale
**Problème :** On ne sait pas quand entrer exactement (avant l'événement).

**Solution :**
- Analyser performance d'entrée à -60min, -30min, -15min, -5min, -1min
- Calculer le meilleur timing par type d'événement
- Identifier si entrée anticipée ou dernière minute est meilleure

**Métriques à ajouter :**
```rust
pub struct OptimalEntryWindow {
    pub event_type: String,
    pub best_entry_minutes_before: i32,    // -15, -30, etc.
    pub avg_profit_at_best_entry: f64,     // R moyen
    pub worst_entry_minutes_before: i32,
    pub entry_windows_analysis: Vec<EntryWindowStat>,
}

pub struct EntryWindowStat {
    pub minutes_before: i32,
    pub avg_profit_r: f64,
    pub success_rate: f64,
    pub sample_size: i32,
}
```

**Impact paramètres EA :**
- `EntryMinutesBeforeEvent` optimal par événement
- `UseSmartTiming = true`

---

### 1.4 - ATR contextualisé par événement
**Problème :** L'ATR actuel ne différencie pas volatilité normale vs événementielle.

**Solution :**
- Calculer ATR moyen 30min AVANT chaque type d'événement
- Calculer ATR moyen 30min APRÈS chaque type d'événement
- Ratio ATR_après/ATR_avant (ex: NFP = +250%, CPI = +120%)

**Métriques à ajouter :**
```rust
pub struct ContextualATR {
    pub event_type: String,
    pub atr_before_event: f64,             // ATR moyen 30min avant
    pub atr_during_event: f64,             // ATR moyen 30min après
    pub atr_increase_ratio: f64,           // Ratio après/avant
    pub recommended_sl_multiplier: f64,    // Multiplicateur optimal
    pub recommended_tp_multiplier: f64,    // Pour TP
}
```

**Impact paramètres EA :**
- `UseEventContextualATR = true`
- `ATRMultiplier` adapté par événement
- `TakeProfitRatio` optimisé

---

### 1.5 - Filtrage et scoring événements
**Problème :** Pas de système de notation pour prioriser les événements.

**Solution :**
- Score de "tradabilité" global (0-10) par événement
- Prise en compte : impact annoncé, mouvement historique, taux succès, qualité mouvement
- Classification : Liste Blanche (>7/10), Liste Grise (4-7/10), Liste Noire (<4/10)

**Métriques à ajouter :**
```rust
pub struct EventTradabilityScore {
    pub event_type: String,
    pub overall_score: f64,                // Score 0-10
    pub impact_level: String,              // "High", "Medium", "Low"
    pub historical_movement_score: f64,    // Basé sur pips moyens
    pub reliability_score: f64,            // Basé sur taux succès
    pub recommendation: String,            // "TRADE", "CAUTION", "AVOID"
    pub reasons: Vec<String>,
}
```

**Impact paramètres EA :**
- `OnlyTradeHighImpact = true`
- `MinHistoricalSuccessRate` par défaut
- Génération `EventBlacklist` automatique

---

## 📈 PHASE 2 : Analyses Avancées

### 2.1 - Analyse de convergence multi-timeframe
**Objectif :** Identifier si plusieurs timeframes confirment le signal.

**Solution :**
- Analyser corrélation événement sur M1, M5, M15, M30 simultanément
- Détecter si volatilité est cohérente sur tous timeframes
- Score de "convergence" pour fiabilité du signal

**Impact paramètres EA :**
- `UseVolatilityFilter = true` activé seulement si convergence
- `ATRTimeframe` optimal recommandé

---

### 2.2 - Patterns saisonniers et cycliques
**Objectif :** Détecter si certains événements sont plus/moins tradables selon le mois/semaine.

**Solution :**
- Analyse par mois (éviter juillet/août ?)
- Analyse par semaine du mois (semaine 1 vs semaine 4)
- Patterns jour de la semaine

**Impact paramètres EA :**
- Validation du paramètre `EventMonths = "SansJuilAou"`
- Optimisation `EventWeeks` par événement

---

### 2.3 - Corrélation inter-paires
**Objectif :** Identifier les paires qui bougent ensemble lors d'un événement.

**Solution :**
- Matrice de corrélation par événement (EUR/USD vs GBP/USD, etc.)
- Détecter opportunités de hedging
- Recommander paires alternatives si paire principale peu liquide

**Impact paramètres EA :**
- Recommandations de paires à trader par événement
- Stratégies de couverture

---

## 🎨 PHASE 3 : Interface & Visualisation

### 3.1 - Dashboard "Configuration Robot"
**Objectif :** Vue dédiée pour consulter paramètres optimaux.

**Composants :**
- Sélecteur de paire
- Sélecteur d'événement
- Affichage paramètres recommandés
- Graphiques de performance historique simulée

### 3.2 - Export paramètres optimaux
**Objectif :** Générer fichier de consultation pour l'utilisateur.

**Formats :**
- JSON structuré par paire/événement
- CSV pour Excel
- PDF rapport complet avec graphiques

**Exemple JSON :**
```json
{
  "analysis_date": "2025-11-08",
  "pairs": {
    "EURUSD": {
      "events": {
        "NFP": {
          "recommendation": "TRADE",
          "tradability_score": 8.7,
          "optimal_params": {
            "EntryMinutesBeforeEvent": 15,
            "ATRMultiplier": 2.5,
            "TakeProfitRatio": 3.2,
            "TradeExpiration": 120,
            "HistoricalSuccessRate": 78
          },
          "statistics": {
            "avg_movement_pips": 45,
            "volatility_duration_min": 118,
            "whipsaw_rate": 12
          }
        }
      }
    }
  }
}
```

### 3.3 - Backtesting visuel
**Objectif :** Simuler l'EA avec paramètres recommandés sur historique.

**Fonctionnalités :**
- Ligne de temps avec entrées/sorties simulées
- Courbe d'équité
- Statistiques de performance
- Comparaison paramètres actuels vs optimaux

---

## 🔧 PHASE 4 : Optimisation & Raffinement

### 4.1 - Machine Learning (optionnel)
**Objectif :** Prédire la qualité d'un événement à venir.

**Approche :**
- Entraîner modèle sur historique
- Features : heure, jour, mois, impact annoncé, contexte marché
- Prédiction : score de tradabilité, mouvement attendu

### 4.2 - Alertes temps réel
**Objectif :** Notifier l'utilisateur des événements à venir avec paramètres.

**Fonctionnalités :**
- Import calendrier économique à venir
- Calcul paramètres recommandés
- Export fichier "événements de la semaine"

### 4.3 - Suivi performance EA réelle
**Objectif :** Comparer paramètres utilisés vs résultats réels.

**Fonctionnalités :**
- Import historique trades MT4/MT5
- Comparaison résultats réels vs attendus
- Ajustement recommandations

---

## 📅 Planning Estimé

| Phase | Durée estimée | Priorité |
|-------|---------------|----------|
| Phase 0 - État des lieux | 1 session | 🔴 CRITIQUE |
| Phase 1.1 - Durée volatilité | 2-3 sessions | 🔴 HAUTE |
| Phase 1.2 - Ratio gagnant/perdant | 2-3 sessions | 🔴 HAUTE |
| Phase 1.3 - Fenêtre entrée | 2-3 sessions | 🔴 HAUTE |
| Phase 1.4 - ATR contextuel | 2 sessions | 🔴 HAUTE |
| Phase 1.5 - Scoring événements | 2 sessions | 🔴 HAUTE |
| Phase 2.1 - Multi-timeframe | 2 sessions | 🟡 MOYENNE |
| Phase 2.2 - Patterns saisonniers | 2 sessions | 🟡 MOYENNE |
| Phase 2.3 - Corrélation paires | 2 sessions | 🟡 MOYENNE |
| Phase 3.1 - Dashboard config | 3-4 sessions | 🔴 HAUTE |
| Phase 3.2 - Export paramètres | 1-2 sessions | 🔴 HAUTE |
| Phase 3.3 - Backtesting visuel | 3-4 sessions | 🟡 MOYENNE |
| Phase 4.x - Optimisations | Variable | 🟢 BASSE |

---

## 🎯 Prochaines Actions

1. ✅ **Compléter Phase 0** - État des lieux exhaustif
2. **Prioriser Phase 1** - Métriques événementielles critiques
3. **Développer Phase 3.2** - Export paramètres (quick win)
4. **Itérer** - Tester avec données réelles, ajuster

---

## 📝 Notes Importantes

### Architecture technique
- Tous les calculs doivent être dans les **services** (respect .clinerules)
- Commands Tauri = interfaces minimalistes
- Stocker résultats dans SQLite pour éviter recalculs
- Cache intelligent pour performances

### Tests & Validation
- Valider chaque métrique sur plusieurs paires
- Comparer résultats avec analyses manuelles
- Tester avec plusieurs périodes historiques (2024, 2023, etc.)

### Documentation
- Documenter chaque nouvelle métrique
- Exemples d'utilisation pour chaque paramètre EA
- Guide utilisateur complet

---

**Version:** 1.0  
**Date création:** 8 novembre 2025  
**Auteur:** GitHub Copilot  
**Statut:** 🚧 EN DÉVELOPPEMENT
