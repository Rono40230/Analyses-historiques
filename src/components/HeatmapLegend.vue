<template>
  <div class="heatmap-scale">
    <span class="scale-item"><span class="scale-color heat-very-low" />≥12 pips</span>
    <span class="scale-item"><span class="scale-color heat-low" />9-11 pips</span>
    <span class="scale-item"><span class="scale-color heat-medium" />6-8 pips</span>
    <span class="scale-item"><span class="scale-color heat-high" />3-5 pips</span>
    <span class="scale-item"><span class="scale-color heat-very-high" />&lt;3 pips</span>
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.heatmap-scale {
  display: flex;
  gap: 20px;
  align-items: center;
  font-size: 0.85em;
  color: #cbd5e0;
}

.scale-item {
  display: flex;
  align-items: center;
  gap: 6px;
  white-space: nowrap;
}

.scale-color {
  display: inline-block;
  width: 16px;
  height: 16px;
  border-radius: 3px;
  border: 1px solid #1a202c;
}

.heat-very-low {
  background: #22c55e;
  color: #1a202c;
}

.heat-low {
  background: #16a34a;
  color: white;
}

.heat-medium {
  background: #f59e0b;
  color: white;
}

.heat-high {
  background: #dc2626;
  color: white;
}

.heat-very-high {
  background: #7f1d1d;
  color: #fca5a5;
}
</style>
