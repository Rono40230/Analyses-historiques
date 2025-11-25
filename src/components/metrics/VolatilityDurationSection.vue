<template>
  <div class="volatility-duration-section">
    <h4>⏱️ DURÉE DE VOLATILITÉ</h4>
    <div class="metrics-grid">
      <!-- Peak Duration -->
      <MetricTooltip
        title="Durée du Pic"
        direction="top"
      >
        <div class="metric-card">
          <div class="metric-label">
            Durée Pic
          </div>
          <div class="metric-value">
            {{ volatilityDuration?.peak_duration_minutes || '—' }} <span class="unit">min</span>
          </div>
        </div>
        <template #definition>
          Nombre de minutes où la volatilité reste supérieure à 80% du pic observé pendant le créneau.
        </template>
        <template #usage>
          Indique combien de temps le mouvement principal persiste avant de perdre son énergie. Exemple : NFP = 90-150min, données faibles = 150-270min.
        </template>
        <template #scoring>
          Calculé à partir de l'ATR, Range et Body Range empiriques. Volatilité très élevée (ATR>50pts) = pic court. Volatilité faible (ATR<25pts) = pic long.
        </template>
      </MetricTooltip>

      <!-- Volatility Half-Life -->
      <MetricTooltip
        title="Demi-Vie de Volatilité"
        direction="top"
      >
        <div class="metric-card">
          <div class="metric-label">
            Demi-Vie
          </div>
          <div class="metric-value">
            {{ volatilityDuration?.volatility_half_life_minutes || '—' }} <span class="unit">min</span>
          </div>
        </div>
        <template #definition>
          Nombre de minutes pour que la volatilité décroisse à 50% de son pic (décroissance exponentielle).
        </template>
        <template #usage>
          Mesure la vitesse de dissipation de la volatilité. Demi-vie courte (30-50min) = volatilité s'effondre rapidement. Demi-vie longue (80-120min) = volatilité persiste.
        </template>
        <template #scoring>
          Basée sur le Noise Ratio et la stabilité du mouvement. NR<1.5 (stable) = demi-vie longue 60-70% du pic. NR>2.5 (décroissant) = demi-vie courte 30-40% du pic.
        </template>
      </MetricTooltip>

      <!-- Trade Duration (NEW PARAM) -->
      <MetricTooltip
        title="Durée du Trade"
        direction="top"
      >
        <div class="metric-card">
          <div class="metric-label">
            Durée Trade
          </div>
          <div class="metric-value">
            {{ tradingPlan?.tradeDurationMinutes || '—' }} <span class="unit">min</span>
          </div>
        </div>
        <template #definition>
          Durée optimale de trading recommandée = max(peak_duration, demi-vie × 2). C'est le temps maximum avant que la volatilité devienne insuffisante.
        </template>
        <template #usage>
          Indique quand fermer le trade pour éviter les whipsaws en fin de mouvement. Exemple : si durée=150min et entrée 14h30, fermer avant 16h20. Crucial pour le trailing stop.
        </template>
        <template #scoring>
          Formula : max(peak_duration, half_life × 2). Protège contre surtemps = perte. Exemple: pic 150min + half-life 60min → max(150, 120) = 150min de trading.
        </template>
      </MetricTooltip>

      <!-- Confidence Score -->
      <MetricTooltip
        title="Score de Confiance"
        direction="top"
      >
        <div class="metric-card">
          <div class="metric-label">
            Confiance
          </div>
          <div class="metric-value">
            {{ volatilityDuration?.confidence_score || '—' }} <span class="unit">%</span>
          </div>
        </div>
        <template #definition>
          Fiabilité des métriques de durée basée sur la taille de l'échantillon historique du créneau.
        </template>
        <template #usage>
          Score ≥90% = métriques très fiables (données abondantes). Score 50-75% = données partielles, variance possible. Influence la position size et le stop loss.
        </template>
        <template #scoring>
          Basé sur le sample_size du créneau : ≥100 occurrences = 100%, 50-99 = 90%, 30-49 = 75%, 15-29 = 60%, &lt;15 = 50%.
        </template>
      </MetricTooltip>
    </div>
  </div>
</template>

<script setup lang="ts">
import MetricTooltip from '../MetricTooltip.vue'

defineProps<{
  volatilityDuration: any
  tradingPlan: any
}>()
</script>

<style scoped>
.volatility-duration-section {
  margin-top: 20px;
  padding: 20px;
  background: linear-gradient(135deg, rgba(45, 90, 123, 0.15) 0%, rgba(78, 205, 196, 0.1) 100%);
  border: 1px solid #2d5a7b;
  border-radius: 8px;
}

.volatility-duration-section h4 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: #e0e7ff;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.metrics-grid {
  margin-top: 15px;
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  gap: 10px;
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
