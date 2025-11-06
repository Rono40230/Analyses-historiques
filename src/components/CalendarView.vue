<template>
  <div class="main-container">
    <!-- Mode selector -->
    <div class="view-mode-selector">
      <button 
        class="mode-button" 
        :class="{ active: activeSubTab === 'calendar' }"
        @click="activeSubTab = 'calendar'"
      >
        📅 Calendrier Économique
      </button>
      <button 
        class="mode-button" 
        :class="{ active: activeSubTab === 'pairs' }"
        @click="activeSubTab = 'pairs'"
      >
        📊 Données de Paires
      </button>
      
      <!-- Sélecteur de fichier calendrier (visible seulement dans l'onglet Calendrier) -->
      <CalendarFileSelector 
        v-if="activeSubTab === 'calendar'"
        @file-selected="handleFileSelected"
        ref="fileSelectorRef"
        class="file-selector-right"
      />
    </div>

    <!-- Content Area -->
    <div class="content-area">
      <!-- Calendrier Économique Content -->
      <div v-if="activeSubTab === 'calendar'">
        <!-- Indicateur d'import -->
        <div v-if="importInfo && importInfo.total_events > 0" class="import-info">
          <div class="info-item">
            <span class="info-label">📊 Événements en base :</span>
            <span class="info-value">{{ importInfo.total_events.toLocaleString() }}</span>
          </div>
          <div class="info-item" v-if="importInfo.last_import_date">
            <span class="info-label">🕒 Dernier import :</span>
            <span class="info-value">{{ new Date(importInfo.last_import_date).toLocaleString('fr-FR') }}</span>
          </div>
          <div class="info-item" v-if="importInfo.oldest_event_date && importInfo.newest_event_date">
            <span class="info-label">📆 Période couverte :</span>
            <span class="info-value">{{ importInfo.oldest_event_date }} → {{ importInfo.newest_event_date }}</span>
          </div>
        </div>
        
        <div v-else-if="!loadingInfo" class="import-info warning">
          <span>⚠️ Aucun calendrier importé. Importez un calendrier pour commencer.</span>
        </div>

        <!-- Liste des fichiers disponibles -->
        <CalendarFilesList ref="calendarFilesListRef" />
      </div>

      <!-- Données de Paires Content -->
      <div v-if="activeSubTab === 'pairs'">
        <PairDataImport />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import PairDataImport from './PairDataImport.vue'
import CalendarFilesList from './CalendarFilesList.vue'
import CalendarFileSelector from './CalendarFileSelector.vue'

interface CalendarImportInfo {
  total_events: number
  last_import_date: string | null
  oldest_event_date: string | null
  newest_event_date: string | null
}

const activeSubTab = ref<'calendar' | 'pairs'>('calendar')
const selectedCalendarFile = ref<string>('') // Nouveau: fichier calendrier actif
const importInfo = ref<CalendarImportInfo | null>(null)
const loadingInfo = ref(false)
const fileSelectorRef = ref<InstanceType<typeof CalendarFileSelector> | null>(null)
const calendarFilesListRef = ref<InstanceType<typeof CalendarFilesList> | null>(null)

// Gérer le changement de fichier calendrier
async function handleFileSelected(filename: string) {
  selectedCalendarFile.value = filename
  console.log('📅 Fichier calendrier sélectionné:', filename)
  
  // Recharger les infos du bandeau vert pour ce fichier
  await loadFileStats(filename)
}

// Charger les statistiques d'un fichier spécifique
async function loadFileStats(filename: string) {
  loadingInfo.value = true
  try {
    // Récupérer les informations du fichier depuis la liste
    const files = await invoke<any[]>('list_calendar_files')
    const file = files.find((f: any) => f.filename === filename)
    
    if (file) {
      // Mettre à jour le bandeau avec les infos du fichier
      const dates = file.date_range ? file.date_range.split(' → ') : []
      
      importInfo.value = {
        total_events: file.event_count || 0,
        last_import_date: file.modified || null,
        oldest_event_date: dates[0] || null,
        newest_event_date: dates[1] || null,
      }
      
      console.log('✅ Stats mises à jour:', importInfo.value)
    } else {
      console.warn('⚠️ Fichier non trouvé:', filename)
      importInfo.value = null
    }
  } catch (e) {
    console.error('Erreur chargement stats fichier:', e)
    importInfo.value = null
  } finally {
    loadingInfo.value = false
  }
}

async function loadImportInfo() {
  loadingInfo.value = true
  try {
    const info = await invoke<CalendarImportInfo>('get_calendar_import_info')
    importInfo.value = info
  } catch (e) {
    console.error('Erreur récupération info:', e)
  } finally {
    loadingInfo.value = false
  }
}

onMounted(() => {
  loadImportInfo()
})
</script>

<style scoped>
.main-container {
  background: #161b22;
  border-radius: 16px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.4);
  border: 1px solid #30363d;
  overflow: hidden;
  color: #e2e8f0;
}

.header-section {
  background: linear-gradient(135deg, #1c2128 0%, #161b22 100%);
  padding: 30px;
  border-bottom: 2px solid #30363d;
}

.main-title {
  display: flex;
  align-items: center;
  gap: 15px;
  color: #e6edf3;
  font-size: 2em;
  margin: 0 0 10px 0;
  font-weight: 700;
}

.main-title .icon {
  font-size: 1.2em;
}

.main-subtitle {
  color: #8b949e;
  font-size: 1.1em;
  margin: 0;
  line-height: 1.5;
}

.view-mode-selector {
  display: flex;
  gap: 15px;
  padding: 20px;
  background: #0d1117;
  border-bottom: 2px solid #30363d;
  align-items: center;
}

.file-selector-right {
  margin-left: auto;
}

.mode-button {
  flex: 0 1 auto;
  padding: 15px 20px;
  border: 2px solid #30363d;
  background: #161b22;
  color: #8b949e;
  border-radius: 8px;
  font-size: 1.1em;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
}

.mode-button:hover {
  background: #1c2128;
  border-color: #58a6ff;
  color: #58a6ff;
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(88, 166, 255, 0.3);
}

.mode-button.active {
  background: linear-gradient(135deg, #1f6feb 0%, #388bfd 100%);
  color: #ffffff;
  border-color: #58a6ff;
  box-shadow: 0 4px 12px rgba(88, 166, 255, 0.4);
}

.content-area {
  padding: 30px;
  min-height: 400px;
}

.content-area h3 {
  color: #e6edf3;
  margin-bottom: 20px;
  font-size: 1.5em;
}

.content-area h4 {
  color: #e6edf3;
  margin-bottom: 10px;
  font-size: 1.2em;
}

.import-info {
  background: linear-gradient(135deg, #238636 0%, #2ea043 100%);
  border-radius: 10px;
  padding: 15px 20px;
  margin-bottom: 20px;
  display: flex;
  gap: 30px;
  align-items: center;
  color: white;
  box-shadow: 0 2px 8px rgba(35, 134, 54, 0.3);
}

.import-info.warning {
  background: linear-gradient(135deg, #ffc107 0%, #ff9800 100%);
  justify-content: center;
  font-weight: 600;
}

.info-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.info-label {
  font-weight: 600;
  opacity: 0.9;
  font-size: 0.9em;
}

.info-value {
  font-weight: 700;
  font-size: 1em;
  background: rgba(255, 255, 255, 0.2);
  padding: 4px 12px;
  border-radius: 6px;
}
</style>
