<template>
  <div v-if="isOpen">
    <div v-if="!showPreview" class="modal-overlay" @click.self="close">
      <div class="modal-content">
        <div class="modal-header">
          <div class="header-title">🖨️ Export PDF</div>
          <button class="close-btn" @click="close">✕</button>
        </div>

        <div class="modal-section">
          <ExportForm
            v-model:selectedReports="selectedReports"
            v-model:selectedCalendarId="selectedCalendarId"
            v-model:selectedPairMode="selectedPairMode"
            :calendars="calendars"
            :symbols="volatilityStore.symbols"
          />

          <div v-if="isGenerating" class="progress-section">
            <div class="spinner">⏳</div>
            <p>Génération du PDF en cours... {{ Math.round(progress) }}%</p>
          </div>
          
          <div v-if="error" class="error-message">
            {{ error }}
          </div>
        </div>

        <div class="modal-footer">
          <button class="btn-secondary" @click="close">Annuler</button>
          <button 
            class="btn-primary" 
            @click="handleGenerate"
            :disabled="isGenerating || selectedReports.length === 0 || !selectedCalendarId"
          >
            {{ isGenerating ? 'Génération...' : 'Prévisualiser' }}
          </button>
        </div>
      </div>
    </div>

    <PdfPreviewModal 
      :is-open="showPreview"
      :pdf-url="previewUrl"
      @close="closePreview"
      @download="handleDownload"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { usePdfExport } from '../composables/usePdfExport'
import { useVolatilityStore } from '../stores/volatility'
import ExportForm, { type CalendarMetadata } from './export/ExportForm.vue'
import PdfPreviewModal from './export/PdfPreviewModal.vue'

const props = defineProps<{
  isOpen: boolean
  currentSymbol?: string
}>()

const emit = defineEmits<{
  (e: 'close'): void
}>()

const volatilityStore = useVolatilityStore()
const { generatePdf, isGenerating, progress, error } = usePdfExport()

const selectedReports = ref<string[]>(['bidi', 'ranking'])
const selectedPairMode = ref('all')
const calendars = ref<CalendarMetadata[]>([])
const selectedCalendarId = ref<number | null>(null)
const showPreview = ref(false)
const previewUrl = ref<string | null>(null)

onMounted(async () => {
  await loadCalendars()
  if (volatilityStore.symbols.length === 0) {
    await volatilityStore.loadSymbols()
  }
})

watch(() => props.isOpen, async (newVal) => {
  if (newVal) {
    showPreview.value = false
    if (previewUrl.value) {
      URL.revokeObjectURL(previewUrl.value)
      previewUrl.value = null
    }
    
    await loadCalendars()
    // Sélectionner le calendrier actif par défaut
    const activeId = localStorage.getItem('activeCalendarId')
    if (activeId) {
      selectedCalendarId.value = parseInt(activeId, 10)
    } else if (calendars.value.length > 0) {
      selectedCalendarId.value = calendars.value[0].id
    }
  } else {
    if (previewUrl.value) {
      URL.revokeObjectURL(previewUrl.value)
      previewUrl.value = null
    }
    showPreview.value = false
  }
})

async function loadCalendars() {
  try {
    calendars.value = await invoke<CalendarMetadata[]>('get_calendars_metadata')
  } catch (e) {
    // Silent error or handle via UI notification if needed
  }
}

function close() {
  emit('close')
}

async function handleGenerate() {
  if (!selectedCalendarId.value) {
    error.value = "Veuillez sélectionner un calendrier"
    return
  }

  const selectedCal = calendars.value.find(c => c.id === selectedCalendarId.value)
  if (!selectedCal) return

  const pairs = selectedPairMode.value === 'all' 
    ? volatilityStore.symbols.map(s => s.symbol)
    : [selectedPairMode.value]

  // Sauvegarder le calendrier sélectionné pour l'analyse
  localStorage.setItem('activeCalendarId', selectedCalendarId.value.toString())

  const result = await generatePdf(selectedReports.value, {
    periodStart: selectedCal.start_date || new Date().toISOString(),
    periodEnd: selectedCal.end_date || new Date().toISOString(),
    pairs
  }, true)
  
  if (result && typeof result === 'string') {
    previewUrl.value = result
    showPreview.value = true
  }
}

function handleDownload() {
  if (previewUrl.value) {
    const link = document.createElement('a')
    link.href = previewUrl.value
    link.download = `analyse_export_${new Date().toISOString().split('T')[0]}.pdf`
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    close()
  }
}

function closePreview() {
  if (previewUrl.value) {
    URL.revokeObjectURL(previewUrl.value)
    previewUrl.value = null
  }
  showPreview.value = false
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(4px);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background: #1e1e1e;
  border: 1px solid #333;
  border-radius: 12px;
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 20px 50px rgba(0, 0, 0, 0.5);
}

.modal-header {
  padding: 1.5rem;
  border-bottom: 1px solid #333;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: #fff;
}

.close-btn {
  background: none;
  border: none;
  color: #888;
  font-size: 1.5rem;
  cursor: pointer;
  padding: 0.5rem;
}

.close-btn:hover {
  color: #fff;
}

.modal-section {
  padding: 1.5rem;
  overflow-y: auto;
}

.modal-footer {
  padding: 1.5rem;
  border-top: 1px solid #333;
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
}

.btn-primary {
  background: #4a9eff;
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-primary:hover:not(:disabled) {
  background: #3a8eef;
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-secondary {
  background: transparent;
  color: #aaa;
  border: 1px solid #333;
  padding: 0.75rem 1.5rem;
  border-radius: 6px;
  cursor: pointer;
}

.btn-secondary:hover {
  border-color: #666;
  color: #fff;
}

.progress-section {
  margin-top: 1.5rem;
  text-align: center;
  color: #4a9eff;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 1rem;
}

.error-message {
  margin-top: 1rem;
  color: #ff4a4a;
  background: rgba(255, 74, 74, 0.1);
  padding: 0.75rem;
  border-radius: 6px;
  font-size: 0.9rem;
}
</style>
