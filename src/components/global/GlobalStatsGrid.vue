<template>
  <div class="global-stats-grid">
    <MetricTooltip title="Analyses Scannées">
      <div class="stat-card glass">
        <div class="stat-icon">
          📊
        </div>
        <div class="stat-info">
          <span class="stat-label">Analyses Scannées</span>
          <span class="stat-value">{{ result.total_analyses }}</span>
        </div>
      </div>
      <template #definition>
        <div class="tooltip-section-title">
          📖 Définition
        </div>
        <div class="tooltip-section-text">
          Nombre total d'archives de type "Volatilité brute" trouvées et analysées dans votre base de données.
        </div>
      </template>
      <template #usage>
        <div class="tooltip-section-title">
          💡 Interprétation
        </div>
        <div class="tooltip-section-text">
          Plus ce nombre est élevé, plus les statistiques de l'IA sont fiables. Avec moins de 5 analyses, les résultats peuvent être biaisés.
        </div>
      </template>
    </MetricTooltip>
  </div>
</template>

<script setup lang="ts">
import MetricTooltip from '../MetricTooltip.vue'

interface GlobalStatsResult {
  total_analyses: number
}

defineProps<{
  result: GlobalStatsResult
}>()
</script>

<style scoped>
.global-stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 20px;
  margin-bottom: 30px;
}

.stat-card {
  padding: 20px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  gap: 16px;
  border: 1px solid rgba(255, 255, 255, 0.05);
  transition: transform 0.2s;
}

.stat-card:hover {
  transform: translateY(-2px);
  border-color: rgba(78, 205, 196, 0.3);
}

.stat-icon {
  font-size: 32px;
  background: rgba(255, 255, 255, 0.05);
  width: 60px;
  height: 60px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.stat-info {
  display: flex;
  flex-direction: column;
}

.stat-label {
  color: #a0aec0;
  font-size: 13px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 4px;
}

.stat-value {
  font-size: 24px;
  font-weight: 700;
  color: #fff;
}

.stat-value.highlight {
  color: #4ecdc4;
}

.tooltip-section-title {
  font-size: 12px;
  font-weight: 700;
  color: #4ecdc4;
  margin-bottom: 4px;
  text-transform: uppercase;
}

.tooltip-section-text {
  font-size: 12px;
  color: #e2e8f0;
  line-height: 1.5;
  margin-bottom: 12px;
}

.tooltip-section-text:last-child {
  margin-bottom: 0;
}

.glass {
  background: rgba(30, 30, 45, 0.6);
  backdrop-filter: blur(10px);
}
</style>
