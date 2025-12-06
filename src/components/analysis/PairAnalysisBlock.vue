<script setup lang="ts">
import { computed } from 'vue'
import { useArchiveStatistics } from '../../composables/useArchiveStatistics'

const { pairStatistics } = useArchiveStatistics()

interface PairDisplay {
  pair: string
  stats: ReturnType<typeof useArchiveStatistics>['pairStatistics']['value'][string]
  performanceColor: string
  performanceIcon: string
}

const sortedPairs = computed<PairDisplay[]>(() => {
  if (!pairStatistics.value) return []

  return Object.entries(pairStatistics.value)
    .map(([pair, stats]) => {
      let performanceColor = 'text-red-500'
      let performanceIcon = '🔴'

      if (stats.performanceRating === '🟢 TRÈS BON') {
        performanceColor = 'text-green-500'
        performanceIcon = '🟢'
      } else if (stats.performanceRating === '🟡 BON') {
        performanceColor = 'text-yellow-500'
        performanceIcon = '🟡'
      } else if (stats.performanceRating === '🟠 MOYEN') {
        performanceColor = 'text-orange-500'
        performanceIcon = '🟠'
      }

      return {
        pair,
        stats,
        performanceColor,
        performanceIcon,
      }
    })
    .sort((a, b) => b.stats.avgConfidence - a.stats.avgConfidence)
})

const totalPairs = computed(() => sortedPairs.value.length)
const strongPairs = computed(() => sortedPairs.value.filter(p => p.stats.performanceRating.includes('TRÈS BON')).length)
const avgATR = computed(() => {
  if (sortedPairs.value.length === 0) return 0
  const sum = sortedPairs.value.reduce((acc, p) => acc + p.stats.avgATR, 0)
  return Math.round((sum / sortedPairs.value.length) * 10) / 10
})

function getTopSensitiveEvent(eventSensitivity: Record<string, number>): { event: string; sensitivity: number } | null {
  const entries = Object.entries(eventSensitivity)
  if (entries.length === 0) return null
  return entries.reduce((best, current) => (current[1] > best[1] ? { event: current[0], sensitivity: current[1] } : best), {
    event: entries[0][0],
    sensitivity: entries[0][1],
  })
}
</script>

<template>
  <div class="pair-analysis-block rounded-lg border border-emerald-500/30 bg-emerald-950/20 p-6">
    <!-- Header -->
    <div class="mb-6 flex items-center justify-between border-b border-emerald-500/20 pb-4">
      <div>
        <h3 class="text-xl font-bold text-white">Performance par Paire</h3>
        <p class="mt-1 text-sm text-gray-400">
          {{ totalPairs }} paires • {{ strongPairs }} très bonnes • ATR moy: {{ avgATR }}p
        </p>
      </div>
      <div class="text-4xl">💱</div>
    </div>

    <!-- Pairs Grid -->
    <div v-if="sortedPairs.length > 0" class="grid gap-4 md:grid-cols-2">
      <div v-for="pairItem in sortedPairs" :key="pairItem.pair" class="rounded-md bg-gray-900/50 p-4 transition-all hover:bg-gray-900/80">
        <!-- Pair Header -->
        <div class="mb-3 flex items-center justify-between">
          <h4 class="text-lg font-semibold text-white">{{ pairItem.pair }}</h4>
          <span :class="pairItem.performanceColor" class="text-2xl">{{ pairItem.performanceIcon }}</span>
        </div>

        <!-- Metrics -->
        <div class="mb-3 space-y-2 text-sm">
          <div class="flex items-center justify-between">
            <span class="text-gray-400">Confiance moyenne</span>
            <span class="font-semibold text-white">{{ Math.round(pairItem.stats.avgConfidence) }}%</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-gray-400">Volatilité ATR</span>
            <span class="font-semibold text-white">{{ Math.round(pairItem.stats.avgATR * 10) / 10 }}p</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-gray-400">Performance</span>
            <span class="font-semibold text-white">{{ pairItem.stats.performanceRating }}</span>
          </div>
        </div>

        <!-- Top Sensitive Event -->
        <div v-if="getTopSensitiveEvent(pairItem.stats.eventSensitivity)" class="rounded bg-blue-950/50 p-2">
          <div class="text-xs text-gray-400">Événement le plus sensible</div>
          <div class="flex items-center justify-between text-sm">
            <span class="font-semibold text-white">{{ getTopSensitiveEvent(pairItem.stats.eventSensitivity)?.event }}</span>
            <span class="text-blue-300">+{{ Math.round((getTopSensitiveEvent(pairItem.stats.eventSensitivity)?.sensitivity ?? 0) * 100) }}%</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="rounded-md border border-dashed border-gray-600 p-8 text-center">
      <p class="text-gray-400">Aucune paire analysée</p>
    </div>
  </div>
</template>

<style scoped>
.pair-analysis-block {
  animation: slideIn 0.3s ease-out 0.1s both;
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
