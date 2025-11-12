<template>
  <div v-if="props.result" class="analysis-panel">
    <!-- DEBUG: Vérifier que result est chargé -->
    <div v-if="props.result" style="display: none;">{{ console.log('AnalysisPanel result:', props.result) }}</div>
    
    <div class="panel-header">
      <div class="header-title">
        <h2>🎯 Analyse: {{ props.result.symbol }}</h2>
        <select :value="currentSymbol" @change="(e) => onSymbolChange((e.target as HTMLSelectElement).value)" class="symbol-select">
          <option v-for="s in symbols" :key="s.symbol" :value="s.symbol">{{ s.symbol }}</option>
        </select>
      </div>
      <div class="badges">
        <span 
          :class="['badge', 'recommendation', recommendationClass]"
          :title="getRecommendationTooltip(props.result.recommendation)"
        >
          {{ formatRecommendation(props.result.recommendation) }}
        </span>
        <span 
          :class="['badge', 'risk', getRiskClass(props.result.risk_level)]"
          :title="getRiskTooltip(props.result.risk_level)"
        >
          {{ formatRisk(props.result.risk_level) }}
        </span>
      </div>
    </div>

    <div class="confidence-section">
      <h3 title="Score de Confiance&#10;Définition: Niveau de confiance dans la recommandation de trading&#10;Score: >= 80% = Très confiant | >= 65% = Confiant | >= 50% = Modéré | >= 35% = Prudent | < 35% = Ne pas trader">Score de Confiance</h3>
      <div class="confidence-bar" :style="{ width: props.result.confidence_score + '%' }"></div>
      <span class="confidence-text">{{ props.result.confidence_score }}%</span>
    </div>

    <div class="metrics-grid">
      <div class="metric-card">
        <h4 title="Nombre de Bougies&#10;Définition: Quantité totale de bougies analysées&#10;Score: Plus grand = Plus de données fiables">🎯 Bougies</h4>
        <div class="metric-value">{{ props.result.global_metrics.total_candles }}</div>
      </div>
      <div class="metric-card">
        <h4 title="ATR (Average True Range)&#10;Définition: Mesure la volatilité moyenne sur 14 périodes&#10;Score: > 0.001 = Excellent | > 0.0005 = Bon | > 0.0001 = Acceptable">� ATR Moyen</h4>
        <div class="metric-value">{{ props.result.global_metrics.mean_atr.toFixed(5) }}</div>
      </div>
      <div class="metric-card">
        <h4 title="Volatilité Globale&#10;Définition: Pourcentage moyen de variation des prix&#10;Score: < 5% = Très basse | 5-15% = Normale | > 15% = Élevée">📈 Volatilité</h4>
        <div class="metric-value">{{ (props.result.global_metrics.mean_volatility * 100).toFixed(2) }}%</div>
      </div>
      <div class="metric-card">
        <h4 title="Body Range&#10;Définition: Pourcentage du corps de la bougie (open-close) par rapport à l'amplitude totale&#10;Score: > 50% = Excellent | > 30% = Bon | > 10% = Acceptable">📦 Body Range</h4>
        <div class="metric-value">{{ props.result.global_metrics.mean_body_range.toFixed(1) }}%</div>
      </div>
      <div class="metric-card">
        <h4 title="Tick Quality&#10;Définition: Qualité des ticks - taille moyenne des mouvements&#10;Score: > 0.001 = Excellent | > 0.0005 = Bon (Plus élevé = Meilleur)">✨ Tick Quality</h4>
        <div class="metric-value">{{ props.result.global_metrics.mean_tick_quality.toFixed(5) }}</div>
      </div>
      <div class="metric-card">
        <h4 title="Noise Ratio&#10;Définition: Ratio bruit/signal - plus bas = moins de bruit inutile&#10;Score: < 2.0 = Excellent | < 3.0 = Bon | > 3.0 = Élevé (À éviter)">🔊 Noise Ratio</h4>
        <div class="metric-value">{{ props.result.global_metrics.mean_noise_ratio.toFixed(2) }}</div>
      </div>
      <div class="metric-card">
        <h4 title="Volume Imbalance&#10;Définition: Déséquilibre entre les volumes d'achat et de vente&#10;Score: Plus proche de 1.0 = Plus équilibré">⚖️ Volume Imbalance</h4>
        <div class="metric-value">{{ props.result.global_metrics.mean_volume_imbalance.toFixed(4) }}</div>
      </div>
      <div class="metric-card">
        <h4 title="Breakout %&#10;Définition: Pourcentage de bougies qui sortent des niveaux de support/résistance&#10;Score: < 10% = Peu de breakouts | 10-30% = Modéré | > 30% = Très actif">🚀 Breakout %</h4>
        <div class="metric-value">{{ props.result.global_metrics.mean_breakout_percentage.toFixed(1) }}%</div>
      </div>
    </div>
  </div>
  <div v-else class="loading">
    <p>Sélectionnez une paire pour analyser...</p>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface GlobalMetrics {
  mean_atr: number
  mean_volatility: number
  mean_body_range: number
  mean_tick_quality: number
  mean_noise_ratio: number
  mean_volume_imbalance: number
  mean_breakout_percentage: number
  total_candles: number
}

interface HourlyStats {
  hour: number
  candle_count: number
  atr_mean: number
}

interface CorrelatedEvent {
  event: any // CalendarEvent
  volatility_hour: number
  volatility_increase: number
  correlation_score: number
}

interface AnalysisResult {
  symbol: string
  period_start: string
  period_end: string
  timeframe: string
  recommendation: string
  risk_level: string
  confidence_score: number
  global_metrics: GlobalMetrics
  hourly_stats: HourlyStats[]
  best_hours: number[]
  correlated_events: CorrelatedEvent[]
}

const props = defineProps<{
  result: AnalysisResult | null
  symbols: Array<{ symbol: string; file_path: string }>
}>()

const emit = defineEmits<{
  symbolSelected: [symbol: string]
}>()

const currentSymbol = computed(() => props.result?.symbol || '')
const symbols = ref<Array<{ symbol: string; file_path: string }>>([])

onMounted(async () => {
  try {
    symbols.value = props.symbols || await invoke('load_symbols')
  } catch (err) {
    console.error('Erreur:', err)
  }
})

function onSymbolChange(newSymbol: string) {
  if (newSymbol && newSymbol !== props.result?.symbol) {
    emit('symbolSelected', newSymbol)
  }
}

function formatRecommendation(rec: string): string {
  const map: { [key: string]: string } = {
    'BUY': '✅ ACHETER',
    'SELL': '⛔ VENDRE',
    'HOLD': '⏸️ ATTENDRE'
  }
  return map[rec] || rec
}

function formatRisk(risk: string): string {
  const map: { [key: string]: string } = {
    'HIGH': '🔴 ÉLEVÉ',
    'MEDIUM': '🟡 MOYEN',
    'LOW': '🟢 BAS'
  }
  return map[risk] || risk
}

function getRecommendationTooltip(rec: string): string {
  const tooltips: { [key: string]: string } = {
    'BUY': 'ACHETER - Le marché offre une configuration favorable pour un achat',
    'SELL': 'VENDRE - Le marché offre une configuration favorable pour une vente',
    'HOLD': 'ATTENDRE - Le marché n\'offre pas de configuration clairement favorable'
  }
  return tooltips[rec] || rec
}

function getRiskClass(risk: string): string {
  const map: { [key: string]: string } = {
    'HIGH': 'high',
    'MEDIUM': 'medium',
    'LOW': 'low'
  }
  return map[risk] || ''
}

function getRiskTooltip(risk: string): string {
  const tooltips: { [key: string]: string } = {
    'HIGH': 'Risque ÉLEVÉ - Volatilité > 15% ou conditions instables. À approcher avec prudence.',
    'MEDIUM': 'Risque MOYEN - Volatilité 5-15% avec conditions acceptables. Gestion stricte du risque recommandée.',
    'LOW': 'Risque BAS - Volatilité < 5% avec conditions stables. Favorise les positions plus agressives.'
  }
  return tooltips[risk] || risk
}

const recommendationClass = computed(() => {
  if (props.result?.recommendation === 'BUY') return 'buy'
  if (props.result?.recommendation === 'SELL') return 'sell'
  return 'hold'
})
</script>

<style scoped>
.analysis-panel { background: #161b22; padding: 30px; border-radius: 12px; border: 1px solid #30363d; }
.panel-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 30px; }
.header-title { display: flex; align-items: center; gap: 15px; }
.header-title h2 { margin: 0; }
.symbol-select { padding: 8px 12px; border: 2px solid #30363d; background: #1a202c; color: #000000; border-radius: 6px; cursor: pointer; font-weight: 600; }
.symbol-select option { background: #ffffff; color: #000000; }
.badges { display: flex; gap: 10px; }
.badge { padding: 8px 16px; border-radius: 6px; font-weight: 600; font-size: 0.9em; color: white; cursor: help; transition: all 0.2s; border: 2px solid transparent; }
.badge:hover { transform: translateY(-2px); box-shadow: 0 4px 12px rgba(0,0,0,0.3); }
.recommendation.buy { background: linear-gradient(135deg, #10b981 0%, #059669 100%); border-color: #047857; }
.recommendation.sell { background: linear-gradient(135deg, #dc2626 0%, #b91c1c 100%); border-color: #991b1b; }
.recommendation.hold { background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%); border-color: #b45309; }
.badge.risk.low { background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%); border-color: #15803d; }
.badge.risk.medium { background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%); border-color: #b45309; }
.badge.risk.high { background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%); border-color: #b91c1c; }
.confidence-section { background: #1a202c; padding: 20px; border-radius: 8px; margin-bottom: 20px; }
.confidence-bar { height: 8px; background: #667eea; border-radius: 4px; }
.confidence-text { color: #cbd5e0; font-size: 0.9em; }
.metrics-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 15px; margin-bottom: 30px; }
.metric-card { background: #1a202c; padding: 15px; border-radius: 8px; border-left: 3px solid #667eea; }
.metric-card h4 { margin: 0 0 10px 0; color: #e2e8f0; }
.metric-value { font-size: 1.5em; font-weight: bold; color: #667eea; }
.metric-card small { color: #a0aec0; }
.volatility-section { background: #1a202c; padding: 20px; border-radius: 8px; margin-bottom: 20px; }
.session-stats { display: grid; grid-template-columns: repeat(auto-fit, minmax(120px, 1fr)); gap: 10px; margin-top: 15px; }
.session-stat { background: #2d3748; padding: 10px; border-radius: 6px; text-align: center; }
.session-name { color: #cbd5e0; font-size: 0.85em; }
.stat-value { color: #667eea; font-weight: bold; }
.metrics-detailed { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px; margin-top: 15px; }
.metric-item { background: #2d3748; padding: 12px; border-radius: 6px; display: flex; justify-content: space-between; align-items: center; }
.metric-label { color: #cbd5e0; font-size: 0.9em; }
.metric-item .metric-value { color: #667eea; font-weight: bold; }
.recommendations-section { background: #1a202c; padding: 20px; border-radius: 8px; }
.recommendations-section h3 { color: #e2e8f0; margin-top: 0; cursor: help; }
.recommendation-text { color: #e2e8f0; padding: 15px; background: #2d3748; border-left: 4px solid #667eea; border-radius: 4px; }
.recommendation-header { display: flex; align-items: center; gap: 10px; margin-bottom: 15px; padding-bottom: 10px; border-bottom: 1px solid #3d4758; }
.rec-type { font-size: 1.1em; font-weight: bold; color: #58a6ff; }
.rec-confidence { color: #8b949e; font-size: 0.9em; }
.recommendation-details p { margin: 10px 0; line-height: 1.7; }
.recommendation-details p strong { color: #58a6ff; }
.metrics-summary { margin-top: 12px; padding-top: 12px; border-top: 1px solid #3d4758; font-size: 0.85em; color: #a0aec0; }
.metrics-summary span { margin-right: 15px; }
.loading { text-align: center; padding: 40px; color: #a0aec0; }
</style>
