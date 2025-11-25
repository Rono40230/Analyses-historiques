<template>
  <div class="bidi-parameters-section">
    <h4>⚙️ PARAMÈTRES BIDI OPTIMISÉS</h4>
    <div class="metrics-grid">
      <MetricTooltip
        title="Meilleur Moment"
        direction="top"
      >
        <div class="metric-card">
          <div class="metric-label">
            Meilleur Moment
          </div>
          <div class="metric-value">
            {{ getBestTimeDisplay() }}
          </div>
        </div>
        <template #definition>
          L'heure exacte d'entrée optimale pour le straddle basée sur l'analyse historique des créneau horaires.
        </template>
        <template #usage>
          Entrée au-delà de 14:00 avec ≥ 3 créneau optimaux et un taux de succès ≥ 55%.
        </template>
        <template #scoring>
          Sélectionné parmi les 3 meilleurs créneau horaires du jour avec le plus haut taux de succès ajusté.
        </template>
      </MetricTooltip>
      <MetricTooltip
        title="Taux de Succès"
        direction="top"
      >
        <div class="metric-card">
          <div class="metric-label">
            Taux de Succès
          </div>
          <div class="metric-value">
            {{ (entryWindowAnalysis.optimal_win_rate * 100).toFixed(0) }}% <span class="unit">événement</span>
          </div>
        </div>
        <template #definition>
          Pourcentage de fois où le créneau horaire a produit un mouvement straddle gagnant (atteint TP avant SL).
        </template>
        <template #usage>
          Critère crucial : minimum 55% pour un biais positif. ≥65% = excellent signal. &lt;50% = dangereux.
        </template>
        <template #scoring>
          Calculé sur l'historique complet du créneau avec ajustement volatilité/range/body-range.
        </template>
      </MetricTooltip>
      <MetricTooltip
        title="Stop Loss"
        direction="top"
      >
        <div class="metric-card">
          <div class="metric-label">
            Stop Loss
          </div>
          <div class="metric-value">
            {{ analysis.tradingPlan.slPips }} <span class="unit">pips</span>
          </div>
        </div>
        <template #definition>
          Distance en pips entre l'entrée et le niveau de stop loss (limite de perte).
        </template>
        <template #usage>
          Calculé dynamiquement : SL = (Score/100 × Range_actuelle) / 1.5. Exemple : score 60 = ±20 pips de range.
        </template>
        <template #scoring>
          Formula: SL_pips = (Score/100) × (ATR × 2.5). Augmente avec la volatilité, diminue si score faible (&lt;50).
        </template>
      </MetricTooltip>
      <MetricTooltip
        title="Win Rate"
        direction="top"
      >
        <div class="metric-card">
          <div class="metric-label">
            Win Rate
          </div>
          <div class="metric-value">
            {{ analysis.tradingPlan.winProbability }}% <span class="unit">histo</span>
          </div>
        </div>
        <template #definition>
          Pourcentage de trades théoriques gagnants selon l'historique des mouvements (atteint TP avant SL).
        </template>
        <template #usage>
          Basé sur les histogrammes de distribution des mouvements du créneau. &gt;55% = profitable à long terme. &lt;50% = stop trading.
        </template>
        <template #scoring>
          Calculé à partir de : success_rate du créneau + volatility_score + body_range_score. Ajustement variance inclus.
        </template>
      </MetricTooltip>
      <MetricTooltip
        title="Avg Gain"
        direction="top"
      >
        <div class="metric-card">
          <div class="metric-label">
            Avg Gain
          </div>
          <div class="metric-value">
            {{ analysis.tradingPlan.avgGainR.toFixed(1) }}R <span class="unit">moyen</span>
          </div>
        </div>
        <template #definition>
          Espérance mathématique moyenne en "R" (risque unitaire). Exemple : 0.5R = 50% du risque en gain moyen.
        </template>
        <template #usage>
          Critère clé : Avg Gain = (Win% × Win_avg) - (Loss% × Loss_avg) × Risk. &gt;0.3R = très bon. &lt;0 = à éviter.
        </template>
        <template #scoring>
          Formula: AvgGain = (win_rate × avg_win_pips - (1-win_rate) × avg_loss_pips) / SL_pips. Mesure la profitabilité nette.
        </template>
      </MetricTooltip>
      <MetricTooltip
        title="Trailing Stop"
        direction="top"
      >
        <div class="metric-card">
          <div class="metric-label">
            Trailing Stop
          </div>
          <div class="metric-value">
            {{ analysis.tradingPlan.trailingStopCoefficient.toFixed(2) }}x <span class="unit">ATR</span>
          </div>
        </div>
        <template #definition>
          Multiplicateur ATR pour recalculer dynamiquement le stop loss en hausse (protection des gains). Fixe le SL à [prix bas × (1.5 + volatilité_ratio)].
        </template>
        <template #usage>
          Trailing = 1.5x ATR + (ATR_current/ATR_avg - 1) × 0.5. Exemple : ATR_actuel 0.002 = 1.8x. Permet de sécuriser les gains sans bloquer.
        </template>
        <template #scoring>
          Formula: Coefficient = 1.5 + (ATR_current/ATR_moyenne - 1) × 0.5. Plage 1.5-2.5x. Volatilité haute = coefficient plus bas (plus serré).
        </template>
      </MetricTooltip>
      <MetricTooltip
        title="Trade Expiration"
        direction="top"
      >
        <div class="metric-card">
          <div class="metric-label">
            Expiration
          </div>
          <div class="metric-value">
            {{ analysis.tradingPlan.tradeExpiration || '—' }} <span class="unit">min</span>
          </div>
        </div>
        <template #definition>
          Limite de temps maximale avant fermeture automatique du trade (dans le robot Bidi). Basée sur la volatilité et remplace les 300min fixes.
        </template>
        <template #usage>
          Entrée à 14h30 + expiration 180min = fermer avant 16h30. Si TP non atteint à l'expiration, fermer à market. Évite les whipsaws post-peak.
        </template>
        <template #scoring>
          Formula: max(peak_duration, half_life × 2). Ajustée selon ATR. Volatilité haute = expiration courte (120-150min). Volatilité faible = expiration longue (240-270min). Max 300min.
        </template>
      </MetricTooltip>
    </div>
  </div>
</template>

<script setup lang="ts">
import MetricTooltip from '../MetricTooltip.vue'
import { useMetricsFormatting } from '../../composables/useMetricsFormatting'

const props = defineProps<{
  sliceAnalyses: any[]
  entryWindowAnalysis: any
  analysis: any
}>()

const { calculateExactTime } = useMetricsFormatting()

const getBestTimeDisplay = () => {
  if (props.sliceAnalyses && props.sliceAnalyses.length > 0) {
    const bestSlice = props.sliceAnalyses[0]
    return calculateExactTime(bestSlice.slice.startTime, props.entryWindowAnalysis.optimal_offset)
  }
  return props.entryWindowAnalysis.optimal_offset + ' min'
}
</script>

<style scoped>
.bidi-parameters-section {
  margin-top: 20px;
  padding: 20px;
  background: #1a1a2e;
  border: 1px solid #16213e;
  border-radius: 8px;
}

.bidi-parameters-section h4 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: #e0e7ff;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.metrics-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  gap: 10px;
  margin-top: 15px;
}

.metric-card {
  padding: 12px;
  background: rgba(255,255,255,0.05);
  border-radius: 6px;
}

.metric-label {
  font-size: 11px;
  color: #999;
  margin-bottom: 6px;
  text-transform: uppercase;
}

.metric-value {
  font-size: 13px;
  color: #4ecdc4;
  font-weight: bold;
}

.unit {
  color: #888;
  font-size: 11px;
}
</style>
