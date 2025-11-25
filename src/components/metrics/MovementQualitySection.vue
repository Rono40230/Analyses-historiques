<template>
  <div class="movement-quality-section">
    <h4>💫 Qualité du Mouvement</h4>
    
    <!-- Pas d'événements -->
    <div
      v-if="analysis.slice.stats.events.length === 0"
      style="color: #999;"
    >
      ⚠️ Pas d'événement dans ce slice
    </div>
    
    <!-- Clé vide -->
    <div
      v-else-if="!getMovementQualityKey(analysis)"
      style="color: #999;"
    >
      ⚠️ Clé vide générée
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
            {{ (movementQualities[getMovementQualityKey(analysis)]?.quality_score || 0).toFixed(1) }}/10
          </div>
        </div>
        <template #definition>
          Notation globale 0-10 de la qualité du setup combinant tous les facteurs : volatilité, signal purity, mouvement directionnel.
        </template>
        <template #usage>
          Score &gt;7 = Excellent (trader) | 5-7 = Acceptable | &lt;5 = Mauvais (skip). Basé sur pondération : Volatilité 40%, Signal 35%, Direction 25%.
        </template>
        <template #scoring>
          Formula: (ATR_score × 0.4 + Body_Range_score × 0.35 + Direction_score × 0.25) / 10. Seuil global qualité.
        </template>
      </MetricTooltip>
      
      <!-- Mouvement Directionnel -->
      <MetricTooltip title="Mouvement Directionnel">
        <div style="padding: 12px; background: rgba(255,255,255,0.05); border-radius: 6px;">
          <div style="font-size: 11px; color: #999; margin-bottom: 6px; text-transform: uppercase;">
            Mouvement Directionnel
          </div>
          <div style="font-size: 13px; color: #4ecdc4; font-weight: bold;">
            {{ ((movementQualities[getMovementQualityKey(analysis)]?.directional_move_rate || 0) * 100).toFixed(0) }}%
          </div>
        </div>
        <template #definition>
          Pourcentage du range total qui s'est déplacé dans une direction cohérente sans retracer significativement.
        </template>
        <template #usage>
          Score &gt;70% = Très directionnel (bon momentum) | 50-70% = Modérément directionnel | &lt;50% = Chaotique/bidirectionnel.
        </template>
        <template #scoring>
          Formula: (Net_directional_pips / Total_range) × 100. Élevé = tendance claire, faible = oscillation indécise.
        </template>
      </MetricTooltip>
      
      <!-- Whipsaw Rate -->
      <MetricTooltip title="Whipsaw Rate">
        <div style="padding: 12px; background: rgba(255,255,255,0.05); border-radius: 6px;">
          <div style="font-size: 11px; color: #999; margin-bottom: 6px; text-transform: uppercase;">
            Whipsaw Rate
          </div>
          <div style="font-size: 13px; color: #4ecdc4; font-weight: bold;">
            {{ ((movementQualities[getMovementQualityKey(analysis)]?.whipsaw_rate || 0) * 100).toFixed(0) }}%
          </div>
        </div>
        <template #definition>
          Pourcentage de fausses sorties : fois où le prix dépasse SL temporairement avant de revenir vers TP (dangereux au scalp).
        </template>
        <template #usage>
          Score &lt;10% = Excellent (peu de faux signaux) | 10-20% = Acceptable | &gt;20% = Danger (trop de whipsaws, avoid).
        </template>
        <template #scoring>
          Formula: (Whipsaw_events / Total_trades) × 100. Barrière psychologique et cash-drag majeure. À minimiser absolument.
        </template>
      </MetricTooltip>
      
      <!-- Taux Succès -->
      <MetricTooltip title="Taux Succès">
        <div style="padding: 12px; background: rgba(255,255,255,0.05); border-radius: 6px;">
          <div style="font-size: 11px; color: #999; margin-bottom: 6px; text-transform: uppercase;">
            Taux Succès
          </div>
          <div style="font-size: 13px; color: #4ecdc4; font-weight: bold;">
            {{ ((movementQualities[getMovementQualityKey(analysis)]?.success_rate || 0) * 100).toFixed(0) }}%
          </div>
        </div>
        <template #definition>
          Pourcentage d'événements dans ce créneau qui ont atteint leur objectif TP avant d'être arrêtés au SL (win rate brut).
        </template>
        <template #usage>
          Score &gt;60% = Excellent (trades qui marche) | 50-60% = Bon (profitable avec R/R) | &lt;50% = Mauvais (éviter ce créneau).
        </template>
        <template #scoring>
          Formula: (Winning_events / Total_events) × 100. Directement utilisé pour profitabilité espérance = WR × TP - (1-WR) × SL.
        </template>
      </MetricTooltip>
      
      <!-- Mouvement Moyen -->
      <MetricTooltip title="Mouvement Moyen">
        <div style="padding: 12px; background: rgba(255,255,255,0.05); border-radius: 6px;">
          <div style="font-size: 11px; color: #999; margin-bottom: 6px; text-transform: uppercase;">
            Mouvement Moyen
          </div>
          <div style="font-size: 13px; color: #4ecdc4; font-weight: bold;">
            {{ (movementQualities[getMovementQualityKey(analysis)]?.avg_pips_moved || 0).toFixed(1) }} <span style="color: #888; font-size: 11px;">pips</span>
          </div>
        </div>
        <template #definition>
          Distance moyenne en pips que le prix parcourt par événement dans ce créneau historiquement.
        </template>
        <template #usage>
          Score &gt;15 pips = Excellent (suffisant pour scalp) | 10-15 pips = Bon | &lt;10 pips = Faible mouvement (skip).
        </template>
        <template #scoring>
          Formula: Sum(|move_pips|) / Number_events. Doit être &gt; SL pour que TP soit atteignable (SL +TP × R/R) = mouvement attendu.
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
  if (!props.analysisData || analysis.slice.stats.events.length === 0) return ''
  const symbol = props.analysisData.symbol || 'UNKNOWN'
  const eventName = analysis.slice.stats.events[0].event_name
  return `${symbol}_${eventName}`
}
</script>

<style scoped>
/* Movement Quality Section */
.movement-quality-section {
  background: linear-gradient(135deg, rgba(29, 78, 216, 0.05) 0%, rgba(99, 102, 241, 0.05) 100%);
  border-left: 3px solid #6366f1;
  padding: 14px;
  border-radius: 6px;
  margin-top: 12px;
}

.movement-quality-section h4 {
  color: #e0e7ff;
  font-size: 13px;
  font-weight: 600;
  margin: 0 0 10px 0;
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
