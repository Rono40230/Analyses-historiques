<template>
  <div v-if="isOpen" class="modal-overlay" @click.self="close">
    <div class="modal-content">
      <div class="modal-header">
        <h2>✨ IAnalyse Statistique</h2>
        <button class="close-button" @click="$emit('close')">×</button>
      </div>

      <!-- Barre de Filtres -->
      <div class="filters-bar glass" :class="{ disabled: loading }">
        <div class="filter-group">
          <label>Période :</label>
          <div class="date-inputs">
            <input type="date" v-model="startDate" placeholder="Début" class="date-input">
            <span class="separator">à</span>
            <input type="date" v-model="endDate" placeholder="Fin" class="date-input">
          </div>
        </div>
        
        <div class="filter-group pairs-filter">
          <label>Paires :</label>
          <div class="pairs-selector">
            <button 
              v-for="pair in availablePairs" 
              :key="pair"
              class="pair-chip"
              :class="{ active: selectedPairs.includes(pair) }"
              @click="togglePair(pair)"
            >
              {{ pair }}
            </button>
          </div>
        </div>
        
        <div class="filter-actions">
          <button class="apply-button" @click="runAnalysis(false)">
            Appliquer les filtres
          </button>
        </div>
      </div>

      <div class="modal-body">
        <!-- ÉTAT 1 : CHARGEMENT (Animation Wow) -->
        <div v-if="loading" class="loading-container">
          <div class="ai-brain">
            <div class="brain-pulse"></div>
            <div class="brain-core">🧠</div>
          </div>
          <h3 class="loading-title">{{ loadingStep }}</h3>
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: progress + '%' }"></div>
          </div>
          <div class="loading-log">
            <p v-for="(log, index) in logs" :key="index" class="log-item">{{ log }}</p>
          </div>
        </div>

        <!-- ÉTAT 2 : RÉSULTATS (Dashboard) -->
        <div v-else-if="result" class="results-container">
          <!-- En-tête Stats Globales -->
          <div class="global-stats-grid">
            <MetricTooltip title="Analyses Scannées">
              <div class="stat-card glass">
                <div class="stat-icon">📊</div>
                <div class="stat-info">
                  <span class="stat-label">Analyses Scannées</span>
                  <span class="stat-value">{{ result.total_analyses }}</span>
                </div>
              </div>
              <template #definition>
                <div class="tooltip-section-title">📖 Définition</div>
                <div class="tooltip-section-text">
                  Nombre total d'archives de type "Volatilité brute" trouvées et analysées dans votre base de données.
                </div>
              </template>
              <template #usage>
                <div class="tooltip-section-title">💡 Interprétation</div>
                <div class="tooltip-section-text">
                  Plus ce nombre est élevé, plus les statistiques de l'IA sont fiables. Avec moins de 5 analyses, les résultats peuvent être biaisés.
                </div>
              </template>
            </MetricTooltip>

            <MetricTooltip title="Confiance Moyenne">
              <div class="stat-card glass">
                <div class="stat-icon">🎯</div>
                <div class="stat-info">
                  <span class="stat-label">Confiance Moyenne</span>
                  <span class="stat-value highlight">{{ result.global_stats.average_confidence.toFixed(1) }}/100</span>
                </div>
              </div>
              <template #definition>
                <div class="tooltip-section-title">📖 Définition</div>
                <div class="tooltip-section-text">
                  Moyenne des scores de confiance de toutes vos analyses archivées. Ce score combine volatilité, qualité des ticks et fiabilité des signaux.
                </div>
              </template>
              <template #scoring>
                <div class="tooltip-section-title">📊 Échelle de Confiance</div>
                <div class="tooltip-section-text">
                  • <strong>80-100</strong> : Scalp Agressif ✅<br>
                  • <strong>65-79</strong> : Scalp Normal 🟢<br>
                  • <strong>50-64</strong> : Scalp Prudent 🟡<br>
                  • <strong>35-49</strong> : Très Prudent 🟠<br>
                  • <strong>0-34</strong> : Ne pas trader ❌
                </div>
              </template>
            </MetricTooltip>

            <MetricTooltip title="Volatilité Moyenne">
              <div class="stat-card glass">
                <div class="stat-icon">📈</div>
                <div class="stat-info">
                  <span class="stat-label">Volatilité Moyenne</span>
                  <span class="stat-value">{{ (result.global_stats.average_volatility * 100).toFixed(2) }}%</span>
                </div>
              </div>
              <template #definition>
                <div class="tooltip-section-title">📖 Définition</div>
                <div class="tooltip-section-text">
                  Mesure l'amplitude moyenne des mouvements de prix sur toutes vos paires analysées. Calculée via l'ATR (Average True Range) normalisé.
                </div>
              </template>
              <template #usage>
                <div class="tooltip-section-title">💡 Interprétation</div>
                <div class="tooltip-section-text">
                  • <strong>\u003c 10%</strong> : Marché calme, peu d'opportunités<br>
                  • <strong>10-25%</strong> : Volatilité idéale pour le scalping<br>
                  • <strong>\u003e 25%</strong> : Marché chaotique, risque élevé
                </div>
              </template>
            </MetricTooltip>
          </div>

          <div class="dashboard-grid">
            <!-- Colonne Gauche : Top Paires -->
            <div class="dashboard-column">
              <h3>🏆 Top Paires Performantes</h3>
              <div class="pairs-list">
                <div v-for="(pair, index) in result.best_pairs.slice(0, 5)" :key="pair.symbol" class="pair-item glass" :class="'rank-' + (index + 1)">
                  <div class="pair-rank">#{{ index + 1 }}</div>
                  <div class="pair-info">
                    <span class="pair-symbol">{{ pair.symbol }}</span>
                    <span class="pair-details">{{ pair.analysis_count }} analyses</span>
                  </div>
                  <div class="pair-score">
                    <span class="score-label">Score IA</span>
                    <span class="score-value">{{ pair.score.toFixed(0) }}</span>
                  </div>
                </div>
              </div>
            </div>

            <!-- Colonne Droite : Golden Hours -->
            <div class="dashboard-column">
              <h3>⏰ Golden Hours (Heures en Or)</h3>
              <div class="hours-chart">
                <div v-for="hour in sortedGoldenHours" :key="hour.hour" class="hour-bar-container">
                  <div class="hour-label">{{ hour.hour }}h</div>
                  <div class="hour-bar-wrapper">
                    <div class="hour-bar" :style="{ width: hour.reliability + '%' }" :class="getHourClass(hour.reliability)"></div>
                  </div>
                  <div class="hour-value">{{ hour.reliability.toFixed(0) }}%</div>
                </div>
              </div>
              <div class="insight-box glass">
                <h4>💡 L'Insight de l'IA</h4>
                <p>
                  D'après l'analyse de {{ result.total_analyses }} sessions, votre créneau optimal est 
                  <strong>{{ bestHour }}h</strong> avec une fiabilité de <strong>{{ bestHourReliability }}%</strong>.
                  Privilégiez la paire <strong>{{ bestPair }}</strong> pour maximiser vos chances.
                </p>
              </div>
            </div>
          </div>

          <!-- Section Événements Tradables (pleine largeur) -->
          <div v-if="result.tradable_events && result.tradable_events.length > 0" class="events-section">
            <h3>🎯 Types d'Événements les Plus Tradables (Stratégie Straddle)</h3>
            <div class="events-grid">
              <div v-for="(event, index) in result.tradable_events.slice(0, 5)" :key="event.event_name" class="event-card glass" :class="'event-rank-' + (index + 1)">
                <div class="event-header">
                  <div class="event-rank">#{{ index + 1 }}</div>
                  <div class="event-name">{{ event.event_name }}</div>
                </div>
                <div class="event-metrics">
                  <div class="event-metric">
                    <span class="metric-label">Score Tradabilité</span>
                    <span class="metric-value score" :class="getScoreClass(event.tradability_score)">
                      {{ event.tradability_score.toFixed(0) }}/100
                    </span>
                  </div>
                  <div class="event-metric">
                    <span class="metric-label">Augmentation Volatilité</span>
                    <span class="metric-value">×{{ event.avg_volatility_increase.toFixed(2) }}</span>
                  </div>
                  <div class="event-metric">
                    <span class="metric-label">Occurrences</span>
                    <span class="metric-value">{{ event.occurrence_count }}</span>
                  </div>
                </div>
                <div class="event-pairs">
                  <span class="pairs-label">Paires affectées:</span>
                  <span class="pairs-list">{{ event.affected_pairs.join(', ') }}</span>
                </div>
              </div>
            </div>
            <div class="insight-box glass">
              <h4>💡 Interprétation</h4>
              <p>
                Le <strong>Score de Tradabilité</strong> mesure l'augmentation de volatilité générée par l'événement. 
                Un score de 100 signifie que la volatilité double pendant l'événement (idéal pour straddle). 
                Privilégiez les événements avec un score > 50 et plusieurs occurrences pour valider la fiabilité.
              </p>
            </div>
          </div>
          <div v-else class="events-placeholder glass">
            <div class="placeholder-icon">🎯</div>
            <h4>Types d'Événements Tradables</h4>
            <p>Cette analyse nécessite des archives de type "Corrélation événement/paire".</p>
            <p class="hint">Créez des analyses de corrélation pour débloquer cette fonctionnalité.</p>
          </div>

          <!-- Section Taux de Réussite Straddle (pleine largeur) -->
          <div v-if="result.pair_straddle_rates && result.pair_straddle_rates.length > 0" class="straddle-section">
            <h3>📊 Taux de Réussite du Straddle par Paire</h3>
            <div class="straddle-grid">
              <div v-for="(rate, index) in result.pair_straddle_rates.slice(0, 6)" :key="rate.pair" class="straddle-card glass" :class="'straddle-rank-' + (index + 1)">
                <div class="straddle-header">
                  <div class="straddle-rank">#{{ index + 1 }}</div>
                  <div class="straddle-pair">{{ rate.pair }}</div>
                  <div class="straddle-main-score" :class="getScoreClass(rate.straddle_score)">
                    {{ rate.straddle_score.toFixed(0) }}
                  </div>
                </div>
                <div class="straddle-metrics">
                  <div class="straddle-metric">
                    <span class="metric-label">Directional Move</span>
                    <div class="metric-bar-container">
                      <div class="metric-bar directional" :style="{ width: rate.directional_move_rate + '%' }"></div>
                      <span class="metric-bar-value">{{ rate.directional_move_rate.toFixed(0) }}%</span>
                    </div>
                  </div>
                  <div class="straddle-metric">
                    <span class="metric-label">Whipsaw Rate</span>
                    <div class="metric-bar-container">
                      <div class="metric-bar whipsaw" :style="{ width: rate.whipsaw_rate + '%' }"></div>
                      <span class="metric-bar-value">{{ rate.whipsaw_rate.toFixed(0) }}%</span>
                    </div>
                  </div>
                  <div class="straddle-metric">
                    <span class="metric-label">Volatilité Moyenne</span>
                    <span class="metric-value">{{ (rate.avg_volatility * 100).toFixed(2) }}%</span>
                  </div>
                  <div class="straddle-metric">
                    <span class="metric-label">Événements Analysés</span>
                    <span class="metric-value">{{ rate.total_events }}</span>
                  </div>
                </div>
                <div class="straddle-events">
                  <span class="events-label">Top événements:</span>
                  <ul class="events-list">
                    <li v-for="event in rate.top_events" :key="event">{{ event }}</li>
                  </ul>
                </div>
              </div>
            </div>
            <div class="insight-box glass">
              <h4>💡 Comment Interpréter</h4>
              <p>
                Le <strong>Score Straddle</strong> = Directional Move Rate - Whipsaw Rate. 
                Un score élevé indique que la paire génère des mouvements directionnels clairs (bon pour straddle) 
                avec peu de whipsaws (allers-retours qui tuent les positions). 
                Privilégiez les paires avec un score > 40 et au moins 10 événements analysés.
              </p>
            </div>
          </div>
          <div v-else class="straddle-placeholder glass">
            <div class="placeholder-icon">📊</div>
            <h4>Taux de Réussite du Straddle par Paire</h4>
            <p>Cette analyse nécessite des archives de type "Corrélation paire/événement".</p>
            <p class="hint">Créez des analyses de corrélation pour débloquer cette fonctionnalité.</p>
          </div>

          <!-- Section Fenêtres Temporelles Optimales (pleine largeur) -->
          <div v-if="result.optimal_time_windows && result.optimal_time_windows.length > 0" class="timing-section">
            <h3>⏱️ Fenêtres Temporelles Optimales Post-Événement</h3>
            <div class="timing-grid">
              <div v-for="(window, index) in result.optimal_time_windows.slice(0, 6)" :key="window.event_type" class="timing-card glass" :class="'timing-rank-' + (index + 1)">
                <div class="timing-header">
                  <div class="timing-rank">#{{ index + 1 }}</div>
                  <div class="timing-event">
                    <div class="event-name-original">{{ window.event_type }}</div>
                    <div class="event-name-translation" v-if="translateEventName(window.event_type) !== window.event_type">
                      ({{ translateEventName(window.event_type) }})
                    </div>
                  </div>
                  <div class="timing-consistency" :class="getScoreClass(window.consistency_score)">
                    {{ window.consistency_score.toFixed(0) }}%
                  </div>
                </div>
                <div class="timing-metrics">
                  <div class="timing-metric">
                    <div class="metric-icon">🎯</div>
                    <div class="metric-content">
                      <span class="metric-label">Peak Time</span>
                      <span class="metric-value">{{ window.avg_peak_time_minutes.toFixed(0) }} min</span>
                      <span class="metric-hint">Temps pour atteindre le pic</span>
                    </div>
                  </div>
                  <div class="timing-metric">
                    <div class="metric-icon">🚪</div>
                    <div class="metric-content">
                      <span class="metric-label">Entry Window</span>
                      <span class="metric-value">{{ window.avg_entry_window_minutes.toFixed(0) }} min avant</span>
                      <span class="metric-hint">Fenêtre d'entrée optimale</span>
                    </div>
                  </div>
                  <div class="timing-metric">
                    <div class="metric-icon">⏳</div>
                    <div class="metric-content">
                      <span class="metric-label">Return to Normal</span>
                      <span class="metric-value">{{ window.avg_return_to_normal_minutes.toFixed(0) }} min</span>
                      <span class="metric-hint">Temps de retour au calme</span>
                    </div>
                  </div>
                </div>
                <div class="timing-footer">
                  <span class="footer-label">{{ window.occurrence_count }} occurrences</span>
                  <span class="footer-separator">•</span>
                  <span class="footer-label">{{ window.affected_pairs.length }} paires</span>
                </div>
              </div>
            </div>
            <div class="insight-box glass">
              <h4>💡 Guide d'Utilisation</h4>
              <p>
                <strong>Peak Time</strong> : Temps moyen pour atteindre le maximum de volatilité après l'événement. 
                <strong>Entry Window</strong> : Placez votre straddle dans cette fenêtre avant l'événement pour maximiser vos chances. 
                <strong>Return to Normal</strong> : Durée pendant laquelle le marché reste volatil. Fermez vos positions avant ce délai.
                Le <strong>Score de Consistance</strong> indique la fiabilité de ces timings (basé sur le nombre d'occurrences).
              </p>
            </div>
          </div>
          <div v-else class="timing-placeholder glass">
            <div class="placeholder-icon">⏱️</div>
            <h4>Fenêtres Temporelles Optimales</h4>
            <p>Cette analyse nécessite des archives de type "Corrélation paire/événement".</p>
            <p class="hint">Créez des analyses de corrélation pour débloquer cette fonctionnalité.</p>
          </div>
        </div>

        <!-- ÉTAT 3 : ERREUR -->
        <div v-else-if="error" class="error-container">
          <div class="error-icon">❌</div>
          <h3>Oups, une erreur est survenue</h3>
          <p>{{ error }}</p>
          <button class="retry-btn" @click="runAnalysis()">Réessayer</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import MetricTooltip from './MetricTooltip.vue'

const props = defineProps<{
  isOpen: boolean
}>()

const emit = defineEmits<{
  (e: 'close'): void
}>()

// Types (miroir du Rust)
interface GlobalAnalysisResult {
  total_analyses: number
  total_days_analyzed: number
  global_stats: {
    average_volatility: number
    average_confidence: number
    most_analyzed_pair: string
    most_frequent_recommendation: string
  }
  best_pairs: Array<{
    symbol: string
    score: number
    avg_volatility: number
    win_rate: number
    analysis_count: number
  }>
  golden_hours: Array<{
    hour: number
    score: number
    avg_volatility: number
    reliability: number
  }>
  tradable_events: Array<{
    event_name: string
    occurrence_count: number
    avg_volatility_increase: number
    tradability_score: number
    affected_pairs: string[]
  }>
  pair_straddle_rates: Array<{
    pair: string
    total_events: number
    directional_move_rate: number
    whipsaw_rate: number
    avg_volatility: number
    straddle_score: number
    top_events: string[]
  }>
  optimal_time_windows: Array<{
    event_type: string
    occurrence_count: number
    avg_peak_time_minutes: number
    avg_entry_window_minutes: number
    avg_return_to_normal_minutes: number
    consistency_score: number
    affected_pairs: string[]
  }>
  generated_at: string
}

// State
const loading = ref(false)
const result = ref<GlobalAnalysisResult | null>(null)
const error = ref<string | null>(null)
const loadingStep = ref('Initialisation...')
const progress = ref(0)
const logs = ref<string[]>([])

// Filtres
const startDate = ref('')
const endDate = ref('')
const selectedPairs = ref<string[]>([])
const availablePairs = ref<string[]>([
  'EURUSD', 'GBPUSD', 'USDJPY', 'AUDUSD', 'USDCAD', 'USDCHF', 'NZDUSD', // Majeures
  'EURGBP', 'EURJPY', 'GBPJPY', 'AUDJPY', 'XAUUSD', 'USAIDX', 'DEUIDX'  // Crosses & Indices
])

// Computed
const sortedGoldenHours = computed(() => {
  if (!result.value) return []
  return [...result.value.golden_hours]
    .sort((a, b) => b.reliability - a.reliability)
    .slice(0, 8)
    .sort((a, b) => a.hour - b.hour)
})

const bestHour = computed(() => {
  if (!result.value || result.value.golden_hours.length === 0) return '?'
  const best = [...result.value.golden_hours].sort((a, b) => b.reliability - a.reliability)[0]
  return best.hour
})

const bestHourReliability = computed(() => {
  if (!result.value || result.value.golden_hours.length === 0) return '0'
  const best = [...result.value.golden_hours].sort((a, b) => b.reliability - a.reliability)[0]
  return best.reliability.toFixed(0)
})

const bestPair = computed(() => {
  if (!result.value || result.value.best_pairs.length === 0) return '?'
  return result.value.best_pairs[0].symbol
})

// Methods
function close() {
  emit('close')
}

function addLog(message: string) {
  logs.value.unshift(message)
  if (logs.value.length > 5) logs.value.pop()
}

function togglePair(pair: string) {
  if (selectedPairs.value.includes(pair)) {
    selectedPairs.value = selectedPairs.value.filter(p => p !== pair)
  } else {
    selectedPairs.value.push(pair)
  }
}

async function runAnalysis(animate = true) {
  loading.value = true
  error.value = null
  result.value = null
  progress.value = 0
  logs.value = []

  if (animate) {
    // Simulation d'étapes pour l'effet "Wow"
    const steps = [
      { msg: 'Lecture des archives...', p: 10 },
      { msg: 'Désérialisation des données JSON...', p: 30 },
      { msg: 'Agrégation des métriques de volatilité...', p: 50 },
      { msg: 'Calcul des corrélations croisées...', p: 70 },
      { msg: 'Identification des Golden Hours...', p: 90 },
      { msg: 'Génération du rapport IA...', p: 100 }
    ]

    for (const step of steps) {
      loadingStep.value = step.msg
      addLog(step.msg)
      progress.value = step.p
      await new Promise(resolve => setTimeout(resolve, 300)) // Un peu plus rapide
    }
  }

  try {
    // Préparer les filtres
    const filters = {
      start_date: startDate.value || null,
      end_date: endDate.value || null,
      pairs: selectedPairs.value.length > 0 ? selectedPairs.value : null
    }
    
    console.log("Lancement analyse globale avec filtres:", filters)

    const data = await invoke<GlobalAnalysisResult>('analyze_all_archives', {
      filters: filters
    })
    
    result.value = data
  } catch (e: any) {
    console.error(e)
    error.value = typeof e === 'string' ? e : "Erreur inconnue lors de l'analyse"
  } finally {
    loading.value = false
  }
}

function getHourClass(reliability: number): string {
  if (reliability >= 80) return 'bar-excellent'
  if (reliability >= 60) return 'bar-good'
  if (reliability >= 40) return 'bar-average'
  return 'bar-poor'
}

function getScoreClass(score: number): string {
  if (score >= 75) return 'score-excellent'
  if (score >= 50) return 'score-good'
  if (score >= 25) return 'score-average'
  return 'score-poor'
}

function translateEventName(eventName: string): string {
  const translations: Record<string, string> = {
    // US Events
    'Non-Farm Employment Change': 'Variation de l\'emploi non agricole',
    'Unemployment Rate': 'Taux de chômage',
    'Average Hourly Earnings': 'Salaire horaire moyen',
    'Consumer Price Index': 'Indice des prix à la consommation',
    'Core CPI': 'IPC de base',
    'Producer Price Index': 'Indice des prix à la production',
    'Retail Sales': 'Ventes au détail',
    'Core Retail Sales': 'Ventes au détail de base',
    'GDP Growth Rate': 'Taux de croissance du PIB',
    'GDP': 'PIB',
    'Fed Interest Rate Decision': 'Décision sur les taux de la Fed',
    'FOMC Statement': 'Déclaration du FOMC',
    'FOMC Press Conference': 'Conférence de presse du FOMC',
    'Fed Chair Powell Speaks': 'Discours du président de la Fed Powell',
    'ISM Manufacturing PMI': 'PMI manufacturier ISM',
    'ISM Services PMI': 'PMI des services ISM',
    'Flash Manufacturing PMI': 'PMI manufacturier flash',
    'Flash Services PMI': 'PMI des services flash',
    'Durable Goods Orders': 'Commandes de biens durables',
    'Initial Jobless Claims': 'Nouvelles demandes d\'allocations chômage',
    'Continuing Jobless Claims': 'Demandes continues d\'allocations chômage',
    'Building Permits': 'Permis de construire',
    'Housing Starts': 'Mises en chantier',
    'Existing Home Sales': 'Ventes de logements existants',
    'New Home Sales': 'Ventes de logements neufs',
    'Consumer Confidence': 'Confiance des consommateurs',
    'Michigan Consumer Sentiment': 'Sentiment des consommateurs Michigan',
    'Trade Balance': 'Balance commerciale',
    'Industrial Production': 'Production industrielle',
    'Capacity Utilization': 'Taux d\'utilisation des capacités',
    'Factory Orders': 'Commandes industrielles',
    'Business Inventories': 'Stocks des entreprises',
    'Personal Spending': 'Dépenses personnelles',
    'Personal Income': 'Revenus personnels',
    'PCE Price Index': 'Indice des prix PCE',
    'Core PCE Price Index': 'Indice des prix PCE de base',
    'ADP Non-Farm Employment Change': 'Variation de l\'emploi ADP',
    'Challenger Job Cuts': 'Suppressions d\'emplois Challenger',
    'JOLTS Job Openings': 'Offres d\'emploi JOLTS',
    'Nonfarm Payrolls': 'Emplois non agricoles',
    'NFP': 'Emplois non agricoles',
    
    // ECB/EU Events
    'ECB Interest Rate Decision': 'Décision sur les taux de la BCE',
    'ECB Press Conference': 'Conférence de presse de la BCE',
    'ECB President Lagarde Speaks': 'Discours de la présidente de la BCE Lagarde',
    'German ZEW Economic Sentiment': 'Sentiment économique ZEW allemand',
    'German IFO Business Climate': 'Climat des affaires IFO allemand',
    'Eurozone CPI': 'IPC de la zone euro',
    'Eurozone GDP': 'PIB de la zone euro',
    
    // UK Events
    'BoE Interest Rate Decision': 'Décision sur les taux de la BoE',
    'BoE MPC Meeting Minutes': 'Procès-verbal du MPC de la BoE',
    'BoE Gov Bailey Speaks': 'Discours du gouverneur de la BoE Bailey',
    'UK CPI': 'IPC britannique',
    'UK GDP': 'PIB britannique',
    'UK Retail Sales': 'Ventes au détail britanniques',
    
    // Japan Events
    'BoJ Interest Rate Decision': 'Décision sur les taux de la BoJ',
    'BoJ Press Conference': 'Conférence de presse de la BoJ',
    'BoJ Gov Ueda Speaks': 'Discours du gouverneur de la BoJ Ueda',
    'Japan CPI': 'IPC japonais',
    'Japan GDP': 'PIB japonais',
    'Tankan Survey': 'Enquête Tankan',
    
    // Canada Events
    'BoC Interest Rate Decision': 'Décision sur les taux de la BdC',
    'BoC Gov Macklem Speaks': 'Discours du gouverneur de la BdC Macklem',
    'Canada CPI': 'IPC canadien',
    'Canada GDP': 'PIB canadien',
    'Canada Employment Change': 'Variation de l\'emploi canadien',
    
    // Australia Events
    'RBA Interest Rate Decision': 'Décision sur les taux de la RBA',
    'RBA Gov Lowe Speaks': 'Discours du gouverneur de la RBA Lowe',
    'RBNZ Gov Orr Speaks': 'Discours du gouverneur de la RBNZ Orr',
    'Australia CPI': 'IPC australien',
    'Australia GDP': 'PIB australien',
    
    // China Events
    'China CPI': 'IPC chinois',
    'China GDP': 'PIB chinois',
    'China Manufacturing PMI': 'PMI manufacturier chinois',
    'China Services PMI': 'PMI des services chinois',
    
    // Political Events
    'President Trump Speaks': 'Discours du président Trump',
    'President Biden Speaks': 'Discours du président Biden',
    'Presidential Election': 'Élection présidentielle',
    'Congressional Testimony': 'Témoignage au Congrès',
    
    // Other
    'OPEC Meeting': 'Réunion de l\'OPEC',
    'G7 Meeting': 'Réunion du G7',
    'G20 Meeting': 'Réunion du G20',
  }
  
  return translations[eventName] || eventName
}

// Watchers
watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    runAnalysis(true)
  }
})
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.85);
  backdrop-filter: blur(5px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10000;
}

.modal-content {
  background: linear-gradient(145deg, #1a1f2c 0%, #13161c 100%);
  border-radius: 16px;
  width: 95%;
  max-width: 1400px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.6), 0 0 20px rgba(100, 200, 255, 0.1);
  border: 1px solid #30363d;
  overflow: hidden;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 30px;
  border-bottom: 1px solid #30363d;
  background: rgba(22, 27, 34, 0.8);
}

.modal-header h2 {
  margin: 0;
  font-size: 1.5em;
  background: linear-gradient(90deg, #58a6ff, #8b949e);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.close-button {
  background: none;
  border: none;
  color: #8b949e;
  font-size: 1.5em;
  cursor: pointer;
  transition: color 0.2s;
}

.close-button:hover {
  color: #e6edf3;
}

/* FILTERS BAR */
.filters-bar {
  margin: 0 20px 20px 20px;
  padding: 15px;
  display: flex;
  flex-direction: column;
  gap: 15px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  transition: opacity 0.3s;
}

.filters-bar.disabled {
  opacity: 0.5;
  pointer-events: none;
}

.filter-group {
  display: flex;
  align-items: center;
  gap: 15px;
}

.filter-group label {
  color: #8b949e;
  font-weight: 600;
  min-width: 70px;
}

.date-inputs {
  display: flex;
  align-items: center;
  gap: 10px;
}

.date-input {
  background: #0d1117;
  border: 1px solid #30363d;
  color: #e6edf3;
  padding: 8px 12px;
  border-radius: 6px;
  font-family: inherit;
}

.date-input:focus {
  border-color: #58a6ff;
  outline: none;
}

.separator {
  color: #8b949e;
}

.pairs-selector {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.pair-chip {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid #30363d;
  color: #8b949e;
  padding: 5px 10px;
  border-radius: 20px;
  font-size: 0.85em;
  cursor: pointer;
  transition: all 0.2s;
}

.pair-chip:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #e6edf3;
}

.pair-chip.active {
  background: rgba(88, 166, 255, 0.2);
  border-color: #58a6ff;
  color: #58a6ff;
}

.filter-actions {
  display: flex;
  justify-content: flex-end;
  margin-top: 5px;
}

.apply-button {
  background: #238636;
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 6px;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.2s;
}

.apply-button:hover {
  background: #2ea043;
}

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
  display: flex;
  flex-direction: column;
  align-items: center;
}

/* LOADING STATE */
.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 0;
}

.ai-brain {
  position: relative;
  font-size: 5em;
  margin-bottom: 30px;
}

.brain-core {
  position: relative;
  z-index: 2;
  animation: float 3s ease-in-out infinite;
}

.brain-pulse {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 100px;
  height: 100px;
  background: radial-gradient(circle, rgba(100, 200, 255, 0.4) 0%, transparent 70%);
  border-radius: 50%;
  animation: pulse 2s infinite;
  z-index: 1;
}

@keyframes float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-10px); }
}

@keyframes pulse {
  0% { transform: translate(-50%, -50%) scale(0.8); opacity: 0.5; }
  100% { transform: translate(-50%, -50%) scale(1.5); opacity: 0; }
}

.loading-title {
  font-size: 1.5em;
  margin-bottom: 20px;
  color: #64c8ff;
}

.progress-bar {
  width: 60%;
  height: 6px;
  background: #30363d;
  border-radius: 3px;
  overflow: hidden;
  margin-bottom: 20px;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #64c8ff, #a78bfa);
  transition: width 0.3s ease;
}

.loading-log {
  height: 100px;
  overflow: hidden;
  text-align: center;
  opacity: 0.7;
}

.log-item {
  margin: 5px 0;
  font-family: monospace;
  color: #8b949e;
  animation: fadeIn 0.5s;
}

@keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }

/* RESULTS STATE */
.results-container {
  animation: slideUp 0.5s ease-out;
}

@keyframes slideUp { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }

.glass {
  background: rgba(255, 255, 255, 0.03);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.05);
  border-radius: 12px;
}

.global-stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
  margin-bottom: 30px;
}

.stat-card {
  padding: 20px;
  display: flex;
  align-items: center;
  gap: 20px;
  transition: transform 0.2s;
}

.stat-card:hover { transform: translateY(-5px); background: rgba(255, 255, 255, 0.05); }

.stat-icon { font-size: 2.5em; }

.stat-info { display: flex; flex-direction: column; }

.stat-label { color: #8b949e; font-size: 0.9em; text-transform: uppercase; letter-spacing: 1px; }

.stat-value { font-size: 1.8em; font-weight: bold; color: #e6edf3; }
.stat-value.highlight { color: #64c8ff; }

.dashboard-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 30px;
}

@media (max-width: 900px) {
  .dashboard-grid { grid-template-columns: 1fr; }
}

.dashboard-column h3 {
  color: #e6edf3;
  margin-bottom: 20px;
  border-bottom: 2px solid #30363d;
  padding-bottom: 10px;
}

.pairs-list { display: flex; flex-direction: column; gap: 10px; }

.pair-item {
  display: flex;
  align-items: center;
  padding: 15px;
  gap: 15px;
}

.pair-rank {
  font-size: 1.2em;
  font-weight: bold;
  color: #8b949e;
  width: 40px;
}

.rank-1 .pair-rank { color: #ffd700; }
.rank-2 .pair-rank { color: #c0c0c0; }
.rank-3 .pair-rank { color: #cd7f32; }

.pair-info { flex: 1; display: flex; flex-direction: column; }
.pair-symbol { font-weight: bold; font-size: 1.1em; }
.pair-details { font-size: 0.85em; color: #8b949e; }

.pair-score { text-align: right; }
.score-label { display: block; font-size: 0.7em; color: #8b949e; text-transform: uppercase; }
.score-value { font-weight: bold; font-size: 1.2em; color: #64c8ff; }

.hours-chart {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 20px;
}

.hour-bar-container {
  display: flex;
  align-items: center;
  gap: 15px;
}

.hour-label { width: 40px; font-weight: bold; color: #8b949e; }

.hour-bar-wrapper { flex: 1; background: #30363d; height: 10px; border-radius: 5px; overflow: hidden; }

.hour-bar { height: 100%; border-radius: 5px; transition: width 1s ease-out; }
.bar-excellent { background: #10b981; }
.bar-good { background: #3b82f6; }
.bar-average { background: #f59e0b; }
.bar-poor { background: #ef4444; }

.hour-value { width: 40px; text-align: right; font-size: 0.9em; color: #e6edf3; }

.insight-box {
  padding: 20px;
  border-left: 4px solid #a78bfa;
}

.insight-box h4 { margin: 0 0 10px 0; color: #a78bfa; }
.insight-box p { margin: 0; line-height: 1.6; color: #cbd5e0; }
.insight-box strong { color: white; }

/* ERROR STATE */
.error-container {
  text-align: center;
  padding: 60px 0;
}
.error-icon { font-size: 4em; margin-bottom: 20px; }
.retry-btn {
  margin-top: 20px;
  padding: 10px 20px;
  background: #30363d;
  border: none;
  color: white;
  border-radius: 6px;
  cursor: pointer;
}
.retry-btn:hover { background: #4a5568; }

/* EVENTS SECTION */
.events-section {
  margin-top: 30px;
  animation: slideUp 0.5s ease-out 0.2s both;
}

.events-section h3 {
  color: #e6edf3;
  margin-bottom: 20px;
  border-bottom: 2px solid #30363d;
  padding-bottom: 10px;
}

.events-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
  margin-bottom: 20px;
}

.event-card {
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 15px;
  transition: transform 0.2s, background 0.2s;
}

.event-card:hover {
  transform: translateY(-5px);
  background: rgba(255, 255, 255, 0.05);
}

.event-header {
  display: flex;
  align-items: center;
  gap: 15px;
}

.event-rank {
  font-size: 1.5em;
  font-weight: bold;
  color: #8b949e;
  min-width: 40px;
}

.event-rank-1 .event-rank { color: #ffd700; }
.event-rank-2 .event-rank { color: #c0c0c0; }
.event-rank-3 .event-rank { color: #cd7f32; }

.event-name {
  font-weight: bold;
  font-size: 1.1em;
  color: #e6edf3;
  flex: 1;
}

.event-metrics {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.event-metric {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.metric-label {
  color: #8b949e;
  font-size: 0.85em;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.metric-value {
  font-weight: bold;
  color: #e6edf3;
}

.metric-value.score {
  font-size: 1.2em;
}

.score-excellent { color: #10b981; }
.score-good { color: #3b82f6; }
.score-average { color: #f59e0b; }
.score-poor { color: #ef4444; }

.event-pairs {
  padding-top: 10px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.pairs-label {
  color: #8b949e;
  font-size: 0.85em;
  font-weight: 600;
}

.pairs-list {
  color: #cbd5e0;
  font-size: 0.9em;
}

.events-placeholder {
  text-align: center;
  padding: 60px 20px;
  margin-top: 30px;
}

.placeholder-icon {
  font-size: 4em;
  margin-bottom: 20px;
  opacity: 0.5;
}

.events-placeholder h4 {
  color: #e6edf3;
  margin-bottom: 10px;
}

.events-placeholder p {
  color: #8b949e;
  margin: 5px 0;
}

.events-placeholder .hint {
  color: #58a6ff;
  font-style: italic;
  margin-top: 10px;
}

/* STRADDLE SECTION */
.straddle-section {
  margin-top: 30px;
  animation: slideUp 0.5s ease-out 0.3s both;
}

.straddle-section h3 {
  color: #e6edf3;
  margin-bottom: 20px;
  border-bottom: 2px solid #30363d;
  padding-bottom: 10px;
}

.straddle-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 20px;
  margin-bottom: 20px;
}

.straddle-card {
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 15px;
  transition: transform 0.2s, background 0.2s;
}

.straddle-card:hover {
  transform: translateY(-5px);
  background: rgba(255, 255, 255, 0.05);
}

.straddle-header {
  display: flex;
  align-items: center;
  gap: 15px;
  padding-bottom: 10px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.straddle-rank {
  font-size: 1.3em;
  font-weight: bold;
  color: #8b949e;
  min-width: 35px;
}

.straddle-rank-1 .straddle-rank { color: #ffd700; }
.straddle-rank-2 .straddle-rank { color: #c0c0c0; }
.straddle-rank-3 .straddle-rank { color: #cd7f32; }

.straddle-pair {
  font-weight: bold;
  font-size: 1.2em;
  color: #e6edf3;
  flex: 1;
}

.straddle-main-score {
  font-size: 1.8em;
  font-weight: bold;
  padding: 5px 15px;
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.05);
}

.straddle-metrics {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.straddle-metric {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.metric-bar-container {
  position: relative;
  background: #30363d;
  height: 8px;
  border-radius: 4px;
  overflow: hidden;
  display: flex;
  align-items: center;
}

.metric-bar {
  height: 100%;
  border-radius: 4px;
  transition: width 0.8s ease-out;
}

.metric-bar.directional {
  background: linear-gradient(90deg, #10b981, #059669);
}

.metric-bar.whipsaw {
  background: linear-gradient(90deg, #ef4444, #dc2626);
}

.metric-bar-value {
  position: absolute;
  right: 8px;
  font-size: 0.8em;
  font-weight: bold;
  color: #e6edf3;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.8);
}

.straddle-events {
  padding-top: 10px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.events-label {
  color: #8b949e;
  font-size: 0.85em;
  font-weight: 600;
  display: block;
  margin-bottom: 8px;
}

.events-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.events-list li {
  color: #cbd5e0;
  font-size: 0.85em;
  padding-left: 12px;
  position: relative;
}

.events-list li::before {
  content: "→";
  position: absolute;
  left: 0;
  color: #58a6ff;
}

.straddle-placeholder {
  text-align: center;
  padding: 60px 20px;
  margin-top: 30px;
}

.straddle-placeholder h4 {
  color: #e6edf3;
  margin-bottom: 10px;
}

.straddle-placeholder p {
  color: #8b949e;
  margin: 5px 0;
}

.straddle-placeholder .hint {
  color: #58a6ff;
  font-style: italic;
  margin-top: 10px;
}

/* TIMING SECTION */
.timing-section {
  margin-top: 30px;
  animation: slideUp 0.5s ease-out 0.4s both;
}

.timing-section h3 {
  color: #e6edf3;
  margin-bottom: 20px;
  border-bottom: 2px solid #30363d;
  padding-bottom: 10px;
}

.timing-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 20px;
  margin-bottom: 20px;
}

.timing-card {
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 15px;
  transition: transform 0.2s, background 0.2s;
}

.timing-card:hover {
  transform: translateY(-5px);
  background: rgba(255, 255, 255, 0.05);
}

.timing-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding-bottom: 12px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.timing-rank {
  font-size: 1.2em;
  font-weight: bold;
  color: #8b949e;
  min-width: 30px;
}

.timing-rank-1 .timing-rank { color: #ffd700; }
.timing-rank-2 .timing-rank { color: #c0c0c0; }
.timing-rank-3 .timing-rank { color: #cd7f32; }

.timing-event {
  font-weight: bold;
  font-size: 1.05em;
  color: #e6edf3;
  flex: 1;
  line-height: 1.3;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.event-name-original {
  color: #e6edf3;
  font-size: 1em;
}

.event-name-translation {
  color: #8b949e;
  font-size: 0.85em;
  font-weight: normal;
  font-style: italic;
}

.timing-consistency {
  font-size: 1.1em;
  font-weight: bold;
  padding: 4px 10px;
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.05);
}

.timing-metrics {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.timing-metric {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 12px;
  background: rgba(255, 255, 255, 0.02);
  border-radius: 8px;
  border-left: 3px solid #58a6ff;
}

.timing-metric .metric-icon {
  font-size: 1.5em;
  line-height: 1;
}

.timing-metric .metric-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.timing-metric .metric-label {
  color: #8b949e;
  font-size: 0.8em;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  font-weight: 600;
}

.timing-metric .metric-value {
  color: #e6edf3;
  font-size: 1.3em;
  font-weight: bold;
}

.timing-metric .metric-hint {
  color: #8b949e;
  font-size: 0.75em;
  font-style: italic;
}

.timing-footer {
  display: flex;
  align-items: center;
  gap: 8px;
  padding-top: 10px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  color: #8b949e;
  font-size: 0.85em;
}

.footer-label {
  color: #cbd5e0;
}

.footer-separator {
  color: #58a6ff;
}

.timing-placeholder {
  text-align: center;
  padding: 60px 20px;
  margin-top: 30px;
}

.timing-placeholder h4 {
  color: #e6edf3;
  margin-bottom: 10px;
}

.timing-placeholder p {
  color: #8b949e;
  margin: 5px 0;
}

.timing-placeholder .hint {
  color: #58a6ff;
  font-style: italic;
  margin-top: 10px;
}
</style>
