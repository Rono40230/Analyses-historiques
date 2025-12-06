<script setup lang="ts">
import { computed } from 'vue'
import { useArchiveStatistics } from '../../composables/useArchiveStatistics'
import { useEventPairCorrelation } from '../../composables/useEventPairCorrelation'
import { useEventDetail } from '../../composables/useEventDetail'
import { useEventTranslation } from '../../composables/useEventTranslation'
import EventDetailModal from './EventDetailModal.vue'

const { eventStatistics } = useArchiveStatistics()
const { getPairsByEvent, hasHeatmapData } = useEventPairCorrelation()
const { isOpen, selectedEvent, openDetail, closeDetail } = useEventDetail()
const { translateEventName } = useEventTranslation()

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
      const score = stats.tradabilityScore || 0
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
    .sort((a, b) => (b.stats.tradabilityScore || 0) - (a.stats.tradabilityScore || 0))
})

const totalEvents = computed(() => sortedEvents.value.length)
const optimalCount = computed(() => sortedEvents.value.filter(e => e.tradability === 'OPTIMAL').length)
const avgConfidence = computed(() => {
  if (sortedEvents.value.length === 0) return 0
  const sum = sortedEvents.value.reduce((acc, e) => acc + e.stats.avgConfidence, 0)
  return Math.round((sum / sortedEvents.value.length) * 100) / 100
})

/**
 * Récupère les paires pour un événement avec couleur d'impact
 */
function getImpactColor(impact: number): string {
  if (impact >= 80) return '#10b981' // Vert - Excellent
  if (impact >= 60) return '#fbbf24' // Ambre - Bon
  if (impact >= 40) return '#f97316' // Orange - Moyen
  return '#ef4444' // Rouge - Faible
}

function getImpactIcon(impact: number): string {
  if (impact >= 80) return '🟢'
  if (impact >= 60) return '🟡'
  if (impact >= 40) return '🟠'
  return '🔴'
}

function openEventDetail(event: any) {
  openDetail({
    eventType: event.eventType,
    score: event.stats.tradabilityScore,
    avgATR: event.stats.avgATR,
    avgPeakDelay: event.stats.avgPeakDelay,
    avgConfidence: event.stats.avgConfidence,
    tradability: event.tradability,
  })
}
</script>

<template>
  <div class="event-analysis-block">
    <!-- Header -->
    <div class="header-section">
      <div class="header-content">
      </div>
    </div>

    <!-- Events List -->
    <div v-if="sortedEvents.length > 0" class="events-list">
      <div v-for="event in sortedEvents" :key="event.eventType" class="event-card">
        <!-- Event Header -->
        <div class="event-header">
          <div class="event-title-group">
            <span class="event-icon">{{ event.icon }}</span>
            <div>
              <h4 class="event-name">{{ translateEventName(event.eventType) }}</h4>
              <span class="tradability-badge" :data-level="event.tradability.toLowerCase()">
                {{ event.tradability }}
              </span>
            </div>
          </div>
          <div class="event-score">
            <div class="score-value">{{ Math.round(event.stats.tradabilityScore || 0) }}/100</div>
            <div class="score-label">Score</div>
          </div>
        </div>

        <!-- Metrics Grid -->
        <div class="metrics-grid">
          <div class="metric-box">
            <div class="metric-label">Volatilité ATR</div>
            <div class="metric-value">{{ Math.round(event.stats.avgATR * 10) / 10 }}p</div>
          </div>
          <div class="metric-box">
            <div class="metric-label">Pic (+/-)</div>
            <div class="metric-value">+{{ Math.round(event.stats.avgPeakDelay * 10) / 10 }}min</div>
          </div>
          <div class="metric-box">
            <div class="metric-label">Confiance</div>
            <div class="metric-value">{{ Math.round(event.stats.avgConfidence) }}%</div>
          </div>
          <div class="metric-box">
            <div class="metric-label">Analyses</div>
            <div class="metric-value">{{ event.stats.count }}</div>
          </div>
        </div>

        <!-- Straddle Setup -->
        <div class="straddle-setup">
          <div class="setup-label">Straddle Setup</div>
          <div class="setup-params">
            <span>SL: <strong>{{ Math.round(event.stats.avgATR * 1.5) }}p</strong></span>
            <span class="separator">•</span>
            <span>TP: <strong>{{ Math.round(event.stats.avgATR * 3) }}p</strong></span>
            <span class="separator">•</span>
            <span>Ratio: <strong>1:2</strong></span>
            <span class="separator">•</span>
            <span>Placement: <strong>{{ Math.round(event.stats.avgPeakDelay * 60) }}sec avant</strong></span>
          </div>
        </div>

        <!-- Best Pairs (if Heatmap data available) -->
        <div v-if="hasHeatmapData" class="best-pairs-section">
          <div class="pairs-label">Meilleures Paires</div>
          <div class="pairs-list">
            <div v-for="(pair, idx) in getPairsByEvent(event.eventType).slice(0, 3)" :key="pair.pair" class="pair-badge" :style="{ borderLeftColor: getImpactColor(pair.impact) }">
              <span class="pair-icon">{{ getImpactIcon(pair.impact) }}</span>
              <span class="pair-name">{{ pair.pair }}</span>
              <span class="pair-impact">{{ Math.round(pair.impact) }}%</span>
            </div>
            <div v-if="getPairsByEvent(event.eventType).length === 0" class="no-pairs">
              Aucune paire détectée
            </div>
          </div>
          <button class="detail-button" @click="openEventDetail(event)">
            → Voir tous les détails
          </button>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="empty-state">
      <p>Aucun événement analysé</p>
    </div>

    <!-- Event Detail Modal -->
    <EventDetailModal :is-open="isOpen" :event="selectedEvent" @close="closeDetail" />
  </div>
</template>

<style scoped>
@import './EventAnalysisBlock.css';
</style>