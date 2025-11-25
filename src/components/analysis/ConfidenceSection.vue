<template>
  <div class="confidence-section">
    <h3>🎲 Score de Confiance</h3>
    <div
      :class="['confidence-bar', getConfidenceBarClass(confidenceScore)]"
      :style="{ width: `${confidenceScore}%` }"
    />
    <div class="confidence-text">
      {{ confidenceScore }}% - 
      <span v-if="confidenceScore >= 80">Conditions optimales pour trader</span>
      <span v-else-if="confidenceScore >= 65">Conditions favorables</span>
      <span v-else-if="confidenceScore >= 50">Conditions acceptables</span>
      <span v-else-if="confidenceScore >= 35">Métrique insuffisante, déconseillé</span>
      <span v-else>Métrique insuffisante, déconseillé</span>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  confidenceScore: number
}>()

function getConfidenceBarClass(score: number): string {
  if (score >= 80) return 'confidence-excellent'
  if (score >= 65) return 'confidence-good'
  if (score >= 50) return 'confidence-acceptable'
  if (score >= 35) return 'confidence-risky'
  return 'confidence-poor'
}
</script>

<style scoped>
.confidence-section { background: #1a202c; padding: 20px; border-radius: 8px; margin-bottom: 20px; }
.confidence-section h3 { margin: 0 0 15px 0; }
.confidence-bar { height: 8px; background: #667eea; border-radius: 4px; margin-bottom: 8px; transition: background 0.3s ease; }
.confidence-bar.confidence-excellent { background: linear-gradient(135deg, #10b981 0%, #059669 100%) !important; box-shadow: 0 0 8px rgba(16, 185, 129, 0.5) !important; }
.confidence-bar.confidence-good { background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%) !important; box-shadow: 0 0 8px rgba(59, 130, 246, 0.5) !important; }
.confidence-bar.confidence-acceptable { background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%) !important; box-shadow: 0 0 8px rgba(245, 158, 11, 0.5) !important; }
.confidence-bar.confidence-risky { background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%) !important; box-shadow: 0 0 8px rgba(239, 68, 68, 0.5) !important; }
.confidence-bar.confidence-poor { background: linear-gradient(135deg, #6b7280 0%, #4b5563 100%) !important; box-shadow: 0 0 8px rgba(107, 114, 128, 0.5) !important; }
.confidence-text { color: #cbd5e0; font-size: 0.9em; }
</style>
