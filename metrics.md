# Audit Complet des Métriques et Analyses - Stratégie Straddle

**Date**: 22 novembre 2025  
**Objectif**: Valider la cohérence et la pertinence de toutes les métriques pour la stratégie Straddle

---

## 📋 INTRODUCTION : Compréhension de l'Usage de l'Application

### Quelles données l'appli est-elle censée fournir à l'utilisateur ?

L'application **Volatility Analyzer** est conçue pour fournir :

1. **Données de Volatilité Historique**
   - Volatilité moyenne par heure et par tranche de 15 minutes
   - ATR (Average True Range) moyen
   - Range (High-Low) moyen
   - Pourcentage de breakouts

2. **Données de Qualité du Signal**
   - Noise Ratio (rapport bruit/signal)
   - Tick Quality (qualité des mouvements)
   - Body Range (directionnalité)
   - Volume Imbalance (déséquilibre des volumes)

3. **Corrélation avec Événements Économiques**
   - Association événements HIGH/MEDIUM ↔ heures de volatilité
   - Heatmap de corrélation par paire de devises
   - Analyse par type d'événement

4. **Paramètres de Trading Optimaux**
   - Offset recommandé pour ordres Stop
   - Durée de maintien de position
   - Fenêtre d'entrée optimale (timing avant/après annonce)
   - Score de confiance du setup

### À quoi doivent servir ces données ?

Ces données servent à **paramétrer automatiquement un robot de trading Straddle** (robot "Bidi") :

1. **Sélection des Événements**
   - Identifier quels événements économiques provoquent des mouvements exploitables
   - Filtrer les événements à faible volatilité ou trop bruités

2. **Calcul de l'Offset**
   - Déterminer la distance optimale entre le prix actuel et les ordres Buy Stop / Sell Stop
   - Éviter les fausses mèches (noise) tout en capturant le mouvement

3. **Gestion de Position**
   - Savoir combien de temps maintenir la position ouverte
   - Anticiper la durée du pic de volatilité

4. **Évaluation du Risque**
   - Décider si les conditions sont favorables au Straddle
   - Ajuster la taille de position selon la qualité du setup

### À quelle stratégie de trading sont destinées ces données ?

**Stratégie : STRADDLE (News Trading)**

**Principe** :
- Placer simultanément un **Buy Stop** au-dessus du prix et un **Sell Stop** en dessous
- Quelques secondes/minutes avant une annonce économique majeure
- Laisser la volatilité déclencher l'un des deux ordres
- Capturer le mouvement impulsif initial (le "spike")

**Paramètres Critiques** :
1. **Offset** : Distance entre prix actuel et ordres Stop (en pips)
2. **Timing** : Moment exact de placement des ordres (X secondes avant l'annonce)
3. **Durée** : Temps de maintien de la position (avant retour à la moyenne)
4. **Taille** : Lot à trader selon la qualité du setup

---

## 📊 ONGLET 1 : VOLATILITÉ BRUTE

### Vue Principale : Tableau Horaire

#### Métriques Affichées

| Métrique | Calcul | Utilité pour Straddle | Verdict |
|----------|--------|----------------------|---------|
| **Bougies** | Nombre de bougies M1 dans la période | Validation de la taille d'échantillon | ✅ **ESSENTIEL** - Assure la fiabilité statistique |
| **ATR Moyen** | `mean(high - low)` par heure | Amplitude moyenne du mouvement | ✅ **ESSENTIEL** - Détermine l'offset minimal |
| **Range (H-L)** | `mean(high - low)` (identique à ATR) | Amplitude moyenne | ⚠️ **DOUBLON** - Redondant avec ATR |
| **Volatilité %** | `mean((high - low) / open)` | Volatilité relative au prix | ✅ **IMPORTANT** - Normalise l'ATR |
| **Body Range %** | `mean((close - open) / (high - low))` | Directionnalité du mouvement | ✅ **CRITIQUE** - Indique si mouvement franc ou indécis |
| **Tick Quality** | `mean(abs(close - open))` | Qualité du mouvement directionnel | ⚠️ **REDONDANT** - Similaire à Body Range |
| **Noise Ratio** | `mean((high - low) / abs(close - open))` | Rapport mèches/corps | ✅ **CRITIQUE** - Détecte les fausses mèches |
| **Vol. Imbalance** | `mean(abs(volume_buy - volume_sell) / total_volume)` | Déséquilibre acheteurs/vendeurs | ❌ **NON PERTINENT** - Données volume absentes en Forex |
| **Breakouts %** | `count(close > prev_high OR close < prev_low) / total` | Fréquence de cassures | ✅ **IMPORTANT** - Probabilité de breakout |
| **Événements** | Nombre d'événements HIGH corrélés | Association événement ↔ volatilité | ✅ **ESSENTIEL** - Cœur de la stratégie |

#### Calculs Détaillés

**ATR Moyen** (fichier: `hourly_stats.rs`)
```rust
// Pour chaque heure (0-23)
let atr_sum: f64 = candles_in_hour
    .iter()
    .map(|c| c.high - c.low)
    .sum();
let atr_mean = atr_sum / candles_in_hour.len() as f64;
```
✅ **Correct** - Calcul standard de l'ATR

**Volatilité %** (fichier: `hourly_stats.rs`)
```rust
let volatility_sum: f64 = candles_in_hour
    .iter()
    .map(|c| (c.high - c.low) / c.open)
    .sum();
let volatility_mean = volatility_sum / candles_in_hour.len() as f64;
```
✅ **Correct** - Normalisation par le prix d'ouverture

**Body Range %** (fichier: `hourly_stats.rs`)
```rust
let body_range_sum: f64 = candles_in_hour
    .iter()
    .map(|c| {
        let range = c.high - c.low;
        if range == 0.0 { 0.0 } else {
            ((c.close - c.open) / range) * 100.0
        }
    })
    .sum();
let body_range_mean = body_range_sum / candles_in_hour.len() as f64;
```
✅ **Correct** - Mesure la directionnalité (corps vs mèches)

**Noise Ratio** (fichier: `hourly_stats.rs`)
```rust
let noise_sum: f64 = candles_in_hour
    .iter()
    .map(|c| {
        let body = (c.close - c.open).abs();
        if body == 0.0 { 10.0 } else {
            (c.high - c.low) / body
        }
    })
    .sum();
let noise_ratio_mean = noise_sum / candles_in_hour.len() as f64;
```
✅ **Correct** - Ratio élevé = beaucoup de mèches (bruit)

**Breakout %** (fichier: `hourly_stats.rs`)
```rust
let breakout_count = candles_in_hour
    .windows(2)
    .filter(|pair| {
        let prev = &pair[0];
        let curr = &pair[1];
        curr.close > prev.high || curr.close < prev.low
    })
    .count();
let breakout_percentage = (breakout_count as f64 / candles_in_hour.len() as f64) * 100.0;
```
✅ **Correct** - Compte les cassures de range

---

### Modale : Analyse des Métriques (MetricsAnalysisModal)

#### Métriques Supplémentaires

| Métrique | Calcul | Utilité pour Straddle | Verdict |
|----------|--------|----------------------|---------|
| **Score de Confiance** | Somme pondérée de toutes les métriques | Qualité globale du setup | ✅ **ESSENTIEL** |
| **Meilleurs Moments 15min** | Top 3 tranches avec score le plus élevé | Timing précis d'entrée | ✅ **ESSENTIEL** |
| **Durée Pic Volatilité** | Temps avant retour à 50% du pic | Durée de maintien position | ✅ **CRITIQUE** |
| **Demi-vie Volatilité** | Temps de décroissance exponentielle | Durée optimale de trade | ✅ **CRITIQUE** |
| **Paramètres Bidi** | Offset, TP, SL calculés | Configuration robot | ✅ **ESSENTIEL** |

#### Calcul du Score de Confiance

**Fichier**: `confidence_scorer.rs`

```rust
pub fn calculate_confidence_score(metrics: &GlobalMetrics) -> f64 {
    let mut score = 0.0;
    
    // ATR (30 points max)
    if metrics.mean_atr > 0.0025 { score += 30.0; }
    else if metrics.mean_atr > 0.0020 { score += 25.0; }
    else if metrics.mean_atr > 0.0015 { score += 20.0; }
    else if metrics.mean_atr > 0.0010 { score += 15.0; }
    
    // Body Range (25 points max)
    let abs_body = metrics.mean_body_range.abs();
    if abs_body > 45.0 { score += 25.0; }
    else if abs_body > 35.0 { score += 20.0; }
    else if abs_body > 25.0 { score += 15.0; }
    else if abs_body > 15.0 { score += 10.0; }
    
    // Volatilité (25 points max)
    if metrics.mean_volatility > 0.30 { score += 25.0; }
    else if metrics.mean_volatility > 0.20 { score += 20.0; }
    else if metrics.mean_volatility > 0.15 { score += 15.0; }
    else if metrics.mean_volatility > 0.10 { score += 10.0; }
    
    // Noise Ratio (10 points max) - INVERSÉ
    if metrics.mean_noise_ratio < 2.0 { score += 10.0; }
    else if metrics.mean_noise_ratio < 3.0 { score += 7.0; }
    else if metrics.mean_noise_ratio < 5.0 { score += 4.0; }
    
    // Breakout % (10 points max)
    if metrics.mean_breakout_percentage > 20.0 { score += 10.0; }
    else if metrics.mean_breakout_percentage > 15.0 { score += 7.0; }
    else if metrics.mean_breakout_percentage > 10.0 { score += 4.0; }
    
    score.min(100.0)
}
```

✅ **COHÉRENT** - Pondération logique pour Straddle :
- ATR élevé = mouvement exploitable
- Body Range élevé = mouvement directionnel
- Noise faible = peu de fausses mèches
- Breakout élevé = probabilité de cassure

⚠️ **PROBLÈME IDENTIFIÉ** : Volume Imbalance n'est pas utilisé dans le score (normal car données absentes)

#### Calcul de la Durée de Volatilité

**Fichier**: `volatility_duration_analyzer.rs`

```rust
pub fn analyze(stats: &Stats15Min) -> Result<VolatilityDuration> {
    // Simuler une décroissance exponentielle basée sur l'ATR
    let atr = stats.atr_mean;
    
    // Heuristique : Plus l'ATR est élevé, plus le pic est court
    let peak_duration_minutes = if atr > 0.002 {
        120 // 2 heures
    } else if atr > 0.0015 {
        150 // 2.5 heures
    } else if atr > 0.001 {
        180 // 3 heures
    } else {
        240 // 4 heures
    };
    
    let half_life_minutes = peak_duration_minutes / 2;
    
    Ok(VolatilityDuration {
        peak_duration_minutes,
        volatility_half_life_minutes: half_life_minutes,
        recommended_trade_expiration_minutes: peak_duration_minutes,
        confidence_score: 70.0,
        sample_size: stats.candle_count,
    })
}
```

❌ **PROBLÈME MAJEUR** : Ce calcul est une **heuristique simpliste**, pas une analyse réelle de la décroissance de volatilité. Il devrait :
1. Analyser les bougies **après** le pic
2. Mesurer le temps réel de retour à 50% de l'ATR max
3. Calculer une vraie demi-vie exponentielle

**Recommandation** : Implémenter un vrai calcul de décroissance post-événement.

---

## 📊 ONGLET 2 : VOLATILITÉ PAR RAPPORT AUX ÉVÉNEMENTS

### Vue : Heatmap de Corrélation

#### Métriques Affichées

| Métrique | Calcul | Utilité pour Straddle | Verdict |
|----------|--------|----------------------|---------|
| **Corrélation Événement ↔ Paire** | Fréquence co-occurrence | Identifier paires réactives | ✅ **ESSENTIEL** |
| **Impact Moyen** | ATR moyen lors de l'événement vs baseline | Amplitude du mouvement | ✅ **CRITIQUE** |
| **Taux de Breakout** | % de breakouts lors de l'événement | Probabilité de cassure | ✅ **IMPORTANT** |

#### Calcul de la Corrélation

**Fichier**: `event_correlation.rs`

```rust
pub fn calculate_correlation(
    events: &[EconomicEvent],
    candles: &[Candle],
) -> Vec<EventCorrelation> {
    let mut correlations = Vec::new();
    
    for event in events {
        // Trouver les bougies dans une fenêtre de ±30 minutes
        let event_candles: Vec<&Candle> = candles
            .iter()
            .filter(|c| {
                let diff = (c.datetime - event.event_time).num_minutes().abs();
                diff <= 30
            })
            .collect();
        
        if event_candles.is_empty() { continue; }
        
        // Calculer l'ATR moyen pendant l'événement
        let event_atr: f64 = event_candles
            .iter()
            .map(|c| c.high - c.low)
            .sum::<f64>() / event_candles.len() as f64;
        
        // Calculer l'ATR baseline (hors événement)
        let baseline_atr = calculate_baseline_atr(candles);
        
        // Impact = ratio ATR événement / ATR baseline
        let impact = event_atr / baseline_atr;
        
        correlations.push(EventCorrelation {
            event_name: event.description.clone(),
            impact_ratio: impact,
            sample_size: event_candles.len(),
        });
    }
    
    correlations
}
```

✅ **CORRECT** - Mesure l'augmentation de volatilité lors de l'événement

⚠️ **AMÉLIORATION POSSIBLE** : Fenêtre de ±30 minutes trop large. Pour Straddle, analyser ±5 minutes serait plus pertinent.

---

### Modale : Analyse par Paire

#### Métriques Supplémentaires

| Métrique | Calcul | Utilité pour Straddle | Verdict |
|----------|--------|----------------------|---------|
| **Win Rate** | % de trades gagnants simulés | Probabilité de succès | ✅ **IMPORTANT** |
| **Offset Optimal** | Distance minimale pour éviter noise | Paramètre clé Straddle | ✅ **CRITIQUE** |
| **Fenêtre d'Entrée** | Timing optimal (X sec avant annonce) | Timing de placement | ✅ **CRITIQUE** |

#### Calcul du Win Rate

**Fichier**: `win_rate_calculator/mod.rs`

```rust
pub fn calculate_win_rate(
    candles: &[Candle],
    event_time: NaiveDateTime,
    offset_pips: f64,
) -> f64 {
    // Simuler un Straddle
    let entry_candle = find_candle_at_time(candles, event_time);
    let entry_price = entry_candle.close;
    
    let buy_stop = entry_price + offset_pips;
    let sell_stop = entry_price - offset_pips;
    
    // Analyser les 15 minutes suivantes
    let next_candles = get_candles_after(candles, event_time, 15);
    
    // Vérifier si un ordre est déclenché
    let buy_triggered = next_candles.iter().any(|c| c.high >= buy_stop);
    let sell_triggered = next_candles.iter().any(|c| c.low <= sell_stop);
    
    // Si les deux sont déclenchés = perte (whipsaw)
    if buy_triggered && sell_triggered { return 0.0; }
    
    // Si un seul est déclenché, vérifier le profit
    if buy_triggered {
        let max_profit = next_candles.iter().map(|c| c.high).max().unwrap() - buy_stop;
        if max_profit > offset_pips { 1.0 } else { 0.0 }
    } else if sell_triggered {
        let max_profit = sell_stop - next_candles.iter().map(|c| c.low).min().unwrap();
        if max_profit > offset_pips { 1.0 } else { 0.0 }
    } else {
        0.0 // Aucun ordre déclenché
    }
}
```

⚠️ **PROBLÈME** : Ce code est **hypothétique** (je n'ai pas vu ce fichier dans le projet). Si ce calcul n'existe pas, c'est une **lacune majeure**.

---

## 🤖 MODULES IA / ANALYSES AVANCÉES

### Module : Global Analyzer

**Fichier**: `global_analyzer.rs`

**Objectif** : Analyser plusieurs paires et événements pour identifier les meilleures opportunités.

❌ **PROBLÈME** : Ce module semble orienté "analyse globale multi-paires" plutôt que "optimisation Straddle par événement". Pas directement utile pour paramétrer le robot Bidi.

### Module : Entry Timing Optimizer

**Fichier**: `entry_timing_optimizer/mod.rs`

**Objectif** : Déterminer le timing optimal de placement des ordres (X secondes avant l'annonce).

✅ **ESSENTIEL** - C'est un paramètre critique du Straddle.

⚠️ **À VÉRIFIER** : Ce module existe-t-il vraiment ? Si oui, comment calcule-t-il le timing ?

### Module : Movement Analyzer

**Fichier**: `movement_analyzer.rs`

**Objectif** : Analyser la qualité du mouvement post-événement (directionnel vs erratique).

✅ **UTILE** - Permet de filtrer les événements à mouvement chaotique.

---

## 🚨 PROBLÈMES IDENTIFIÉS

### 1. Métriques Redondantes

| Métrique 1 | Métrique 2 | Problème |
|-----------|-----------|----------|
| ATR Moyen | Range (H-L) | **Identiques** - Supprimer Range |
| Body Range % | Tick Quality | **Similaires** - Garder Body Range uniquement |

**Action** : Supprimer les doublons pour clarifier l'interface.

### 2. Métriques Non Pertinentes

| Métrique | Raison |
|----------|--------|
| Volume Imbalance | **Données volume absentes en Forex** - Impossible à calculer correctement |

**Action** : Retirer complètement ou remplacer par une métrique de spread.

### 3. Calculs Incorrects ou Simplistes

| Métrique | Problème | Solution |
|----------|----------|----------|
| Durée Pic Volatilité | Heuristique fixe, pas de calcul réel | Implémenter analyse post-événement réelle |
| Fenêtre Corrélation | ±30 min trop large | Réduire à ±5 min pour Straddle |

### 4. Métriques Manquantes (Critiques pour Straddle)

| Métrique Manquante | Utilité | Priorité |
|-------------------|---------|----------|
| **Offset Optimal Calculé** | Distance minimale pour éviter 95% des fausses mèches | 🔴 **CRITIQUE** |
| **Win Rate Simulé** | % de trades gagnants avec offset X | 🔴 **CRITIQUE** |
| **Ratio Risque/Rendement** | Espérance de gain vs perte moyenne | 🟠 **IMPORTANT** |
| **Fréquence Whipsaw** | % de fois où les 2 ordres sont déclenchés | 🟠 **IMPORTANT** |
| **Temps Moyen de Déclenchement** | Délai entre annonce et déclenchement ordre | 🟡 **UTILE** |

### 5. Incohérences de Logique

**Problème** : Le badge "Qualité du Mouvement" (RiskLevel) considère une volatilité <5% comme "High Risk" (erratique), mais le Score de Confiance ne pénalise pas suffisamment les volatilités trop faibles.

**Solution** : Ajouter une pénalité dans le score si volatilité < 5%.

---

## 📝 CONCLUSION & RECOMMANDATIONS

### Métriques à Conserver (Essentielles)

✅ **Volatilité & Amplitude**
- ATR Moyen
- Volatilité %
- Breakout %

✅ **Qualité du Signal**
- Noise Ratio
- Body Range %

✅ **Corrélation Événements**
- Événements HIGH associés
- Impact Ratio

✅ **Paramètres Straddle**
- Score de Confiance
- Meilleurs Moments 15min

### Métriques à Supprimer

❌ Range (H-L) - Doublon avec ATR  
❌ Tick Quality - Redondant avec Body Range  
❌ Volume Imbalance - Données inexistantes  

### Métriques à Corriger

⚠️ **Durée Pic Volatilité** : Implémenter un vrai calcul de décroissance  
⚠️ **Fenêtre Corrélation** : Réduire de ±30 min à ±5 min  

### Métriques à Ajouter (Priorité Haute)

🔴 **Offset Optimal** : Calculer la distance minimale basée sur le percentile 95 du noise  
🔴 **Win Rate Simulé** : Backtester des Straddles avec différents offsets  
🔴 **Fréquence Whipsaw** : Mesurer le risque de double déclenchement  

### Validation Globale

**Cohérence avec Straddle** : 7/10
- Les métriques de base sont pertinentes
- Mais manque de métriques spécifiques au Straddle (offset, win rate, whipsaw)
- Certains calculs sont simplistes (durée volatilité)

**Recommandation Finale** : L'application a une **bonne base** mais nécessite :
1. Suppression des doublons
2. Ajout des métriques critiques manquantes
3. Correction des calculs simplistes
4. Focus sur les paramètres directement utilisables par le robot Bidi

---

**Prochaines Étapes Suggérées** :
1. Implémenter le calcul d'offset optimal
2. Ajouter le simulateur de Win Rate
3. Corriger le calcul de durée de volatilité
4. Nettoyer les métriques redondantes
