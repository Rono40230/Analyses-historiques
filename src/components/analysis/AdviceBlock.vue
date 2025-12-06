<script setup lang="ts">
import { computed } from 'vue'
import { useArchiveStatistics } from '../../composables/useArchiveStatistics'

const { dynamicAdvice, eventStatistics, globalStats } = useArchiveStatistics()

interface AdviceItem {
  type: 'success' | 'warning' | 'info' | 'strategy'
  emoji: string
  title: string
  description: string
}

const formattedAdvice = computed<AdviceItem[]>(() => {
  const items: AdviceItem[] = []

  if (!dynamicAdvice.value || dynamicAdvice.value.length === 0) {
    return items
  }

  // Parse advice strings
  dynamicAdvice.value.forEach((advice) => {
    if (advice.includes('optimal') || advice.includes('OPTIMAL')) {
      items.push({
        type: 'success',
        emoji: '✅',
        title: 'Configuration Optimale Détectée',
        description: advice,
      })
    } else if (advice.includes('volatilité') || advice.includes('Volatilité')) {
      items.push({
        type: 'info',
        emoji: '📊',
        title: 'Analyse de Volatilité',
        description: advice,
      })
    } else if (advice.includes('cautious') || advice.includes('Risqué')) {
      items.push({
        type: 'warning',
        emoji: '⚠️',
        title: 'Attention Requise',
        description: advice,
      })
    } else {
      items.push({
        type: 'strategy',
        emoji: '🎯',
        title: 'Recommandation Stratégique',
        description: advice,
      })
    }
  })

  return items
})

const optimalEventCount = computed(() => {
  if (!eventStatistics.value) return 0
  return Object.values(eventStatistics.value).filter((stats) => stats.tradabilityScore >= 80).length
})

const riskLevel = computed(() => {
  if (!globalStats.value) return 'Moyen'
  const avgConfidence = globalStats.value.avgConfidence
  if (avgConfidence >= 85) return 'Très Faible'
  if (avgConfidence >= 70) return 'Faible'
  if (avgConfidence >= 50) return 'Moyen'
  return 'Élevé'
})

const winRateEstimate = computed(() => {
  if (!globalStats.value) return 55
  return Math.min(75, Math.max(45, globalStats.value.estimatedWinRate))
})
</script>

<template>
  <div class="advice-block rounded-lg border border-purple-500/30 bg-purple-950/20 p-6">
    <!-- Header -->
    <div class="mb-6 flex items-center justify-between border-b border-purple-500/20 pb-4">
      <div>
        <h3 class="text-xl font-bold text-white">Recommandations Stratégiques</h3>
        <p class="mt-1 text-sm text-gray-400">
          {{ optimalEventCount }} événements OPTIMAL • Risque: {{ riskLevel }} • Win Rate estimé: {{ winRateEstimate }}%
        </p>
      </div>
      <div class="text-4xl">🎯</div>
    </div>

    <!-- Risk & Win Rate Summary -->
    <div class="mb-6 grid gap-3 md:grid-cols-3">
      <div class="rounded-md bg-gray-900/50 p-3">
        <div class="text-xs text-gray-400">Niveau de Risque</div>
        <div class="mt-1 text-xl font-bold">
          <span v-if="riskLevel === 'Très Faible'" class="text-green-400">🟢 {{ riskLevel }}</span>
          <span v-else-if="riskLevel === 'Faible'" class="text-green-300">🟢 {{ riskLevel }}</span>
          <span v-else-if="riskLevel === 'Moyen'" class="text-yellow-400">🟡 {{ riskLevel }}</span>
          <span v-else class="text-red-400">🔴 {{ riskLevel }}</span>
        </div>
      </div>
      <div class="rounded-md bg-gray-900/50 p-3">
        <div class="text-xs text-gray-400">Win Rate Estimé</div>
        <div class="mt-1 text-xl font-bold text-blue-400">📈 {{ winRateEstimate }}%</div>
      </div>
      <div class="rounded-md bg-gray-900/50 p-3">
        <div class="text-xs text-gray-400">Configuration</div>
        <div class="mt-1 text-xl font-bold text-purple-400">{{ optimalEventCount > 3 ? '⭐ Excellente' : '🔶 Bonne' }}</div>
      </div>
    </div>

    <!-- Advice Items -->
    <div v-if="formattedAdvice.length > 0" class="space-y-3">
      <div
        v-for="(item, idx) in formattedAdvice"
        :key="idx"
        :class="{
          'border-l-4 border-green-500 bg-green-950/20': item.type === 'success',
          'border-l-4 border-blue-500 bg-blue-950/20': item.type === 'info',
          'border-l-4 border-red-500 bg-red-950/20': item.type === 'warning',
          'border-l-4 border-purple-500 bg-purple-950/20': item.type === 'strategy',
        }"
        class="rounded-r-md p-4"
      >
        <div class="flex items-start gap-3">
          <span class="text-2xl">{{ item.emoji }}</span>
          <div class="flex-1">
            <h4 class="font-semibold text-white">{{ item.title }}</h4>
            <p class="mt-1 text-sm text-gray-300">{{ item.description }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- General Recommendations -->
    <div v-if="formattedAdvice.length === 0" class="space-y-3">
      <div class="border-l-4 border-purple-500 bg-purple-950/20 rounded-r-md p-4">
        <div class="flex items-start gap-3">
          <span class="text-2xl">💡</span>
          <div class="flex-1">
            <h4 class="font-semibold text-white">Données d'Archives Chargées</h4>
            <p class="mt-1 text-sm text-gray-300">
              {{ globalStats?.totalArchives || 0 }} archives détectées. Analysez les événements pour obtenir des recommandations personnalisées.
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- Action Button -->
    <div class="mt-6 flex justify-center">
      <button class="rounded-lg bg-gradient-to-r from-purple-600 to-indigo-600 px-6 py-2.5 font-semibold text-white transition hover:from-purple-500 hover:to-indigo-500">
        🚀 Lancer Analyse Complète
      </button>
    </div>
  </div>
</template>

<style scoped>
.advice-block {
  animation: slideIn 0.3s ease-out 0.3s both;
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
