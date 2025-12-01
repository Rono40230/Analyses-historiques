# 🤖 Copilot Instructions - Analyses Historiques

Volatility Analyzer for Forex Straddle Trading - Tauri 2.0 + Vue 3 + Rust

## 📚 Quick Navigation

| Section | Purpose | Read if... |
|---------|---------|-----------|
| Mission Project | Project goal | You're new to the codebase |
| Architecture Blueprint | Folder structure | You need to understand layout |
| Developer Workflows | Build & test commands | You're setting up environment |
| Critical Patterns | Code examples | You're writing code |
| Complete Rules (RÈGLES 1-22) | All rules from .clinerules | You need full reference |
| Domain Knowledge: Straddle | Business logic | You're implementing metrics |
| Automated Audit System | Quality gates & scripts | You're validating code |
| Technical Constraints | Performance/reliability | You're optimizing |
| Security & Privacy | Data handling | You're adding features |
| Testing & QA | Test patterns | You're writing tests |
| File Organization | Structure examples | You're creating files |
| Debugging & Monitoring | Logging tips | You're debugging |
| FAQ | Common questions | You're stuck |

---

## 🎯 Mission Project
Generate optimal Straddle trade parameters (offset, TP/SL, duration) by analyzing historical volatility and economic event correlations. **Single rule**: "Does this metric serve the Straddle? If no, don't implement it."

---

## 🏗️ Architecture Blueprint

### Frontend Layer (`src/`)
- **Entry**: `App.vue` - Tab-based interface (Volatility | Correlation | Archives | Import)
- **Components**: Vue 3 Composition API with `<script setup>`, French naming (`ouvrirModale()`, `calculerMoyenne()`)
- **State**: `stores/volatility.ts` (Pinia store) + 12 composables for domain logic
- **Patterns**:
  - 📏 Max 250 lines per component (300 for modals/complex tables)
  - 🚫 Never: `console.log()`, `console.error()`, `alert()`, type `any`
  - ✅ Always: Try/catch + proper error handling, explicit TypeScript types, `invoke()` for Tauri commands

### Backend Layer (`src-tauri/src/`)
```
commands/          ← Tauri handlers (max 200 lines each)
├── volatility/    ← Symbol analysis, hourly stats, Straddle metrics
├── calendar_*     ← Economic events import/management
├── correlation/   ← Event×Pair impact analysis
├── event_metrics/ ← Movement quality, entry timing
└── [other]        ← Archive, session, deletion, metadata

services/          ← Business logic (max 300 lines each)
├── volatility/    ← ATR, noise ratio, breakout detection
├── event_*        ← Event correlation, duration analysis
├── csv_*          ← Parsing, cleaning, loading
├── straddle_*     ← Parameter optimization for Straddle
├── global_*       ← Cross-pair, cross-event analysis
└── [other]        ← Entry timing, movement analysis

models/            ← Data structures (max 150 lines each)
├── errors.rs      ← VolatilityError + Result<T> alias
├── candle.rs      ← OHLCV data
├── hourly_stats.rs ← Volatility metrics by hour
├── calendar_event.rs
├── trading_recommendation.rs ← Enum: Optimal|Good|Cautious|Risky
└── [other]

db/
├── migrations.rs  ← Schema creation (calendar_events, pairs, candles)
└── mod.rs         ← Pool management + Diesel setup
```

### Data Flow
1. **Import**: CSV → `csv_loader.service` → validate → store in `pairs.db` or `volatility.db`
2. **Analysis**: Select symbol → `analyze_symbol` command → volatility services → HourlyStats
3. **Correlation**: Calendar events + candles → `event_correlation.service` → impact scores
4. **Output**: AnalysisResult (hourly_stats + stats_15min + recommendation) → Vue store → UI

### Technology Stack
- **Frontend**: Vue 3.5, TypeScript 5.6, Pinia 3, Vite 6, Tauri API 2
- **Backend**: Rust 1.70+, Tauri 2, Polars (analytics), Diesel (ORM), async/tokio
- **Database**: SQLite (2 databases: `volatility.db` for calendar, `pairs.db` for candles)
- **Key deps**: `csv`, `chrono`, `validator`, `thiserror`, `tracing`

---

## 📋 Developer Workflows

### Build & Dev
```bash
npm install && cd src-tauri && cargo build  # First-time setup
npm run tauri dev                           # Frontend + Backend hot-reload
cargo test                                  # Unit tests (in src-tauri/)
./scripts/impact-detection/check-quality.sh # Quick validation
```

### Quality Gates
```bash
make check         # Size limits, unwrap detection, dead code
make validate      # Full Phase 2: tests + impact detection + commit
cargo fmt && cargo clippy -- -D warnings   # Rust linting
npm run build && vue-tsc --noEmit          # TypeScript checking
```

### Commit Workflow (PHASE 1 → PHASE 2)
1. **Phase 1**: Code locally, accumulate changes, tests pass → NO COMMIT
2. **Phase 2**: User says "valide tout" → Run `./scripts/impact-detection/validate-phase2.sh`
3. Script reports impact + regressions → Auto-commit if ✅

### Database
```bash
cd src-tauri
diesel migration generate add_new_table     # Create migration
diesel migration run                        # Apply migrations
# DB lives in ~/.local/share/volatility-analyzer/ (avoid hot-reload)
```

---

## 🔑 Critical Patterns & Conventions

### Naming (MANDATORY FRENCH)
```rust
// ❌ BAD: Snake_case in English
fn calculate_atr() {}
let volatility_mean = 0.0;

// ✅ GOOD: Snake_case in French
fn calculerAtr() {}
let moyenneVolatilité = 0.0;
```

```typescript
// ✅ GOOD: Vue component methods in French
async function analyserSymbole(symbol: string) { ... }
function ouvrirModalArchive() { ... }
const moyenneVolatilité = computed(() => ...) 
```

### Error Handling (Result<T, VolatilityError>)
```rust
// ❌ BAD: unwrap() forbidden (except tests)
let data = parse_csv(file).unwrap();

// ✅ GOOD: Propagate with ?
pub fn parse_csv(file: &str) -> Result<Vec<Candle>> {
    let reader = csv::Reader::from_path(file)?;
    // ... 
    Ok(candles)
}
```

### Service Architecture (DAG: No Circular Dependencies)
**Levels** (dependency only downward):
1. `utils/`, `models/`, `config.rs` ← No dependencies
2. `services/csv_loader.rs`, `db.rs` ← Depends on Level 1
3. `services/volatility.rs`, `event_correlation.rs` ← Depends on Levels 1–2
4. `commands/volatility.rs` ← Depends on Levels 1–3 only

**Never**: `services/volatility.rs` imports `services/event_correlation.rs` (same level)

### Tauri Command Anatomy
```rust
#[tauri::command]
pub async fn analyze_symbol(symbol: String) -> Result<AnalysisResult> {
    // 1. Validate input
    if symbol.is_empty() {
        return Err(VolatilityError::ValidationError("Symbol empty".into()));
    }
    
    // 2. Call service
    let service = VolatilityService::new();
    let result = service.analyze(symbol).await?;
    
    // 3. Return Result (Tauri auto-serializes via Serde)
    Ok(result)
}
```

### Vue Component Setup Pattern
```typescript
<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useStore } from '../stores/...'
import { invoke } from '@tauri-apps/api/core'

const store = useStore()
const { symbol, loading } = storeToRefs(store)

const localData = ref<DataType | null>(null)
const error = ref<string | null>(null)

onMounted(async () => {
  await chargerDonnees()
})

async function chargerDonnees() {
  try {
    localData.value = await invoke('command_name', { param: '...' })
  } catch (e) {
    error.value = String(e)
  }
}
</script>

<template>
  <div v-if="loading" class="spinner">⏳</div>
  <div v-else-if="error" class="error">{{ error }}</div>
  <div v-else class="content">{{ localData }}</div>
</template>
```

### File Size Limits (HARD LIMITS - Split if exceeded)
| Component | Max Lines | Exception |
|-----------|-----------|-----------|
| **Rust Command** | 200 | — |
| **Rust Service** | 300 | — |
| **Rust Model** | 150 | — |
| **Vue Component** | 250 | 300 for modals/tables |
| **Pinia Store** | 200 | 500 for data-only stores |
| **Composable** | 150 | — |
| **Utils/Helpers** | 200 | — |

---

## 🧪 Testing & Quality Assurance

### Rust Test Example
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculer_atr_valid_range() {
        let candles = vec![
            Candle { open: 1.0, high: 1.1, low: 0.9, close: 1.05, volume: 100 },
            Candle { open: 1.05, high: 1.15, low: 0.95, close: 1.1, volume: 120 },
        ];
        let atr = calculer_atr(&candles, 2).unwrap();
        assert!(atr > 0.0);
    }
}
```

### TypeScript Test Patterns
- Use component emits for event testing
- Mock `invoke()` via vitest mock API
- Test composables in isolation (no component mounting)

### Code Quality Checklist
- ✅ No `console.log()`, `debugger`, or `alert()`
- ✅ All errors are `Result<T, VolatilityError>`
- ✅ No `.clone()` > 5 times per function
- ✅ No magic numbers (use `const MAGIC = 42`)
- ✅ Comments for "why", not "what"
- ✅ Test coverage > 80% for new modules

---

## 📊 Domain Knowledge: Straddle Trading

### Project Objectives
**Primary Mission**: Automatically parameterize Straddle robot (Bidi) via historical volatility analysis

**Strategic Goal**: STRADDLE (News Trading) - Buy Stop/Sell Stop placement before economic announcements

**Critical Parameters**:
- **Offset**: Order distance (based ATR + Noise Ratio)
- **Timing**: Placement moment (X seconds before announcement)
- **Duration**: Position hold time (based volatility decay)
- **TP/SL**: Take Profit / Stop Loss (ratio 1:2-3)

**Golden Rule**: Every metric, calculation, analysis MUST serve optimizing these 4 parameters for Straddle.

### Strategic Obligations for Straddle
1. **ATR Based** → SL/TP and offset = f(ATR local)
2. **Noise Aware** → Filter events with Noise Ratio > 3.0
3. **Body% Filter** → Ignore hours with directionality < 20%
4. **Event Correlated** → High volatility must = HIGH event
5. **Duration Adaptive** → Trade duration = f(ATR + event_type)

### Key Metrics Used
| Metric | Source | Use Case |
|--------|--------|----------|
| **ATR** | Candles (True Range) | Offset = ATR × 1.5–2.0 |
| **Noise Ratio** | Range / Body | Filter if > 3.0 (too choppy) |
| **Volatility Mean** | ATR by hour | Identify golden hours |
| **Break% (Breakout %)** | Count candles breaking MA | Entry quality |
| **Event Impact** | Volatility before/after | Tradability score |
| **Decay Duration** | Time until half-volatility | Hold duration |
| **Body Range** | Body / Range | Directional strength |
| **Shadow Ratio** | Wicks / Range | Market indecision |
| **Volume Imbalance** | Bid/Ask pressure | Direction confirmation |

### Analysis Workflow
1. Load candles (OHLCV from CSV)
2. Calculate hourly stats: ATR, volatility, range, body%, breakout%
3. Fetch calendar events for date range
4. Correlate: Which events increase volatility?
5. Recommend: Best hour + pair + parameters

### Output: TradingRecommendation Enum
```rust
pub enum TradingRecommendation {
    Optimal,      // High volatility + low noise + event correlated
    Good,         // Decent metrics
    Cautious,     // Marginal conditions
    Risky,        // High noise, low event correlation
    NoData,       // Insufficient data
}
```

### Bidi Robot Obligations
1. **Export API** → Data always in standardized JSON
2. **Confidence Score** → Each recommendation with score 0-100
3. **Risk Percent = 1.0** → Immutable, no modifier
4. **Dynamic Trailing Stop** → Coefficient 1.5-2.5 based volatility
5. **Event Time Exact** → To the second (H:MM:SS)

### User Obligations
1. **Transparency** → Each number = explained/justified
2. **Ease** → Zero config = smart defaults
3. **Visibility** → Clear dashboard with alerts
4. **Validation** → Validate data before import
5. **History** → Archive analyses for comparison

### What NOT to Do (Anti-Objectives)
1. ❌ Provide direct trading signals
   - App = historical analysis, not real-time predictor
2. ❌ Simulate future performance
   - "Guaranteed 50R gain" = Lie
3. ❌ Replace human decision
   - User remains responsible
4. ❌ Allow backtesting
   - Separate app; too complex for MVP

### Anti-Metric Rule
**If metric does NOT serve Straddle → DELETE**
Ask always: "Does this help place a better Straddle?"
If NO → Don't implement it.

---

## 🚫 Anti-Patterns to Avoid

| ❌ DON'T | ✅ DO INSTEAD |
|----------|----------------|
| `let x = parse().unwrap()` | `let x = parse()?` |
| `println!()` | `tracing::info!()` or `tracing::debug()` |
| `async { ... }` in command params | `pub async fn` or `tokio::spawn()` |
| `pub` everything | Only `pub` for intended API surface |
| Mock data in production | Symulator with realistic patterns |
| Import from same-level service | Depends on DAG structure |
| 400-line component | Split into composable + sub-components |
| Type `any` | Explicit struct or generic |
| `unwrap()` anywhere | Use `?` for propagation |
| Code commented > 3 lines | Delete it |
| `.clone()` > 5x per function | Refactor for ownership |
| Magic numbers | Use `const MAGIC_NAME = 42` |

---

## 📋 Complete Rules (RÈGLES)

### RÈGLE 1: Identity & Philosophy
- **Role**: Expert "Vibe Coding" - UX, esthétique, rapidité
- **Priority**: Visual/functional result. Premium interface, fluid, "Wow effect"
- **Communication**: French, collaborative, solution-oriented

### RÈGLE 2: Naming Conventions (CRITICAL)
- **Functions & Methods**: FRENCH MANDATORY
  - ✅ `calculerMoyenne()`, `ouvrirModale()`, `sauvegarderArchive()`
  - ❌ `calculateAverage()`, `openModal()`, `saveArchive()`
- **Variables**: French preference
- **Comments**: French, clear

### RÈGLE 3: Tech Stack
- **Frontend**: Vue.js 3 (Composition API `script setup`), TypeScript
- **Backend**: Rust (Tauri)
- **Styling**: CSS Vanilla, Dark Mode, vibrant colors, glassmorphism
- **Architecture**: Modular, one file/component, Pinia for state

### RÈGLE 4: Workflow
1. Understand the "Vibe"
2. Plan (clear action plan)
3. Implement (French naming)
4. Document (`task.md`, `walkthrough.md`)

### RÈGLE 5: Error Management
- Never blank pages/cryptic errors
- Loading states (spinners)
- User-friendly error messages
- Data persistence

### RÈGLE 6: Communication & Responses
- **Conciseness**: Short, direct answers
- **Action-Oriented**: Prioritize code and concrete actions
- **No verbosity** unless explicitly requested
- **French**: Always, even technical terms

### RÈGLE 7: Compilation Workflow
- **No automatic compilation**: Wait for explicit user order
- **User Responsibility**: User orders compilations and reports problems
- **Wait for feedback**: If errors, wait for signaling

### RÈGLE 8: Non-Regression Verification
- **Always check**: Nothing broken after modification
- **Test coherence**: Impact on other code parts
- **Cross validation**: Existing functionalities OK

### RÈGLE 9: Premium Design
- **Never placeholders**: Use `generate_image` if needed
- **Rich colors**: No generic red/blue/green
- **Typography**: Google Fonts (Inter, Roboto, Outfit)
- **Animations**: Micro-animations, hover effects
- **Responsive**: Always

### RÈGLE 10: SEO & Accessibility
- Descriptive title tags
- Meta descriptions
- Heading structure (H1 unique)
- Semantic HTML5
- Unique IDs for interactive elements

### RÈGLE 10.5: Frontend Code Quality (CRITICAL)
**Production Strict Prohibitions:**
- ❌ `console.log()` : FORBIDDEN (use logger if necessary)
- ❌ `console.error/warn/debug()` : FORBIDDEN
- ❌ `debugger` : FORBIDDEN
- ❌ `alert()` : FORBIDDEN (use UI notifications)
- ❌ Type `any` : TO AVOID (type explicitly)

**Best Practices:**
- ✅ Error handling with try/catch + logger
- ✅ Explicit TypeScript types
- ✅ No dead code (unused imports/components)
- ✅ No circular imports
- ✅ TODO formatted: `TODO(name): description`

### RÈGLE 11: Auto-Verification (Rust/Tauri)
After each generation:
```bash
cargo fmt && cargo clippy -- -D warnings && cargo test
```
If failure → DO NOT DELIVER. Fix first.

### RÈGLE 12: Zero Automatic Commit (PROHIBITION)
AI NEVER mentions: `git commit`, `git push`, `git add`

### RÈGLE 13: Zero Dead Code
Before generation, analyze and DELETE:
- Functions never called
- Unused imports
- Unused variables
- Code comments > 3 lines

### RÈGLE 14: Systematic Tests
For each public function:
- Unit test BEFORE implementation (TDD)
- `cargo test` MUST PASS
- Coverage > 80% for new modules

### RÈGLE 15: Strict Size Limits (HARD LIMITS)

**Backend (Rust):**
- Services: < 300 lines (HARD LIMIT)
- Commands: < 200 lines (HARD LIMIT)
- Models: < 150 lines (HARD LIMIT)
- main.rs: < 120 lines (HARD LIMIT)

**Frontend (Vue.js/TypeScript):**
- Vue Components: < 250 lines (HARD LIMIT)
  - Exception modals/complex tables: < 300 lines
- Pinia Stores: < 200 lines (HARD LIMIT)
  - Exception static data stores: < 500 lines
- Composables: < 150 lines (HARD LIMIT)
- Utils/Helpers: < 200 lines (HARD LIMIT)

If exceeds → STOP + Split

### RÈGLE 16: 3-Level Architecture (DAG)
- **LEVEL 1**: utils/config.rs, models/errors.rs → NO dependencies
- **LEVEL 2**: db/cache.rs, services/api_client.rs → Depends LEVEL 1
- **LEVEL 3**: services/trading.rs → Depends LEVEL 1+2
- **LEVEL 4**: commands/*.rs → Depends LEVEL 3
❌ NEVER import between services same level

### RÈGLE 17: Unified Error Handling
- All public functions → `Result<T, VolatilityError>`
- ❌ `unwrap()`, `unwrap_or()` forbidden (except tests)
- ✅ Use `?` for propagation

### RÈGLE 18: Forbidden Anti-Patterns
❌ unwrap() without context | Mock data | Clone() > 5x/fn
❌ Magic numbers | Code comments > 3 lines | panic!() in services
❌ Circular imports | pub on everything

### RÈGLE 19: Mandatory Report
After each task:
```
═══════════════════════════════════════════
✅ TASK COMPLETED: [name]
═══════════════════════════════════════════
📏 Size: [X] lines - Compliant
🧪 Tests: [X] tests written - Passing
🔒 Coverage: [X]% - OK
⚠️  Warnings: [X]
🧹 Dead code: [X] lines cleaned
═══════════════════════════════════════════
```

### RÈGLE 20: Ignore System Dependency Errors
These errors are IGNORED:
- cannot find -lwebkit2gtk-4.1
- cannot find -ljavascriptcoregtk-4.1
- missing GTK, Cairo, GLib, Soup libraries
✅ Continue despite these errors
❌ BUT fix all other types (syntax, logic, types)

### RÈGLE 21: Mandatory Understanding Validation
BEFORE any line of code:

**1. UNDERSTAND**
"I understand you want: [detailed reformulation]"

**2. CLARIFY**
"But I have questions:"
- What date range?
- Modify DB or just display?
- Keep compatibility?
- Performance constraints?
- Affected modules?

**3. PROPOSE**
"My plan:"
1. Modify [module X] for [action Y]
2. Add tests for [case Z]
3. Update [interface]
4. Validate with [check]

**4. WAIT APPROVAL**
"Does this work for you? Any adjustments?"
DO NOT CODE UNTIL APPROVAL

### RÈGLE 22: Phase 1 & 2 - Accumulation & Validation

**PHASE 1: ACCUMULATION**
1. Apply RÈGLE 21 (understanding validation)
2. Generate code
3. Run `cargo test` → MUST PASS
4. Accumulate changes
5. NO COMMIT
6. Wait for Phase 2 instructions

**PHASE 2: IMPACT VALIDATION**
When user says "valide tout":
```bash
./scripts/impact-detection/validate-phase2.sh
```

Script verifies:
1. Impact: who changed
2. Regressions: tests vs baseline
3. Report: impact + decision

If ✅ APPROVED → Auto commit + push
If ❌ BLOCKED → Signal problem, fix, re-test

**INITIALIZATION** (once):
```bash
./scripts/impact-detection/init-impact-system.sh
```

---

## 🔍 Automated Audit System

### Quality Gate Scripts

The project includes comprehensive **automated audit scripts** in `scripts/`:

#### Size Validation
```bash
./scripts/check-file-size.sh        # Check max line limits per file
./scripts/check-file-size-complete.sh # Complete size analysis
```
- Commands: < 200 lines
- Services: < 300 lines
- Models: < 150 lines
- Vue Components: < 250 lines (300 for modals)
- Pinia Stores: < 200 lines (500 for data stores)
- Composables: < 150 lines

#### Anti-Pattern Detection
```bash
./scripts/check-unwrap.sh           # Detect all unwrap()/expect() violations
./scripts/check-antipatterns.sh     # Find magic numbers, dead code, etc.
./scripts/check-dead-code.sh        # Identify unused functions/imports
```

#### Architecture Validation
```bash
./scripts/check-architecture.sh     # Verify DAG structure (no circular imports)
./scripts/check-circular-imports.sh # Detect circular dependencies
```

#### Code Quality
```bash
./scripts/check-french-naming.sh           # Verify French naming convention
./scripts/check-french-naming-frontend.sh  # Check Vue/TypeScript naming
./scripts/check-frontend-quality.sh        # Vue/TypeScript quality gates
./scripts/check-typescript-quality.sh      # TypeScript strict mode
./scripts/check-vue-quality.sh             # Vue component patterns
./scripts/check-vue-size.sh                # Vue file size limits
./scripts/check-coverage.sh                # Test coverage > 80%
```

#### Comprehensive Checks
```bash
make check              # Run quick validation (size, unwrap, dead code)
make validate           # Full Phase 2 validation
./scripts/full-code-audit.sh  # Complete audit report
```

### Impact Detection System

When you say "valide tout", the system runs:

```bash
./scripts/impact-detection/validate-phase2.sh
```

This script sequence:
1. **Snapshot current state** → `snapshot-dependencies.sh`
2. **Track changes** → `change-tracker.sh`
3. **Detect regressions** → `regression-detector.sh`
4. **Verify impact** → `verify-impact.sh`
5. **Generate report** → `final-approval.sh`

### Expected Reports

After `make validate`, expect:

```
═══════════════════════════════════════════
✅ VALIDATION REPORT
═══════════════════════════════════════════
📊 Files Changed: [X] (+[X] -[X] ~[X])
📏 Size Impact: [+/- X lines]
🧪 Tests: [X] passed / [X] failed
🔒 Coverage: [X]% → [X]% (diff: [+/-X]%)
⚠️  Warnings: [X]
🧹 Dead Code: [X] issues
🔄 Circular Imports: [X] detected
✔️  French Naming: OK / [X] violations
═══════════════════════════════════════════
Decision: ✅ APPROVED / ❌ BLOCKED
═══════════════════════════════════════════
```

### Rules Enforced by Audit

| Rule | Check | Severity |
|------|-------|----------|
| RÈGLE 2 (French naming) | `check-french-naming*.sh` | HARD |
| RÈGLE 13 (Zero dead code) | `check-dead-code.sh` | HARD |
| RÈGLE 14 (Tests > 80%) | `check-coverage.sh` | HARD |
| RÈGLE 15 (Size limits) | `check-file-size*.sh` | HARD |
| RÈGLE 16 (DAG architecture) | `check-architecture.sh` | HARD |
| RÈGLE 17 (Result<T> errors) | `check-antipatterns.sh` | HARD |
| RÈGLE 18 (No unwrap) | `check-unwrap.sh` | HARD |
| RÈGLE 10.5 (No console.log) | `check-frontend-quality.sh` | HARD |

### Quick Audit Before Commit

Always run before asking for validation:

```bash
# Quick check (< 30 seconds)
make check

# Full validation (< 2 minutes)
cargo test && cargo fmt && cargo clippy -- -D warnings

# For TypeScript
npm run build && vue-tsc --noEmit
```

### Common Audit Failures & Fixes

| Failure | Cause | Fix |
|---------|-------|-----|
| Size > 300 lines (service) | Monolithic service | Split into smaller services |
| Unwrap() detected | Error handling | Use `?` operator or explicit Result |
| French naming violation | English names used | Rename to French |
| Circular import | DAG broken | Move to parent level or extract to models |
| Coverage < 80% | Missing tests | Add unit tests before implementation |
| Clippy warning | Non-idiomatic Rust | Run `cargo fmt` and fix |

---

---

## ⚙️ Technical Constraints

### Performance Requirements
- ✅ Import CSV 1-10 years historical < 30 seconds
- ✅ Analysis calculation < 5 seconds
- ✅ Display heatmap 50×20 events/pairs < 1 second
- ✅ RAM usage < 500 MB even with 10 years of data

### Reliability Requirements
- ✅ Graceful error handling (no crashes)
- ✅ Data validation before processing
- ✅ Detailed logs for debugging
- ✅ Auto-recovery after minor error

### Compatibility
- ✅ Linux (Fedora, Ubuntu)
- ✅ Support varied CSV formats (MetaTrader, TradingView, Dukascopy)

### Code Quality Metrics
- ✅ Respect `.clinerules` (file size, no unwrap, DAG architecture)
- ✅ 80%+ test coverage (105 tests minimum)
- ✅ 0 clippy warnings
- ✅ Zero dead code

### Success Metrics
**Business Metrics**:
- **Accuracy**: Predicted volatility vs real < 10% error
- **Win Rate**: Straddle setup wins > 60% of cases
- **Profitability**: Average gain > 2R (Risk:Reward)
- **Adoption**: Bidi robot uses data for > 80% of trades

**Technical Metrics**:
- **Uptime**: > 99.5%
- **Performance**: Analysis < 5sec, Import < 30sec
- **Stability**: 0 crashes in 100 hours usage
- **Coverage**: Tests > 80% of code

---

## 🔒 Security & Privacy

### User Data
- ✅ No personal data collected
- ✅ Local data (no cloud)
- ✅ No tracking or telemetry
- ✅ Local history in SQLite

### Input Validation
- ✅ CSV parsed strictly (no injection)
- ✅ Datetimes validated (no arbitrary parsing)
- ✅ Numbers range-checked (min/max checks)
- ✅ No user code execution

---

## 🔗 Cross-Component Communication

### Frontend → Backend
```typescript
const result = await invoke('analyze_symbol', { symbol: 'EURUSD' })
// Tauri auto-routes to: commands/volatility/analysis.rs → analyze_symbol()
```

### Backend → Frontend
Via Pinia store:
```typescript
await store.analyzeSymbol(symbol)  // Calls invoke internally
// Store updates: analysisResult, hourlyStats, recommendation
```

### Calendar + Pair Data
- Separate SQLite databases (`volatility.db` vs `pairs.db`)
- Joined in services for correlation analysis
- UI manages calendar selection separately from pair selection

---

## 📁 File Organization Examples

**Rust New Service** (`src-tauri/src/services/my_analyzer.rs`):
```rust
use crate::models::*;
use thiserror::Error;

pub struct MyAnalyzer;

impl MyAnalyzer {
    pub fn new() -> Self { MyAnalyzer }
    
    pub fn analyze(&self, data: &[Candle]) -> Result<MyResult> {
        // Implementation < 300 lines
        Ok(MyResult { ... })
    }
}

// Export in src-tauri/src/services/mod.rs:
// pub use my_analyzer::MyAnalyzer;
```

**Vue New Component** (`src/components/MyView.vue`):
```vue
<script setup lang="ts">
// 1. Imports (Tauri, Vue, internal)
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// 2. Types & State
const resultat = ref<MyType | null>(null)

// 3. Methods
async function charger() {
  resultat.value = await invoke('my_command')
}
</script>

<template>
  <!-- Simple, accessible, French labels -->
</template>

<style scoped>
/* Dark mode, Tailwind-compatible or vanilla CSS */
</style>
```

---

## 🔧 Debugging & Monitoring

### Logging (Rust)
```rust
tracing::info!("✅ Success: {}", msg);      // Info
tracing::debug!("🔍 Debug: {:?}", data);    // Debug
tracing::warn!("⚠️  Warning: {}", msg);     // Warning
tracing::error!("❌ Error: {}", msg);       // Error
```

### Logging (Vue)
Use Tauri's built-in console or a logger library (not `console.log()`)

### Performance Profiling
- Use `criterion.rs` benchmarks for hot paths
- Cache candle index with `CandleIndexState` (in-memory, keyed by symbol)
- Lazy-load 15-min stats only when needed

---

## 📚 Key Files Reference

| File | Purpose | Lines | Status |
|------|---------|-------|--------|
| `.clinerules` | Absolute rules (read first!) | 400+ | Latest |
| `SYSTEM_PROMPT.md` | Phase 1/2 workflow | 91 | Latest |
| `projet.md` | Business objectives & constraints | ~150 | Latest |
| `src-tauri/src/lib.rs` | Backend entry, state management | ~120 | ✅ Conf |
| `src/stores/volatility.ts` | Frontend state (Pinia) | 188 | ✅ Conf |
| `src-tauri/src/commands/volatility/mod.rs` | Command exports | ~25 | ✅ Conf |
| `src-tauri/src/models/errors.rs` | Error types + Result alias | ~60 | ✅ Conf |
| `src-tauri/Cargo.toml` | Rust dependencies | ~50 | ✅ Latest |
| `package.json` | Node dependencies + scripts | ~30 | ✅ Latest |

---

## 🎓 Getting Started (AI Agent)

### First Task Checklist
1. **Read** `.clinerules` and this file
2. **Understand** project mission (Straddle parameters)
3. **Confirm** you'll follow:
   - French naming conventions
   - Result<T> error handling
   - File size limits
   - DAG architecture
   - Phase 1/2 workflow
4. **Ask** if requirements are unclear (RULE 21)
5. **Propose** implementation plan before coding

### Common Tasks
| Task | Location | Pattern |
|------|----------|---------|
| Add new analysis metric | `services/volatility/...rs` | Service + test |
| New import format | `services/csv_cleaner.rs` | Add parser |
| Calendar event correlation | `services/event_correlation.rs` | Impact score |
| New UI view | `src/components/...vue` | Composable + component |
| Database schema change | `src-tauri/src/db/migrations/` | Diesel migration |
| Straddle parameter calculation | `services/straddle_*.rs` | Formula + test |

---

## ❓ FAQ for AI Agents

**Q: Should I use unwrap()?**  
A: Only in tests and main.rs error cases. Everywhere else: `?` operator or explicit error handling.

**Q: How do I handle async?**  
A: Rust: `#[tauri::command] pub async fn` + `.await`. Vue: `async function` with await + try/catch.

**Q: What if a file exceeds size limits?**  
A: Split into 2+ files with shared types in models/. Don't create monoliths.

**Q: How do I test without Tauri?**  
A: Unit tests in Rust use lib crate directly. Vue components mock `invoke()` with vitest.

**Q: Where do calendar imports go?**  
A: CSV parsed in `services/csv_loader.rs`, stored in `volatility.db` via Diesel.

**Q: New symbol in analysis?**  
A: Add to `src/stores/volatility.ts`, invoke `analyze_symbol` command, render result in UI.

---

**Last Updated**: Nov 29, 2025  
**Maintainer**: Rono40230  
**Project**: Analyses-historiques (Volatility Analyzer for Straddle Trading)
