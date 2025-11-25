<template>
  <div class="color-legend-container">
    <div class="color-legend">
      <div class="legend-grid">
        <div class="legend-item">
          <div class="legend-color excellent" />
          <div class="legend-text">
            <strong>Excellent</strong>
            <span>Conditions optimales, idéal pour trader</span>
          </div>
        </div>
        <div class="legend-item">
          <div class="legend-color good" />
          <div class="legend-text">
            <strong>Bon</strong>
            <span>Conditions favorables, approche recommandée</span>
          </div>
        </div>
        <div class="legend-item">
          <div class="legend-color acceptable" />
          <div class="legend-text">
            <strong>Acceptable</strong>
            <span>Conditions moyen, prudence recommandée</span>
          </div>
        </div>
        <div class="legend-item">
          <div class="legend-color poor" />
          <div class="legend-text">
            <strong>Pauvre</strong>
            <span>Conditions inadaptées, à éviter</span>
          </div>
        </div>
      </div>
    </div>
    <button
      class="analysis-btn"
      title="Ouvrir l'analyse détaillée des métriques"
      @click="$emit('analyze')"
    >
      📊 Analyse des métriques<br><span style="font-size: 0.85em">du meilleur créneau horaire</span>
    </button>
  </div>
</template>

<script setup lang="ts">
defineEmits<{
  analyze: []
}>()
</script>

<style scoped>
.color-legend-container {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 20px;
  margin-top: 30px;
}
.color-legend {
  flex: 1;
  background: #1a202c;
  padding: 20px;
  border-radius: 8px;
  border: 1px solid #30363d;
}
.analysis-btn {
  background: linear-gradient(135deg, #64c8ff 0%, #3b82f6 100%);
  color: #1a1a2e;
  border: none;
  padding: 12px 20px;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  white-space: nowrap;
  box-shadow: 0 4px 12px rgba(100, 200, 255, 0.3);
  font-size: 14px;
  height: fit-content;
}
.analysis-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(100, 200, 255, 0.4);
  background: linear-gradient(135deg, #4db8ff 0%, #2972e1 100%);
}
.analysis-btn:active {
  transform: translateY(0px);
  box-shadow: 0 2px 8px rgba(100, 200, 255, 0.3);
}
.legend-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px; }
.legend-item { display: flex; gap: 10px; align-items: center; }
.legend-color { width: 16px; height: 16px; border-radius: 3px; flex-shrink: 0; }
.legend-color.excellent { background: linear-gradient(135deg, #10b981 0%, #059669 100%); box-shadow: 0 0 8px rgba(16, 185, 129, 0.3); }
.legend-color.good { background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%); box-shadow: 0 0 8px rgba(59, 130, 246, 0.3); }
.legend-color.acceptable { background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%); box-shadow: 0 0 8px rgba(245, 158, 11, 0.3); }
.legend-color.poor { background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%); box-shadow: 0 0 8px rgba(239, 68, 68, 0.3); }
.legend-text { flex: 1; }
.legend-text strong { display: block; color: #e2e8f0; }
.legend-text span { color: #a0aec0; font-size: 0.85em; display: block; }
@media (max-width: 768px) {
  .color-legend-container { flex-direction: column; }
  .analysis-btn { width: 100%; }
}
</style>
