<template>
  <div
    class="filters-bar glass"
    :class="{ disabled: loading }"
  >
    <div class="filter-group">
      <label>Période :</label>
      <div class="date-inputs">
        <input
          v-model="startDate"
          type="date"
          placeholder="Début"
          class="date-input"
        >
        <span class="separator">à</span>
        <input
          v-model="endDate"
          type="date"
          placeholder="Fin"
          class="date-input"
        >
      </div>
    </div>
    
    <div class="filter-group pairs-filter">
      <label>Paires :</label>
      <div class="pairs-selector">
        <button 
          v-for="pair in availablePairs" 
          :key="pair"
          class="pair-chip"
          :class="{ active: selectedPairs.includes(pair) }"
          @click="togglePair(pair)"
        >
          {{ pair }}
        </button>
      </div>
    </div>
    
    <div class="filter-actions">
      <button
        class="apply-button"
        @click="$emit('run-analysis')"
      >
        Appliquer les filtres
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  loading: boolean
  availablePairs: string[]
}>()

const startDate = defineModel<string>('startDate')
const endDate = defineModel<string>('endDate')
const selectedPairs = defineModel<string[]>('selectedPairs', { default: [] })

const emit = defineEmits<{
  (e: 'run-analysis'): void
}>()

function togglePair(pair: string) {
  if (selectedPairs.value.includes(pair)) {
    selectedPairs.value = selectedPairs.value.filter(p => p !== pair)
  } else {
    selectedPairs.value.push(pair)
  }
}
</script>

<style scoped>
.filters-bar {
  padding: 16px 24px;
  display: flex;
  align-items: center;
  gap: 32px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  flex-wrap: wrap;
}

.filters-bar.disabled {
  opacity: 0.5;
  pointer-events: none;
}

.filter-group {
  display: flex;
  align-items: center;
  gap: 12px;
}

.filter-group label {
  color: #a0aec0;
  font-size: 14px;
  font-weight: 500;
}

.date-inputs {
  display: flex;
  align-items: center;
  gap: 8px;
}

.date-input {
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid rgba(255, 255, 255, 0.1);
  color: #fff;
  padding: 6px 12px;
  border-radius: 6px;
  font-family: inherit;
  font-size: 13px;
}

.date-input:focus {
  outline: none;
  border-color: #4ecdc4;
}

.separator {
  color: #718096;
  font-size: 13px;
}

.pairs-filter {
  flex: 1;
}

.pairs-selector {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.pair-chip {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  color: #a0aec0;
  padding: 4px 10px;
  border-radius: 20px;
  font-size: 11px;
  cursor: pointer;
  transition: all 0.2s;
}

.pair-chip:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #fff;
}

.pair-chip.active {
  background: rgba(78, 205, 196, 0.2);
  border-color: #4ecdc4;
  color: #4ecdc4;
}

.apply-button {
  background: linear-gradient(135deg, #4ecdc4 0%, #2d9ca6 100%);
  border: none;
  color: #000;
  padding: 8px 20px;
  border-radius: 6px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 13px;
}

.apply-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(78, 205, 196, 0.3);
}

.glass {
  background: rgba(30, 30, 45, 0.6);
  backdrop-filter: blur(10px);
}
</style>
