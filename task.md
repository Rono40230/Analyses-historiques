# 📋 TASK: Améliorer le graphique des métriques rétrospectives

## 🔍 ANALYSE PRÉALABLE

### Question: Avons-nous l'ATR détaillé minute par minute?

**RÉPONSE: OUI ET NON** ⚠️

#### ✅ Données disponibles (actuellement)
1. **Candles M1 complètes** : `loader.load_candles_by_pair(&pair, "M1", start, end)`
   - Chaque candle a: `high`, `low`, `close`, `open`, `volume`
   - Permet de **recalculer l'ATR pour chaque minute**

2. **ATR moyen agrégé** : Actuellement retourné
   - `peak_atr: f64` → ATR max observé
   - `decay_rate_pips_per_minute: f64` → Taux moyen de décroissance
   - Mais **pas la courbe détaillée minute par minute**

#### ❌ Ce qui manque
- **Série temporelle d'ATR** : Array de 180+ ATR (une minute d'analyse sur 3h après événement)
- **Timestamps associés** : Pour savoir à quelle minute chaque ATR correspond
- **Volatilité moyenne par minute** : Agrégée sur toutes les occurrences de l'événement

#### 📊 Où les candles sont chargées
**Fichier** : `src-tauri/src/commands/retrospective_analysis/commands.rs` (ligne 74+)

```rust
let candles = loader.load_candles_by_pair(&pair, "M1", window_start, window_end)
  .unwrap_or_default();
// On a les candles brutes, mais on les jette après extraction du peak_atr
```

---

## 📐 PLAN D'IMPLÉMENTATION

### Phase 1: Backend Rust (Nouveau calcul)

**Fichier à modifier** : `src-tauri/src/commands/retrospective_analysis/commands.rs`

#### Étape 1.1: Créer une nouvelle structure de résultat
- **Nom** : `DecayProfileDetailedResult` (enrichi du courant)
- **Ajouter champs** :
  ```rust
  pub atr_timeline: Vec<f64>        // ATR par minute (180+ points)
  pub timestamps: Vec<String>        // ISO 8601 pour chaque minute
  pub volatility_mean: Vec<f64>      // Volatilité moyenne (agrégée)
  pub volatility_std: Vec<f64>       // Écart-type (pour bandes)
  pub peak_minute: u16               // Minute où ATR = max
  ```

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
