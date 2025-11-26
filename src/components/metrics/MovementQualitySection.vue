<template>
  <div class="movement-quality-section">
    <h4>💫 Qualité du Mouvement</h4>
    
    <!-- Pas de données -->
    <div
      v-if="!getMovementQualityKey(analysis)"
      style="color: #999;"
    >
      ⚠️ Pas de données de qualité
    </div>
    
    <!-- Données chargées -->
    <div
      v-else-if="movementQualities[getMovementQualityKey(analysis)]"
      style="display: grid; grid-template-columns: repeat(auto-fit, minmax(140px, 1fr)); gap: 10px; margin-top: 15px;"
    >
      <!-- Score Qualité -->
      <MetricTooltip title="Score Qualité">
        <div style="padding: 12px; background: rgba(255,255,255,0.05); border-radius: 6px;">
          <div style="font-size: 11px; color: #999; margin-bottom: 6px; text-transform: uppercase;">
            Score Qualité
          </div>
          <div style="font-size: 13px; color: #4ecdc4; font-weight: bold;">
            {{ (movementQualities[getMovementQualityKey(analysis)]?.score || 0).toFixed(1) }}/100
          </div>
        </div>
        <template #definition>
          Notation globale 0-100 de la qualité du setup combinant tous les facteurs : volatilité, signal purity, mouvement directionnel.
        </template>
        <template #usage>
          Score &gt;80 = Excellent | 60-80 = Bon | &lt;40 = Faible.
        </template>
        <template #scoring>
          Basé sur Trend Score, Smoothness et Consistance des bougies.
        </template>
      </MetricTooltip>
      
      <!-- Label Qualité -->
      <MetricTooltip title="Label Qualité">
        <div style="padding: 12px; background: rgba(255,255,255,0.05); border-radius: 6px;">
          <div style="font-size: 11px; color: #999; margin-bottom: 6px; text-transform: uppercase;">
            Label
          </div>
          <div style="font-size: 13px; color: #4ecdc4; font-weight: bold;">
            {{ movementQualities[getMovementQualityKey(analysis)]?.label || 'N/A' }}
          </div>
        </div>
        <template #definition>
          Appréciation qualitative du mouvement.
        </template>
      </MetricTooltip>
    </div>

    <!-- Chargement en cours -->
    <div
      v-else
      class="quality-loading"
    >
      ⏳ Analyse du mouvement en cours...
    </div>
  </div>
</template>

<script setup lang="ts">
import MetricTooltip from '../MetricTooltip.vue'

const props = defineProps<{
  analysis: any
  analysisData: any
  movementQualities: Record<string, any>
}>()

/**
 * Helper: construit la clé pour accéder une qualité de mouvement
 */
const getMovementQualityKey = (analysis: any): string => {
  if (!analysis?.slice) return ''
  return `${analysis.slice.hour}-${analysis.slice.quarter}`
}
</script>

<style scoped>
/* Movement Quality Section */
.movement-quality-section {
  padding: 20px;
  background: linear-gradient(135deg, rgba(45, 90, 123, 0.15) 0%, rgba(78, 205, 196, 0.1) 100%);
  border: 1px solid #2d5a7b;
  border-radius: 8px;
}

.movement-quality-section h4 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: #e0e7ff;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.quality-loading {
  text-align: center;
  padding: 12px;
  color: #64748b;
  font-size: 12px;
  font-style: italic;
}
</style>
