<template>
  <div class="stats-section">
    <div class="stat-item">
      <span class="stat-label">Noise Ratio AVANT:</span>
      <span class="stat-value" :class="noiseQualityBefore">{{ noiseRatioBefore.toFixed(2) }}</span>
    </div>
    <div class="stat-item">
      <span class="stat-label">Noise Ratio PENDANT:</span>
      <span class="stat-value" :class="noiseQualityDuring">{{ noiseRatioDuring.toFixed(2) }}</span>
    </div>
    <div class="stat-item">
      <span class="stat-label">Noise Ratio APRÈS:</span>
      <span class="stat-value" :class="noiseQualityAfter">{{ noiseRatioAfter.toFixed(2) }}</span>
    </div>
    <div class="stat-item impact-item">
      <span class="stat-label">Impact Volatilité:</span>
      <span class="stat-value impact-value">+{{ volatilityIncreasePercent.toFixed(1) }}%</span>
    </div>
    <div class="stat-item">
      <span class="stat-label">Occurrences analysées:</span>
      <span class="stat-value">{{ eventCount }}</span>
    </div>
  </div>

  <RetroAnalysisBidiParams
    :meilleur-moment="meilleurMoment"
    :offset="offset"
    :stop-loss="stopLoss"
    :trailing-stop="trailingStop"
    :timeout="timeout"
    :stop-loss-recovery="stopLossRecovery"
    :pair="pair"
  />
</template>

<script setup lang="ts">
import { computed } from 'vue'
import RetroAnalysisBidiParams from './RetroAnalysisBidiParams.vue'

interface Props {
  noiseRatioBefore: number
  noiseRatioDuring: number
  noiseRatioAfter: number
  volatilityIncreasePercent: number
  eventCount: number
  meilleurMoment?: number
  stopLoss?: number
  trailingStop?: number
  timeout?: number
  offset?: number
  stopLossRecovery?: number
  pair: string
}

const props = withDefaults(defineProps<Props>(), {
  noiseRatioBefore: 0,
  noiseRatioDuring: 0,
  noiseRatioAfter: 0,
  volatilityIncreasePercent: 0,
  eventCount: 0,
  meilleurMoment: 0,
  stopLoss: 0,
  trailingStop: 0,
  timeout: 60,
  offset: 0,
  stopLossRecovery: 0,
  pair: 'EURUSD'
})

const noiseQualityBefore = computed(() => {
  return props.noiseRatioBefore < 1.5 ? 'clean' : props.noiseRatioBefore < 2.5 ? 'mixed' : 'choppy'
})

const noiseQualityDuring = computed(() => {
  return props.noiseRatioDuring < 1.5 ? 'clean' : props.noiseRatioDuring < 2.5 ? 'mixed' : 'choppy'
})

const noiseQualityAfter = computed(() => {
  return props.noiseRatioAfter < 1.5 ? 'clean' : props.noiseRatioAfter < 2.5 ? 'mixed' : 'choppy'
})
</script>

<style scoped>
.stats-section {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 12px;
  margin-bottom: 10px;
}

.stat-item {
  background: #0d1117;
  padding: 10px 12px;
  border-radius: 6px;
  border: 1px solid #30363d;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.stat-label {
  color: #8b949e;
  font-size: 0.8em;
  font-weight: 500;
  text-transform: uppercase;
}

.stat-value {
  font-weight: 600;
  font-size: 1.1em;
}

.stat-value.clean {
  color: #3fb950;
}

.stat-value.mixed {
  color: #fbbf24;
}

.stat-value.choppy {
  color: #f85149;
}

.impact-item .impact-value {
  color: #58a6ff;
  font-size: 1.15em;
}

@media (max-width: 1024px) {
  .stats-section { grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 10px; }
}

@media (max-width: 768px) {
  .stats-section { grid-template-columns: repeat(2, 1fr); gap: 8px; }
  .stat-item { padding: 8px; }
  .stat-label { font-size: 0.7em; }
  .stat-value { font-size: 0.95em; }
}

@media (max-width: 480px) {
  .stats-section { grid-template-columns: 1fr; gap: 6px; }
  .stat-item { padding: 6px 8px; flex-direction: row; justify-content: space-between; align-items: center; }
  .stat-label { font-size: 0.65em; }
  .stat-value { font-size: 0.9em; }
}
</style>
