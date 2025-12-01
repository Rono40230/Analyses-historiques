<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  slPips: number
  tpPips: number
  peakDurationMinutes: number
  timeoutMinutes: number
  winRate: number
  whipsawFrequency: number
  entryTime: string
}

const props = defineProps<Props>()

const winRateColor = computed(() => {
  if (props.winRate >= 60) return '#22c55e' // green
  if (props.winRate >= 40) return '#eab308' // yellow
  return '#ef4444' // red
})

const whipsawRiskColor = computed(() => {
  if (props.whipsawFrequency < 10) return '#22c55e' // green
  if (props.whipsawFrequency < 35) return '#eab308' // yellow
  return '#ef4444' // red
})

const riskLevel = computed(() => {
  const whipsawRisk = props.whipsawFrequency
  if (whipsawRisk < 10) return 'LOW'
  if (whipsawRisk < 35) return 'MEDIUM'
  return 'HIGH'
})
</script>

<template>
  <div class="straddle-legend">
    <div class="legend-grid">
      <!-- Entry -->
      <div class="legend-item">
        <span class="label">Entry Time:</span>
        <span class="value">{{ entryTime }}</span>
      </div>

      <!-- SL -->
      <div class="legend-item">
        <span class="label">SL:</span>
        <span class="value">{{ slPips.toFixed(1) }} pips</span>
      </div>

      <!-- TP -->
      <div class="legend-item">
        <span class="label">TP:</span>
        <span class="value">{{ tpPips.toFixed(1) }} pips</span>
      </div>

      <!-- Peak Duration -->
      <div class="legend-item">
        <span class="label">Peak Duration:</span>
        <span class="value">{{ peakDurationMinutes }}min</span>
      </div>

      <!-- Timeout -->
      <div class="legend-item">
        <span class="label">Timeout:</span>
        <span class="value">{{ timeoutMinutes }}min</span>
      </div>

      <!-- Win Rate -->
      <div class="legend-item">
        <span class="label">Win Rate:</span>
        <span class="value" :style="{ color: winRateColor }">{{ winRate.toFixed(0) }}%</span>
      </div>

      <!-- Whipsaw Risk -->
      <div class="legend-item">
        <span class="label">Whipsaw Risk:</span>
        <span class="value" :style="{ color: whipsawRiskColor }">{{ whipsawFrequency.toFixed(0) }}% ({{ riskLevel }})</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.straddle-legend {
  background: rgba(45, 55, 72, 0.5);
  border: 1px solid #4a5568;
  border-radius: 6px;
  padding: 15px;
  width: 100%;
}

.legend-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 15px;
}

.legend-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px;
  background: rgba(0, 0, 0, 0.2);
  border-radius: 4px;
  border-left: 3px solid #667eea;
}

.label {
  color: #cbd5e0;
  font-weight: 600;
  font-size: 0.9em;
}

.value {
  color: #e2e8f0;
  font-weight: 700;
  font-size: 1em;
  font-family: monospace;
}
</style>
