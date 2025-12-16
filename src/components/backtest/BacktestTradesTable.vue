<script setup lang="ts">
import type { TradeResult } from '../../stores/backtest'

defineProps<{
  trades: TradeResult[]
}>()

function formatDate(iso: string) {
  return new Date(iso).toLocaleString('fr-FR')
}

function getOutcomeClass(outcome: string) {
  switch (outcome) {
    case 'TakeProfit': return 'outcome-win'
    case 'RecoveryWin': return 'outcome-recovery'
    case 'StopLoss': return 'outcome-loss'
    case 'DoubleLoss': return 'outcome-loss'
    case 'Timeout': return 'outcome-neutral'
    case 'NoEntry': return 'outcome-neutral'
    default: return ''
  }
}
</script>

<template>
  <div class="trades-list">
    <h3>Journal des Trades</h3>
    <div class="table-container">
      <table>
        <thead>
          <tr>
            <th>Date Événement</th>
            <th>Entrée</th>
            <th>Sortie</th>
            <th>Durée</th>
            <th>Pips Net</th>
            <th>Résultat</th>
            <th>MFE / MAE</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(trade, index) in trades" :key="index">
            <td>{{ formatDate(trade.event_date) }}</td>
            <td>{{ trade.entry_time ? formatDate(trade.entry_time).split(' ')[1] : '-' }}</td>
            <td>{{ trade.exit_time ? formatDate(trade.exit_time).split(' ')[1] : '-' }}</td>
            <td>{{ trade.duration_minutes }}m</td>
            <td :class="trade.pips_net > 0 ? 'win' : (trade.pips_net < 0 ? 'loss' : 'neutral')">
              {{ trade.pips_net.toFixed(1) }}
            </td>
            <td>
              <span :class="['outcome-badge', getOutcomeClass(trade.outcome)]">
                {{ trade.outcome }}
              </span>
            </td>
            <td class="excursion">
              <span class="mfe" title="Max Favorable Excursion">+{{ trade.max_favorable_excursion.toFixed(1) }}</span> / 
              <span class="mae" title="Max Adverse Excursion">-{{ trade.max_adverse_excursion.toFixed(1) }}</span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<style scoped>
.trades-list {
  background: #1a202c;
  padding: 1.5rem;
  border-radius: 8px;
  border: 1px solid #2d3748;
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.trades-list h3 {
  margin: 0 0 1rem 0;
  color: #e2e8f0;
}

.table-container {
  flex: 1;
  overflow-y: auto;
  border: 1px solid #2d3748;
  border-radius: 6px;
}

table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.9rem;
}

th, td {
  padding: 0.75rem 1rem;
  text-align: left;
  border-bottom: 1px solid #2d3748;
}

th {
  background: #2d3748;
  color: #a0aec0;
  font-weight: 600;
  position: sticky;
  top: 0;
}

tr:hover {
  background: #2d3748;
}

.win { color: #48bb78; }
.loss { color: #f56565; }
.neutral { color: #a0aec0; }

.outcome-badge {
  padding: 0.2rem 0.5rem;
  border-radius: 4px;
  font-size: 0.8rem;
  font-weight: 600;
}

.outcome-win { background: rgba(72, 187, 120, 0.2); color: #48bb78; }
.outcome-recovery { background: rgba(66, 153, 225, 0.2); color: #4299e1; }
.outcome-loss { background: rgba(245, 101, 101, 0.2); color: #f56565; }
.outcome-neutral { background: rgba(160, 174, 192, 0.2); color: #a0aec0; }

.excursion {
  font-family: monospace;
  font-size: 0.85rem;
}

.mfe { color: #48bb78; }
.mae { color: #f56565; }
</style>
