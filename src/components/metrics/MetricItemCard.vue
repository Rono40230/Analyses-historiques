<template>
  <div class="metric-item">
    <span class="metric-name">{{ metric.label }}</span>
    <div class="metric-values">
      <span :class="['value15', getMetricClass(metric.value15, metric.goodThreshold, metric.excellentThreshold)]">
        {{ formatNumber(metric.value15, metric.decimals ?? 5) }}{{ metric.suffix }}
      </span>
      <span class="separator">|</span>
      <span class="valueglobal">{{ formatNumber(metric.valueGlobal, metric.decimals ?? 5) }}{{ metric.suffix }}</span>
      <span class="separator">|</span>
      <span class="threshold">>{{ metric.excellentThreshold }}</span>
    </div>
    <span :class="['status', getMetricStatus(metric.value15, metric.excellentThreshold)]">
      {{ getMetricStatusText(metric.value15, metric.excellentThreshold) }}
    </span>
  </div>
</template>

<script setup lang="ts">

interface Metric {
  label: string
  value15: number
  valueGlobal: number
  goodThreshold: number
  excellentThreshold: number
  definition: string
  usage: string
  scoring: string
  suffix?: string
  decimals?: number
}

defineProps<{
  metric: Metric
}>()

function formatNumber(value: number, decimals: number): string {
  return value.toFixed(decimals)
}

function getMetricClass(value: number, goodThreshold: number, excellentThreshold: number): string {
  if (value > excellentThreshold) return 'excellent'
  if (value > goodThreshold) return 'good'
  return 'poor'
}

function getMetricStatus(value: number, excellentThreshold: number): string {
  return value > excellentThreshold ? 'good' : 'warning'
}

function getMetricStatusText(value: number, excellentThreshold: number): string {
  return value > excellentThreshold ? '✓ Bon' : '⚠ Faible'
}
</script>

<style scoped>
.metric-item { padding: 12px; background: #0d1117; border-radius: 6px; }
.metric-name { display: block; font-weight: 600; margin-bottom: 6px; color: #e2e8f0; }
.metric-values { display: flex; align-items: center; gap: 6px; margin-bottom: 6px; font-size: 0.95em; }
.value15 { font-weight: 600; }
.value15.excellent { color: #10b981; }
.value15.good { color: #3b82f6; }
.value15.poor { color: #ef4444; }
.separator { opacity: 0.5; }
.valueglobal { opacity: 0.7; }
.threshold { opacity: 0.5; font-size: 0.85em; }
.status { font-size: 0.85em; padding: 2px 6px; border-radius: 3px; }
.status.good { background: rgba(16, 185, 129, 0.2); color: #10b981; }
.status.warning { background: rgba(239, 68, 68, 0.2); color: #ef4444; }
</style>
