<script setup lang="ts">
import { computed } from 'vue'
import { useArchiveStatistics } from '../../composables/useArchiveStatistics'

const { eventStatistics } = useArchiveStatistics()

interface EventDisplay {
  eventType: string
  stats: ReturnType<typeof useArchiveStatistics>['eventStatistics']['value'][string]
  tradability: 'OPTIMAL' | 'BON' | 'RISQUÉ'
  colorClass: string
  icon: string
}

const sortedEvents = computed<EventDisplay[]>(() => {
  if (!eventStatistics.value) return []

  return Object.entries(eventStatistics.value)
    .map(([eventType, stats]) => {
      const score = stats.tradabilityScore
      let tradability: 'OPTIMAL' | 'BON' | 'RISQUÉ' = 'RISQUÉ'
      let colorClass = 'text-red-500'
      let icon = '🔴'

      if (score >= 80) {
        tradability = 'OPTIMAL'
        colorClass = 'text-green-500'
        icon = '🟢'
      } else if (score >= 60) {
        tradability = 'BON'
        colorClass = 'text-yellow-500'
        icon = '🟡'
      }

      return {
        eventType,
        stats,
        tradability,
        colorClass,
        icon,
      }
    })
    .sort((a, b) => b.stats.tradabilityScore - a.stats.tradabilityScore)
})

const totalEvents = computed(() => sortedEvents.value.length)
const optimalCount = computed(() => sortedEvents.value.filter(e => e.tradability === 'OPTIMAL').length)
const avgConfidence = computed(() => {
  if (sortedEvents.value.length === 0) return 0
  const sum = sortedEvents.value.reduce((acc, e) => acc + e.stats.confidence, 0)
  return Math.round((sum / sortedEvents.value.length) * 100) / 100
})
</script>

<template>
  <div class="event-analysis-block rounded-lg border border-blue-500/30 bg-blue-950/20 p-6">
    <!-- Header -->
    <div class="mb-6 flex items-center justify-between border-b border-blue-500/20 pb-4">
      <div>
        <h3 class="text-xl font-bold text-white">Types d'Événements Tradables</h3>
        <p class="mt-1 text-sm text-gray-400">
          {{ totalEvents }} événements analysés • {{ optimalCount }} OPTIMAL • Confiance moyenne: {{ avgConfidence }}%
        </p>
      </div>
      <div class="text-4xl">📊</div>
    </div>

    <!-- Events List -->
    <div v-if="sortedEvents.length > 0" class="space-y-4">
      <div v-for="event in sortedEvents" :key="event.eventType" class="rounded-md bg-gray-900/50 p-4 transition-all hover:bg-gray-900/80">
        <!-- Event Header -->
        <div class="mb-3 flex items-start justify-between">
          <div class="flex items-center gap-3">
            <span :class="event.colorClass" class="text-2xl">{{ event.icon }}</span>
            <div>
              <h4 class="text-lg font-semibold text-white">{{ event.eventType }}</h4>
              <span :class="`inline-block px-2 py-1 rounded text-xs font-medium ${event.colorClass.replace('text-', 'bg-').replace('500', '950/30')}`">
                {{ event.tradability }}
              </span>
            </div>
          </div>
          <div class="text-right">
            <div class="text-2xl font-bold text-white">{{ Math.round(event.stats.tradabilityScore) }}/100</div>
            <div class="text-xs text-gray-400">Score</div>
          </div>
        </div>

        <!-- Metrics Grid -->
        <div class="mb-3 grid grid-cols-2 gap-2 text-sm md:grid-cols-4">
          <div class="rounded bg-gray-800/50 p-2">
            <div class="text-gray-400">Volatilité ATR</div>
            <div class="text-white">{{ Math.round(event.stats.avgATR * 10) / 10 }}p</div>
          </div>
          <div class="rounded bg-gray-800/50 p-2">
            <div class="text-gray-400">Pic (+/-)</div>
            <div class="text-white">+{{ Math.round(event.stats.avgPeakDelay * 10) / 10 }}min</div>
          </div>
          <div class="rounded bg-gray-800/50 p-2">
            <div class="text-gray-400">Confiance</div>
            <div class="text-white">{{ Math.round(event.stats.confidence) }}%</div>
          </div>
          <div class="rounded bg-gray-800/50 p-2">
            <div class="text-gray-400">Analyses</div>
            <div class="text-white">{{ event.stats.count }}</div>
          </div>
        </div>

        <!-- Straddle Setup -->
        <div class="mb-3 rounded bg-gradient-to-r from-purple-950/30 to-indigo-950/30 p-3">
          <div class="mb-2 text-xs font-semibold text-purple-300">Straddle Setup</div>
          <div class="flex flex-wrap items-center gap-2 text-sm text-gray-100">
            <span>SL: <span class="font-semibold">{{ Math.round(event.stats.avgATR * 1.5) }}p</span></span>
            <span class="text-gray-600">•</span>
            <span>TP: <span class="font-semibold">{{ Math.round(event.stats.avgATR * 3) }}p</span></span>
            <span class="text-gray-600">•</span>
            <span>Ratio: <span class="font-semibold">1:2</span></span>
            <span class="text-gray-600">•</span>
            <span>Placement: <span class="font-semibold">{{ Math.round(event.stats.avgPeakDelay * 60) }}sec avant</span></span>
          </div>
        </div>

        <!-- Actions -->
        <div class="flex gap-2">
          <button class="flex-1 rounded bg-blue-600/30 px-3 py-1.5 text-sm text-blue-300 transition hover:bg-blue-600/50">
            📋 Détails
          </button>
          <button class="flex-1 rounded bg-indigo-600/30 px-3 py-1.5 text-sm text-indigo-300 transition hover:bg-indigo-600/50">
            📈 Impact
          </button>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="rounded-md border border-dashed border-gray-600 p-8 text-center">
      <p class="text-gray-400">Aucun événement analysé</p>
    </div>
  </div>
</template>

<style scoped>
.event-analysis-block {
  animation: slideIn 0.3s ease-out;
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
