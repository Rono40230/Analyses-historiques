<template>
  <div class="import-summary-wrapper">
    <!-- Success Message -->
    <div v-if="importSuccess" class="success">
      ✅ {{ importSuccess }}
    </div>

    <!-- Error Message -->
    <div v-if="importError" class="error">
      ❌ {{ importError }}
    </div>

    <!-- Import Summary Details -->
    <div v-if="importSummary" class="import-summary">
      <h5>📊 Résumé de l'import</h5>
      <div class="summary-grid">
        <div class="summary-item">
          <span class="summary-label">Fichiers traités :</span>
          <span class="summary-value success-text">
            {{ importSummary.successful }} / {{ importSummary.total_files }}
          </span>
        </div>
        
        <div class="summary-item" v-if="importSummary.failed > 0">
          <span class="summary-label">Échecs :</span>
          <span class="summary-value error-text">{{ importSummary.failed }}</span>
        </div>
        
        <div class="summary-item">
          <span class="summary-label">Paires mises à jour :</span>
          <span class="summary-value">
            {{ importSummary.pairs_updated.join(', ') || 'Aucune' }}
          </span>
        </div>
        
        <div class="summary-item">
          <span class="summary-label">Timeframes :</span>
          <span class="summary-value">
            {{ importSummary.timeframes.join(', ') || 'Aucun' }}
          </span>
        </div>
      </div>

      <!-- Error Details -->
      <div v-if="importSummary.errors.length > 0" class="error-details">
        <h6>Erreurs détaillées :</h6>
        <ul>
          <li v-for="(err, idx) in importSummary.errors" :key="idx">{{ err }}</li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface ImportSummary {
  total_files: number
  successful: number
  failed: number
  pairs_updated: string[]
  timeframes: string[]
  errors: string[]
}

defineProps<{
  importSuccess?: string
  importError?: string
  importSummary: ImportSummary | null
}>()
</script>

<style scoped>
.import-summary-wrapper {
  margin-top: 20px;
}

.success {
  background: linear-gradient(135deg, #2ea043 0%, #238636 100%);
  color: white;
  padding: 15px;
  border-radius: 8px;
  margin-bottom: 15px;
  font-weight: 600;
  box-shadow: 0 2px 8px rgba(46, 160, 67, 0.3);
}

.error {
  background: linear-gradient(135deg, #f85149 0%, #da3633 100%);
  color: white;
  padding: 15px;
  border-radius: 8px;
  margin-bottom: 15px;
  font-weight: 600;
  box-shadow: 0 2px 8px rgba(248, 81, 73, 0.3);
}

.import-summary {
  background: rgba(56, 139, 253, 0.1);
  border: 1px solid rgba(56, 139, 253, 0.3);
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 20px;
}

.import-summary h5 {
  color: #58a6ff;
  margin-bottom: 15px;
  font-size: 1.1em;
}

.import-summary h6 {
  color: #f85149;
  font-size: 0.95em;
  margin: 15px 0 8px 0;
}

.summary-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 12px;
  margin-bottom: 15px;
}

.summary-item {
  background: rgba(0, 0, 0, 0.2);
  padding: 12px;
  border-radius: 6px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.summary-label {
  color: #8b949e;
  font-weight: 500;
  font-size: 0.9em;
}

.summary-value {
  color: #e6edf3;
  font-weight: 600;
}

.success-text {
  color: #7ee787;
}

.error-text {
  color: #f85149;
}

.error-details {
  background: rgba(248, 81, 73, 0.1);
  border: 1px solid rgba(248, 81, 73, 0.3);
  border-radius: 8px;
  padding: 15px;
  margin-top: 15px;
}

.error-details ul {
  list-style: none;
  padding: 0;
  margin: 10px 0 0 0;
}

.error-details li {
  color: #f85149;
  padding: 6px 0;
  border-bottom: 1px solid rgba(248, 81, 73, 0.2);
  font-size: 0.9em;
}

.error-details li:last-child {
  border-bottom: none;
}
</style>
