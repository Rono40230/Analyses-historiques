<script setup lang="ts">
import { ref } from 'vue'
import BacktestConfigPanel from '../components/BacktestConfigPanel.vue'
import BacktestResultsPanel from '../components/BacktestResultsPanel.vue'
import { BacktestType } from '../stores/backtest'

const activeTab = ref<BacktestType>(BacktestType.Event)
</script>

<template>
  <div class="backtest-view">
    <div class="tabs-header">
      <button 
        :class="['tab-btn', { active: activeTab === BacktestType.Event }]"
        @click="activeTab = BacktestType.Event"
      >
        📅 Par Événement
      </button>
      <button 
        :class="['tab-btn', { active: activeTab === BacktestType.Time }]"
        @click="activeTab = BacktestType.Time"
      >
        ⏰ Par Horaire
      </button>
    </div>

    <BacktestConfigPanel :backtest-type="activeTab" />
    
    <div class="results-container">
      <BacktestResultsPanel />
    </div>
  </div>
</template>

<style scoped>
.backtest-view {
  padding: 1rem;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  overflow: hidden;
}

.tabs-header {
  display: flex;
  gap: 1rem;
  border-bottom: 1px solid #2d3748;
  padding-bottom: 0.5rem;
}

.tab-btn {
  background: transparent;
  border: none;
  color: #a0aec0;
  padding: 0.5rem 1rem;
  cursor: pointer;
  font-size: 1rem;
  font-weight: 600;
  border-radius: 6px;
  transition: all 0.2s;
}

.tab-btn:hover {
  color: #e2e8f0;
  background: #2d3748;
}

.tab-btn.active {
  color: #4299e1;
  background: rgba(66, 153, 225, 0.1);
}

.results-container {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
}
</style>

