<template>
  <div class="results-container">
    <RetroAnalysisGraph
      :atr-timeline-before="atrTimelineBefore"
      :atr-timeline-after="atrTimelineAfter"
      :body-timeline-before="bodyTimelineBefore"
      :body-timeline-after="bodyTimelineAfter"
      :noise-ratio-before="noiseRatioBefore"
      :noise-ratio-during="noiseRatioDuring"
      :noise-ratio-after="noiseRatioAfter"
      :volatility-increase-percent="volatilityIncreasePercent"
      :event-count="eventCount"
      :event-type="eventType"
      :pair="pair"
      :event-datetime="eventDatetime"
      :timezone-offset="timezoneOffset"
      :is-archive-mode="isArchiveMode"
      :event-label="eventLabel"
      :meilleur-moment="meilleurMoment"
      @archive="$emit('archive')"
    />
    
    <RetroAnalysisStats
      :noise-ratio-before="noiseRatioBefore"
      :noise-ratio-during="noiseRatioDuring"
      :noise-ratio-after="noiseRatioAfter"
      :volatility-increase-percent="volatilityIncreasePercent"
      :event-count="eventCount"
      :meilleur-moment="meilleurMoment"
      :stop-loss="stopLoss"
      :trailing-stop="trailingStop"
      :timeout="timeout"
      :pair="pair"
    />
  </div>
</template>

<script setup lang="ts">
import RetroAnalysisGraph from './analysis/RetroAnalysisGraph.vue'
import RetroAnalysisStats from './analysis/RetroAnalysisStats.vue'

defineProps<{
  atrTimelineBefore?: number[]
  atrTimelineAfter?: number[]
  bodyTimelineBefore?: number[]
  bodyTimelineAfter?: number[]
  noiseRatioBefore: number
  noiseRatioDuring: number
  noiseRatioAfter: number
  volatilityIncreasePercent: number
  eventCount: number
  eventType: string
  pair: string
  eventDatetime?: string
  timezoneOffset?: string
  isArchiveMode?: boolean
  eventLabel?: string
  meilleurMoment?: number
  stopLoss?: number
  trailingStop?: number
  timeout?: number
}>()

defineEmits<{ archive: [] }>()
</script>

<style scoped>
.results-container {
  display: flex;
  flex-direction: column;
  gap: 15px;
  width: 100%;
  overflow-y: auto;
  flex: 1;
  padding: 0;
  margin: 0;
}
</style>
