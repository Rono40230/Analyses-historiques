# 📊 Structures des Archives Explorées

**Date**: 6 décembre 2025  
**Étape**: 1.1 - Explorer Structure des Archives  
**Status**: ✅ COMPLÉTÉE

---

## 🗄️ Modèle de Base: Archive (Rust/Diesel)

### Stockage en Base de Données

```rust
pub struct Archive {
    pub id: i32,
    pub title: String,              // Ex: "EURUSD - NFP Analysis"
    pub archive_type: String,        // "Volatilité" | "Métriques Rétrospectives" | "Heatmap"
    pub period_start: String,        // ISO date "2025-12-01"
    pub period_end: String,          // ISO date "2025-12-06"
    pub comment: Option<String>,     // Optionnel
    pub created_at: chrono::NaiveDateTime,
    pub data_json: String,           // JSON sérialisé des données
}
```

**Champs Clés**:
- `id`: Identifiant unique en BD
- `archive_type`: Détermine comment parser `data_json`
- `data_json`: String JSON brute → doit être désérialisée selon le type
- `created_at`: Pour trier chronologiquement

---

## 📦 Type 1: "Volatilité" (4 archives)

### Structure Probable

Basée sur analyse du code `global_analyzer_types.rs` :

```json
{
  "symbol": "EURUSD",
  "best_hours": [14, 15, 16],
  "confidence_score": 0.82,
  "global_metrics": {
    "mean_volatility": 28.5
  }
}
```

**OU** (avec wrapper):

```json
{
  "analysisResult": {
    "symbol": "EURUSD",
    "best_hours": [14, 15, 16],
    "confidence_score": 0.82,
    "global_metrics": {
      "mean_volatility": 28.5
    }
  }
}
```

### Champs Parsables

- `symbol`: String (pair) ✓
- `best_hours`: Vec<u8> (heures)
- `confidence_score`: f64 (0-1 ou 0-100?) → **À vérifier**
- `global_metrics.mean_volatility`: f64

### Cas d'Usage

Ces archives sont simplement des **statistiques de volatilité brutes**, sans corrélation événement spécifique.

---

## 📦 Type 2: "Métriques Rétrospectives" (20 archives)

### Structure Probable

Basée sur analyse du code `useMetricsModalLoad.ts` :

```typescript
interface ArchivedAnalysisData {
  analysisResult: AnalysisResult
  sliceAnalyses: SliceAnalysis[]
  movementQualities: Record<string, MovementQuality>
  volatilityDuration: VolatilityDuration
  tradingPlan: TradingPlan
  entryWindowAnalysis: EntryWindowAnalysis
  offsetOptimal: OptimalOffset
  winRate: WinRateMetric
  whipsawAnalysis: WhipsawMetric
}
```

### Champs Clés à Extraire

```typescript
interface MetricsArchive {
  // De analysisResult
  symbol: string                           // "EURUSD"
  period_start: string                     // "2025-12-01"
  period_end: string                       // "2025-12-06"
  
  // De volatilityDuration (custom calculation possible)
  peak_delay_minutes: number                // T+3.2 minutes
  decay_timeout_minutes: number             // T+18.5 minutes
  peak_atr: number                          // 45 pips
  decay_rate_pips_per_minute: number        // X pips/min
  
  // De tradingPlan ou calculation
  confidence: number                        // 0.82 (0-1 range)
  event_type?: string                       // "NFP", "CPI", etc. → À vérifier
  event_count?: number                      // Nombre d'analyses cet événement
}
```

### Cas d'Usage

**Ces archives contiennent les données rétrospectives d'un événement spécifique sur une paire**:
- Pour NFP: quelle est la volatilité peak? Combien de temps elle dure?
- Pour chaque paire/événement: confidence score de prédictibilité

---

## 📦 Type 3: "Heatmap" (1 archive)

### Structure Probable

```json
{
  "pairs": ["EURUSD", "GBPUSD", "USDJPY", "AUDUSD", "NZDUSD", "CADJPY"],
  "events": ["NFP", "CPI", "Inflation", "BOE Rate", "FED Rate", "PPI", ...],
  "impactMatrix": [
    [85, 92, 65, 88, 78, ...],  // EURUSD impact pour chaque événement
    [78, 88, 58, 91, 70, ...],  // GBPUSD impact pour chaque événement
    ...
  ],
  "volatilityData": {
    "EURUSD": {
      "NFP": {
        "volatility_peak": 45,
        "volatility_before": 15,
        "volatility_after": 8,
        "avg_peak_time_minutes": 3.2
      },
      ...
    }
  }
}
```

### Champs Clés à Extraire

```typescript
interface HeatmapArchive {
  pairs: string[]                    // ["EURUSD", "GBPUSD", ...]
  events: string[]                   // ["NFP", "CPI", ...]
  impactMatrix: number[][]           // pairs.length × events.length
  volatilityData?: {
    [pair: string]: {
      [event: string]: {
        volatility_peak: number
        volatility_before: number
        volatility_after: number
        avg_peak_time_minutes: number
      }
    }
  }
}
```

### Cas d'Usage

**C'est la source de vérité pour les corrélations événement × paire**:
- Quelle paire réagit le plus à NFP?
- Quel événement impacte le plus EURUSD?
- Détails timing: quand le pic, combien de temps ça dure

---

## 🔄 Flux de Données Actuel

```
Frontend: GlobalAnalysisModal.vue
    ↓
useGlobalAnalysis() composable
    ↓
Tauri command: invoke('analyze_all_archives', { filters })
    ↓
Backend: global_analysis_commands.rs
    ↓
GlobalAnalyzer.analyze_all_archives()
    ↓
ArchiveService.list_archives()  [charge TOUTES les 25 archives]
    ↓
global_analyzer.rs: filter_and_weight_archives()
    ├─ Essaie de parser en AnalyzableArchiveData
    ├─ Filtre par date
    ├─ Filtre par paires
    └─ Pondère par date (recent = poids plus élevé)
    ↓
Calculs: compute_tradable_events(), compute_pair_straddle_rates(), etc.
    ↓
Résultat: GlobalAnalysisResult avec 3 sections vides!
    ↓
Frontend: Affiche messages "Cette analyse nécessite..."
```

**PROBLÈME**: Les calculs `compute_*()` cherchent des archives de type "corrélation" qui n'existent pas!

---

## 📋 Champs Détectés par Archive Type

| Champ | Type 1 (Volatilité) | Type 2 (Rétrospectives) | Type 3 (Heatmap) |
|-------|-------------------|----------------------|-----------------|
| `symbol`/`pair` | ✓ | ✓ | ✓ (tableau) |
| `confidence_score` | ✓ | ✓ | ✗ |
| `peak_volatility` | ✓ | ✓ | ✓ |
| `peak_delay_minutes` | ✗ | ✓ | ✓ |
| `event_type` | ✗ | ? | ✓ (tableau) |
| `event_count` | ✗ | ? | ✓ (implicit) |
| `impact_score` | ✗ | ✗ | ✓ |

---

## ✅ Recommandations pour Parsing

### Parser Stratégie

```typescript
function parseArchiveByType(archive: Archive): NormalizedArchive | null {
  const type = archive.archive_type;
  
  if (type === "Volatilité") {
    return parseVolatilityArchive(archive.data_json);
  } else if (type === "Métriques Rétrospectives") {
    return parseRetrospectiveArchive(archive.data_json);
  } else if (type === "Heatmap") {
    // Special case: return multiple entries, one per (pair, event)
    return parseHeatmapArchive(archive.data_json);
  }
  
  return null;
}
```

### Normalisation

Tous les types doivent être convertis en interface commune:

```typescript
interface NormalizedArchive {
  id: string
  type: ArchiveType
  pair: string
  eventType: string
  peakATR: number           // en pips
  peakDelay: number         // en minutes
  decayTimeout: number      // en minutes
  confidence: number        // 0-1
  impactScore?: number      // 0-100, seulement pour heatmap
  timestamp: Date
}
```

---

## 🧪 Prochaines Étapes

**Étape 1.2**: Créer composable `useArchiveStatistics.ts` avec:
- [ ] `loadAllArchives()`: Charger les 25 archives via Tauri
- [ ] `parseArchiveByType()`: Parser selon le type
- [ ] Normaliser en `NormalizedArchive[]`
- [ ] Grouper par paire/événement

**Test**:
- Vérifier que 25 archives sont chargées
- Vérifier que parsing ne plante pas
- Afficher les archives normalisées dans console pour validation

---

## 🔗 Références Code

- **Archive model**: `src-tauri/src/models/archive.rs`
- **Archive service**: `src-tauri/src/services/archive_service.rs`
- **Global analyzer**: `src-tauri/src/services/global_analyzer.rs`
- **Type definitions**: `src-tauri/src/services/global_analyzer_types.rs`
- **Frontend modal**: `src/components/GlobalAnalysisModal.vue`
- **Frontend composable**: `src/composables/useGlobalAnalysis.ts`
