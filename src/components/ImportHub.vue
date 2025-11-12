<template>
  <div class="import-hub">
    <div class="sections-container">
      <section class="import-section">
        <h3>📅 Calendriers Économique</h3>
        <div v-if="calendarsMetadata.length > 0" class="info-box">
          <div>Calendriers: {{ calendarsMetadata.length }}</div>
          <div>Événements: {{ calendarsMetadata.reduce((s, c) => s + c.event_count, 0).toLocaleString() }}</div>
        </div>
        <div v-else class="info-box warning">Aucun calendrier importé.</div>
        <div v-if="calendarsMetadata.length > 0" class="table-container">
          <table class="data-table">
            <thead>
              <tr><th>Nom</th><th>Événements</th><th>Période</th><th>Actions</th></tr>
            </thead>
            <tbody>
              <tr v-for="cal in calendarsMetadata" :key="cal.id">
                <td><strong>{{ cal.name }}</strong></td>
                <td>{{ cal.event_count.toLocaleString() }}</td>
                <td>{{ formatCalendarPeriod(cal) }}</td>
                <td><button @click="deleteCalendar(cal.id)" class="btn-delete">🗑️ Supprimer</button></td>
              </tr>
            </tbody>
          </table>
        </div>
        <button @click="importCalendars" class="btn-import" :disabled="loadingCalendars">
          <span v-if="loadingCalendars" class="spinner">⏳</span>
          <span v-else>📥</span>
          Importer calendrier
        </button>
      </section>

      <section class="import-section">
        <h3>💱 Paires de Trading</h3>
        <div v-if="pairsMetadata.length > 0" class="info-box">
          <div>Paires: {{ pairsMetadata.length }}</div>
          <div>Bougies: {{ pairsMetadata.reduce((s, p) => s + p.candle_count, 0).toLocaleString() }}</div>
        </div>
        <div v-else class="info-box warning">Aucune paire importée.</div>
        <div v-if="pairsMetadata.length > 0" class="table-container">
          <table class="data-table">
            <thead>
              <tr><th>Paire</th><th>Bougies</th><th>Période</th><th>Actions</th></tr>
            </thead>
            <tbody>
              <tr v-for="pair in pairsMetadata" :key="pair.symbol">
                <td><strong>{{ pair.symbol }}</strong></td>
                <td>{{ pair.candle_count.toLocaleString() }}</td>
                <td>{{ formatPeriod(pair) }}</td>
                <td><button @click="deletePair(pair.symbol)" class="btn-delete">🗑️ Supprimer</button></td>
              </tr>
            </tbody>
          </table>
        </div>
        <button @click="importPairs" class="btn-import" :disabled="loadingPairs">
          <span v-if="loadingPairs" class="spinner">⏳</span>
          <span v-else>📥</span>
          Importer paires
        </button>
      </section>
    </div>

    <div v-if="showDeleteConfirm" class="modal-overlay" @click.self="showDeleteConfirm = false">
      <div class="modal">
        <h3>Confirmation</h3>
        <p>{{ deleteMessage }}</p>
        <div class="modal-buttons">
          <button @click="confirmDelete" class="btn-confirm">✅ Confirmer</button>
          <button @click="showDeleteConfirm = false" class="btn-cancel">❌ Annuler</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { useVolatilityStore } from '../stores/volatility'

const store = useVolatilityStore()

const calendarsMetadata = ref<any[]>([])
const pairsMetadata = ref<any[]>([])
const loadingCalendars = ref(false)
const loadingPairs = ref(false)
const showDeleteConfirm = ref(false)
const deleteMessage = ref('')
const deleteType = ref<'calendar' | 'pair'>('calendar')
const deleteId = ref(0)
const deleteSymbol = ref('')
const deleteTimeframe = ref('')

onMounted(async () => {
  await loadMetadata()
})

async function loadMetadata() {
  try {
    const calendars = await invoke<any[]>('get_calendars_metadata')
    const pairs = await invoke<any[]>('get_pairs_metadata')
    calendarsMetadata.value = calendars || []
    pairsMetadata.value = pairs || []
  } catch (err) {
    console.error('Erreur chargement métadonnées:', err)
  }
}

function formatPeriod(pair: any): string {
  if (!pair.start_date && !pair.end_date) return 'N/A'
  
  const formatDate = (dateString: string | null | undefined): string => {
    if (!dateString) return '?'
    try {
      const date = new Date(dateString)
      return date.toLocaleDateString('fr-FR', { year: 'numeric', month: '2-digit', day: '2-digit' })
    } catch {
      return dateString.substring(0, 10)
    }
  }
  
  const start = formatDate(pair.start_date)
  const end = formatDate(pair.end_date)
  return `du ${start} au ${end}`
}

function formatCalendarPeriod(calendar: any): string {
  if (!calendar.start_date && !calendar.end_date) return 'N/A'
  
  const formatDate = (dateString: string | null | undefined): string => {
    if (!dateString) return '?'
    try {
      const date = new Date(dateString)
      return date.toLocaleDateString('fr-FR', { year: 'numeric', month: '2-digit', day: '2-digit' })
    } catch {
      return dateString.substring(0, 10)
    }
  }
  
  const start = formatDate(calendar.start_date)
  const end = formatDate(calendar.end_date)
  return `du ${start} au ${end}`
}

async function importCalendars() {
  loadingCalendars.value = true
  try {
    const selected = await open({
      multiple: true,
      filters: [{ name: 'CSV', extensions: ['csv'] }]
    })
    
    if (!selected) return
    
    const paths = Array.isArray(selected) ? selected : [selected]
    await invoke('import_calendar_files', { paths })
    await loadMetadata()
    store.triggerDataRefresh() // Rafraîchir les données
  } catch (err) {
    console.error('Erreur import calendrier:', err)
  } finally {
    loadingCalendars.value = false
  }
}

async function importPairs() {
  loadingPairs.value = true
  try {
    const selected = await open({
      multiple: true,
      filters: [{ name: 'CSV', extensions: ['csv'] }]
    })
    
    if (!selected) return
    
    const paths = Array.isArray(selected) ? selected : [selected]
    await invoke('import_pair_data', { paths })
    await loadMetadata()
    store.triggerDataRefresh() // Rafraîchir les données
  } catch (err) {
    console.error('Erreur import paires:', err)
  } finally {
    loadingPairs.value = false
  }
}

function deleteCalendar(id: number) {
  deleteType.value = 'calendar'
  deleteId.value = id
  deleteMessage.value = 'Supprimer ce calendrier et tous ses événements?'
  showDeleteConfirm.value = true
}

function deletePair(symbol: string) {
  const pair = pairsMetadata.value.find(p => p.symbol === symbol)
  if (!pair) return
  
  deleteType.value = 'pair'
  deleteId.value = pair.id
  deleteSymbol.value = pair.symbol
  deleteTimeframe.value = pair.timeframe
  deleteMessage.value = 'Supprimer cette paire et toutes ses bougies?'
  showDeleteConfirm.value = true
}

async function confirmDelete() {
  try {
    if (deleteType.value === 'calendar') {
      await invoke('delete_calendar_from_db', { calendarId: deleteId.value })
    } else {
      await invoke('delete_pair_from_db', { 
        symbol: deleteSymbol.value, 
        timeframe: deleteTimeframe.value 
      })
    }
    await loadMetadata()
  } catch (err) {
    console.error('Erreur suppression:', err)
  } finally {
    showDeleteConfirm.value = false
  }
}
</script>

<style scoped>
.import-hub { padding: 30px; }
.sections-container { display: grid; grid-template-columns: 1fr 1fr; gap: 30px; }
.import-section { background: #1a202c; padding: 25px; border-radius: 12px; border: 1px solid #2d3748; }
.import-section h3 { color: #e2e8f0; margin-top: 0; }
.info-box { padding: 15px; background: #2d3748; border-radius: 8px; color: #e2e8f0; margin-bottom: 20px; }
.info-box.warning { background: #7f3f1f; color: #fbbf24; }
.table-container { overflow-x: auto; margin-bottom: 20px; }
.data-table { width: 100%; border-collapse: collapse; }
.data-table th { background: #2d3748; padding: 12px; text-align: left; font-weight: 600; color: #e2e8f0; border-bottom: 2px solid #4a5568; }
.data-table td { padding: 12px; border-bottom: 1px solid #2d3748; color: #e2e8f0; }
.btn-import { 
  display: block;
  width: 100%;
  padding: 12px 20px; 
  background: linear-gradient(135deg, #1f6feb 0%, #388bfd 100%);
  color: white; 
  border: none; 
  border-radius: 6px; 
  cursor: pointer; 
  font-weight: 600;
  margin-top: 15px;
  transition: all 0.3s;
  font-size: 1em;
}
.btn-import:hover {
  background: linear-gradient(135deg, #1664d9 0%, #2d7ee5 100%);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(31, 111, 235, 0.4);
}
.btn-import:disabled {
  opacity: 0.7;
  cursor: not-allowed;
  transform: none;
}
.spinner {
  display: inline-block;
  animation: spin 1s linear infinite;
  margin-right: 6px;
}
@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
.btn-delete { padding: 6px 12px; background: #dc2626; color: white; border: none; border-radius: 6px; cursor: pointer; }
.modal-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.7); display: flex; align-items: center; justify-content: center; z-index: 1000; }
.modal { background: #1a202c; padding: 30px; border-radius: 12px; border: 1px solid #2d3748; max-width: 400px; }
.modal h3 { color: #e2e8f0; }
.modal p { color: #cbd5e0; }
.modal-buttons { display: flex; gap: 10px; margin-top: 20px; }
.btn-confirm { flex: 1; padding: 10px; background: #10b981; color: white; border: none; border-radius: 6px; cursor: pointer; }
.btn-cancel { flex: 1; padding: 10px; background: #6b7280; color: white; border: none; border-radius: 6px; cursor: pointer; }
</style>
