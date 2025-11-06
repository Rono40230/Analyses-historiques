<template>
  <div class="cleaner-section">
    <h4>🧹 Nettoyage et Import</h4>
    <p class="info-text">
      ⚠️ Vos fichiers utilisent la virgule comme séparateur décimal (format européen) ? 
      Nettoyez-les avant l'import.
    </p>
    
    <div class="cleaner-controls">
      <button @click="selectFilesToClean" class="btn btn-cleaner" :disabled="cleaning">
        🧹 Nettoyer vos fichiers avant leur importation
      </button>
    </div>

    <!-- Cleaning Progress -->
    <div v-if="cleaning" class="progress-bar">
      <div class="progress-fill" :style="{ width: `${cleanProgress}%` }"></div>
      <span class="progress-text">Nettoyage en cours... {{ cleanProgress }}%</span>
    </div>

    <!-- Cleaning Results -->
    <div v-if="cleanedFiles.length > 0" class="cleaned-files-list">
      <h5>✅ Fichiers nettoyés ({{ cleanedFiles.length }})</h5>
      <div class="clean-item" v-for="(report, index) in cleanedFiles" :key="index">
        <span v-if="report.status === 'success'" class="status-icon">✅</span>
        <span v-else-if="report.status === 'partial'" class="status-icon">⚠️</span>
        <span v-else class="status-icon">❌</span>
        <span class="file-name">{{ report.original_file }}</span>
        <span class="file-stats">{{ report.lines_cleaned }} lignes</span>
        <span v-if="getErrorRate(report) >= 1.0" class="file-errors">{{ report.errors }} erreurs ({{ getErrorRate(report).toFixed(2) }}%)</span>
        <button 
          @click="handleImport(report.cleaned_file, index)" 
          class="btn btn-import-single"
          :class="{ 'importing': importing && importingIndex === index }"
          :disabled="importing"
        >
          <span :class="{ 'spin': importing && importingIndex === index }">
            {{ importing && importingIndex === index ? '⏳' : '📥' }}
          </span>
          Importer
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'

interface CleaningReport {
  original_file: string
  cleaned_file: string
  status: string
  lines_processed: number
  lines_cleaned: number
  errors: number
  warnings: string[]
}

const emit = defineEmits<{
  importStarted: [index: number]
  importCompleted: [success: boolean, index: number]
  error: [message: string]
}>()

const cleaning = ref(false)
const cleanProgress = ref(0)
const cleanedFiles = ref<CleaningReport[]>([])
const importing = ref(false)
const importingIndex = ref<number | null>(null)

function getErrorRate(report: CleaningReport): number {
  if (report.lines_processed === 0) return 0
  return (report.errors / report.lines_processed) * 100
}

async function selectFilesToClean() {
  try {
    const selected = await open({
      multiple: true,
      filters: [{
        name: 'Fichiers CSV',
        extensions: ['csv']
      }]
    })

    if (selected) {
      const paths = Array.isArray(selected) ? selected : [selected]
      await cleanFiles(paths)
    }
  } catch (error) {
    console.error('Erreur sélection fichiers:', error)
    emit('error', `Erreur: ${error}`)
  }
}

async function cleanFiles(paths: string[]) {
  cleaning.value = true
  cleanProgress.value = 0
  
  try {
    const progressInterval = setInterval(() => {
      if (cleanProgress.value < 90) {
        cleanProgress.value += 10
      }
    }, 200)
    
    const reports = await invoke<CleaningReport[]>('clean_csv_files', { paths })
    
    clearInterval(progressInterval)
    cleanProgress.value = 100
    
    cleanedFiles.value = reports
    
    setTimeout(() => {
      cleanProgress.value = 0
      cleaning.value = false
    }, 1000)
    
  } catch (error) {
    console.error('Erreur nettoyage:', error)
    emit('error', `Erreur nettoyage: ${error}`)
    cleaning.value = false
  }
}

async function handleImport(cleanedFilePath: string, index: number) {
  importing.value = true
  importingIndex.value = index
  emit('importStarted', index)
  
  try {
    await invoke('import_pair_data', { paths: [cleanedFilePath] })
    
    // Remove from cleaned files list after successful import
    cleanedFiles.value.splice(index, 1)
    emit('importCompleted', true, index)
    
  } catch (error) {
    console.error('Erreur import:', error)
    emit('error', `Erreur import: ${error}`)
    emit('importCompleted', false, index)
  } finally {
    importing.value = false
    importingIndex.value = null
  }
}
</script>

<style scoped>
.cleaner-section {
  background: rgba(56, 139, 253, 0.05);
  border: 1px solid rgba(56, 139, 253, 0.2);
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 25px;
}

.cleaner-section h4 {
  color: #58a6ff;
  margin-bottom: 10px;
  font-size: 1.1em;
}

.info-text {
  color: #8b949e;
  font-size: 0.9em;
  margin-bottom: 15px;
  line-height: 1.5;
}

.cleaner-controls {
  margin-bottom: 15px;
}

.btn-cleaner {
  background: linear-gradient(180deg, #388bfd 0%, #1f6feb 100%);
  color: white;
  border: none;
  padding: 12px 24px;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  box-shadow: 0 2px 8px rgba(56, 139, 253, 0.2);
}

.btn-cleaner:hover:not(:disabled) {
  background: linear-gradient(180deg, #539bf5 0%, #388bfd 100%);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(56, 139, 253, 0.3);
}

.btn-cleaner:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.progress-bar {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 8px;
  overflow: hidden;
  height: 30px;
  position: relative;
  margin-bottom: 15px;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #1f6feb, #58a6ff, #1f6feb);
  background-size: 200% 100%;
  animation: gradient-shift 2s ease infinite;
  transition: width 0.3s ease;
}

.progress-text {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  color: white;
  font-weight: 600;
  font-size: 0.9em;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
}

@keyframes gradient-shift {
  0% { background-position: 0% 50%; }
  50% { background-position: 100% 50%; }
  100% { background-position: 0% 50%; }
}

.cleaned-files-list {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 8px;
  padding: 15px;
  margin-top: 15px;
}

.cleaned-files-list h5 {
  color: #7ee787;
  margin-bottom: 12px;
  font-size: 0.95em;
}

.clean-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px;
  background: rgba(255, 255, 255, 0.03);
  border-radius: 6px;
  margin-bottom: 8px;
  font-size: 0.9em;
  flex-wrap: wrap;
  position: relative;
}

.status-icon {
  font-size: 1.2em;
}

.file-name {
  flex: 1;
  min-width: 200px;
}

.file-stats {
  color: #7ee787;
  font-weight: 500;
}

.file-errors {
  color: #f85149;
  font-weight: 500;
  margin-right: 10px;
}

.btn-import-single {
  background: linear-gradient(180deg, #2ea043 0%, #238636 100%);
  color: white;
  border: none;
  padding: 6px 16px;
  border-radius: 6px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  margin-left: auto;
  font-size: 0.85em;
}

.btn-import-single:hover:not(:disabled) {
  background: linear-gradient(180deg, #3fb950 0%, #2ea043 100%);
  transform: translateY(-1px);
  box-shadow: 0 3px 8px rgba(46, 160, 67, 0.3);
}

.btn-import-single:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.spin {
  display: inline-block;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>
