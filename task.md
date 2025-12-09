# 📋 TASK: Graphique Volatilité Comparative (COMPLÉTÉ ✅)

## ✅ RÉSUMÉ FINAL (Phase 12)

### Status: FRONTEND IMPLÉMENTATION TERMINÉE

**Date**: 9 décembre 2025  
**Commits**: Phase 1→12 (backend redesign → frontend integration)

---

## 🎯 OBJECTIF PRINCIPAL

Afficher l'impact d'un type d'événement économique sur la volatilité d'une paire Forex en:
1. Comparant la volatilité **AVANT** l'événement (T-30 à T0)
2. Comparant la volatilité **APRÈS** l'événement (T0 à T+90)
3. Quantifiant l'impact en **% d'augmentation** (volatility_increase_percent)
4. Évaluant la **qualité du mouvement** (clean vs choppy) via Noise Ratio

---

## 📊 ARCHITECTURE FINALE

### Backend (Rust) ✅

**Commande**: `analyze_volatility_profile`
- **Entrée**: pair (String), eventType (String)
- **Logique**:
  1. Charge ALL candles M1 pour la période du calendrier
  2. Trouve ALL occurrences du type d'événement
  3. Extrait fenêtre T-30→T+90 pour chaque occurrence
  4. Calcule ATR, Body%, Noise Ratio à chaque minute
  5. **MOYENNE** tous les résultats par occurrence
- **Sortie**: `EventImpactResult`

**Type**: `EventImpactResult` (10 champs)
```rust
pub atr_timeline_before: Vec<f64>              // 30 points
pub atr_timeline_after: Vec<f64>               // 90 points
pub body_timeline_before: Vec<f64>             // Directionalité
pub body_timeline_after: Vec<f64>              // Directionalité
pub noise_ratio_before: f64                    // [0-∞]
pub noise_ratio_during: f64                    // [0-∞]
pub noise_ratio_after: f64                     // [0-∞]
pub volatility_increase_percent: f64           // Impact en %
pub event_count: usize                         // Occurrences analysées
pub event_type: String
pub pair: String
```

**Status**: ✅ Compiles, tested

---

### Frontend (Vue 3) ✅

**Composant Principal**: `RetroAnalysisResults.vue`
- **Props**: Accepte `EventImpactResult`
- **Graphique**: 2 courbes ATR comparatives
  - Bleu (AVANT): T-30→T0
  - Rouge (APRÈS): T0→T+90
  - Ligne séparatrice jaune à T0
- **Axes**:
  - **X**: Temps en minutes (-30 à 0 | 0 à 90)
  - **Y**: ATR (dynamique min-max)
- **Stats en bas**:
  - Noise Ratio AVANT/PENDANT/APRÈS
  - Impact Volatilité (%)
  - Occurrences analysées
- **Conclusion**: Texte verte/rouge (volatilité détectée ou non)

**Status**: ✅ Compiles, renderability OK

**Composables**:
- `useRetroAnalysisGraphData.ts`: Interface updated
  - Ancien: `RetroGraphData` (ATR5 single curve)
  - Nouveau: `RetroGraphData` (2 timelines comparative)
  
**Status**: ✅ Updated

---

### Intégration ✅

**Flow**:
1. User sélectionne pair + eventType dans `RetroactiveAnalysisView.vue`
2. Appel: `chargerDonnéesGraph(pair, eventType)`
3. Tauri invoque: `analyze_volatility_profile`
4. Backend retourne: `EventImpactResult`
5. Frontend reçoit dans `graphData.value`
6. `RetroAnalysisResults` reçoit props et render 2 courbes

**Status**: ✅ Connected

---

## 📝 FICHIERS MODIFIÉS

### Phase 11: Backend Redesign ✅

| Fichier | Changement | Status |
|---------|-----------|--------|
| `src-tauri/src/commands/retrospective_analysis/services.rs` | Réécrit `compute_event_impact()` | ✅ 140+ lignes |
| `src-tauri/src/commands/retrospective_analysis/types.rs` | Ajout `EventImpactResult` | ✅ 11 champs |
| `src-tauri/src/commands/retrospective_analysis/commands.rs` | Modifié `analyze_volatility_profile` | ✅ Retourne EventImpactResult |

### Phase 12: Frontend Integration (CETTE SESSION) ✅

| Fichier | Changement | Status |
|---------|-----------|--------|
| `src/components/RetroAnalysisResults.vue` | **Rewritten from scratch** | ✅ 2 courbes SVG |
| `src/components/RetroactiveAnalysisView.vue` | Props update | ✅ Passe EventImpactResult |
| `src/components/RetroactiveAnalysisResultsViewer.vue` | Archive viewer update | ✅ Accepte nouveau format |
| `src/composables/useRetroAnalysisGraphData.ts` | Interface update | ✅ RetroGraphData refactorisée |

---

## 🧪 VALIDATION

### Compilation

**Frontend**: ✅ No errors, No warnings
```
RetroAnalysisResults.vue: OK
RetroactiveAnalysisView.vue: OK  
RetroactiveAnalysisResultsViewer.vue: OK
useRetroAnalysisGraphData.ts: OK
```

**Backend**: ✅ Compiles successfully
```
cargo check: PASSED (16.22s)
Warnings: 8 (dead code only, acceptable)
Errors: 0
```

---

## 📊 GRAPHIQUE DÉTAILS

### Layout SVG

```
┌─────────────────────────────────────────────┐
│  📊 Impact de l'événement sur la volatilité  │
├─────────────────────────────────────────────┤
│                                             │
│  ┌───────────────┬───────────────────────┐ │
│  │    AVANT      │       APRÈS           │ │
│  │  (T-30→T0)    │    (T0→T+90)          │ │
│  │               │                       │ │
│  │  Bleu ═════════ T0 ═════ Rouge        │ │
│  │      curve    (event)   curve        │ │
│  │               │                       │ │
│  └───────────────┴───────────────────────┘ │
│                                             │
├─────────────────────────────────────────────┤
│ Noise Ratio AVANT: 1.23 ✓ (clean)          │
│ Noise Ratio PENDANT: 2.54 ⚠ (mixed)        │
│ Noise Ratio APRÈS: 1.89 ⚠ (mixed)          │
│ Impact Volatilité: +45.3%                  │
│ Occurrences analysées: 24                  │
├─────────────────────────────────────────────┤
│ ✅ Événement génère 45.3% de volatilité     │
│    directionnelle                           │
└─────────────────────────────────────────────┘
```

### Courbes

**AVANT (T-30→T0)**: 30 points
- Gradient bleu clair → transparent
- Polyline bleu #58a6ff
- Montre volatilité "baseline"

**APRÈS (T0→T+90)**: 90 points
- Gradient rouge/orange clair → transparent
- Polyline rouge #f85149
- Montre réaction à l'événement

### Stats

**Noise Ratio Classification**:
- `< 1.5` = "clean" (vert #3fb950)
- `1.5-2.5` = "mixed" (jaune #fbbf24)
- `> 2.5` = "choppy" (rouge #f85149)

---

## 🔧 PROCHAINES ÉTAPES OPTIONNELLES

### Post-Implementation (Si désiré)
1. [ ] Ajouter Body% en couleur gradient sur les courbes
2. [ ] Ajouter sélecteur pour afficher courbes côte-à-côte vs superposées
3. [ ] Intégrer dates min/max de l'analyse
4. [ ] Ajouter bouton "Exporter données" (CSV)
5. [ ] Slider pour filtrer occurrences par date
6. [ ] Heatmap pour comparer N événements simultanément

---

## 📌 NOTES CRITIQUES

### Architecture Decisions

1. **Moyenne vs Médiane**
   - Choisie: **MOYENNE** (volatility_increase_percent = sum/count)
   - Raison: Plus intuitive, influence des pics

2. **30 points AVANT, 90 APRÈS**
   - T-30: Baseline 30 minutes avant
   - T0: Événement (ligne jaune)
   - T+90: Décroissance post-événement
   - Raison: Pattern Straddle = 90min post-event

3. **Comparative vs Absolute**
   - Affichée: COMPARATIVE (before/after côte-à-côte)
   - Raison: Permet évaluer impact événement

4. **Noise Ratio (Range/Body)**
   - Clean: Mouvement directional, peu de wicks
   - Choppy: Bruit, indécision, mauvaise tradability

---

## 🎓 KEY LEARNINGS

### Problem that was fixed
- **Initial Misunderstanding**: Pensait que c'était analyse d'un événement individuel
- **Correction**: C'est analyse d'**impact du type d'événement** en comparant ALL occurrences
- **Root Cause**: Averaging logic était appliqué à tous les événements ensemble (bug logic PHASE 8)
- **Solution**: Réécrit pour boucler chaque occurrence, extraire fenêtre, puis MOYENNER les résultats

### Technical Excellence
- Utilisé `EventImpactResult` type pour type-safety
- SVG direct rendering pour contrôle fin des axes
- Computed properties pour réactivité Vue
- Props bien typés (Interfaces TS)

---



#### Étape 1.2: Refactoriser `analyze_decay_profile`
- **Avant** : Boucle sur événements → somme moyennes
- **Après** :
  1. Boucle sur événements
  2. Pour chaque événement, extraire **ATR minute par minute**
  3. **Aligner temporellement** (T0 = événement, T+1, T+2, ..., T+180)
  4. **Calculer moyenne + écart-type** pour chaque minute
  5. Retourner la **série complète**

#### Étape 1.3: Modifier la signature Tauri
```rust
#[tauri::command]
pub async fn analyze_decay_profile_detailed(
    pair: String,
    event_type: String,
    state: tauri::State<'_, CalendarState>,
) -> Result<DecayProfileDetailedResult, String>
```

---

### Phase 2: Frontend Vue (Affichage du nouveau graphique)

**Fichier à modifier** : `src/components/RetroAnalysisResults.vue`

#### Étape 2.1: Enrichir les props
```typescript
interface Props {
  // ... props actuels
  atrTimeline?: number[]       // Courbe réelle
  volatilityMean?: number[]    // Moyenne par minute
  volatilitySigma?: number[]   // Écart-type par minute
  peakMinute?: number          // Minute du pic
  totalMinutesAnalyzed?: number // Durée totale (ex: 180)
}
```

#### Étape 2.2: Remplir le SVG
1. **Axe X** :
   - Labels: T0, T+5min, T+10min, ..., T+180min
   - Points d'ancrage tous les 5 minutes

2. **Axe Y** :
   - 0% → min(ATR)
   - 100% → max(ATR)
   - Échelons: 0%, 25%, 50%, 75%, 100%

3. **Courbe réelle** :
   - Tracer `atrTimeline` en **courbe lisse** (Bézier ou polyline)
   - Couleur: **bleu clair** (#58a6ff)

4. **Bandes volatilité (opt)** :
   - Zone grisée: `volatilityMean ± volatilitySigma`
   - Opacité: 0.2

5. **Ligne verticale Peak** :
   - À `peakMinute`
   - Couleur: **rouge** (#f85149)
   - Label: "Peak ATR"

6. **Zones colorées qualité** :
   - 80-100% ATR: **VERT** → Tradable
   - 50-80% ATR: **JAUNE** → Marginal
   - <50% ATR: **ROUGE** → À éviter

#### Étape 2.3: Calculer les coordonnées SVG
```typescript
// Pseudo-code
const maxAtr = Math.max(...atrTimeline)
const pixelPerPoint = 200 / maxAtr  // 200px pour max ATR
const pixelPerMinute = 700 / totalMinutesAnalyzed

for (let i = 0; i < atrTimeline.length; i++) {
  const x = 80 + (i * pixelPerMinute)
  const y = 320 - (atrTimeline[i] * pixelPerPoint)
  pathPoints.push(`${x},${y}`)
}

// Tracer la courbe
const svgPath = `M ${pathPoints.join(' L ')}`
```

---

### Phase 3: Composable Vue (Hook pour charger les données)

**Fichier à créer** : `src/composables/useRetroAnalysisGraphData.ts`

```typescript
export function useRetroAnalysisGraphData() {
  const graphData = ref<{
    atrTimeline: number[]
    volatilityMean: number[]
    volatilitySigma: number[]
    peakMinute: number
  } | null>(null)
  
  async function chargerDonnéesGraph(pair: string, eventType: string) {
    graphData.value = await invoke('analyze_decay_profile_detailed', {
      pair,
      eventType
    })
  }
  
  return { graphData, chargerDonnéesGraph }
}
```

---

### Phase 4: Intégration dans RetroactiveAnalysisView

**Fichier** : `src/components/RetroactiveAnalysisView.vue`

#### Étape 4.1: Appeler le nouveau endpoint
```typescript
const { graphData, chargerDonnéesGraph } = useRetroAnalysisGraphData()

async function load() {
  await chargerDonnéesGraph(selected.value, selectedEventType.value)
  // ... appels existants
}
```

#### Étape 4.2: Passer les props à RetroAnalysisResults
```vue
<RetroAnalysisResults
  :peak-delay="peakDelayResults.peak_delay_minutes"
  :atr-timeline="graphData?.atrTimeline"
  :volatility-mean="graphData?.volatilityMean"
  :volatility-sigma="graphData?.volatilitySigma"
  :peak-minute="graphData?.peakMinute"
  ...
/>
```

---

## 🎯 PRIORITÉS

### Déroulement recommandé

| Phase | Étapes | Complexité | Temps est. |
|-------|--------|-----------|-----------|
| **1** | 1.1 → 1.2 → 1.3 (Backend) | 🟠 Moyen | 30 min |
| **2** | 2.1 → 2.2 → 2.3 (SVG) | 🟡 Facile-Moyen | 40 min |
| **3** | 3 (Composable) | 🟢 Facile | 10 min |
| **4** | 4.1 → 4.2 (Intégration) | 🟢 Facile | 10 min |

**Total estimé** : ~90 minutes

---

## ✅ CHECKLIST DE VALIDATION

Après implémentation, vérifier:

- [ ] Backend compile sans erreurs
- [ ] Nouvelle commande Tauri responsive (< 2sec)
- [ ] GraphData retourné avec 180+ points ATR
- [ ] Frontend affiche la courbe sans distorsion
- [ ] Responsivité préservée (petit/grand écran)
- [ ] Ligne verticale Peak au bon endroit
- [ ] Tests unitaires pour calcul timeline
- [ ] Pas de console.log() / debugger

---

## 🔧 FICHIERS À MODIFIER

1. `src-tauri/src/commands/retrospective_analysis/commands.rs` ← **CRÉER DecayProfileDetailedResult**
2. `src-tauri/src/commands/retrospective_analysis/types.rs` ← **AJOUTER nouvelle struct**
3. `src/components/RetroAnalysisResults.vue` ← **ENRICHIR SVG + props**
4. `src/components/RetroactiveAnalysisView.vue` ← **APPELER nouvel endpoint**
5. `src/composables/useRetroAnalysisGraphData.ts` ← **CRÉER composable** (nouveau fichier)
6. `src/composables/useRetrospectiveAnalysis.ts` ← **AJOUTER nouvel appel Tauri**

---

## 🚨 CONSIDÉRATIONS

### Performance
- **Risque** : 180+ points d'ATR × 20 événements = calcul lourd
- **Mitigation** : Mettre en cache les résultats au niveau Rust

### Precision temporelle
- **Risque** : Décalages si candles manquantes
- **Mitigation** : Interpolation linéaire si gap > 1 minute

### Responsivité SVG
- **Risque** : SVG trop grand tue le rendu
- **Mitigation** : Réduire à 60 points ATR max (tous les 3 min)

---

**Status** : 📝 Planification terminée. Prêt pour Phase 1.
