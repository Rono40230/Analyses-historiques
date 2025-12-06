<script setup lang="ts">
import { computed } from 'vue'
import { useArchiveStatistics } from '../../composables/useArchiveStatistics'

const { eventStatistics } = useArchiveStatistics()

interface TimingEvent {
  eventType: string
  peakDelay: number
  placementSeconds: number
  exitMinutes: number
  estimatedGain: number
  confidence: number
  tradabilityScore: number
}

const sortedByTiming = computed<TimingEvent[]>(() => {
  if (!eventStatistics.value) return []

  return Object.entries(eventStatistics.value)
    .map(([eventType, stats]) => ({
      eventType,
      peakDelay: stats.avgPeakDelay,
      placementSeconds: Math.round(stats.avgPeakDelay * 60),
      exitMinutes: Math.round(stats.avgDecayTimeout * 1.5),
      estimatedGain: Math.round(stats.avgATR * 2.5),
      confidence: stats.confidence,
      tradabilityScore: stats.tradabilityScore,
    }))
    .sort((a, b) => a.peakDelay - b.peakDelay)
})

const fastestEvent = computed(() => sortedByTiming.value[0] || null)
const slowestEvent = computed(() => {
  if (sortedByTiming.value.length === 0) return null
  return sortedByTiming.value[sortedByTiming.value.length - 1]
})
const avgPlacement = computed(() => {
  if (sortedByTiming.value.length === 0) return 0
  const sum = sortedByTiming.value.reduce((acc, e) => acc + e.placementSeconds, 0)
  return Math.round(sum / sortedByTiming.value.length)
})
</script>

<template>
  <div class="timing-analysis-block rounded-lg border border-amber-500/30 bg-amber-950/20 p-6">
    <!-- Header -->
    <div class="mb-6 flex items-center justify-between border-b border-amber-500/20 pb-4">
      <div>
        <h3 class="text-xl font-bold text-white">Analyse Timing & Setup Straddle</h3>
        <p class="mt-1 text-sm text-gray-400">
          Placement moyen: {{ avgPlacement }}sec avant pic • {{ sortedByTiming.length }} événements
        </p>
      </div>
      <div class="text-4xl">⏱️</div>
    </div>

    <!-- Fastest Event -->
    <div v-if="fastestEvent" class="mb-4 rounded-md border border-emerald-500/30 bg-emerald-950/20 p-4">
      <div class="mb-2 text-xs font-semibold text-emerald-300">⚡ Réaction la plus rapide</div>
      <div class="flex items-center justify-between">
        <h4 class="text-lg font-bold text-white">{{ fastestEvent.eventType }}</h4>
        <div class="text-right">
          <div class="text-sm text-gray-400">Pic après placement</div>
          <div class="text-xl font-bold text-emerald-400">+{{ fastestEvent.peakDelay.toFixed(1) }}min</div>
        </div>
      </div>
    </div>

    <!-- Timeline Table -->
    <div v-if="sortedByTiming.length > 0" class="mb-4 overflow-hidden rounded-md bg-gray-900/50">
      <div class="grid grid-cols-5 gap-2 border-b border-gray-700/50 p-3 text-xs font-semibold text-gray-400">
        <div>Événement</div>
        <div class="text-center">Placement</div>
        <div class="text-center">Durée exit</div>
        <div class="text-center">Gain estim.</div>
        <div class="text-center">Score</div>
      </div>
      <div v-for="event in sortedByTiming" :key="event.eventType" class="grid grid-cols-5 gap-2 border-b border-gray-800/30 p-3 text-sm transition-colors hover:bg-gray-800/30">
        <div class="font-semibold text-white">{{ event.eventType }}</div>
        <div class="text-center text-amber-300">{{ event.placementSeconds }}sec</div>
        <div class="text-center text-blue-300">{{ event.exitMinutes }}min</div>
        <div class="text-center font-semibold text-green-400">+{{ event.estimatedGain }}p</div>
        <div class="text-center">
          <span v-if="event.tradabilityScore >= 80" class="rounded bg-green-950/50 px-2 py-1 text-green-300">{{ Math.round(event.tradabilityScore) }}</span>
          <span v-else-if="event.tradabilityScore >= 60" class="rounded bg-yellow-950/50 px-2 py-1 text-yellow-300">{{ Math.round(event.tradabilityScore) }}</span>
          <span v-else class="rounded bg-red-950/50 px-2 py-1 text-red-300">{{ Math.round(event.tradabilityScore) }}</span>
        </div>
      </div>
    </div>

    <!-- Slowest Event -->
    <div v-if="slowestEvent" class="rounded-md border border-red-500/30 bg-red-950/20 p-4">
      <div class="mb-2 text-xs font-semibold text-red-300">🐢 Réaction la plus lente</div>
      <div class="flex items-center justify-between">
        <h4 class="text-lg font-bold text-white">{{ slowestEvent.eventType }}</h4>
        <div class="text-right">
          <div class="text-sm text-gray-400">Pic après placement</div>
          <div class="text-xl font-bold text-red-400">+{{ slowestEvent.peakDelay.toFixed(1) }}min</div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-if="sortedByTiming.length === 0" class="rounded-md border border-dashed border-gray-600 p-8 text-center">
      <p class="text-gray-400">Aucune donnée de timing disponible</p>
    </div>
  </div>
</template>

<style scoped>
.timing-analysis-block {
  animation: slideIn 0.3s ease-out 0.2s both;
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
