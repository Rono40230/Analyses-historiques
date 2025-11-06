<template>
  <div class="pair-import">
    <!-- Liste des fichiers CSV disponibles -->
    <PairFilesList />

    <!-- CSV Cleaner Section -->
    <CleanerSection 
      @import-started="handleImportStarted"
      @import-completed="handleImportCompleted"
      @error="handleError"
    />

    <!-- Import Summary -->
    <ImportSummary 
      :import-success="importSuccess"
      :import-error="importError"
      :import-summary="importSummary"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import PairFilesList from './PairFilesList.vue'
import CleanerSection from './CleanerSection.vue'
import ImportSummary from './ImportSummary.vue'

interface ImportSummary {
  total_files: number
  successful: number
  failed: number
  pairs_updated: string[]
  timeframes: string[]
  errors: string[]
}

const importSuccess = ref('')
const importError = ref('')
const importSummary = ref<ImportSummary | null>(null)

function handleImportStarted(_index: number) {
  importSuccess.value = ''
  importError.value = ''
}

function handleImportCompleted(success: boolean, _index: number) {
  if (success) {
    importSuccess.value = '✅ Fichier importé avec succès !'
  } else {
    importError.value = '❌ Échec de l\'import'
  }
}

function handleError(message: string) {
  importError.value = message
}
</script>

<style scoped>
.pair-import {
  padding: 20px 0;
}
</style>
