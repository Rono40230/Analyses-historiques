<template>
  <div
    class="slice-card"
    :class="getRankClass(analysis.rank)"
  >
    <!-- Rang + Heure + Recommandation -->
    <div
      class="slice-header"
      style="display: flex; justify-content: space-between; align-items: center; gap: 20px;"
    >
      <div style="display: flex; gap: 12px; align-items: center;">
        <div
          class="rank-badge"
          :class="`rank-${analysis.rank}`"
        >
          <span class="rank-medal">⭐</span>
        </div>
        <div class="slice-time">
          <div class="time">
            {{ analysis.slice.startTime }}
          </div>
        </div>
      </div>

      <!-- Score & Whipsaw Blocs -->
      <div style="display: flex; gap: 12px; align-items: center;">
        <!-- Score Bloc -->
        <MetricTooltip
          title="Score"
          direction="bottom"
          style="max-width: 250px;"
        >
          <div style="flex: 0 0 auto; padding: 12px 16px; background: rgba(45, 90, 123, 0.15); border: 1px solid #2d5a7b; border-radius: 6px; font-size: 12px; min-width: 140px;">
            <div style="color: #64a5d8; margin-bottom: 6px; font-weight: bold;">
              ⭐ SCORE
            </div>
            <div style="display: flex; align-items: center; gap: 8px;">
              <div style="font-size: 18px; font-weight: bold;" :style="{ color: getScoreColor(adjustedScore) }">
                {{ adjustedScore.toFixed(0) }}/100
              </div>
            </div>
          </div>
          <template #definition>
            <div class="tooltip-section">
              <div class="tooltip-section-title">📖 Définition</div>
              <div class="tooltip-section-text">Score de viabilité du Straddle ajusté par la fréquence whipsaw. Reflète la qualité réelle du setup en tenant compte des retournements.</div>
            </div>
          </template>
          <template #interpretation>
            <div class="tooltip-section">
              <div class="tooltip-section-title">📊 Interprétation</div>
              <div class="interpretation-item"><strong>🟢 Excellent:</strong> ≥75/100 → Setup optimal</div>
              <div class="interpretation-item"><strong>🔵 Bon:</strong> 60-74/100 → Setup viable</div>
              <div class="interpretation-item"><strong>🟡 Acceptable:</strong> 45-59/100 → À considérer</div>
              <div class="interpretation-item"><strong>🔴 Faible:</strong> &lt;45/100 → À éviter</div>
            </div>
          </template>
        </MetricTooltip>

        <!-- Fréquence Whipsaw -->
        <MetricTooltip
          title="Fréquence Whipsaw"
          direction="bottom"
        >
          <div style="flex: 0 0 auto; padding: 12px 16px; background: rgba(45, 90, 123, 0.15); border: 1px solid #2d5a7b; border-radius: 6px; font-size: 12px; min-width: 160px;">
            <div style="color: #64a5d8; margin-bottom: 6px; font-weight: bold;">
              📊 WHIPSAW
            </div>
            <div style="display: flex; align-items: center; gap: 8px;">
              <div style="font-size: 18px; font-weight: bold;" :style="{ color: getWhipsawColor(props.whipsawAnalysis?.whipsaw_frequency_percentage || 0) }">
                {{ props.whipsawAnalysis?.whipsaw_frequency_percentage?.toFixed(1) ?? 'N/A' }}%
              </div>
            </div>
          </div>
          <template #definition>
            <div class="tooltip-section">
              <div class="tooltip-section-title">📖 Définition</div>
              <div class="tooltip-section-text">Fréquence des retournements rapides (whipsaws) dans le quarter. Mesure la volatilité et l'instabilité du mouvement.</div>
            </div>
          </template>
          <template #interpretation>
            <div class="tooltip-section">
              <div class="tooltip-section-title">📊 Interprétation</div>
              <div class="interpretation-item"><strong>🟢 Très Bas:</strong> &lt;5% → Excellent, très stable</div>
              <div class="interpretation-item"><strong>🔵 Bas:</strong> 5-10% → Bon, acceptable</div>
              <div class="interpretation-item"><strong>🟡 Modéré:</strong> 10-20% → Attention requise</div>
              <div class="interpretation-item"><strong>🟠 Élevé:</strong> 20-30% → Risqué</div>
              <div class="interpretation-item"><strong>🔴 Très Élevé:</strong> &gt;30% → À éviter</div>
            </div>
          </template>
        </MetricTooltip>
      </div>

      <!-- Recommandation inline -->
      <div style="flex: 1; padding: 12px 16px; background: rgba(78, 205, 196, 0.1); border: 1px solid rgba(78, 205, 196, 0.3); border-radius: 6px; font-size: 12px;">
        <div style="color: #4ecdc4; margin-bottom: 6px; font-weight: bold;">
          🎯 RECOMMANDATION
        </div>
        <div style="color: #e0e0e0; line-height: 1.5;">
          {{ recommendation.emoji }} <strong>{{ recommendation.decision }}</strong> — {{ recommendation.advice }}
        </div>
      </div>
    </div>

    <!-- Slot pour le contenu (MetricsGrid, etc.) -->
    <slot></slot>
  </div>
</template>

<script setup lang="ts">
import { useMetricsCalculations } from '../../composables/useMetricsCalculations'
import MetricTooltip from '../MetricTooltip.vue'
import { computed } from 'vue'
import {
  getScoreColor,
  getWhipsawColor,
  calculateAdjustedScore,
  getMovementQualityKey,
  generateRecommendation
} from './BestSliceCard.helpers'

const props = defineProps<{
  analysis: any
  volatilityDuration: any
  movementQualities?: Record<string, any>
  whipsawAnalysis?: any
}>()

const { getRankClass } = useMetricsCalculations()

const adjustedScore = computed(() => {
  const brut = props.analysis?.slice?.straddleScore || 0
  return calculateAdjustedScore(brut, props.whipsawAnalysis?.whipsaw_frequency_percentage)
})

const recommendation = computed(() => {
  const adjustedWinRate = props.analysis?.slice?.win_rate_adjusted || 0
  const whipsawFreq = props.whipsawAnalysis?.whipsaw_frequency_percentage || 0
  const confidence = props.volatilityDuration?.confidence_score || 0
  
  return generateRecommendation(adjustedScore.value, whipsawFreq, adjustedWinRate, confidence)
})
</script>

<style scoped>
.slice-card {
  background: rgba(30, 30, 45, 0.6);
  border: 1px solid #2d3748;
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 20px;
}

.slice-card.rank-1 {
  border: 2px solid #ffd700;
  background: rgba(255, 215, 0, 0.05);
  box-shadow: 0 0 20px rgba(255, 215, 0, 0.1);
}

.rank-badge {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  background: #2d3748;
  border: 2px solid #4a5568;
}

.rank-badge.rank-1 {
  background: linear-gradient(135deg, #ffd700 0%, #ffa500 100%);
  border-color: #ffd700;
  color: #000;
  box-shadow: 0 0 10px rgba(255, 215, 0, 0.3);
}

.slice-time .time {
  font-size: 24px;
  font-weight: 800;
  color: #fff;
  line-height: 1;
  margin-bottom: 4px;
}

.slice-time .score {
  font-size: 13px;
  font-weight: 600;
  padding: 2px 6px;
  border-radius: 4px;
  display: inline-block;
}

.score-excellent {
  background: rgba(34, 197, 94, 0.2);
  color: #86efac;
}

.score-good {
  background: rgba(59, 130, 246, 0.2);
  color: #93c5fd;
}

.score-acceptable {
  background: rgba(234, 179, 8, 0.2);
  color: #facc15;
}

.score-poor {
  background: rgba(239, 68, 68, 0.2);
  color: #fca5a5;
}

/* Quality Badges */
.quality-badge {
  padding: 4px 8px;
  border-radius: 4px;
  font-weight: 600;
  font-size: 11px;
  white-space: nowrap;
}

.quality-badge.excellent {
  background: rgba(16, 185, 129, 0.2);
  color: #10b981;
}

.quality-badge.good {
  background: rgba(59, 130, 246, 0.2);
  color: #3b82f6;
}

.quality-badge.acceptable {
  background: rgba(234, 179, 8, 0.2);
  color: #eab308;
}

.quality-badge.poor {
  background: rgba(239, 68, 68, 0.2);
  color: #ef4444;
}
</style>
