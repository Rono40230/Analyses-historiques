<script setup lang="ts">
import { useBacktestConfig } from '../composables/useBacktestConfig'
import { BacktestType } from '../stores/backtest'
import SymbolSelector from './SymbolSelector.vue'
import SearchableEventDropdown from './SearchableEventDropdown.vue'

const props = defineProps<{
  backtestType: BacktestType
}>()

const {
  config,
  mode,
  loading,
  selectedSymbol,
  selectedEvent,
  selectedTime,
  startDate,
  endDate,
  availableEvents,
  lancerBacktest,
  StrategyMode
} = useBacktestConfig(props)
</script>

<template>
  <div class="config-panel">
    
    <div class="header-controls">
      <div class="control-group symbol-group">
        <SymbolSelector v-model="selectedSymbol" />
      </div>

      <!-- Mode Événement -->
      <div v-if="backtestType === BacktestType.Event" class="control-group event-group">
        <label>Événement</label>
        <SearchableEventDropdown 
          v-model="selectedEvent" 
          :events="availableEvents"
        />
      </div>

      <!-- Mode Horaire -->
      <template v-else>
        <div class="control-group time-group">
          <label>Heure (UTC)</label>
          <input type="time" v-model="selectedTime" class="time-input" />
        </div>
        <div class="control-group date-group">
          <label>Début</label>
          <input type="date" v-model="startDate" class="date-input" />
        </div>
        <div class="control-group date-group">
          <label>Fin</label>
          <input type="date" v-model="endDate" class="date-input" />
        </div>
      </template>

      <div class="control-group strategy-group">
        <label>Mode Stratégie</label>
        <select v-model="mode" class="strategy-select">
          <option :value="StrategyMode.Directionnel">Directionnel</option>
          <option :value="StrategyMode.Simultane">Simultané</option>
        </select>
      </div>
    </div>

    <div class="params-grid">
      <div class="param">
        <label>Offset (pips)</label>
        <input 
          type="number" 
          v-model.number="config.offset_pips" 
          step="0.1" 
          :disabled="mode === StrategyMode.Simultane"
        />
      </div>
      <div class="param">
        <label>Stop Loss (pips)</label>
        <input type="number" v-model.number="config.stop_loss_pips" step="0.1" />
      </div>
      <div class="param">
        <label>Trailing Stop (pips)</label>
        <input type="number" v-model.number="config.trailing_stop_pips" step="0.1" />
      </div>
      <div class="param">
        <label>Timeout (min)</label>
        <input type="number" v-model.number="config.timeout_minutes" />
      </div>
      <div class="param">
        <label>Spread (pips)</label>
        <input type="number" v-model.number="config.spread_pips" step="0.1" />
      </div>
      
      <!-- Paramètres spécifiques Simultané -->
      <template v-if="mode === StrategyMode.Simultane">
        <div class="param">
          <label>SL Recovery (pips)</label>
          <input type="number" v-model.number="config.sl_recovery_pips" step="0.1" placeholder="Auto = SL" />
        </div>
      </template>
    </div>

    <div class="actions">
      <button 
        class="run-btn" 
        @click="lancerBacktest" 
        :disabled="loading || !selectedSymbol || (backtestType === BacktestType.Event && !selectedEvent)"
      >
        <span v-if="loading">⏳ Simulation...</span>
        <span v-else>🚀 Lancer Backtest</span>
      </button>
    </div>
  </div>
</template>

<style scoped>
.config-panel {
  background: #1a202c;
  padding: 1.5rem;
  border-radius: 8px;
  border: 1px solid #2d3748;
}

.header-controls {
  display: flex;
  gap: 1.5rem;
  margin-bottom: 1.5rem;
  align-items: flex-end;
}

.control-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.symbol-group { flex: 1; max-width: 200px; }
.event-group { flex: 2; }
.time-group { flex: 1; max-width: 150px; }
.date-group { flex: 1; max-width: 180px; }
.strategy-group { flex: 1; max-width: 200px; }

input, select {
  height: 42px;
  padding: 0 12px;
  background: #ffffff;
  border: 1px solid #ccc;
  border-radius: 6px;
  color: #000000;
  font-size: 0.9rem;
  width: 100%;
  box-sizing: border-box;
}

/* Specific fix for native select padding */
.strategy-select, :deep(.symbol-select) {
  padding: 0 12px !important; /* Native selects need this */
}

/* Fix for SearchableEventDropdown internals */
:deep(.dropdown-header) {
  height: 42px !important;
  padding: 0 12px !important;
  border: 1px solid #ccc !important;
  box-sizing: border-box;
}

:deep(.dropdown-input) {
  color: #000000 !important;
  font-size: 0.9rem !important;
  height: 100%;
}

:deep(.symbol-select option) {
  background: #ffffff;
  color: #000000;
}

.time-input, .date-input {
  height: 42px;
  padding: 0 12px;
  background: #ffffff;
  border: 1px solid #ccc;
  border-radius: 6px;
  color: #000000;
  font-size: 0.9rem;
  width: 100%;
  box-sizing: border-box;
}

.form-group {
  margin-bottom: 1rem;
}

.params-grid {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.param {
  display: flex;
  flex-direction: column;
}

label {
  font-size: 0.9rem;
  color: #a0aec0;
  margin-bottom: 0.4rem;
  font-weight: 500;
}

.actions {
  display: flex;
  justify-content: flex-end;
}

.run-btn {
  background: #48bb78;
  color: white;
  border: none;
  padding: 0.8rem 2rem;
  border-radius: 6px;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.2s;
  font-size: 1rem;
}

.run-btn:hover:not(:disabled) {
  background: #38a169;
}

.run-btn:disabled {
  background: #2d3748;
  color: #718096;
  cursor: not-allowed;
}
</style>
