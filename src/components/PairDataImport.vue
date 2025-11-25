<template>
  <div class="pair-import">
    <!-- Bandeau d'informations -->
    <div
      v-if="summary && summary.total_files > 0"
      class="import-info"
    >
      <div class="info-item">
        <span class="info-label">💱 Paires en base :</span>
        <span class="info-value">{{ summary.total_pairs }}</span>
      </div>
      <div class="info-item">
        <span class="info-label">📊 Fichiers CSV :</span>
        <span class="info-value">{{ summary.total_files }}</span>
      </div>
      <div class="info-item">
        <span class="info-label">📈 Lignes de données :</span>
        <span class="info-value">{{ summary.total_lines.toLocaleString() }}</span>
      </div>
      <div
        v-if="summary.date_range_start && summary.date_range_end"
        class="info-item"
      >
        <span class="info-label">📆 Période couverte :</span>
        <span class="info-value">{{ summary.date_range_start }} → {{ summary.date_range_end }}</span>
      </div>
      <div
        v-if="summary.last_import_date"
        class="info-item"
      >
        <span class="info-label">🕒 Dernier import :</span>
        <span class="info-value">{{ summary.last_import_date }}</span>
      </div>
    </div>
    
    <div
      v-else-if="!loadingInfo"
      class="import-info warning"
    >
      <span>⚠️ Aucune donnée de paire importée. Importez vos fichiers CSV pour commencer.</span>
    </div>

    <!-- Liste des fichiers CSV disponibles -->
    <PairFilesList
      ref="filesListRef"
      @files-refreshed="loadSummary"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import PairFilesList from './PairFilesList.vue'

interface PairDataSummary {
  total_pairs: number
  total_files: number
  total_lines: number
  total_size_bytes: number
  date_range_start: string | null
  date_range_end: string | null
  last_import_date: string | null
}

const filesListRef = ref<InstanceType<typeof PairFilesList>>()
const summary = ref<PairDataSummary | null>(null)
const loadingInfo = ref(false)

async function loadSummary() {
  loadingInfo.value = true
  try {
    const data = await invoke<PairDataSummary>('get_pair_data_summary')
    summary.value = data
  } catch (e) {
    // Erreur silencieuse - summary non disponible
  } finally {
    loadingInfo.value = false
  }
}

onMounted(() => {
  loadSummary()
})
</script>

<style scoped>
.pair-import {
  padding: 20px 0;
}

.import-info {
  background: linear-gradient(135deg, #238636 0%, #2ea043 100%);
  border-radius: 10px;
  padding: 15px 20px;
  margin-bottom: 20px;
  display: flex;
  gap: 20px;
  flex-wrap: wrap;
  align-items: center;
  box-shadow: 0 2px 8px rgba(46, 160, 67, 0.2);
}

.import-info.warning {
  background: linear-gradient(135deg, #d29922 0%, #dcaf3a 100%);
  justify-content: center;
  color: #1a1a1a;
  font-weight: 600;
}

.info-item {
  display: flex;
  gap: 8px;
  align-items: center;
}

.info-label {
  color: rgba(255, 255, 255, 0.8);
  font-size: 0.9em;
  font-weight: 500;
}

.info-value {
  color: white;
  font-weight: 700;
  font-size: 1em;
  background: rgba(255, 255, 255, 0.2);
  padding: 4px 12px;
  border-radius: 6px;
}

.import-error-message {
  background: rgba(248, 81, 73, 0.1);
  border: 1px solid #f85149;
  border-radius: 8px;
  padding: 15px;
  margin: 20px 0;
  color: #f85149;
  font-weight: 500;
  animation: fadeIn 0.3s ease;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(-10px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>
