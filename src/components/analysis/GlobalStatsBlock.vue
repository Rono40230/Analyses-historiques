<script setup lang="ts">
import { computed } from 'vue'
import { useArchiveStatistics } from '../../composables/useArchiveStatistics'
const { globalStats, eventStatistics, pairStatistics } = useArchiveStatistics()
const statsDisplay = computed(() => {
  const stats = globalStats.value
  if (!stats) {
    return {
      totalArchives: 0,
      avgConfidence: 0,
      estimatedWinRate: 0,
      totalEvents: 0,
      totalPairs: 0,
    }
  }
  const events = eventStatistics.value || {}
  const pairs = pairStatistics.value || {}
  return {
    totalArchives: stats.totalArchives || 0,
    avgConfidence: parseInt(String(stats.avgConfidence || '0')),
    estimatedWinRate: parseInt(String(stats.estimatedWinRate || '0')),
    totalEvents: Object.keys(events).length || 0,
    totalPairs: Object.keys(pairs).length || 0,
  }
})
const metrics = computed(() => [
  {
    label: 'Événements Suivis',
    value: statsDisplay.value.totalEvents,
    icon: '📅',
    color: 'from-yellow-600 to-orange-600',
  },
  {
    label: 'Paires Analysées',
    value: statsDisplay.value.totalPairs,
    icon: '💱',
    color: 'from-indigo-600 to-purple-600',
  },
])
const qualityScore = computed(() => {
  const confidence = statsDisplay.value.avgConfidence
  const events = statsDisplay.value.totalEvents
  const pairs = statsDisplay.value.totalPairs
  // Score based on data completeness and confidence
  let score = Math.min(100, confidence)
  if (events >= 10) score = Math.min(100, score + 10)
  if (pairs >= 5) score = Math.min(100, score + 10)
  return Math.round(score)
})
const qualityLabel = computed(() => {
  const score = qualityScore.value
  if (score >= 85) return '⭐ Excellente'
  if (score >= 70) return '🟢 Très Bonne'
  if (score >= 55) return '🟡 Bonne'
  if (score >= 40) return '🟠 Correcte'
  return '🔴 Insuffisante'
})
</script>
<template>
  <div class="global-stats-block">
    <!-- Métriques Grid -->
    <div class="metrics-grid">
      <div v-for="metric in metrics" :key="metric.label" class="metric-card">
        <div class="metric-icon">{{ metric.icon }}</div>
        <div class="metric-label">{{ metric.label }}</div>
        <div class="metric-value">{{ metric.value }}</div>
      </div>
    </div>
    <!-- Blocs horizontaux - SUPPRIMÉS -->
  </div>
</template>
<style scoped>
.global-stats-block {
  animation: slideIn 0.3s ease-out 0.4s both;
  border: 1px solid rgba(78, 205, 196, 0.3);
  background: rgba(30, 70, 100, 0.2);
  border-radius: 12px;
  padding: 16px;
}
/* Metrics Grid */
.metrics-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
  margin-bottom: 16px;
}
.metric-card {
  background: linear-gradient(135deg, rgba(78, 205, 196, 0.15), rgba(85, 98, 112, 0.15));
  border: 1px solid rgba(78, 205, 196, 0.3);
  border-radius: 10px;
  padding: 14px;
  text-align: center;
  transition: all 0.3s ease;
}
.metric-card:hover {
  background: linear-gradient(135deg, rgba(78, 205, 196, 0.2), rgba(85, 98, 112, 0.2));
  border-color: rgba(78, 205, 196, 0.5);
  transform: translateY(-2px);
}
.metric-icon {
  font-size: 24px;
  margin-bottom: 6px;
}
.metric-label {
  font-size: 11px;
  color: #a0aec0;
  margin-bottom: 6px;
  text-transform: uppercase;
  letter-spacing: 0.3px;
}
.metric-value {
  font-size: 22px;
  font-weight: 700;
  color: #4ecdc4;
}
/* Blocs Horizontaux - SUPPRIMÉS */
.blocks-row {
  display: none;
}
.block-item {
  background: rgba(0, 0, 0, 0.2);
  border: 1px solid rgba(255, 255, 255, 0.05);
  border-radius: 10px;
  padding: 12px;
}
.block-title {
  font-size: 11px;
  color: #a0aec0;
  margin-bottom: 8px;
  text-transform: uppercase;
  letter-spacing: 0.3px;
  font-weight: 600;
}
.quality-bar,
.coverage-bar {
  height: 8px;
  background: rgba(0, 0, 0, 0.3);
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 6px;
}
.quality-fill {
  height: 100%;
  background: linear-gradient(90deg, #4ecdc4, #0eeaf5);
  transition: width 0.3s ease;
}
.coverage-fill {
  height: 100%;
  background: linear-gradient(90deg, #10b981, #34d399);
  transition: width 0.3s ease;
}
.event-badge,
.pair-badge {
  font-size: 18px;
  font-weight: 700;
  padding: 6px 0;
}
.event-badge {
  color: #fb923c;
}
.pair-badge {
  color: #10b981;
}
.quality-label-badge {
  font-size: 14px;
  font-weight: 600;
}
.block-footer {
  font-size: 12px;
  color: #e2e8f0;
  font-weight: 600;
}
@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
