<template>
  <div class="metrics-grid">
    <div
      v-for="(metric, index) in displayedMetrics"
      :key="index"
      class="metric-card"
    >
      <h4>{{ metric.label }}</h4>
      <div
        :class="['metric-value', getColorClass(metric.key, metric.value)]"
      >
        {{ metric.formattedValue }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface GlobalMetrics {
  mean_atr: number
  mean_volatility: number
  mean_body_range: number
  mean_tick_quality: number
  mean_noise_ratio: number
  mean_volume_imbalance: number
  mean_breakout_percentage: number
  mean_range: number
  total_candles: number
}

const props = defineProps<{
  globalMetrics: GlobalMetrics
  estimatedPrice: number
}>()

function getMetricQuality(metric: string, value: number): string {
  switch (metric) {
    case 'bougies':
      if (value > 500) return 'excellent'
      if (value > 200) return 'good'
      if (value > 100) return 'acceptable'
      return 'poor'
    case 'atr':
    case 'range':
      const atrPercent = value < 1 ? value * 100 : (value / props.estimatedPrice) * 100
      if (atrPercent > 2.5) return 'excellent'
      if (atrPercent > 1.5) return 'good'
      if (atrPercent > 1.0) return 'acceptable'
      return 'poor'
    case 'volatility':
      if (value >= 0.30) return 'excellent'
      if (value >= 0.15) return 'good'
      if (value >= 0.05) return 'acceptable'
      return 'poor'
    case 'bodyrange':
      if (value > 45) return 'excellent'
      if (value > 35) return 'good'
      if (value > 15) return 'acceptable'
      return 'poor'
    case 'tickquality':
      const tickPercent = value < 1 ? value * 100 : (value / props.estimatedPrice) * 100
      if (tickPercent > 1.0) return 'excellent'
      if (tickPercent > 0.5) return 'good'
      if (tickPercent > 0.1) return 'acceptable'
      return 'poor'
    case 'noiseratio':
      if (value < 2.0) return 'excellent'
      if (value < 3.0) return 'good'
      if (value < 4.0) return 'acceptable'
      return 'poor'
    case 'directionstrength':
      if (value >= 0.20) return 'excellent'
      if (value >= 0.10) return 'good'
      if (value >= 0.05) return 'acceptable'
      return 'poor'
    case 'volumeimbalance':
      if (value > 0.5) return 'excellent'
      if (value > 0.3) return 'good'
      if (value > 0.1) return 'acceptable'
      return 'poor'
    case 'breakout':
      if (value >= 20) return 'excellent'
      if (value >= 10) return 'good'
      if (value >= 5) return 'acceptable'
      return 'poor'
    default:
      return 'neutral'
  }
}

function formatATR(atr: number): string {
  const atrPercent = (atr / props.estimatedPrice) * 100
  return `${atrPercent.toFixed(2)}%`
}

function formatTickQuality(tick: number): string {
  const tickPercent = (tick / props.estimatedPrice) * 100
  return `${tickPercent.toFixed(2)}%`
}

function getColorClass(metric: string, value: number): string {
  return `metric-${getMetricQuality(metric, value)}`
}

const displayedMetrics = computed(() => [
  {
    key: 'bougies',
    label: 'Bougies',
    value: props.globalMetrics.total_candles,
    formattedValue: props.globalMetrics.total_candles.toLocaleString()
  },
  {
    key: 'atr',
    label: 'ATR moyen',
    value: props.globalMetrics.mean_atr,
    formattedValue: formatATR(props.globalMetrics.mean_atr)
  },
  {
    key: 'volatility',
    label: 'Volatilité',
    value: props.globalMetrics.mean_volatility,
    formattedValue: `${(props.globalMetrics.mean_volatility * 100).toFixed(1)}%`
  },
  {
    key: 'range',
    label: 'Range',
    value: props.globalMetrics.mean_range,
    formattedValue: `${(props.globalMetrics.mean_range / props.estimatedPrice * 100).toFixed(2)}%`
  },
  {
    key: 'bodyrange',
    label: 'Body Range',
    value: props.globalMetrics.mean_body_range,
    formattedValue: `${props.globalMetrics.mean_body_range.toFixed(1)}%`
  },
  {
    key: 'tickquality',
    label: 'Tick Quality',
    value: props.globalMetrics.mean_tick_quality,
    formattedValue: formatTickQuality(props.globalMetrics.mean_tick_quality)
  },
  {
    key: 'noiseratio',
    label: 'Noise Ratio',
    value: props.globalMetrics.mean_noise_ratio,
    formattedValue: `${props.globalMetrics.mean_noise_ratio.toFixed(2)}`
  },
  {
    key: 'volumeimbalance',
    label: 'Vol. Imbalance',
    value: props.globalMetrics.mean_volume_imbalance,
    formattedValue: `${(props.globalMetrics.mean_volume_imbalance * 100).toFixed(1)}%`
  },
  {
    key: 'breakout',
    label: 'Breakout %',
    value: props.globalMetrics.mean_breakout_percentage,
    formattedValue: `${props.globalMetrics.mean_breakout_percentage.toFixed(1)}%`
  }
])
</script>

<style scoped>
.metrics-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 15px; margin-bottom: 30px; }
.metric-card { background: #1a202c; padding: 15px; border-radius: 8px; border-left: 3px solid #667eea; }
.metric-card h4 { margin: 0 0 10px 0; color: #e2e8f0; }
.metric-value { font-size: 1.5em; font-weight: bold; transition: color 0.3s ease; }
.metric-value.metric-excellent { color: #10b981; text-shadow: 0 0 8px rgba(16, 185, 129, 0.3); }
.metric-value.metric-good { color: #3b82f6; text-shadow: 0 0 8px rgba(59, 130, 246, 0.3); }
.metric-value.metric-acceptable { color: #f59e0b; text-shadow: 0 0 8px rgba(245, 158, 11, 0.3); }
.metric-value.metric-poor { color: #ef4444; text-shadow: 0 0 8px rgba(239, 68, 68, 0.3); }
.metric-value.metric-neutral { color: #667eea; text-shadow: 0 0 8px rgba(102, 126, 234, 0.3); }
.metric-card:has(.metric-excellent) { border-left-color: #10b981; }
.metric-card:has(.metric-good) { border-left-color: #3b82f6; }
.metric-card:has(.metric-acceptable) { border-left-color: #f59e0b; }
.metric-card:has(.metric-poor) { border-left-color: #ef4444; }
</style>
