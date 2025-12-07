# 🔧 Feuille de Route: Migration Pips → Points MT5

## 📋 Contexte
L'application actuelle calcule les ATR et les paramètres Straddle en **PIPS** (supposément), mais:
1. Les données source (crypto, indices) sont déjà en **POINTS** (unité MT5)
2. La multiplication par 10000 crée des valeurs aberrantes (ex: 664415 au lieu de 664)
3. MetaTrader 5 (MQL5) raisonne nativement en **POINTS**, pas en PIPS

**Solution**: Raisonner **systématiquement en POINTS MT5** pour:
- Forex: EURUSD ATR=12 points (0.0012)
- Crypto: BTCUSD ATR=650 points (650 unités)
- Indices: SP500 ATR=150 points (150 unités)

---

## 🎯 Objectif
Convertir tous les calculs de **PIPS → POINTS** et mettre à jour:
1. Les parsers de données (import)
2. Les calculs de statistiques
3. Les formules Straddle
4. L'affichage UI (labels + unités)
5. La documentation (formules.ts)

---

## 📊 Fichiers à Modifier (Ordre de Priorité)

### 🔴 PRIORITÉ HAUTE - Parsers & Imports

#### 1. `src/composables/useArchiveParsers.ts`
**Problème**: 
- Ligne 50-52: Multiplication par 10000 pour convertir "décimales en pips"
- Ligne 65-67: `peakAtrDecimal * 10000` crée des valeurs énormes
- Ligne 127: Commentaire "volatilityValue est déjà en pips" (FAUX pour crypto)

**À faire**:
- [ ] Supprimer la multiplication par 10000 dans `parseRetrospectiveArchive()`
- [ ] Remplacer `peakAtrPips` par `peakAtrPoints`
- [ ] Utiliser directement: `peakAtrPoints = peakAtrDecimal` (ou selon format source)
- [ ] Mettre à jour commentaires pour clarifier "points MT5"
- [ ] Tester avec données réelles (BTC, EURUSD, etc.)

**Code à changer**:
```typescript
// AVANT:
const peakAtrPips = peakAtrDecimal * 10000

// APRÈS:
const peakAtrPoints = peakAtrDecimal // Valeur déjà en points MT5
```

---

### 🟠 PRIORITÉ HAUTE - Calculs de Statistiques

#### 2. `src/composables/useArchiveMetrics.ts`
**Problème**:
- Ligne 137: `stat.slAdjusted = Math.round((stat.avgATR * 1.5) * 10) / 10`
- Formule SL correcte en points: `SL = ATR × 1.5` (simple)
- Ligne 140: Appel à `calculateTrailingStop(stat.avgATR)`

**À faire**:
- [ ] Renommer `avgATR` → `avgATRPoints` (clarté)
- [ ] Simplifier SL: `slAdjusted = Math.round(avgATRPoints * 1.5)`
- [ ] Pas de multiplication supplémentaire par 10
- [ ] Vérifier calcul Trailing Stop (voir #3)

---

#### 3. `src/composables/useTrailingStopCalculation.ts`
**Problème**:
- Ligne 10: Documentation dit "retourne distance en pips"
- Fonction suppose ATR en pips, mais reçoit maintenant des points

**À faire**:
- [ ] Mettre à jour commentaires: "retourne coefficient en points"
- [ ] Vérifier formule: `TS = ATR × 0.75 × (1 + whipsaw × 0.3)`
- [ ] Aucune conversion supplémentaire nécessaire
- [ ] Ajouter clarification: "Fonctionne directement avec points MT5"

---

### 🟠 PRIORITÉ HAUTE - Calculs Straddle

#### 4. `src/utils/straddleAnalysis.ts`
**Problème**:
- Ligne 41: `slPips: Math.round((bestSlice.atr_mean * 1.5) * 10000)`
- Ligne 42: `tpPips: Math.round((bestSlice.atr_mean * 2.5) * 10000)`
- Multiplication par 10000 crée des aberrations

**À faire**:
- [ ] Renommer `slPips` → `slPoints`
- [ ] Renommer `tpPips` → `tpPoints`
- [ ] Supprimer multiplication par 10000
- [ ] Recalculer: `slPoints = Math.round(atr_mean × 1.5)`
- [ ] Recalculer: `tpPoints = Math.round(atr_mean × 2.5)`

**Code à changer**:
```typescript
// AVANT:
slPips: Math.round((bestSlice.atr_mean * 1.5) * 10000),
tpPips: Math.round((bestSlice.atr_mean * 2.5) * 10000),

// APRÈS:
slPoints: Math.round(bestSlice.atr_mean * 1.5),
tpPoints: Math.round(bestSlice.atr_mean * 2.5),
```

---

#### 5. `src/utils/straddleCalculators.ts`
**Problème**:
- Ligne 161: `slPips: Math.round(slPoints * 10000)`
- Ligne 164: `tpPips: Math.round(tpPoints * 10000)`
- Confusion entre variables `slPoints` (points) et `slPips` (faux)

**À faire**:
- [ ] Renommer toutes les variables `*Pips` → `*Points`
- [ ] Supprimer les multiplications par 10000
- [ ] Vérifier logique: `slPoints` reste `slPoints` en output
- [ ] Mettre à jour interface retour

---

#### 6. `src/utils/straddleTypes.ts`
**Problème**:
- Ligne 17: Interface `SliceAnalysis` avec `slPips`, `tpPips`
- Ligne 27: Interface `TradingRecommendation` avec `slPips`, `tpPips`
- Confusion terminologie

**À faire**:
- [ ] Renommer tous les champs `*Pips` → `*Points`
- [ ] Mettre à jour docs: "Tous les paramètres en POINTS MetaTrader 5"
- [ ] Garder `slUsd`, `tpUsd`, `tpRatio` (inchangés)
- [ ] Propager changement dans toutes les interfaces

---

### 🟡 PRIORITÉ MOYENNE - Composants & Affichage

#### 7. `src/composables/useMetricsModalLoad.ts`
**Problème**:
- Ligne 30: `offset_pips: number`
- Ligne 33, 51: `sl_adjusted_pips: number`
- Noms trompeurs en points réels

**À faire**:
- [ ] Renommer `offset_pips` → `offset_points`
- [ ] Renommer `sl_adjusted_pips` → `sl_adjusted_points`
- [ ] Mettre à jour logique: pas de conversion additionnelle

---

#### 8. `src/components/metrics/BidiParametersSection.vue`
**Problème**:
- Ligne 50: Affiche `sl_adjusted_pips` avec label "pips"
- Ligne 75: Affiche Trailing Stop avec label "pips"
- Ligne 161: Calcul `atr = sl_adjusted_pips / 1.5`

**À faire**:
- [ ] Renommer prop/affichage: `sl_adjusted_points`
- [ ] Changer label: "points" au lieu de "pips"
- [ ] Mettre à jour calcul: `atr = sl_adjusted_points / 1.5`

---

#### 9. `src/components/analysis/EventGroupedByColor.vue`
**Problème**:
- Affiche ATR avec label "Volatilité ATR" et unit "p" (pips)
- Ligne 171: `Math.round(event.stats.avgATR * 10) / 10`

**À faire**:
- [ ] Mettre à jour label UI: "Volatilité ATR (points)"
- [ ] Simplifier affichage: `Math.round(avgATR * 10) / 10` reste correct
- [ ] Ajouter clarification tooltip: "Volatilité en points MetaTrader 5"

---

### 🟡 PRIORITÉ MOYENNE - Analyse Rétrospective

#### 10. `src/components/RetroAnalysisResults.vue`
**Problème**:
- Ligne 49: "Taux: pips/min"
- Ligne 68: "Taux de décroissance: pips/min"
- Supposément en pips, mais données source déjà en points

**À faire**:
- [ ] Renommer label: "Taux de décroissance: **points/min**"
- [ ] Mettre à jour SVG text: "Taux: X points/min"
- [ ] Vérifier source données `decayRate`

---

#### 11. `src/components/RetroactiveAnalysisView.vue`
**Problème**:
- Ligne 42: Passe `decay_rate_pips_per_minute` au composant

**À faire**:
- [ ] Renommer prop: `decay_rate_points_per_minute`
- [ ] Vérifier source dans Rust backend

---

#### 12. `src/components/RetroactiveAnalysisResultsViewer.vue`
**Problème**:
- Ligne 32: Interface avec `decay_rate_pips_per_minute`

**À faire**:
- [ ] Renommer champ: `decay_rate_points_per_minute`

---

### 🟢 PRIORITÉ BASSE - Documentation & Données

#### 13. `src/data/formules.ts`
**Problème**:
- Ligne 104, 106: Exemples et unités en "pips"
- Ligne 125, 127: Autres exemples en "pips"
- Documentation obsolète

**À faire**:
- [ ] Remplacer tous "pips" → "points"
- [ ] Mettre à jour exemples:
  - "EURUSD M1: ATR = 12.5 **points** (volatilité moyenne)"
  - "Hour 12:00-12:59: Range = 45 **points**"
- [ ] Ajouter clarification: "Points = unité native MetaTrader 5"

---

## 🧪 Plan de Test

Après chaque modification:
1. **Affichage UI**: Vérifier que les valeurs affichées sont réalistes
   - BTC: 500-2000 points ✅
   - EURUSD: 10-100 points ✅
   - SP500: 100-500 points ✅

2. **Calculs Straddle**: Vérifier SL/TP corrects
   - `SL = ATR × 1.5` (en points)
   - `TP = ATR × 2.5` (en points)

3. **Base de données**: Vérifier format d'export
   - Valeurs en points, pas en pips
   - Pas de multiplication par 10000

4. **Integration MT5**: Tester avec Bidi robot
   - Paramètres attendus en points
   - Pas de conversion supplémentaire nécessaire

---

## 📌 Checklist de Validation

- [ ] Tous les fichiers parseurs mis à jour
- [ ] Tous les calculs SL/TP corrigés
- [ ] Interfaces TypeScript renommées
- [ ] UI affiche "points" au lieu de "pips"
- [ ] Documentation mise à jour
- [ ] Tests d'affichage (valeurs réalistes)
- [ ] Tests Straddle (SL/TP corrects)
- [ ] Commit + Push

---

## 🚀 Étapes d'Exécution

**Phase 1 (Parsers)**: #1-2 → Données correctes dès l'import
**Phase 2 (Calculs)**: #3-6 → Formules correctes
**Phase 3 (UI)**: #7-12 → Affichage cohérent
**Phase 4 (Docs)**: #13 → Documentation à jour
**Phase 5 (Tests)**: Plan de test complet

---

## 📝 Notes Importantes

1. **NE PAS utiliser `* 10000`** nulle part
2. **Les points MT5 sont l'unité universelle** - pas de conversion
3. **Vérifier les données source** - déjà en points pour crypto/indices
4. **Mettre à jour tous les labels UI** - "pips" → "points"
5. **Commenter clairement** - "Points MetaTrader 5" dans chaque fonction

---

**Dernière mise à jour**: 7 décembre 2025
**Status**: À faire
**Impact**: CRITIQUE - Correction des valeurs aberrantes
