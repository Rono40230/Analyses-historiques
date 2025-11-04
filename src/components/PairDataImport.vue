<template>
  <div class="pair-import">
    <!-- Liste des fichiers CSV disponibles -->
    <PairFilesList />

    <!-- File Selection -->
    <div class="import-section">
      <h4>📁 Sélection des fichiers</h4>
      <div class="import-controls">
        <button @click="selectFiles" class="btn btn-browse" :disabled="importing">
          📁 Parcourir (Multi-sélection)
        </button>
        <button 
          @click="clearSelection" 
          class="btn btn-secondary" 
          :disabled="selectedFiles.length === 0 || importing"
        >
          🗑️ Effacer la sélection
        </button>
      </div>

      <!-- Selected Files List -->
      <div v-if="selectedFiles.length > 0" class="files-list">
        <h5>📋 Fichiers sélectionnés ({{ selectedFiles.length }})</h5>
        <div class="file-item" v-for="(file, index) in selectedFiles" :key="index">
          <span class="file-icon">📄</span>
          <span class="file-name">{{ file.name }}</span>
          <span class="file-size">{{ formatSize(file.size) }}</span>
          <button @click="removeFile(index)" class="btn-remove" :disabled="importing">❌</button>
        </div>
      </div>

      <div v-else class="no-files">
        <span>📂 Aucun fichier sélectionné</span>
      </div>

      <!-- Import Button -->
      <button 
        @click="importFiles" 
        class="btn btn-primary btn-large" 
        :disabled="selectedFiles.length === 0 || importing"
      >
        {{ importing ? '⏳ Import en cours...' : `📥 Importer ${selectedFiles.length} fichier(s)` }}
      </button>

      <!-- Progress Bar (if importing) -->
      <div v-if="importing" class="progress-bar">
        <div class="progress-fill" :style="{ width: `${importProgress}%` }"></div>
        <span class="progress-text">{{ importProgress }}%</span>
      </div>

      <!-- Success Message -->
      <div v-if="importSuccess" class="success">
        ✅ {{ importSuccess }}
      </div>

      <!-- Error Message -->
      <div v-if="importError" class="error">
        ❌ {{ importError }}
      </div>

      <!-- Import Summary -->
      <div v-if="importSummary" class="import-summary">
        <h5>📊 Résumé de l'import</h5>
        <div class="summary-grid">
          <div class="summary-item">
            <span class="summary-label">Fichiers traités :</span>
            <span class="summary-value success-text">{{ importSummary.successful }} / {{ importSummary.total_files }}</span>
          </div>
          <div class="summary-item" v-if="importSummary.failed > 0">
            <span class="summary-label">Échecs :</span>
            <span class="summary-value error-text">{{ importSummary.failed }}</span>
          </div>
          <div class="summary-item">
            <span class="summary-label">Paires mises à jour :</span>
            <span class="summary-value">{{ importSummary.pairs_updated.join(', ') || 'Aucune' }}</span>
          </div>
          <div class="summary-item">
            <span class="summary-label">Timeframes :</span>
            <span class="summary-value">{{ importSummary.timeframes.join(', ') || 'Aucun' }}</span>
          </div>
        </div>
        <div v-if="importSummary.errors.length > 0" class="error-details">
          <h6>Erreurs détaillées :</h6>
          <ul>
            <li v-for="(err, idx) in importSummary.errors" :key="idx">{{ err }}</li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import PairFilesList from './PairFilesList.vue'

interface FileInfo {
  path: string
  name: string
  size: number
}

interface ImportSummary {
  total_files: number
  successful: number
  failed: number
  pairs_updated: string[]
  timeframes: string[]
  errors: string[]
}

const selectedFiles = ref<FileInfo[]>([])
const importing = ref(false)
const importProgress = ref(0)
const importSuccess = ref('')
const importError = ref('')
const importSummary = ref<ImportSummary | null>(null)

async function selectFiles() {
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
      
      // Clear previous selection
      selectedFiles.value = []
      
      // Add files with metadata
      for (const path of paths) {
        const fileName = path.split('/').pop() || path.split('\\').pop() || 'unknown.csv'
        
        // Get file size (we'll estimate or fetch from backend)
        selectedFiles.value.push({
          path: path,
          name: fileName,
          size: 0 // Will be populated by backend or estimated
        })
      }
      
      importSuccess.value = ''
      importError.value = ''
      importSummary.value = null
    }
  } catch (e) {
    importError.value = `Erreur sélection fichiers: ${e}`
  }
}

function clearSelection() {
  selectedFiles.value = []
  importSuccess.value = ''
  importError.value = ''
  importSummary.value = null
}

function removeFile(index: number) {
  selectedFiles.value.splice(index, 1)
}

function formatSize(bytes: number): string {
  if (bytes === 0) return '—'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

async function importFiles() {
  if (selectedFiles.value.length === 0) return
  
  importing.value = true
  importProgress.value = 0
  importSuccess.value = ''
  importError.value = ''
  importSummary.value = null
  
  try {
    const paths = selectedFiles.value.map(f => f.path)
    
    // Simulate progress (replace with real progress tracking if backend supports it)
    const progressInterval = setInterval(() => {
      if (importProgress.value < 90) {
        importProgress.value += 10
      }
    }, 200)
    
    const summary = await invoke<ImportSummary>('import_pair_data', {
      paths: paths
    })
    
    clearInterval(progressInterval)
    importProgress.value = 100
    
    importSummary.value = summary
    
    if (summary.successful > 0) {
      importSuccess.value = `✅ ${summary.successful} fichier(s) importé(s) avec succès !`
      
      // Clear selection after success
      setTimeout(() => {
        selectedFiles.value = []
        importProgress.value = 0
      }, 2000)
    }
    
    if (summary.failed > 0) {
      importError.value = `⚠️ ${summary.failed} fichier(s) ont échoué. Voir détails ci-dessous.`
    }
    
  } catch (e) {
    importProgress.value = 0
    importError.value = `Échec import: ${e}`
  } finally {
    importing.value = false
  }
}
</script>

<style scoped>
.pair-import {
  padding: 20px 0;
}

.pair-import h3 {
  color: #e6edf3;
  margin-bottom: 10px;
}

.pair-import h4 {
  color: #e6edf3;
  margin-bottom: 15px;
}

.pair-import h5 {
  color: #e6edf3;
  font-size: 1.1em;
  margin-bottom: 10px;
}

.pair-import h6 {
  color: #f97583;
  font-size: 0.95em;
  margin-bottom: 8px;
}

.import-hint {
  font-size: 0.9em;
  color: #8b949e;
  margin-bottom: 20px;
  line-height: 1.6;
}

.import-hint small {
  font-size: 0.85em;
  opacity: 0.8;
  font-style: italic;
}

.import-section {
  background: linear-gradient(135deg, #1f6feb 0%, #388bfd 100%);
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 25px;
  color: white;
}

.import-controls {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 6px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
  font-size: 0.95em;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-browse {
  background: #17a2b8;
  color: white;
}

.btn-browse:hover:not(:disabled) {
  background: #138496;
  transform: translateY(-2px);
}

.btn-secondary {
  background: #6c757d;
  color: white;
}

.btn-secondary:hover:not(:disabled) {
  background: #5a6268;
  transform: translateY(-2px);
}

.btn-primary {
  background: #238636;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: #2ea043;
  transform: translateY(-2px);
}

.btn-large {
  width: 100%;
  padding: 15px 30px;
  font-size: 1.1em;
  margin-top: 15px;
}

.files-list {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 8px;
  padding: 15px;
  margin-bottom: 15px;
  max-height: 300px;
  overflow-y: auto;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  margin-bottom: 8px;
  transition: background 0.2s;
}

.file-item:hover {
  background: rgba(255, 255, 255, 0.15);
}

.file-icon {
  font-size: 1.5em;
}

.file-name {
  flex: 1;
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.file-size {
  color: rgba(255, 255, 255, 0.7);
  font-size: 0.9em;
  min-width: 80px;
  text-align: right;
}

.btn-remove {
  background: transparent;
  border: none;
  cursor: pointer;
  font-size: 1em;
  padding: 5px;
  opacity: 0.7;
  transition: opacity 0.2s;
}

.btn-remove:hover:not(:disabled) {
  opacity: 1;
}

.no-files {
  text-align: center;
  padding: 40px 20px;
  background: rgba(0, 0, 0, 0.2);
  border-radius: 8px;
  color: rgba(255, 255, 255, 0.6);
  font-style: italic;
  margin-bottom: 15px;
}

.progress-bar {
  position: relative;
  width: 100%;
  height: 30px;
  background: rgba(0, 0, 0, 0.3);
  border-radius: 15px;
  overflow: hidden;
  margin-top: 15px;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #238636, #2ea043);
  transition: width 0.3s ease;
  border-radius: 15px;
}

.progress-text {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-weight: 700;
  color: white;
  text-shadow: 0 0 4px rgba(0,0,0,0.5);
}

.success {
  padding: 12px;
  background: rgba(35, 134, 54, 0.2);
  border: 1px solid rgba(35, 134, 54, 0.5);
  border-radius: 6px;
  color: #3fb950;
  font-weight: 500;
  margin-top: 15px;
}

.error {
  padding: 12px;
  background: rgba(220, 53, 69, 0.2);
  border: 1px solid rgba(220, 53, 69, 0.5);
  border-radius: 6px;
  color: #f97583;
  font-weight: 500;
  margin-top: 15px;
}

.import-summary {
  background: rgba(0, 0, 0, 0.3);
  border-radius: 8px;
  padding: 15px;
  margin-top: 15px;
}

.summary-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 12px;
  margin-bottom: 15px;
}

.summary-item {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.summary-label {
  font-size: 0.9em;
  opacity: 0.8;
}

.summary-value {
  font-weight: 700;
  font-size: 1.1em;
}

.success-text {
  color: #3fb950;
}

.error-text {
  color: #f97583;
}

.error-details {
  margin-top: 15px;
  padding-top: 15px;
  border-top: 1px solid rgba(255, 255, 255, 0.2);
}

.error-details ul {
  list-style: none;
  padding: 0;
}

.error-details li {
  padding: 5px 0;
  color: #f97583;
  font-size: 0.9em;
}

.info-section {
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 8px;
  padding: 20px;
}

.info-section h4 {
  margin-bottom: 15px;
}

.format-list {
  list-style: none;
  padding: 0;
  margin-bottom: 15px;
}

.format-list li {
  padding: 8px 0;
  color: #8b949e;
  border-bottom: 1px solid #30363d;
}

.format-list li:last-child {
  border-bottom: none;
}

.format-list strong {
  color: #58a6ff;
}

.info-note {
  background: rgba(88, 166, 255, 0.1);
  border-left: 3px solid #58a6ff;
  padding: 12px;
  border-radius: 4px;
  color: #8b949e;
  line-height: 1.6;
}

.info-note strong {
  color: #e6edf3;
}

.info-note code {
  background: #0d1117;
  padding: 2px 6px;
  border-radius: 3px;
  color: #58a6ff;
  font-family: 'Courier New', monospace;
}
</style>
