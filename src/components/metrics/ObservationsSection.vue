<template>
  <div class="observations-section">
    <h4>📋 OBSERVATIONS & CONSEILS</h4>
    <div class="observations-grid">
      <!-- Analyse Range -->
      <div class="observation-card">
        <div class="obs-label">
          📊 Range
        </div>
        <div class="obs-value">
          {{ (analysis.slice.stats.range_mean * 10000).toFixed(0) }} pips
        </div>
        <div class="obs-status">
          <span
            v-if="analysis.slice.stats.range_mean > 0.0025"
            style="color: #4ecdc4;"
          >✅ Excellent</span>
          <span
            v-else-if="analysis.slice.stats.range_mean > 0.0015"
            style="color: #ffd700;"
          >⚠️ Bon</span>
          <span
            v-else
            style="color: #ff6b6b;"
          >❌ Faible</span>
        </div>
        <div class="obs-hint">
          💡 Plus élevé = meilleure opportunité straddle
        </div>
      </div>

      <!-- Analyse ATR -->
      <div class="observation-card">
        <div class="obs-label">
          ⚡ ATR
        </div>
        <div class="obs-value">
          {{ (analysis.slice.stats.atr_mean * 10000).toFixed(0) }} pips
        </div>
        <div class="obs-status">
          <span
            v-if="analysis.slice.stats.atr_mean > 0.0020"
            style="color: #4ecdc4;"
          >✅ Excellent</span>
          <span
            v-else-if="analysis.slice.stats.atr_mean > 0.0010"
            style="color: #ffd700;"
          >⚠️ Bon</span>
          <span
            v-else
            style="color: #ff6b6b;"
          >❌ Faible</span>
        </div>
        <div class="obs-hint">
          💡 Volatilité confirmée = SL/TP plus larges
        </div>
      </div>

      <!-- Analyse Body Range -->
      <div class="observation-card">
        <div class="obs-label">
          📈 Body Range
        </div>
        <div class="obs-value">
          {{ analysis.slice.stats.body_range_mean.toFixed(1) }}%
        </div>
        <div class="obs-status">
          <span
            v-if="analysis.slice.stats.body_range_mean > 45"
            style="color: #4ecdc4;"
          >✅ Très Pur</span>
          <span
            v-else-if="analysis.slice.stats.body_range_mean > 25"
            style="color: #ffd700;"
          >⚠️ Acceptable</span>
          <span
            v-else
            style="color: #ff6b6b;"
          >❌ Très Bruité</span>
        </div>
        <div class="obs-hint">
          💡 Élevé = signal pur, peu de bruit
        </div>
      </div>

      <!-- Analyse Qualité Mouvement (Phase 1.2) -->
      <div class="observation-card">
        <div class="obs-label">
          💫 Qualité
        </div>
        <div class="obs-value">
          <template v-if="movementQualities[getMovementQualityKey(analysis, analysisData)]?.quality_score">
            {{ (movementQualities[getMovementQualityKey(analysis, analysisData)]?.quality_score || 0).toFixed(1) }}/10
          </template>
          <template v-else>
            —
          </template>
        </div>
        <div class="obs-status">
          <template v-if="movementQualities[getMovementQualityKey(analysis, analysisData)]?.quality_score">
            <span
              v-if="(movementQualities[getMovementQualityKey(analysis, analysisData)]?.quality_score || 0) >= 8"
              style="color: #4ecdc4;"
            >✅ Excellent</span>
            <span
              v-else-if="(movementQualities[getMovementQualityKey(analysis, analysisData)]?.quality_score || 0) >= 6"
              style="color: #ffd700;"
            >⚠️ Bon</span>
            <span
              v-else
              style="color: #ff6b6b;"
            >❌ Faible</span>
          </template>
          <span
            v-else
            style="color: #999;"
          >Calcul...</span>
        </div>
        <div class="obs-hint">
          💡 Basé sur mouvements directionnel
        </div>
      </div>

      <!-- Analyse Durée de Volatilité (Phase 1.1) -->
      <div class="observation-card">
        <div class="obs-label">
          ⏱️ Durée Vol.
        </div>
        <div class="obs-value">
          <template v-if="volatilityDuration?.peak_duration_minutes">
            {{ volatilityDuration.peak_duration_minutes }}min
          </template>
          <template v-else>
            —
          </template>
        </div>
        <div class="obs-status">
          <template v-if="volatilityDuration?.confidence_score">
            <span
              v-if="volatilityDuration.confidence_score >= 75"
              style="color: #4ecdc4;"
            >✅ Haute conf.</span>
            <span
              v-else-if="volatilityDuration.confidence_score >= 50"
              style="color: #ffd700;"
            >⚠️ Moyenne</span>
            <span
              v-else
              style="color: #ff6b6b;"
            >❌ Basse</span>
          </template>
          <span
            v-else
            style="color: #999;"
          >Calcul...</span>
        </div>
        <div class="obs-hint">
          💡 Pic {{ volatilityDuration?.peak_duration_minutes || '?' }}min
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useMetricsFormatting } from '../../composables/useMetricsFormatting'

defineProps<{
  analysis: any
  analysisData: any
  movementQualities: Record<string, any>
  volatilityDuration: any
}>()

const { getMovementQualityKey } = useMetricsFormatting()
</script>

<style scoped>
.observations-section {
  margin-top: 20px;
  padding: 20px;
  background: #1a1a2e;
  border: 1px solid #16213e;
  border-radius: 8px;
}

.observations-section h4 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: #e0e7ff;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.observations-grid {
  margin-top: 15px;
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  gap: 12px;
}

.observation-card {
  padding: 12px;
  background: rgba(255,255,255,0.05);
  border-radius: 6px;
  display: flex;
  flex-direction: column;
}

.obs-label {
  font-size: 12px;
  color: #999;
  margin-bottom: 6px;
  font-weight: bold;
}

.obs-value {
  font-size: 13px;
  color: #fff;
  font-weight: bold;
}

.obs-status {
  font-size: 11px;
  margin: 6px 0;
}

.obs-hint {
  font-size: 10px;
  color: #888;
  margin-top: auto;
}
</style>
