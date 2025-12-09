<template>
  <div class="container">
    <div class="controls">
      <div v-if="props.showCalendarSelector" class="control-group">
        <CalendarFileSelector 
          class="file-selector-inline"
          @file-selected="onCalendarSelected"
        />
      </div>

      <label for="pair-select">Paire:</label>
      <select id="pair-select" v-model="selected" @change="load" class="pair-select">
        <option value="">-- Choisir --</option>
        <option v-for="p in pairs" :key="p" :value="p">{{ p }}</option>
      </select>
      <label for="event-type-select">Type d'événement:</label>
      <SearchableEventDropdown 
        id="event-type-select"
        v-model="selectedEventType"
        :events="eventTypeOptions"
        :loading="eventTypesLoading"
        :error="eventTypesError"
        @update:modelValue="load"
      />
      <div v-if="eventTypesError" class="error-small">⚠️ {{ eventTypesError }}</div>
      <div v-if="!eventTypesError && eventTypes.length === 0 && !eventTypesLoading" class="warning-small">📭 Aucun événement trouvé</div>
      <div v-if="eventTypesLoading" class="warning-small">⏳ Chargement des événements...</div>
    </div>

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
import CalendarFileSelector from './CalendarFileSelector.vue'
import RetroAnalysisResults from './RetroAnalysisResults.vue'
import SearchableEventDropdown from './SearchableEventDropdown.vue'

interface Symbol { symbol: string; file_path?: string }

const props = defineProps<{ 
  calendarId: number | null
  showCalendarSelector?: boolean
}>()

const emit = defineEmits<{
  'calendar-selected': [filename: string]
}>()

const onCalendarSelected = (filename: string) => {
  emit('calendar-selected', filename)
}

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
  eventTypes.value.map(et => ({
    name: et.name,
    label: getEventLabel(et.name),
    count: et.count
  }))
)

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
  archivePeriodStart.value = graphData.value.event_type // Format sera mis à jour avec la date réelle
  archivePeriodEnd.value = graphData.value.pair // Format sera mis à jour avec la date réelle
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

function handleArchiveSaved() {
  showArchiveModal.value = false
}
</script>

<style scoped>
.container { 
  min-height: 100%;
  padding: 12px 20px 20px 20px; 
  background: #0d1117; 
  border-radius: 8px; 
  color: #e2e8f0; 
  display: flex; 
  flex-direction: column; 
  overflow: auto;
}

.controls { 
  margin-bottom: 15px; 
  display: flex; 
  gap: 15px; 
  align-items: flex-end; 
  flex-wrap: wrap; 
  flex-shrink: 0;
}

.control-group { 
  display: flex; 
  align-items: flex-end; 
  gap: 10px; 
  flex-shrink: 0;
}

label { 
  display: block; 
  color: #e2e8f0; 
  font-weight: 600; 
  margin-bottom: 6px;
  font-size: 0.95em;
}

:deep(.file-selector-inline) { 
  margin: 0; 
}

.pair-select { 
  min-width: 150px;
  padding: 10px 12px; 
  font-size: 0.95em; 
  border: 2px solid #4a5568; 
  border-radius: 6px; 
  background: #2d3748; 
  color: #000000 !important; 
  cursor: pointer; 
  transition: all 0.3s;
}

.pair-select:hover { 
  border-color: #667eea; 
  background: #374151; 
}

.pair-select:focus { 
  outline: none; 
  border-color: #667eea; 
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.2); 
}

.pair-select option { 
  background: #ffffff; 
  color: #000000 !important; 
}

.spinner { 
  text-align: center; 
  color: #8b949e; 
  padding: 20px; 
  font-size: 1.1em; 
  display: flex; 
  flex-direction: column; 
  align-items: center; 
  justify-content: center; 
  gap: 15px; 
  min-height: auto;
}

.spinner::before { 
  content: '⏳'; 
  font-size: 50px; 
  animation: hourglassFlip 1s ease-in-out infinite; 
  display: block; 
  order: -1; 
}

.empty { 
  text-align: center; 
  color: #8b949e; 
  padding: 20px; 
  font-size: 1.1em; 
  display: flex;
  align-items: center;
  justify-content: center;
}

.error { 
  background: #3d2626; 
  color: #f85149; 
  padding: 15px; 
  border-radius: 8px; 
  margin-bottom: 20px;
  flex-shrink: 0;
}

.error-small { 
  font-size: 0.8em; 
  color: #f85149; 
  margin-top: 5px; 
}

.warning-small { 
  font-size: 0.8em; 
  color: #d29922; 
  margin-top: 5px; 
}

@keyframes hourglassFlip { 
  0% { transform: scaleX(1) rotateY(0deg); } 
  50% { transform: scaleX(-1) rotateY(180deg); } 
  100% { transform: scaleX(1) rotateY(360deg); } 
}

/* Responsive pour tablettes et petits écrans */
@media (max-width: 768px) {
  .container {
    padding: 10px 15px 15px 15px;
  }
  
  .controls {
    gap: 10px;
    margin-bottom: 15px;
  }
  
  label {
    font-size: 0.9em;
    margin-bottom: 4px;
  }
  
  .pair-select {
    min-width: 120px;
    padding: 8px 10px;
    font-size: 0.9em;
  }
}

/* Responsive pour petits téléphones */
@media (max-width: 480px) {
  .container {
    padding: 8px 10px 10px 10px;
  }
  
  .controls {
    flex-direction: column;
    gap: 8px;
    margin-bottom: 12px;
  }
  
  .control-group {
    width: 100%;
    flex-direction: column;
  }
  
  label {
    font-size: 0.85em;
    margin-bottom: 3px;
  }
  
  .pair-select {
    width: 100%;
    min-width: unset;
    padding: 8px;
    font-size: 0.85em;
  }
  
  .spinner {
    padding: 20px 10px;
    font-size: 1em;
  }
}
</style>
