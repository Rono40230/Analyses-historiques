<template>
  <div class="bidi-parameters-section">
    <h4>⚙️ PARAMÈTRES BIDI OPTIMISÉS</h4>
    <div class="metrics-grid">
      <div class="metric-card">
        <div class="metric-label">
          Meilleur Moment
        </div>
        <div class="metric-value" style="color: #fff;">
          {{ getBestTimeDisplay() }}
        </div>
      </div>
      <MetricTooltip
        title="Winrate"
        direction="top"
      >
        <div class="metric-card">
          <div class="metric-label">
            Winrate
          </div>
          <div class="metric-value" :style="{ color: getWinrateColor(entryWindowAnalysis.optimal_win_rate * 100) }">
            {{ (entryWindowAnalysis.optimal_win_rate * 100).toFixed(0) }}% <span class="unit">événement</span>
          </div>
        </div>
        <template #definition>
          <div class="tooltip-section">
            <div class="tooltip-section-title">📖 Définition</div>
            <div class="tooltip-section-text">Pourcentage de fois où le créneau horaire élu a produit un mouvement straddle gagnant (atteint TP avant SL), calculé sur l'historique complet.</div>
          </div>
        </template>
        <template #interpretation>
          <div class="tooltip-section">
            <div class="tooltip-section-title">📊 Interprétation du Score</div>
            <div class="interpretation-item"><strong>🟢 Excellent:</strong> ≥65% → Biais fortement positif, très fiable</div>
            <div class="interpretation-item"><strong>🔵 Bon:</strong> 55-64% → Biais positif clair, profitable long terme</div>
            <div class="interpretation-item"><strong>🟡 Acceptable:</strong> 50-54% → Légèrement positif, margin serré</div>
            <div class="interpretation-item"><strong>🔴 Faible:</strong> &lt;50% → Biais négatif, dangereux à trader</div>
          </div>
        </template>
        <template #color-scale>
          <div class="tooltip-section">
            <div class="tooltip-section-title">🎨 Échelle de Couleurs</div>
            <div class="color-item excellent">🟢 Excellent: ≥65%</div>
            <div class="color-item good">🔵 Bon: 55-64%</div>
            <div class="color-item acceptable">🟡 Acceptable: 50-54%</div>
            <div class="color-item poor">🔴 Faible: &lt;50%</div>
          </div>
        </template>
      </MetricTooltip>
      <MetricTooltip
        title="Stop Loss"
        direction="top"
      >
        <div class="metric-card">
          <div class="metric-label">
            Stop Loss
          </div>
          <div class="metric-value" :style="{ color: getSlColor(analysis.tradingPlan.slPoints, analysis.tradingPlan.atrPoints) }">
            {{ Math.ceil(analysis.tradingPlan.slPoints) }} <span class="unit">points, soit {{ Math.ceil(analysis.tradingPlan.slPoints) }} pips</span>
          </div>
        </div>
        <template #definition>
          <div class="tooltip-section">
            <div class="tooltip-section-title">📖 Définition</div>
            <div class="tooltip-section-text">Distance en points entre l'entrée et le niveau de stop loss (limite maximale de perte). Adapté dynamiquement selon la volatilité réelle du créneau.</div>
          </div>
        </template>
        <template #interpretation>
          <div class="tooltip-section">
            <div class="tooltip-section-title">📊 Interprétation du Score</div>
            <div class="interpretation-item"><strong>🟢 Excellent:</strong> SL ≤0.8×ATR → Très serré, marché calme, peu de bruit</div>
            <div class="interpretation-item"><strong>🔵 Bon:</strong> SL ≈1.0×ATR → Standard, volatilité normale, straddle symétrique</div>
            <div class="interpretation-item"><strong>🟡 Acceptable:</strong> SL ≈1.2×ATR → Élargi, volatilité augmente, plus de marge</div>
            <div class="interpretation-item"><strong>🔴 Faible:</strong> SL ≥1.5×ATR → Très large, volatilité extrême, risque augmenté</div>
          </div>
        </template>
        <template #color-scale>
          <div class="tooltip-section">
            <div class="tooltip-section-title">🎨 Échelle de Couleurs</div>
            <div class="color-item excellent">🟢 Excellent: &lt;0.8×ATR</div>
            <div class="color-item good">🔵 Bon: 0.8-1.1×ATR</div>
            <div class="color-item acceptable">🟡 Acceptable: 1.1-1.3×ATR</div>
            <div class="color-item poor">🔴 Faible: ≥1.3×ATR</div>
          </div>
        </template>
      </MetricTooltip>
      <MetricTooltip
        title="Trailing Stop"
        direction="top"
      >
        <div class="metric-card">
          <div class="metric-label">
            Trailing Stop
          </div>
          <div class="metric-value" style="color: #fff;">
            {{ analysis.tradingPlan.trailingStopCoefficient.toFixed(2) }}x <span class="unit">SL</span>
          </div>
        </div>
        <template #definition>
          <div class="tooltip-section">
            <div class="tooltip-section-title">📖 Définition</div>
            <div class="tooltip-section-text">Multiplicateur du SL utilisé pour placer un stop dynamique qui monte avec le prix, protégeant les gains tout en laissant la position croître. S'active après TP initial touché.</div>
          </div>
        </template>
        <template #interpretation>
          <div class="tooltip-section">
            <div class="tooltip-section-title">📊 Explication</div>
            <div class="tooltip-section-text">La valeur est calculée de manière optimale en fonction de la volatilité réelle du créneau. Elle représente le multiplicateur du SL : une valeur de 0.9x signifie que le trailing stop sera placé à 90% du SL initial, protégeant les gains tout en permettant une croissance potentielle.</div>
          </div>
        </template>
      </MetricTooltip>
      <div class="metric-card">
        <div class="metric-label">
          Timeout Recommandé
        </div>
        <div class="metric-value" style="color: #fff;">
          {{ Math.round((volatilityDuration?.peak_duration_minutes || 21) * 1.5) }} <span class="unit">min</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import MetricTooltip from '../MetricTooltip.vue'
import { useMetricsFormatting } from '../../composables/useMetricsFormatting'
import {
  getWinrateColor,
  getSlColor,
  getTsColor,
  getOffsetColor,
  getTimeoutColor
} from './BidiParametersSection.helpers'

const props = defineProps<{
  sliceAnalyses: any[]
  entryWindowAnalysis: any
  analysis: any
  volatilityDuration: any
}>()

const { calculateExactTime } = useMetricsFormatting()

const getBestTimeDisplay = () => {
  if (props.sliceAnalyses && props.sliceAnalyses.length > 0) {
    const bestSlice = props.sliceAnalyses[0]
    return calculateExactTime(bestSlice.slice.startTime, props.entryWindowAnalysis.optimal_offset)
  }
  return props.entryWindowAnalysis.optimal_offset + ' min'
}
</script>

<style scoped>
.bidi-parameters-section {
  margin-top: 20px;
  padding: 20px;
  background: linear-gradient(135deg, rgba(45, 90, 123, 0.15) 0%, rgba(78, 205, 196, 0.1) 100%);
  border: 1px solid #2d5a7b;
  border-radius: 8px;
}

.bidi-parameters-section h4 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: #e0e7ff;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.metrics-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  gap: 10px;
  margin-top: 15px;
}

.metric-card {
  padding: 12px;
  background: rgba(255,255,255,0.05);
  border-radius: 6px;
}

.metric-label {
  font-size: 11px;
  color: #999;
  margin-bottom: 6px;
  text-transform: uppercase;
}

.metric-value {
  font-size: 13px;
  color: #4ecdc4;
  font-weight: bold;
}

.unit {
  color: #888;
  font-size: 11px;
}

.interpretation-item { margin: 8px 0; font-size: 12px; line-height: 1.4; color: #cbd5e1; padding: 6px; border-left: 2px solid #4ecdc4; }
.interpretation-item strong { color: #e2e8f0; }
.color-item { display: inline-block; padding: 4px 8px; margin: 4px; border-radius: 3px; font-size: 11px; font-weight: 600; border: 1px solid; }
.color-item.excellent { background: rgba(16, 185, 129, 0.15); color: #10b981; border-color: #10b981; }
.color-item.good { background: rgba(59, 130, 246, 0.15); color: #3b82f6; border-color: #3b82f6; }
.color-item.acceptable { background: rgba(234, 179, 8, 0.15); color: #eab308; border-color: #eab308; }
.color-item.poor { background: rgba(239, 68, 68, 0.15); color: #ef4444; border-color: #ef4444; }
.tooltip-section { margin-bottom: 12px; }
.tooltip-section:last-child { margin-bottom: 0; }
.tooltip-section-title { font-weight: 600; color: #58a6ff; margin-bottom: 8px; font-size: 0.9em; text-transform: uppercase; letter-spacing: 0.5px; }
.tooltip-section-text { color: #cbd5e0; font-size: 0.9em; line-height: 1.6; white-space: pre-wrap; word-wrap: break-word; }
</style>
