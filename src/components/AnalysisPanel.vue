<template>
  <div v-if="result" class="analysis-panel">
    <div class="panel-header">
      <h2>🎯 Analyse : {{ result.symbol }}</h2>
      <div class="header-badges">
        <span :class="['badge', 'recommendation', recommendationClass]">
          {{ formatRecommendation(result.recommendation) }}
        </span>
        <span :class="['badge', 'risk', riskClass]">
          Risque: {{ formatRisk(result.risk_level) }}
        </span>
      </div>
    </div>

    <div class="confidence-section">
      <h3>Score de Confiance</h3>
      <div class="confidence-bar-container">
        <div 
          class="confidence-bar" 
          :style="{ width: `${result.confidence_score}%` }"
          :class="confidenceClass"
        ></div>
        <span class="confidence-value">{{ result.confidence_score.toFixed(1) }}%</span>
      </div>
    </div>

    <div class="metrics-grid">
      <div class="metric-card">
        <div class="metric-icon">📈</div>
        <div class="metric-label">ATR Moyen</div>
        <div class="metric-value">{{ formatNumber(result.global_metrics.mean_atr, 5) }}</div>
      </div>
      
      <div class="metric-card">
        <div class="metric-icon">📊</div>
        <div class="metric-label">Volatilité</div>
        <div class="metric-value">{{ (result.global_metrics.mean_volatility * 100).toFixed(2) }}%</div>
      </div>
      
      <div class="metric-card">
        <div class="metric-icon">🎯</div>
        <div class="metric-label">Body Range</div>
        <div class="metric-value">{{ result.global_metrics.mean_body_range.toFixed(1) }}%</div>
      </div>
      
      <div class="metric-card">
        <div class="metric-icon">⚡</div>
        <div class="metric-label">Tick Quality</div>
        <div class="metric-value">{{ formatNumber(result.global_metrics.mean_tick_quality, 5) }}</div>
      </div>
      
      <div class="metric-card">
        <div class="metric-icon">🔊</div>
        <div class="metric-label">Noise Ratio</div>
        <div class="metric-value">{{ result.global_metrics.mean_noise_ratio.toFixed(2) }}</div>
      </div>
      
      <div class="metric-card">
        <div class="metric-icon">📦</div>
        <div class="metric-label">Total Bougies</div>
        <div class="metric-value">{{ result.global_metrics.total_candles.toLocaleString() }}</div>
      </div>
    </div>

    <div class="best-hours-section">
      <h3>⭐ Top 5 Heures de Trading (UTC)</h3>
      <div class="hours-badges">
        <span 
          v-for="hour in result.best_hours" 
          :key="hour"
          class="hour-badge"
        >
          {{ formatHour(hour) }}
        </span>
      </div>
    </div>

    <!-- Section Événements Économiques Corrélés -->
    <div v-if="result.correlated_events && result.correlated_events.length > 0" class="correlated-events-section">
      <h3>📅 Événements Économiques Corrélés</h3>
      <p class="section-subtitle">
        Événements détectés pendant les périodes de haute volatilité
      </p>
      <div class="events-list">
        <div 
          v-for="(corr, index) in result.correlated_events" 
          :key="index"
          class="event-card"
          :class="impactClass(corr.event.impact)"
        >
          <div class="event-header">
            <span class="event-icon">📅</span>
            <span class="event-time">{{ formatEventTime(corr.event.event_time) }}</span>
            <span :class="['event-impact', impactClass(corr.event.impact)]">
              {{ corr.event.impact }}
            </span>
          </div>
          <div class="event-title">{{ corr.event.description }}</div>
          <div class="event-metrics">
            <div class="event-metric">
              <span class="metric-label">Heure de volatilité:</span>
              <span class="metric-value">{{ formatHour(corr.volatility_hour) }}</span>
            </div>
            <div class="event-metric">
              <span class="metric-label">Augmentation:</span>
              <span class="metric-value volatility-increase">+{{ corr.volatility_increase.toFixed(1) }}%</span>
            </div>
            <div class="event-metric">
              <span class="metric-label">Score corrélation:</span>
              <span class="metric-value">{{ corr.correlation_score.toFixed(1) }}</span>
            </div>
          </div>
          <div v-if="hasEconomicData(corr.event)" class="event-data">
            <span v-if="corr.event.actual !== null" class="data-item">
              Réel: <strong>{{ corr.event.actual }}</strong>
            </span>
            <span v-if="corr.event.forecast !== null" class="data-item">
              Prévu: {{ corr.event.forecast }}
            </span>
            <span v-if="corr.event.previous !== null" class="data-item">
              Précédent: {{ corr.event.previous }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { AnalysisResult } from '../stores/volatility'

const props = defineProps<{
  result: AnalysisResult | null
}>()

const recommendationClass = computed(() => {
  if (!props.result) return ''
  const rec = props.result.recommendation
  if (rec.includes('Aggressive')) return 'aggressive'
  if (rec.includes('Normal')) return 'normal'
  if (rec.includes('Cautious')) return 'cautious'
  if (rec.includes('NoTrade')) return 'no-trade'
  return ''
})

const riskClass = computed(() => {
  if (!props.result) return ''
  const risk = props.result.risk_level.toLowerCase()
  if (risk.includes('high')) return 'high'
  if (risk.includes('medium')) return 'medium'
  return 'low'
})

const confidenceClass = computed(() => {
  if (!props.result) return ''
  const score = props.result.confidence_score
  if (score >= 80) return 'excellent'
  if (score >= 60) return 'good'
  if (score >= 40) return 'fair'
  return 'poor'
})

function formatRecommendation(rec: string): string {
  if (rec === 'ScalpAggressive') return 'Scalp Agressif'
  if (rec === 'ScalpNormal') return 'Scalp Normal'
  if (rec === 'ScalpCautious') return 'Scalp Prudent'
  if (rec === 'VeryCautious') return 'Très Prudent'
  if (rec === 'NoTrade') return 'Pas de Trading'
  return rec
}

function formatRisk(risk: string): string {
  if (risk === 'High') return 'Élevé'
  if (risk === 'Medium') return 'Moyen'
  if (risk === 'Low') return 'Faible'
  return risk
}

function formatHour(hour: number): string {
  return `${hour.toString().padStart(2, '0')}:00`
}

function formatNumber(num: number, decimals: number): string {
  return num.toFixed(decimals)
}

function formatEventTime(dateTimeStr: string): string {
  const date = new Date(dateTimeStr)
  return date.toLocaleString('fr-FR', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

function impactClass(impact: string): string {
  const upper = impact.toUpperCase()
  if (upper === 'HIGH') return 'impact-high'
  if (upper === 'MEDIUM') return 'impact-medium'
  return 'impact-low'
}

function hasEconomicData(event: any): boolean {
  return event.actual !== null || event.forecast !== null || event.previous !== null
}
</script>

<style scoped>
.analysis-panel {
  background: #161b22;
  border-radius: 12px;
  padding: 2rem;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
  border: 1px solid #30363d;
  margin-bottom: 2rem;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
  border-bottom: 2px solid #30363d;
  padding-bottom: 1rem;
}

.panel-header h2 {
  margin: 0;
  color: #e6edf3;
}

.header-badges {
  display: flex;
  gap: 0.5rem;
}

.badge {
  padding: 0.5rem 1rem;
  border-radius: 20px;
  font-weight: bold;
  font-size: 0.9rem;
}

.recommendation.aggressive {
  background: #22c55e;
  color: white;
}

.recommendation.normal {
  background: #3b82f6;
  color: white;
}

.recommendation.cautious {
  background: #f59e0b;
  color: white;
}

.recommendation.no-trade {
  background: #ef4444;
  color: white;
}

.risk.high {
  background: #2d1215;
  color: #f97583;
  border: 1px solid #dc3545;
}

.risk.medium {
  background: #2d2715;
  color: #f9c513;
  border: 1px solid #ffc107;
}

.risk.low {
  background: #1a2d1f;
  color: #3fb950;
  border: 1px solid #22c55e;
}

.confidence-section {
  margin-bottom: 2rem;
}

.confidence-section h3 {
  margin: 0 0 1rem 0;
  color: #8b949e;
}

.confidence-bar-container {
  position: relative;
  width: 100%;
  height: 40px;
  background: #0d1117;
  border-radius: 20px;
  overflow: hidden;
  border: 1px solid #30363d;
}

.confidence-bar {
  height: 100%;
  transition: width 0.5s ease;
  border-radius: 20px;
}

.confidence-bar.excellent {
  background: linear-gradient(90deg, #22c55e, #10b981);
}

.confidence-bar.good {
  background: linear-gradient(90deg, #3b82f6, #2563eb);
}

.confidence-bar.fair {
  background: linear-gradient(90deg, #f59e0b, #d97706);
}

.confidence-bar.poor {
  background: linear-gradient(90deg, #ef4444, #dc2626);
}

.confidence-value {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-weight: bold;
  color: #e6edf3;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.5);
}

.metrics-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.metric-card {
  background: linear-gradient(135deg, #0d1117 0%, #161b22 100%);
  padding: 1.5rem;
  border-radius: 12px;
  text-align: center;
  border: 1px solid #30363d;
}

.metric-icon {
  font-size: 2rem;
  margin-bottom: 0.5rem;
}

.metric-label {
  font-size: 0.9rem;
  color: #8b949e;
  margin-bottom: 0.5rem;
}

.metric-value {
  font-size: 1.5rem;
  font-weight: bold;
  color: #e6edf3;
}

.best-hours-section h3 {
  margin: 0 0 1rem 0;
  color: #8b949e;
}

.hours-badges {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.hour-badge {
  background: linear-gradient(135deg, #1f6feb 0%, #388bfd 100%);
  color: white;
  padding: 0.75rem 1.5rem;
  border-radius: 25px;
  font-weight: bold;
  font-size: 1.1rem;
}

/* Événements Économiques Corrélés */
.correlated-events-section {
  margin-top: 2rem;
  padding-top: 2rem;
  border-top: 2px solid #30363d;
}

.correlated-events-section h3 {
  margin: 0 0 0.5rem 0;
  color: #e6edf3;
  font-size: 1.5rem;
}

.section-subtitle {
  color: #8b949e;
  font-size: 0.9rem;
  margin-bottom: 1.5rem;
}

.events-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.event-card {
  background: #0d1117;
  border-left: 4px solid #3b82f6;
  border-radius: 8px;
  padding: 1.5rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  transition: transform 0.2s, box-shadow 0.2s;
  border: 1px solid #30363d;
}

.event-card:hover {
  transform: translateX(4px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.5);
}

.event-card.impact-high {
  border-left-color: #dc2626;
  background: linear-gradient(to right, #2d1215 0%, #0d1117 10%);
}

.event-card.impact-medium {
  border-left-color: #f59e0b;
  background: linear-gradient(to right, #2d2715 0%, #0d1117 10%);
}

.event-card.impact-low {
  border-left-color: #10b981;
  background: linear-gradient(to right, #1a2d1f 0%, #0d1117 10%);
}

.event-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 0.75rem;
}

.event-icon {
  font-size: 1.5rem;
}

.event-time {
  font-size: 0.9rem;
  color: #8b949e;
  font-weight: 500;
}

.event-impact {
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: bold;
  text-transform: uppercase;
  margin-left: auto;
}

.event-impact.impact-high {
  background: #dc2626;
  color: white;
}

.event-impact.impact-medium {
  background: #f59e0b;
  color: white;
}

.event-impact.impact-low {
  background: #10b981;
  color: white;
}

.event-title {
  font-size: 1.1rem;
  font-weight: 600;
  color: #e6edf3;
  margin-bottom: 1rem;
}

.event-metrics {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 0.75rem;
  margin-bottom: 1rem;
}

.event-metric {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.event-metric .metric-label {
  font-size: 0.8rem;
  color: #8b949e;
}

.event-metric .metric-value {
  font-size: 1rem;
  font-weight: 600;
  color: #e6edf3;
}

.event-metric .volatility-increase {
  color: #f97583;
  font-weight: bold;
}

.event-data {
  display: flex;
  gap: 1.5rem;
  padding-top: 1rem;
  border-top: 1px solid #30363d;
  font-size: 0.9rem;
}

.data-item {
  color: #8b949e;
}

.data-item strong {
  color: #e6edf3;
  font-weight: 700;
}
</style>
