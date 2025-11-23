# 📋 TÂCHES RESTANTES - Optimisation Stratégie Straddle

**Date**: 23 novembre 2025  
**Status**: TÂCHE 1-4 ✅ COMPLÉTÉES  
**Objectif**: Implémenter les améliorations post-TÂCHE 4  
**Priorité**: Haute → Moyenne → Basse

---

## ✅ TÂCHES COMPLÉTÉES (À NE PAS REFAIRE)

### ✅ TÂCHE 1: Supprimer Volume Imbalance
**Status**: FAIT (commit cfac358)  
**Remplacée par**: Direction Strength = (body_range_mean × breakout_percentage) / 100

### ✅ TÂCHE 2: Corriger le Calcul du Stop Loss  
**Status**: FAIT (commit cfac358)  
**Implémentation**: SL adaptatif = ATR × noiseFactor, où noiseFactor = max(0.6, min(0.9, 1.0 - noise_ratio/10))

### ✅ TÂCHE 3: Remplacer Range par True Range
**Status**: FAIT (commit cfac358)  
**Implémentation**: Range → True Range + ATR Wilder's Smoothing

### ✅ TÂCHE 4: Implémenter le Calcul Réel de Durée de Volatilité
**Status**: FAIT (commits 9c508a7, e0551b2, 1010f4b)  
**Implémentation**: ATR Wilder's EMA decay analysis + affichage UI (Peak, Half-Life, Trade Exp)

---

## 🟠 PRIORITÉ HAUTE (Améliorations importantes)

### 🎯 TÂCHE 5: Ajouter Métriques Manquantes Critiques pour Straddle

#### 5.1 - Offset Optimal Calculé
**Objectif**: Calculer la distance minimale pour éviter 95% des fausses mèches  
**Fichiers à créer/modifier**:
- `src-tauri/src/services/volatility/offset_calculator.rs` (nouveau)
- `src/utils/straddleAnalysis.ts` - Ajouter affichage

**Algorithme**:
```rust
pub fn calculate_optimal_offset(candles: &[Candle]) -> f64 {
    // 1. Calculer toutes les mèches (wick size)
    let wicks: Vec<f64> = candles
        .iter()
        .map(|c| {
            let body = (c.close - c.open).abs();
            let upper_wick = c.high - c.close.max(c.open);
            let lower_wick = c.open.min(c.close) - c.low;
            upper_wick.max(lower_wick)
        })
        .collect();
    
    // 2. Calculer le percentile 95
    let mut sorted_wicks = wicks.clone();
    sorted_wicks.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let p95_index = (sorted_wicks.len() as f64 * 0.95) as usize;
    let offset_95 = sorted_wicks[p95_index];
    
    // 3. Ajouter marge de sécurité (10%)
    offset_95 * 1.1
}
```

**Estimation**: 2-3 heures  
**Validation**: Vérifier que offset calculé > ATR × 0.75 actuel

---

#### 5.2 - Win Rate Simulé
**Objectif**: Backtester des Straddles avec différents offsets pour calculer le win rate réel  
**Fichiers à créer/modifier**:
- `src-tauri/src/services/volatility/win_rate_calculator.rs` (nouveau)
- `src/components/MetricsAnalysisModal.vue` - Afficher win rate

**Algorithme**:
```rust
pub fn simulate_straddle_win_rate(
    candles: &[Candle],
    event_times: &[NaiveDateTime],
    offset_pips: f64,
) -> f64 {
    let mut wins = 0;
    let mut total = 0;
    
    for event_time in event_times {
        // Trouver la bougie au moment de l'événement
        let entry_candle = find_candle_at(candles, event_time);
        let entry_price = entry_candle.close;
        
        let buy_stop = entry_price + offset_pips;
        let sell_stop = entry_price - offset_pips;
        
        // Analyser les 15 minutes suivantes
        let next_15min = get_candles_after(candles, event_time, 15);
        
        // Vérifier déclenchement
        let buy_triggered = next_15min.iter().any(|c| c.high >= buy_stop);
        let sell_triggered = next_15min.iter().any(|c| c.low <= sell_stop);
        
        // Whipsaw = perte
        if buy_triggered && sell_triggered {
            total += 1;
            continue;
        }
        
        // Un seul déclenché = vérifier profit
        if buy_triggered {
            let max_profit = next_15min.iter().map(|c| c.high).max().unwrap() - buy_stop;
            if max_profit > offset_pips { wins += 1; }
            total += 1;
        } else if sell_triggered {
            let max_profit = sell_stop - next_15min.iter().map(|c| c.low).min().unwrap();
            if max_profit > offset_pips { wins += 1; }
            total += 1;
        }
    }
    
    if total == 0 { 0.0 } else { wins as f64 / total as f64 }
}
```

**Estimation**: 4-5 heures  
**Validation**: Win rate doit être entre 40-70% pour être réaliste

---

#### 5.3 - Fréquence Whipsaw
**Objectif**: Mesurer le % de fois où les 2 ordres sont déclenchés (perte garantie)  
**Fichiers à créer/modifier**:
- `src-tauri/src/services/volatility/whipsaw_detector.rs` (nouveau)

**Algorithme**:
```rust
pub fn calculate_whipsaw_frequency(
    candles: &[Candle],
    event_times: &[NaiveDateTime],
    offset_pips: f64,
) -> f64 {
    let mut whipsaw_count = 0;
    let mut total = 0;
    
    for event_time in event_times {
        let entry_candle = find_candle_at(candles, event_time);
        let entry_price = entry_candle.close;
        
        let buy_stop = entry_price + offset_pips;
        let sell_stop = entry_price - offset_pips;
        
        let next_15min = get_candles_after(candles, event_time, 15);
        
        let buy_triggered = next_15min.iter().any(|c| c.high >= buy_stop);
        let sell_triggered = next_15min.iter().any(|c| c.low <= sell_stop);
        
        if buy_triggered && sell_triggered {
            whipsaw_count += 1;
        }
        total += 1;
    }
    
    if total == 0 { 0.0 } else { whipsaw_count as f64 / total as f64 }
}
```

**Estimation**: 2 heures  
**Validation**: Whipsaw < 20% = bon, > 40% = mauvais setup

---

## 🟡 PRIORITÉ MOYENNE (Optimisations)

### 🔄 TÂCHE 6: Fusionner Tick Quality et Body Range
**Problème**: Redondance conceptuelle  
**Impact**: Simplifie l'interface  
**Fichiers à modifier**:
- `src-tauri/src/services/volatility/hourly_stats.rs` - Retirer `tick_quality_mean`
- `src/components/HourlyTable.vue` - Retirer colonne
- `src/components/AnalysisPanel.vue` - Vérifier usage

**Alternative**: Renommer Tick Quality en "Body Size Moyen" si on veut garder la métrique absolue

**Estimation**: 1 heure  
**Validation**: Vérifier que Body Range % suffit pour les analyses

---

### 📐 TÂCHE 7: Améliorer la Formule de Trade Duration
**Problème**: Formule actuelle ignore event_type et hour_of_day  
**Impact**: Durée peut être sous-optimale  
**Fichiers à modifier**:
- `src/utils/straddleAnalysis.ts` - Fonction `calculateTradeDuration()`

**Nouvelle formule**:
```typescript
function calculateTradeDuration(
  atrMean: number,
  eventType: string,
  hourOfDay: number
): number {
  // Base duration from ATR
  let baseDuration = 240; // 4h default
  if (atrMean > 50) baseDuration = 120;
  else if (atrMean > 40) baseDuration = 150;
  else if (atrMean > 25) baseDuration = 180;
  
  // Adjust for event type
  const eventFactors: Record<string, number> = {
    'NFP': 0.5,           // Pic court, intense
    'CPI': 0.7,           // Pic moyen
    'Interest Rate': 0.8, // Pic long
    'GDP': 1.0,           // Pic très long
  };
  const eventFactor = eventFactors[eventType] || 1.0;
  
  // Adjust for hour of day
  const hourFactors: Record<number, number> = {
    8: 0.8,  // London open - pic court
    13: 0.6, // NY open - pic très court
    14: 0.7, // Overlap - pic court
    // Autres heures: 1.0 (normal)
  };
  const hourFactor = hourFactors[hourOfDay] || 1.0;
  
  return Math.round(baseDuration * eventFactor * hourFactor);
}
```

**Estimation**: 2-3 heures  
**Validation**: Comparer durées calculées avec observations empiriques

---

### 🎨 TÂCHE 8: Améliorer l'Affichage des Tooltips des Métriques
**Problème**: Certains tooltips manquent d'exemples concrets  
**Impact**: Utilisateur ne comprend pas bien les métriques  
**Fichiers à modifier**:
- `src/components/AnalysisPanel.vue` - Enrichir tooltips

**Exemple d'amélioration**:
```html
<template #usage>
  <div class="tooltip-section-title">📊 Interprétation</div>
  <div class="tooltip-section-text">
    <strong>Noise Ratio < 2.0</strong> : Excellent (peu de mèches)<br/>
    Exemple: Range 20 pips, Body 15 pips → Noise = 1.33 ✅<br/><br/>
    
    <strong>Noise Ratio 2.0-3.0</strong> : Acceptable<br/>
    Exemple: Range 20 pips, Body 8 pips → Noise = 2.5 ⚠️<br/><br/>
    
    <strong>Noise Ratio > 3.0</strong> : Danger (fausses mèches)<br/>
    Exemple: Range 20 pips, Body 4 pips → Noise = 5.0 ❌
  </div>
</template>
```

**Estimation**: 2 heures  
**Validation**: Tester avec utilisateur final

---

## 🔵 PRIORITÉ BASSE (Nice to have)

### 📊 TÂCHE 9: Ajouter Graphique de Décroissance de Volatilité
**Objectif**: Visualiser comment la volatilité décroît après le pic  
**Impact**: Aide à comprendre la durée optimale  
**Fichiers à créer/modifier**:
- `src/components/VolatilityDecayChart.vue` (nouveau)
- `src/components/MetricsAnalysisModal.vue` - Intégrer graphique

**Estimation**: 4-5 heures  
**Validation**: Graphique doit montrer clairement le pic et la décroissance

---

### 🔍 TÂCHE 10: Ajouter Filtre par Type d'Événement dans Heatmap
**Objectif**: Filtrer heatmap pour voir uniquement NFP, CPI, etc.  
**Impact**: Facilite l'analyse ciblée  
**Fichiers à modifier**:
- `src/components/EventCorrelationHeatmap.vue` - Ajouter dropdown filtre

**Estimation**: 2 heures  
**Validation**: Filtre fonctionne correctement

---

### 📈 TÂCHE 11: Exporter Paramètres Bidi en JSON
**Objectif**: Permettre export direct des paramètres pour le robot  
**Impact**: Automatisation complète  
**Fichiers à créer/modifier**:
- `src/utils/straddleAnalysis.ts` - Fonction `exportBidiConfig()`
- `src/components/BidiParametersModal.vue` - Bouton export

**Format JSON**:
```json
{
  "symbol": "EURUSD",
  "event_name": "NFP",
  "event_time": "2025-11-22T14:29:50Z",
  "entry_offset_pips": 12,
  "stop_loss_pips": 11,
  "take_profit_pips": 33,
  "trailing_stop_multiplier": 2.0,
  "trade_duration_minutes": 150,
  "confidence_score": 78.5,
  "win_rate_estimated": 0.65
}
```

**Estimation**: 2 heures  
**Validation**: JSON valide et importable par robot Bidi

---

## 📝 RÉCAPITULATIF DES PRIORITÉS

| Priorité | Tâches | Estimation Totale |
|----------|--------|-------------------|
| 🟠 **HAUTE** | 5.1, 5.2, 5.3 | 8-10 heures |
| 🟡 **MOYENNE** | 6, 7, 8 | 5-6 heures |
| 🔵 **BASSE** | 9, 10, 11 | 8-9 heures |

**TOTAL ESTIMÉ**: 21-25 heures de développement

---

## ✅ CRITÈRES DE VALIDATION GLOBALE

Avant de considérer l'application "production-ready" :

1. ✅ **Aucune métrique fictive** (Volume Imbalance supprimé)
2. ✅ **Stop Loss adaptatif** (prend en compte Noise Ratio)
3. ✅ **Durée calculée réellement** (pas d'heuristique)
4. ⏳ **Offset optimal calculé** (percentile 95 des mèches)
5. ⏳ **Win Rate affiché** (backtest réel)
6. ⏳ **Whipsaw détecté** (< 20% acceptable)
7. ✅ **Interface claire** (pas de doublons)
8. ⏳ **Export JSON** (paramètres Bidi)

---

## 🚀 ORDRE D'EXÉCUTION RECOMMANDÉ

### Sprint 1 (Haute - 2-3 jours)
- 5.1 Offset Optimal
- 5.2 Win Rate Simulé
- 5.3 Whipsaw Frequency

### Sprint 2 (Moyenne - 1 jour)
- 6 Fusionner Tick Quality
- 7 Améliorer Trade Duration
- 8 Enrichir Tooltips

### Sprint 3 (Basse - 1-2 jours)
- 9 Graphique décroissance
- 10 Filtre événements
- 11 Export JSON

**TOTAL**: 4-6 jours de développement
