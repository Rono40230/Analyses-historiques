<script setup lang="ts">
import { computed } from 'vue'
import { useArchiveStatistics } from '../../composables/useArchiveStatistics'

const { globalStats, eventStatistics, pairStatistics, archives, rawArchivesCount, parseErrors } = useArchiveStatistics()

const statsDisplay = computed(() => {
  if (!globalStats.value) {
    return {
      totalArchives: 0,
      avgConfidence: 0,
      estimatedWinRate: 0,
      totalEvents: 0,
      totalPairs: 0,
    }
  }

  return {
    totalArchives: globalStats.value.totalArchives,
    avgConfidence: Math.round(globalStats.value.avgConfidence),
    estimatedWinRate: Math.round(globalStats.value.estimatedWinRate),
    totalEvents: Object.keys(eventStatistics.value || {}).length,
    totalPairs: Object.keys(pairStatistics.value || {}).length,
  }
})

const metrics = computed(() => [
  {
    label: 'Archives Analysées',
    value: statsDisplay.value.totalArchives,
    icon: '📦',
    color: 'from-blue-600 to-cyan-600',
  },
  {
    label: 'Confiance Moyenne',
    value: `${statsDisplay.value.avgConfidence}%`,
    icon: '📊',
    color: 'from-green-600 to-emerald-600',
  },
  {
    label: 'Win Rate Estimé',
    value: `${statsDisplay.value.estimatedWinRate}%`,
    icon: '🎯',
    color: 'from-purple-600 to-pink-600',
  },
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
  <div class="global-stats-block rounded-lg border border-cyan-500/30 bg-cyan-950/20 p-6">
    <!-- Header -->
    <div class="mb-6 flex items-center justify-between border-b border-cyan-500/20 pb-4">
      <div>
        <h3 class="text-xl font-bold text-white">Statistiques Globales</h3>
        <p class="mt-1 text-sm text-gray-400">
          Qualité de l'analyse: {{ qualityLabel }}
        </p>
      </div>
      <div class="text-4xl">🔍</div>
    </div>

    <!-- Metrics Grid -->
    <div class="mb-6 grid gap-4 sm:grid-cols-2 lg:grid-cols-5">
      <div v-for="metric in metrics" :key="metric.label" :class="`bg-gradient-to-br ${metric.color}`" class="rounded-lg p-4 text-white shadow-lg transition-transform hover:scale-105">
        <div class="flex items-center justify-between">
          <div>
            <div class="text-sm text-gray-100 opacity-90">{{ metric.label }}</div>
            <div class="mt-1 text-3xl font-bold">{{ metric.value }}</div>
          </div>
          <div class="text-3xl opacity-50">{{ metric.icon }}</div>
        </div>
      </div>
    </div>

    <!-- Quality Score Bar -->
    <div class="mb-6 rounded-lg bg-gray-900/50 p-4">
      <div class="mb-2 flex items-center justify-between">
        <span class="text-sm font-semibold text-gray-300">Score de Qualité Général</span>
        <span class="text-lg font-bold text-cyan-400">{{ qualityScore }}/100</span>
      </div>
      <div class="h-3 w-full overflow-hidden rounded-full bg-gray-800">
        <div :style="{ width: `${qualityScore}%` }" class="h-full bg-gradient-to-r from-cyan-500 to-blue-500 transition-all duration-500"></div>
      </div>
      <p class="mt-2 text-xs text-gray-400">
        Basé sur: confiance moyenne ({{ statsDisplay.avgConfidence }}%), nombre d'événements ({{ statsDisplay.totalEvents }}), nombre de paires ({{ statsDisplay.totalPairs }})
      </p>
    </div>

    <!-- Data Coverage -->
    <div class="rounded-lg bg-gray-900/50 p-4">
      <h4 class="mb-3 text-sm font-semibold text-white">Couverture des Données</h4>
      <div class="space-y-2 text-sm">
        <div class="flex items-center justify-between">
          <span class="text-gray-400">Archives chargées</span>
          <span class="font-semibold text-cyan-400">{{ statsDisplay.totalArchives }}/25</span>
        </div>
        <div class="h-2 w-full overflow-hidden rounded-full bg-gray-800">
          <div :style="{ width: `${(statsDisplay.totalArchives / 25) * 100}%` }" class="h-full bg-cyan-500"></div>
        </div>

        <div class="mt-3 flex items-center justify-between">
          <span class="text-gray-400">Événements détectés</span>
          <span class="font-semibold text-purple-400">{{ statsDisplay.totalEvents }} types</span>
        </div>

        <div class="mt-3 flex items-center justify-between">
          <span class="text-gray-400">Paires tradées</span>
          <span class="font-semibold text-green-400">{{ statsDisplay.totalPairs }} paires</span>
        </div>
      </div>
    </div>

    <!-- Insights -->
    <div class="mt-6 grid gap-3 md:grid-cols-2">
      <div class="rounded-lg border border-blue-500/30 bg-blue-950/20 p-3">
        <div class="text-xs font-semibold text-blue-300">💡 Insight #1</div>
        <p class="mt-1 text-sm text-gray-200">
          {{ statsDisplay.avgConfidence >= 75 ? 'Analyse très solide avec une confiance élevée' : 'Confiance modérée, considérez plus de données' }}
        </p>
      </div>
      <div class="rounded-lg border border-green-500/30 bg-green-950/20 p-3">
        <div class="text-xs font-semibold text-green-300">💡 Insight #2</div>
        <p class="mt-1 text-sm text-gray-200">
          {{ statsDisplay.totalEvents >= 15 ? 'Bonne diversification événementielle' : 'Augmentez l\'analyse sur plus d\'événements' }}
        </p>
      </div>
    </div>

    <!-- Debug Info -->
    <div class="mt-4 rounded-lg border border-red-500/30 bg-red-950/20 p-3">
      <div class="text-xs font-semibold text-red-300">🔴 DEBUG INFO</div>
      <p class="mt-1 text-xs text-gray-300">
        Archives brutes chargées: {{ rawArchivesCount }} | 
        Globales: {{ statsDisplay.totalArchives }} | 
        Événements: {{ statsDisplay.totalEvents }} | 
        Paires: {{ statsDisplay.totalPairs }}
      </p>
      <div v-if="parseErrors.length > 0" class="mt-2">
        <p class="text-xs text-red-400">Erreurs de parsing:</p>
        <p v-for="(err, idx) in parseErrors" :key="idx" class="text-xs text-red-300">
          • {{ err }}
        </p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.global-stats-block {
  animation: slideIn 0.3s ease-out 0.4s both;
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
