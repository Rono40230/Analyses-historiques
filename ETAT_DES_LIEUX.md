# 📊 ÉTAT DES LIEUX - Analyses Historiques Volatilité

**Date:** 8 novembre 2025  
**Version Application:** 0.1.0  
**Stack:** Tauri 2.0 + Rust + Vue 3 + TypeScript + SQLite

---

## 🎯 Vue d'ensemble

L'application actuelle permet d'analyser la volatilité historique des paires Forex, cryptos et indices en croisant :
- Données de prix OHLC (minute par minute)
- Calendrier économique (événements et leur impact)
- Sessions de trading Forex (Tokyo, Londres, New York, Sydney)

---

## 📁 Architecture Technique

### Structure des données

**Sources de données :**
- **CSV Paires** : `data/csv/[SYMBOL]_M1_[DATE-RANGE].csv` (OHLCV minute)
- **Calendrier SQLite** : `volatility.db` avec table `calendar_events`
- **Stockage local** : `~/.local/share/volatility-analyzer/`

### Modules principaux

```
src-tauri/
├── commands/          # Interfaces Tauri (<200L par fichier)
│   ├── volatility_commands.rs
│   ├── session_commands.rs
│   ├── file_management_commands.rs
│   ├── pair_data_commands.rs
│   └── ...
├── services/          # Logique métier (<300L par fichier)
│   ├── volatility/
│   │   ├── analyzer.rs
│   │   ├── hourly_stats.rs
│   │   ├── metrics.rs
│   │   └── ...
│   ├── session_analyzer.rs
│   ├── pair_data_stats.rs
│   └── ...
├── models/            # Structures de données (<150L)
│   ├── hourly_stats.rs
│   ├── analysis_result.rs
│   └── ...
└── db/                # Base SQLite
```

---

## 📈 MÉTRIQUES CALCULÉES ACTUELLEMENT

### 1️⃣ Analyse Volatilité Brute (Onglet "Volatilité")

**Commande:** `analyze_symbol(symbol: String)`

**Métriques HourlyStats** (par heure UTC 0-23) :
| Métrique | Description | Utilité Robot Straddle |
|----------|-------------|------------------------|
| `hour` | Heure UTC (0-23) | Timing placement positions |
| `candle_count` | Nombre de bougies | Fiabilité statistique |
| `atr_mean` | ATR moyen (période 14) | **✅ Base calcul SL/TP** |
| `atr_max` | ATR maximum observé | Worst case scenario |
| `volatility_mean` | Volatilité moyenne (période 20) | Amplitude mouvement attendue |
| `range_mean` | Range moyen (High-Low) | Mouvement brut en pips |
| `body_range_mean` | Taille corps bougie moyenne | Force du mouvement directionnel |
| `shadow_ratio_mean` | Ratio mèches/corps | Indécision vs direction |
| `tick_quality_mean` | Qualité tick (bougie complète) | Fiabilité données |
| `volume_imbalance_mean` | Déséquilibre volume | Pression achat/vente |
| `noise_ratio_mean` | Ratio bruit/signal | **⚠️ Fausses cassures** |
| `breakout_percentage` | % breakouts (>P80 ATR) | **✅ Probabilité mouvement fort** |

**Métriques Globales (GlobalMetrics) :**
- `mean_atr` : ATR moyen toutes heures confondues
- `mean_volatility` : Volatilité moyenne globale
- `mean_body_range` : Taille corps moyenne
- `mean_tick_quality` : Qualité moyenne données
- `mean_noise_ratio` : Niveau bruit moyen
- `mean_volume_imbalance` : Déséquilibre volume moyen
- `total_candles` : Nombre total bougies analysées

**Scores calculés :**
- `confidence_score` : Score confiance analyse (0-100)
- `quality_score` : Score qualité par heure (0-100)
- `best_hours` : Top 5 heures plus volatiles

**Recommandations générées :**
- `recommendation` : "Trade/Observe/Avoid"
- `risk_level` : "Low/Medium/High"

**✅ CE QUI EST EXPLOITABLE :**
- ATR moyen par heure → Base calcul SL
- Meilleures heures → Timing robot
- Breakout % → Probabilité mouvement

**❌ CE QUI MANQUE :**
- Pas de distinction volatilité "normale" vs "événementielle"
- Pas de durée mouvement après pic
- Pas de ratio gain/perte
- ATR non contextualisé par type d'événement

---

### 2️⃣ Analyse par Sessions (Onglet "Sessions")

**Commande:** `analyze_sessions(pair_symbol: String)`

**Métriques SessionStats** (par session Forex) :
| Métrique | Description | Utilité Robot Straddle |
|----------|-------------|------------------------|
| `name` | Nom session (Tokyo/Londres/NY/Sydney) | Identification période |
| `icon` | Emoji pays | UX uniquement |
| `paris_hours` | Horaires Paris (HH:MM-HH:MM) | Timing local user |
| `avg_volatility` | Volatilité moyenne (pips) | **✅ Amplitude attendue** |
| `percentage` | % temps total | Couverture temporelle |
| `candle_count` | Nombre bougies | Fiabilité statistique |

**Métriques Overlaps (Chevauchements) :**
| Métrique | Description | Utilité Robot Straddle |
|----------|-------------|------------------------|
| `name` | "Tokyo + Londres", "Londres + New York" | Identification zone |
| `paris_hours` | Horaires Paris | Timing local |
| `avg_volatility` | Volatilité moyenne (pips) | **✅ Pics de volatilité** |
| `volatility_multiplier` | Multiplicateur vs volatilité normale | **✅ Boost événement** |

**Métriques CalendarCorrelation :**
| Métrique | Description | Utilité Robot Straddle |
|----------|-------------|------------------------|
| `session` | Nom session | Identification |
| `high_impact_events` | Nombre événements HIGH | **✅ Fréquence événements majeurs** |
| `event_volatility` | Volatilité moyenne lors événements | **✅ Impact événements** |
| `impact_percentage` | % volatilité due aux événements | Importance événements |

**Recommandations générées :**
- Meilleure session à trader (plus volatile)
- Session à éviter (moins volatile)
- Importance des zones de chevauchement

**✅ CE QUI EST EXPLOITABLE :**
- Sessions les plus volatiles → Timing robot
- Chevauchements = pics volatilité → Opportunités Straddle
- Corrélation événements/sessions → Filtrage timing

**❌ CE QUI MANQUE :**
- Pas de durée moyenne volatilité par session
- Pas de taux succès par session
- Pas d'ATR spécifique session

---

### 3️⃣ Corrélation Événements (Onglet "Événements")

**Commande:** `correlate_events_with_volatility(pair_symbol: String)`

**Métriques CorrelatedEvent :**
| Métrique | Description | Utilité Robot Straddle |
|----------|-------------|------------------------|
| `event` | Objet CalendarEvent complet | Identification événement |
| `event.impact` | "HIGH", "MEDIUM", "LOW" | **✅ Filtrage impact** |
| `event.description` | Type événement (NFP, CPI, etc.) | **✅ Identification pattern** |
| `volatility_hour` | Heure UTC événement | Timing précis |
| `volatility_increase` | Augmentation volatilité (%) | **✅ Amplitude boost** |
| `correlation_score` | Score corrélation (0-1) | Force lien événement/volatilité |

**CalendarEvent (structure complète) :**
```rust
pub struct CalendarEvent {
    pub id: i32,
    pub symbol: String,           // Devise concernée
    pub event_time: String,       // Timestamp événement
    pub impact: String,           // HIGH/MEDIUM/LOW
    pub description: String,      // Type événement
    pub actual: Option<f64>,      // Valeur réelle
    pub forecast: Option<f64>,    // Valeur prévue
    pub previous: Option<f64>,    // Valeur précédente
    pub created_at: String,
}
```

**✅ CE QUI EST EXPLOITABLE :**
- Liste événements avec impact mesuré
- Filtrage par impact (HIGH/MEDIUM/LOW)
- Corrélation score → Fiabilité événement

**❌ CE QUI MANQUE (CRITIQUE) :**
- ⚠️ **Pas de durée mouvement post-événement**
- ⚠️ **Pas de ratio gagnant/perdant**
- ⚠️ **Pas de fenêtre d'entrée optimale** (-15min, -5min, etc.)
- ⚠️ **Pas d'ATR contextualisé par événement**
- ⚠️ **Pas de détection faux événements** (annoncés mais sans mouvement)
- ⚠️ **Pas de score de tradabilité** par événement
- ⚠️ **Pas de recommandation SL/TP** par événement

---

## 🗂️ DONNÉES STOCKÉES

### Base SQLite (`volatility.db`)

**Table `calendar_events` :**
```sql
CREATE TABLE calendar_events (
    id INTEGER PRIMARY KEY,
    symbol TEXT NOT NULL,
    event_time TEXT NOT NULL,
    impact TEXT,              -- HIGH/MEDIUM/LOW
    description TEXT,
    actual REAL,
    forecast REAL,
    previous REAL,
    created_at TEXT
);
```

**Indexes :**
- Index sur `symbol`
- Index sur `event_time`
- Index sur `impact`

---

## 🎨 INTERFACE UTILISATEUR

### Onglets disponibles

1. **Calendrier Économique** : Import/gestion fichiers calendar
2. **Données de Paires** : Import/gestion fichiers CSV paires
3. **Volatilité brute d'un actif** : Analyse horaire (HourlyStats)
4. **Volatilité d'un actif par rapport aux événements économiques** : Corrélation
5. **Volatilité d'un actif par rapport aux ouvertures boursières** : Sessions

### Fonctionnalités UX

- Import automatique avec nettoyage CSV européens
- Statistiques globales (banner vert)
- Sélection paire via dropdown
- Tableaux tri/filtrage
- Export pas encore implémenté ❌

---

## 🔍 ANALYSES DISPONIBLES PAR ONGLET

### Onglet 1 : "Volatilité brute"
**Objectif :** Identifier les heures naturellement volatiles

**Données affichées :**
- Tableau 24 heures avec stats complètes
- Top 5 meilleures heures
- Score de confiance global
- Recommandation trade/observe/avoid

**Utilité Straddle :**
- ✅ Timing placement positions (meilleures heures)
- ✅ ATR moyen pour SL de base
- ⚠️ Manque contexte événementiel

---

### Onglet 2 : "Événements économiques"
**Objectif :** Mesurer impact événements historiques

**Données affichées :**
- Liste événements corrélés avec volatilité
- Augmentation % volatilité
- Score corrélation
- Filtrage par impact

**Utilité Straddle :**
- ✅ Identification événements impactants
- ✅ Filtrage HIGH/MEDIUM/LOW
- ⚠️ Manque durée mouvement
- ⚠️ Manque timing d'entrée optimal
- ⚠️ Manque ATR spécifique événement

---

### Onglet 3 : "Sessions boursières"
**Objectif :** Identifier sessions et chevauchements volatiles

**Données affichées :**
- Volatilité par session (Tokyo, Londres, NY, Sydney)
- Volatilité zones chevauchement (Tokyo+Londres, Londres+NY)
- Corrélation événements/sessions
- Recommandations timing

**Utilité Straddle :**
- ✅ Meilleure session = meilleur timing
- ✅ Chevauchements = opportunités maximales
- ✅ Corrélation avec événements
- ⚠️ Manque stats événements PAR session

---

## 📊 RÉSUMÉ : CE QUI EXISTE vs CE QUI MANQUE

### ✅ MÉTRIQUES EXISTANTES EXPLOITABLES

| Métrique | Disponible | Utilisable Robot | Suffisant ? |
|----------|-----------|------------------|-------------|
| ATR moyen horaire | ✅ | ✅ Oui | ⚠️ Pas contextualisé |
| Meilleures heures | ✅ | ✅ Oui | ✅ OK |
| Breakout % | ✅ | ✅ Oui | ✅ OK |
| Sessions volatiles | ✅ | ✅ Oui | ✅ OK |
| Chevauchements | ✅ | ✅ Oui | ✅ OK |
| Impact événements | ✅ | ✅ Oui (filtrage) | ⚠️ Incomplet |
| Corrélation événements | ✅ | ⚠️ Partiel | ❌ Insuffisant |

---

### ❌ MÉTRIQUES MANQUANTES CRITIQUES

| Métrique Manquante | Priorité | Nécessaire pour |
|-------------------|----------|-----------------|
| **Durée volatilité post-événement** | 🔴 HAUTE | TradeExpiration optimal |
| **Ratio gagnant/perdant par événement** | 🔴 HAUTE | Filtrage événements |
| **Fenêtre d'entrée optimale** | 🔴 HAUTE | EntryMinutesBeforeEvent |
| **ATR contextualisé par événement** | 🔴 HAUTE | ATRMultiplier adapté |
| **Score tradabilité événement** | 🔴 HAUTE | Filtrage auto + Blacklist |
| **Détection faux événements** | 🔴 HAUTE | AvoidFakeEvents |
| **Mouvement moyen par événement (pips)** | 🔴 HAUTE | Calcul TP |
| **Taux succès historique par événement** | 🔴 HAUTE | MinHistoricalSuccessRate |
| **Breakout confirmation patterns** | 🟡 MOYENNE | UseBreakoutConfirmation |
| **Multi-timeframe convergence** | 🟡 MOYENNE | Fiabilité signal |
| **Patterns saisonniers** | 🟡 MOYENNE | Validation EventMonths |
| **Corrélation inter-paires** | 🟢 BASSE | Hedging |

---

## 🎯 PROCHAINES ÉTAPES

### Phase 0 : Validation (EN COURS)
✅ État des lieux terminé

### Phase 1 : Métriques Critiques (PRIORITÉ 1)
Les 6 métriques manquantes à développer en priorité :

1. **Durée volatilité post-événement**
   - Calculer durée pic (en minutes)
   - Temps retour à normale
   - Par type d'événement

2. **Ratio gagnant/perdant**
   - % mouvements directionnels >X pips
   - % whipsaw (aller-retours)
   - Score qualité mouvement

3. **Fenêtre d'entrée optimale**
   - Tester -60min, -30min, -15min, -5min
   - Identifier meilleur timing
   - Par type d'événement

4. **ATR contextualisé**
   - ATR 30min avant événement
   - ATR 30min après événement
   - Ratio augmentation

5. **Score tradabilité**
   - Note globale 0-10
   - Recommandation TRADE/CAUTION/AVOID
   - Liste blanche/noire auto

6. **Détection faux événements**
   - Événements annoncés mais sans mouvement
   - % échec par type
   - Blacklist automatique

### Phase 2 : Interface Export (PRIORITÉ 2)
- Dashboard "Configuration Robot"
- Export JSON/CSV paramètres optimaux
- Format structuré par paire/événement

### Phase 3 : Analyses Avancées (PRIORITÉ 3)
- Multi-timeframe
- Patterns saisonniers
- Backtesting visuel

---

## 💡 RECOMMANDATIONS IMMÉDIATES

### Pour commencer Phase 1 :

**1. Service `event_metrics.rs`** (nouveau) :
```rust
pub struct EventMetrics {
    pub event_type: String,
    pub avg_duration_minutes: f64,
    pub success_rate: f64,
    pub avg_movement_pips: f64,
    pub whipsaw_rate: f64,
    pub best_entry_minutes_before: i32,
    pub contextual_atr_before: f64,
    pub contextual_atr_after: f64,
    pub tradability_score: f64,
}
```

**2. Commande `calculate_event_metrics`** :
- Analyser tous événements historiques
- Mesurer volatilité avant/pendant/après
- Calculer ratios et scores
- Stocker dans DB (cache)

**3. Nouvelle table SQLite `event_metrics`** :
```sql
CREATE TABLE event_metrics (
    event_type TEXT PRIMARY KEY,
    avg_duration_minutes REAL,
    success_rate REAL,
    avg_movement_pips REAL,
    whipsaw_rate REAL,
    best_entry_minutes INTEGER,
    atr_before REAL,
    atr_after REAL,
    tradability_score REAL,
    last_calculated TEXT
);
```

---

## 📌 NOTES TECHNIQUES

### Contraintes .clinerules (RESPECTÉES) ✅
- Commands : 183L / 200L max ✅
- Services : 287L / 300L max ✅
- Models : 125L / 150L max ✅
- Pas de `unwrap()` ni `expect()` ✅

### Performance actuelle
- Chargement CSV : ~2-3s pour 1M lignes
- Analyse volatilité : ~1-2s
- Analyse sessions : ~1-2s
- Corrélation événements : ~0.5s

### Points d'attention
- Import CSV européens (virgules) : nettoyage automatique ✅
- Timeframe fixe M1 (minute)
- Pas de support multi-paires simultanées (pour l'instant)

---

**FIN DE L'ÉTAT DES LIEUX**

Ce document servira de référence pour toutes les phases de développement à venir.
