<template>
  <div class="straddle-performance-section">
    <h4>📊 Performance Straddle Simulée</h4>
    <div class="performance-grid">
      <!-- Win Rate -->
      <div class="performance-metric">
        <div class="metric-label">
          Win Rate
        </div>
        <div
          v-if="winRate"
          class="metric-display"
        >
          <span
            class="metric-value"
            :style="{ color: winRateColor }"
          >{{ winRate.win_rate_percentage.toFixed(1) }}%</span>
          <span class="metric-subtext">({{ winRate.wins }}/{{ winRate.total_trades }} trades)</span>
        </div>
        <div
          v-else
          class="metric-loading"
        >
          <span>⏳ Calcul...</span>
        </div>
      </div>

      <!-- Whipsaw Frequency -->
      <div class="performance-metric">
        <div class="metric-label">
          Fréquence Whipsaw
        </div>
        <div
          v-if="whipsawAnalysis"
          class="metric-display"
        >
          <span
            class="metric-value"
            :style="{ color: whipsawAnalysis.risk_color }"
          >{{ whipsawAnalysis.whipsaw_frequency_percentage.toFixed(1) }}%</span>
          <span class="metric-subtext">({{ whipsawAnalysis.risk_level }})</span>
        </div>
        <div
          v-else
          class="metric-loading"
        >
          <span>⏳ Calcul...</span>
        </div>
      </div>

      <!-- Offset Optimal -->
      <div class="performance-metric">
        <div class="metric-label">
          Offset Optimal
        </div>
        <div
          v-if="offsetOptimal"
          class="metric-display"
        >
          <span class="metric-value">{{ offsetOptimal.offset_pips.toFixed(1) }} pips</span>
          <span class="metric-subtext">(P95: {{ offsetOptimal.percentile_95_wicks.toFixed(1) }})</span>
        </div>
        <div
          v-else
          class="metric-loading"
        >
          <span>⏳ Calcul...</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  winRate: any
  whipsawAnalysis: any
  offsetOptimal: any
  winRateColor: string
}>()
</script>

<style scoped>
.straddle-performance-section {
  background: linear-gradient(135deg, rgba(139, 92, 246, 0.05) 0%, rgba(168, 85, 247, 0.05) 100%);
  border-left: 3px solid #a855f7;
  padding: 14px;
  border-radius: 6px;
  margin-top: 12px;
}

.straddle-performance-section h4 {
  color: #e9d5ff;
  font-size: 13px;
  font-weight: 600;
  margin: 0 0 10px 0;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.performance-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 12px;
}

.performance-metric {
  background: rgba(30, 30, 45, 0.6);
  border: 1px solid rgba(168, 85, 247, 0.2);
  border-radius: 6px;
  padding: 12px;
  text-align: center;
}

.metric-label {
  font-size: 11px;
  color: #cbd5e1;
  text-transform: uppercase;
  letter-spacing: 0.4px;
  margin-bottom: 8px;
  font-weight: 600;
}

.metric-display {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.metric-value {
  font-size: 18px;
  font-weight: bold;
  line-height: 1;
}

.metric-subtext {
  font-size: 10px;
  color: #94a3b8;
}

.metric-loading {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 40px;
  color: #64748b;
  font-size: 12px;
  font-style: italic;
}
</style>
