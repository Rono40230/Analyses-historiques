# 🚀 OPTIMISATIONS DE PERFORMANCE - RÉSUMÉ TECHNIQUE

## 📊 Problème Identifié

**Symptôme**: Par Paire et Heatmap très lents (30+ secondes)

**Cause Racine**: 
```
Avant optimisation:
  Par Paire: 500 événements × 14 paires = 7,000 appels CSV
  Heatmap: 100 événements × 14 paires = 1,400 appels CSV
  
Chaque appel CSV:
  - Lecture fichier: 500-800ms
  - Parse 970k lignes
  - Iteration linéaire O(n): 970k × 5-6 comparaisons datetime
  = 4.85M+ opérations POUR CHAQUE événement
```

---

## ✅ Solutions Implémentées

### 1️⃣ **CandleIndex (HashMap par Date)**
**Fichier**: `src/services/candle_index.rs`

```rust
CandleIndex {
  data: HashMap<String, BTreeMap<NaiveDate, Vec<Candle>>>
}
```

**Bénéfice**:
- ✅ Charge ALL CSV une seule fois au startup (~2-3 secondes)
- ✅ Requêtes par date = O(log n) au lieu de O(n)
- ✅ Recherche 7 jours avant = ~21 candles au lieu de 970k

**Impact**: 🔴 **90% amélioration** (~50x plus rapide)

### 2️⃣ **Optimized Helpers (Requêtes Rapides)**
**Fichier**: `src/commands/correlation/optimized_helpers.rs`

```rust
pub fn calculate_volatilities_optimized(
    candle_index: &CandleIndex,
    pair_symbol: &str,
    event_datetime: NaiveDateTime,
    event_window_minutes: i64,
    baseline_days_back: i64,
) -> Result<VolatilityMetrics, String>
```

**Méthodes de CandleIndex utilisées**:
- `get_candles_in_range()` - pour fenêtre événement (±30min) = ~60 candles
- `get_baseline_candles()` - pour baseline (7j même heure) = ~21 candles

**Impact**: 🟢 **99% réduction** itérations vs scan linéaire

### 3️⃣ **Integration dans 3 Onglets**

| Onglet | Fichier | Avant | Après | Gain |
|--------|---------|-------|-------|------|
| **Par Événement** | `event_impact.rs` | 1 CSV load × 14 paires | CandleIndex | ~3x |
| **Par Paire** | `pair_history.rs` | 1 CSV load (ancien) | CandleIndex | ~50x |
| **Heatmap** | `heatmap.rs` | 15 CSV loads | CandleIndex | ~50x |

### 4️⃣ **Startup Initialization**
**Fichier**: `src/commands/candle_index_commands.rs`

```rust
#[tauri::command]
pub async fn init_candle_index(
    state: State<'_, CandleIndexState>,
) -> Result<String, String>
```

**Appel** depuis le frontend au démarrage (une seule fois)

---

## 📈 RÉSULTATS ESTIMÉS

### Avant Optimisation
```
Par Paire (6 mois, 500 événements):
  - Charger 14 paires × 1 fois = 14 CSV loads = 10 secondes
  - Itérer 500 événements × 14 paires × 970k candles = 6.79M opérations
  - Temps TOTAL: ~30-40 secondes 🐢

Heatmap (6 mois, 100 événements × 14 paires):
  - 15 CSV loads = 10 secondes
  - Itérer 1,400 requêtes = 1.358M opérations
  - Temps TOTAL: ~15-25 secondes 🐢
```

### Après Optimisation
```
Par Paire (6 mois, 500 événements):
  - Charger index au démarrage = 2-3 secondes (UNE FOIS)
  - Itérer 500 événements × 14 paires × 60 candles (max) = 420k opérations
  - Temps TOTAL: ~1-2 secondes 🚀 (15-20x plus rapide)

Heatmap (6 mois, 100 événements × 14 paires):
  - Charger index au démarrage = 2-3 secondes (UNE FOIS)
  - Itérer 1,400 requêtes × 60 candles (max) = 84k opérations
  - Temps TOTAL: ~0.5-1 secondes 🚀 (20-50x plus rapide)
```

---

## 🔧 INTÉGRATION FRONTEND

Appeler `init_candle_index` au démarrage de l'app (App.vue):

```typescript
// Dans setup() ou onMounted()
const initializeIndex = async () => {
  try {
    const result = await invoke('init_candle_index', {})
    console.log('✅ CandleIndex initialisé:', result)
  } catch (error) {
    console.error('❌ Erreur init CandleIndex:', error)
  }
}

// Appeler une seule fois au startup
initializeIndex()
```

---

## 🧪 CHECKLIST DE TEST

- [ ] Frontend appelle `init_candle_index` au démarrage
- [ ] Vérifier logs: "CandleIndex initialized: 14 pairs loaded"
- [ ] Tester **Par Événement**: Doit être <3 secondes
- [ ] Tester **Par Paire**: Doit être <2 secondes (au lieu de 30-40s)
- [ ] Tester **Heatmap**: Doit être <1 seconde (au lieu de 15-25s)
- [ ] Vérifier que les résultats sont identiques avant/après

---

## 🎯 ÉTAPES SUIVANTES

### Court terme (immédiat):
- ✅ Intégrer appel `init_candle_index` dans App.vue

### Moyen terme (future):
- ⏳ **Phase 3.1**: Pré-parser les dates (60% gain supplémentaire)
  - Parser NaiveDateTime à la lecture CSV
  - Stocker directement dans CandleIndex
  - Impact: 0.1-0.2s au lieu de 1-2s

- ⏳ **Phase 3.2**: Cache persistant SQLite (optionnel)
  - Sauvegarder CandleIndex en SQLite
  - Réutiliser au redémarrage (skip loading CSV)
  - Impact: Démarrage 0.1s au lieu de 2-3s

### Long terme:
- ⏳ **Phase 4**: Parallélization Rayon
  - Charger paires en parallèle au startup
  - Impact: Startup 0.5-1s au lieu de 2-3s

---

## 📋 ARCHITECTURE FINALE

```
App.vue (Frontend)
  └── invoke('init_candle_index')
      └── candle_index_commands.rs
          └── CandleIndex::load_all_pairs()
              ├─ CsvLoader::load_candles() x 14
              ├─ BTreeMap indexing par date
              └─ State<CandleIndexState>::index = Some(CandleIndex)

Onglets (Par Événement, Par Paire, Heatmap):
  └─ Récupérer candle_index depuis State
  └─ calculate_volatilities_optimized()
     ├─ candle_index.get_candles_in_range() = O(log n)
     └─ candle_index.get_baseline_candles() = O(log n)
```

---

## 🔐 NOTES IMPORTANTES

1. **Une seule initialisation**: `init_candle_index` charge TOUTES les paires en mémoire
2. **Pas de rechargement**: CandleIndex reste en mémoire pour toute la session
3. **Thread-safe**: Utilise `Mutex<Option<CandleIndex>>` pour concurrence
4. **Fallback**: Si index non initialisé, retourne erreur explicite

---

## 📞 QUESTION DE SUIVI

Voulez-vous aussi implémenter l'**Optimisation 3 (Pré-parser les dates)** pour un gain supplémentaire de 60%?
- Coût: 15-20 min de code
- Gain: 0.6-1.2s de performance supplémentaire
- Complexité: Faible (simple refactor parsing)
