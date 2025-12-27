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

import { computed } from 'vue'
import '../styles/backtest-config.css'

const isRunDisabled = computed(() => {
  if (loading.value) return true
  if (!selectedSymbol.value) return true
  if (props.backtestType === BacktestType.Event && !selectedEvent.value) return true
  return false
})

const disabledReason = computed(() => {
  if (loading.value) return 'Simulation en cours...'
  if (!selectedSymbol.value) return 'Veuillez sélectionner une paire'
  if (props.backtestType === BacktestType.Event && !selectedEvent.value) return 'Veuillez sélectionner un événement'
  return 'Prêt à lancer'
})
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
      <div class="param">
        <label>Slippage (pips)</label>
        <input type="number" v-model.number="config.slippage_pips" step="0.1" title="Glissement estimé à l'exécution" />
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
        :disabled="isRunDisabled"
        :title="disabledReason"
      >
        <span v-if="loading">⏳ Simulation...</span>
        <span v-else>🚀 Lancer Backtest</span>
      </button>
    </div>
  </div>
</template>

