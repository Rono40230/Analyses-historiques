<template>
  <div class="container">
    <div class="controls">
      <label>Win Rates (T-10, T-5, T-0, T+3):</label>
      <div class="input-group">
        <input v-model.number="wins[0]" type="number" min="0" max="100" placeholder="T-10 %" class="input"/>
        <input v-model.number="wins[1]" type="number" min="0" max="100" placeholder="T-5 %" class="input"/>
        <input v-model.number="wins[2]" type="number" min="0" max="100" placeholder="T-0 %" class="input"/>
        <input v-model.number="wins[3]" type="number" min="0" max="100" placeholder="T+3 %" class="input"/>
        <button @click="analyze" class="btn">Analyser</button>
      </div>
    </div>
    <div v-if="entryTimingLoading" class="spinner">⏳</div>
    <div v-else-if="entryTimingError" class="error">{{ entryTimingError }}</div>
    <div v-else-if="entryTimingResults.length === 0" class="empty">📭 Entrez des win rates et cliquez Analyser</div>
    <table v-else>
      <thead><tr><th>Offset</th><th>Win %</th><th>Whipsaw %</th><th>Profit Moyen</th><th>Score</th><th>Meilleur</th></tr></thead>
      <tbody><tr v-for="r in entryTimingResults" :key="r.entry_offset_minutes"><td>T{{ r.entry_offset_minutes > 0 ? '+' : '' }}{{ r.entry_offset_minutes }} min</td><td>{{ r.win_rate.toFixed(1) }}%</td><td>{{ r.whipsaw_rate.toFixed(1) }}%</td><td>{{ r.avg_profit_pips.toFixed(2) }}p</td><td>{{ r.quality_score.toFixed(0) }}</td><td>{{ r.is_best ? '⭐' : '' }}</td></tr></tbody>
    </table>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
import { useRetrospectiveAnalysis } from '../composables/useRetrospectiveAnalysis'

const { entryTimingLoading, entryTimingError, entryTimingResults, analyzeEntryTiming } = useRetrospectiveAnalysis()
const wins = ref<[number, number, number, number]>([50, 55, 48, 42])

async function analyze() {
  await analyzeEntryTiming(wins.value)
}
</script>
<style scoped>
.container { padding: 20px; background: #0d1117; border-radius: 8px; color: #e2e8f0; }
.controls { margin-bottom: 15px; }
.input-group { display: flex; gap: 10px; flex-wrap: wrap; align-items: center; }
.input { padding: 8px; background: #161b22; border: 1px solid #30363d; color: #e2e8f0; border-radius: 4px; flex: 1; min-width: 100px; }
.btn { padding: 8px 16px; background: #238636; color: white; border: none; border-radius: 4px; cursor: pointer; font-weight: 700; }
.btn:hover { background: #2ea043; }
.spinner, .empty { text-align: center; color: #8b949e; padding: 30px; }
.error { background: #3d2626; color: #f85149; padding: 10px; border-radius: 4px; }
table { width: 100%; border-collapse: collapse; background: #161b22; margin-top: 15px; }
th, td { padding: 10px; text-align: left; border-bottom: 1px solid #30363d; }
th { color: #1f6feb; font-weight: 700; }
</style>
