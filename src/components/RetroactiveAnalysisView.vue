<template>
  <div class="container">
    <RetroAnalysisControls
      :pairs="pairs"
      :selected-pair="selected"
      :selected-event-type="selectedEventType"
      :event-types="eventTypeOptions"
      :event-types-loading="eventTypesLoading"
      :event-types-error="eventTypesError"
      :show-calendar-selector="props.showCalendarSelector"
      @update:selected-pair="selected = $event"
      @update:selected-event-type="selectedEventType = $event"
      @calendar-selected="onCalendarSelected"
      @load="load"
    />

    <div v-if="loading" class="spinner">⏳ Chargement...</div>
    <div v-else-if="error" class="error">{{ error }}</div>
    <div v-else-if="peakDelayLoading || decayLoading" class="spinner">⏳ Chargement des analyses...</div>
    <div v-else-if="peakDelayError" class="error">❌ Erreur Peak Delay: {{ peakDelayError }}</div>
    <div v-else-if="decayError" class="error">❌ Erreur Décroissance: {{ decayError }}</div>
    <div v-else-if="!peakDelayResults || !decayResults" class="empty">📭 Chargez une paire et sélectionnez un événement</div>

    <RetroAnalysisResults
      v-else
      :atr-timeline-before="graphData?.atr_timeline_before"
      :atr-timeline-after="graphData?.atr_timeline_after"
      :body-timeline-before="graphData?.body_timeline_before"
      :body-timeline-after="graphData?.body_timeline_after"
      :noise-ratio-before="graphData?.noise_ratio_before ?? 0"
      :noise-ratio-during="graphData?.noise_ratio_during ?? 0"
      :noise-ratio-after="graphData?.noise_ratio_after ?? 0"
      :volatility-increase-percent="graphData?.volatility_increase_percent ?? 0"
      :event-count="graphData?.event_count ?? 0"
      :event-type="selectedEventType"
      :pair="selected"
      :event-datetime="graphData?.event_datetime"
      :timezone-offset="graphData?.timezone_offset"
      :meilleur-moment="graphData?.meilleur_moment ?? 0"
      :stop-loss="graphData?.stop_loss ?? 0"
      :trailing-stop="graphData?.trailing_stop ?? 0"
      :timeout="graphData?.timeout ?? 60"
      :event-label="getEventLabel(selectedEventType)"
      @archive="openArchiveModal"
    />

    <ArchiveModal 
      :show="showArchiveModal" 
      archive-type="Métriques Rétrospectives" 
      :period-start="archivePeriodStart" 
      :period-end="archivePeriodEnd" 
      :symbol="selected" 
      :event-name="selectedEventType" 
      :event-name-fr="getEventLabel(selectedEventType)" 
      :data-json="archiveDataJson" 
      @close="showArchiveModal = false" 
      @saved="handleArchiveSaved" 
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useRetrospectiveAnalysis } from '../composables/useRetrospectiveAnalysis'
import { useRetroAnalysisGraphData } from '../composables/useRetroAnalysisGraphData'
import ArchiveModal from './ArchiveModal.vue'
import RetroAnalysisControls from './RetroAnalysisControls.vue'
import RetroAnalysisResults from './RetroAnalysisResults.vue'

interface Symbol { symbol: string; file_path?: string }

const props = defineProps<{ 
  calendarId: number | null
  showCalendarSelector?: boolean
}>()

const emit = defineEmits<{
  'calendar-selected': [filename: string]
}>()

const { peakDelayLoading, peakDelayError, peakDelayResults, analyzePeakDelay,
         decayLoading, decayError, decayResults, analyzeDecayProfile,
         eventTypes, eventTypesError, eventTypesLoading, loadEventTypes, getEventLabel } = useRetrospectiveAnalysis()
const { graphData, loading: graphLoading, chargerDonnéesGraph } = useRetroAnalysisGraphData()

const pairs = ref<string[]>([])
const selected = ref('')
const selectedEventType = ref('')
const loading = ref(false)
const error = ref<string | null>(null)
const showArchiveModal = ref(false)
const archivePeriodStart = ref('')
const archivePeriodEnd = ref('')
const archiveDataJson = ref('')

const eventTypeOptions = computed(() =>
  eventTypes.value.map(et => ({ name: et.name, label: getEventLabel(et.name), count: et.count }))
)

const onCalendarSelected = (filename: string) => { emit('calendar-selected', filename) }

onMounted(async () => {
  try {
    const s = await invoke<Symbol[]>('load_symbols')
    pairs.value = s.map((x: Symbol) => x.symbol)
  } catch (e) {
    pairs.value = ['EURUSD']
  }
  await loadEventTypes(props.calendarId ?? undefined)
})

async function load() {
  if (!selected.value || !selectedEventType.value) return
  loading.value = true
  error.value = null
  try {
    await analyzePeakDelay(selected.value, selectedEventType.value)
    await analyzeDecayProfile(selected.value, selectedEventType.value)
    await chargerDonnéesGraph(selected.value, selectedEventType.value)
  } catch (e) {
    error.value = String(e)
  } finally {
    loading.value = false
  }
}

function openArchiveModal() {
  if (!graphData.value || !selected.value || !selectedEventType.value) return
  archivePeriodStart.value = graphData.value.event_type
  archivePeriodEnd.value = graphData.value.pair
  archiveDataJson.value = JSON.stringify({
    atrTimelineBefore: graphData.value.atr_timeline_before,
    atrTimelineAfter: graphData.value.atr_timeline_after,
    bodyTimelineBefore: graphData.value.body_timeline_before,
    bodyTimelineAfter: graphData.value.body_timeline_after,
    noiseRatioBefore: graphData.value.noise_ratio_before,
    noiseRatioDuring: graphData.value.noise_ratio_during,
    noiseRatioAfter: graphData.value.noise_ratio_after,
    volatilityIncreasePercent: graphData.value.volatility_increase_percent,
    eventCount: graphData.value.event_count,
    pair: selected.value,
    eventType: selectedEventType.value,
    eventLabel: getEventLabel(selectedEventType.value)
  })
  showArchiveModal.value = true
}

function handleArchiveSaved() { showArchiveModal.value = false }
</script>

<style scoped>
.container { min-height: 100%; padding: 12px 20px 20px 20px; background: #0d1117; border-radius: 8px; color: #e2e8f0; display: flex; flex-direction: column; overflow: auto; }
.spinner { text-align: center; color: #8b949e; padding: 20px; font-size: 1.1em; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 15px; }
.spinner::before { content: '⏳'; font-size: 50px; animation: flip 1s ease-in-out infinite; display: block; order: -1; }
.empty { text-align: center; color: #8b949e; padding: 20px; font-size: 1.1em; display: flex; align-items: center; justify-content: center; }
.error { background: #3d2626; color: #f85149; padding: 15px; border-radius: 8px; margin-bottom: 20px; flex-shrink: 0; }
@keyframes flip { 0% { transform: scaleX(1) rotateY(0deg); } 50% { transform: scaleX(-1) rotateY(180deg); } 100% { transform: scaleX(1) rotateY(360deg); } }
@media (max-width: 768px) { .container { padding: 10px 15px 15px 15px; } }
@media (max-width: 480px) { .container { padding: 8px 10px 10px 10px; } .spinner { padding: 20px 10px; font-size: 1em; } }
</style>
