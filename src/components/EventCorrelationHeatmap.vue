<template>
  <div v-if="loadingPair" class="loading">
    <div class="spinner"></div>
    <p>Chargement de l'historique...</p>
  </div>

  <!-- Message de bienvenue avec dropdown paire -->
  <div v-if="!selectedPairSymbol && !loadingPair" class="welcome">
    <div class="welcome-icon">💱</div>
    <h3>Analyse Rétrospective par Paire</h3>
    <div class="welcome-select-container">
      <select 
        id="pair-select" 
        v-model="selectedPairSymbol" 
        class="welcome-symbol-select"
        @change="loadPairEventHistory"
      >
        <option value="">Choisir une paire</option>
        <option v-for="pair in availablePairs" :key="pair" :value="pair">
          {{ pair }}
        </option>
      </select>
    </div>
  </div>

  <div v-if="pairHistory && !loadingPair" class="pair-history-results">
    <!-- En-tête paire avec dropdown pour changer -->
    <div class="pair-header-card">
      <div class="pair-header-with-selector">
        <h3>💱 {{ pairHistory.symbol }}</h3>
        <select 
          v-model="selectedPairSymbol" 
          class="inline-select"
          @change="loadPairEventHistory"
        >
          <option :value="selectedPairSymbol">{{ selectedPairSymbol }}</option>
          <option value="">Changer de paire</option>
          <option v-for="pair in availablePairs" :key="pair" :value="pair" v-show="pair !== selectedPairSymbol">
            {{ pair }}
          </option>
        </select>
      </div>
      <p>Historique des réactions aux événements ({{ pairHistory.period }})</p>
    </div>

    <!-- Statistiques agrégées -->
    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-icon">📊</div>
        <div class="stat-value">{{ pairHistory.total_events }}</div>
        <div class="stat-label">Événements analysés</div>
      </div>
      <div class="stat-card">
        <div class="stat-icon">📈</div>
        <div class="stat-value">{{ pairHistory.avg_volatility.toFixed(1) }} pips</div>
        <div class="stat-label">Volatilité moyenne</div>
      </div>
      <div class="stat-card">
        <div class="stat-icon">🎯</div>
        <div class="stat-value">{{ pairHistory.max_volatility.toFixed(1) }} pips</div>
        <div class="stat-label">Impact maximum</div>
      </div>
      <div class="stat-card">
        <div class="stat-icon">⚡</div>
        <div class="stat-value">×{{ pairHistory.avg_multiplier.toFixed(2) }}</div>
        <div class="stat-label">Multiplicateur moyen</div>
      </div>
    </div>

    <!-- Tableau historique -->
    <div class="history-table-container">
      <h3>📅 Historique détaillé</h3>
      
      <table class="history-table">
        <thead>
          <tr>
            <th class="sortable" style="cursor: pointer;" @click="sortTable('date')">Date</th>
            <th class="sortable" style="cursor: pointer;" @click="sortTable('event')">Événement</th>
            <th class="sortable" style="cursor: pointer;" @click="sortTable('impact')">Impact</th>
            <th class="sortable" style="cursor: pointer;" @click="sortTable('volatility')">Volatilité</th>
            <th class="sortable" style="cursor: pointer;" @click="sortTable('baseline')">vs Baseline</th>
            <th class="sortable" style="cursor: pointer;" @click="sortTable('direction')">Direction</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="event in pairHistory.events" :key="event.event_id">
            <td class="date-cell">{{ formatDate(event.datetime) }}</td>
            <td class="event-name">{{ event.event_name }}</td>
            <td>
              <span class="impact-badge" :class="`impact-${event.impact.toLowerCase()}`">
                {{ event.impact }}
              </span>
            </td>
            <td class="volatility">{{ event.volatility_formatted || event.volatility.toFixed(1) }} pips</td>
            <td class="percentage-change" :class="getChangeClass(event.change_percent)">
              +{{ Math.round(event.change_percent) }}%
            </td>
            <td>
              <span class="direction-badge" :class="`dir-${event.direction}`">
                {{ getDirectionIcon(event.direction) }}
              </span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Top événements impactants -->
    <div class="top-events-card">
      <h3>🔥 Top 5 événements les plus impactants</h3>
      <div class="top-events-list">
        <div v-for="(event, index) in pairHistory.top_events" :key="index" class="top-event-item">
          <span class="rank">{{ index + 1 }}.</span>
          <span class="event-name">{{ event.name }}</span>
          <span class="event-date">({{ formatDate(event.datetime) }})</span>
          <span class="event-volatility">→ {{ event.volatility.toFixed(1) }} pips</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// Types
interface PairEventHistory {
  symbol: string
  period: string
  total_events: number
  avg_volatility: number
  max_volatility: number
  avg_multiplier: number
  events: any[]
  top_events: any[]
}

// Props et Emits
const props = defineProps<{
  availablePairs: string[]
}>()

const emit = defineEmits<{
  'pair-loaded': [PairEventHistory]
}>()

// État
const selectedPairSymbol = ref<string>('')
const pairHistory = ref<PairEventHistory | null>(null)
const loadingPair = ref(false)

// État du tri
const sortColumn = ref<'date' | 'event' | 'impact' | 'volatility' | 'baseline' | 'direction'>('date')
const sortOrder = ref<'asc' | 'desc'>('desc')

// Fonctions
async function loadPairEventHistory() {
  if (!selectedPairSymbol.value) return
  
  loadingPair.value = true
  try {
    // Charger les vraies données depuis le backend
    pairHistory.value = await invoke<PairEventHistory>('get_pair_event_history', {
      pairSymbol: selectedPairSymbol.value,
      monthsBack: 6
    })
    
    emit('pair-loaded', pairHistory.value)
  } catch (error) {
    console.error('Erreur historique paire:', error)
    pairHistory.value = null
  } finally {
    loadingPair.value = false
  }
}

function formatDate(datetime: string): string {
  return new Date(datetime).toLocaleDateString('fr-FR', { 
    day: '2-digit', 
    month: '2-digit', 
    year: 'numeric' 
  })
}

function getDirectionIcon(direction: string): string {
  if (direction === 'HAUSSIER') return '📈'
  if (direction === 'BAISSIER') return '📉'
  return '➡️'
}

function getChangeClass(percent: number): string {
  if (percent >= 1000) return 'change-extreme'
  if (percent >= 500) return 'change-very-high'
  if (percent >= 200) return 'change-high'
  return 'change-medium'
}

function sortTable(column: 'date' | 'event' | 'impact' | 'volatility' | 'baseline' | 'direction') {
  // Si on clique sur la même colonne, inverser l'ordre
  if (sortColumn.value === column) {
    sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc'
  } else {
    // Nouvelle colonne: trier en desc par défaut
    sortColumn.value = column
    sortOrder.value = 'desc'
  }

  if (!pairHistory.value?.events) return

  // Créer une copie et trier
  const sorted = [...pairHistory.value.events]
  
  sorted.sort((a, b) => {
    let aVal: any = a
    let bVal: any = b

    switch (column) {
      case 'date':
        aVal = new Date(a.datetime).getTime()
        bVal = new Date(b.datetime).getTime()
        break
      case 'event':
        aVal = a.event_name
        bVal = b.event_name
        break
      case 'impact':
        const impactOrder = { HIGH: 3, MEDIUM: 2, LOW: 1 }
        aVal = impactOrder[a.impact as keyof typeof impactOrder] || 0
        bVal = impactOrder[b.impact as keyof typeof impactOrder] || 0
        break
      case 'volatility':
        aVal = a.volatility || 0
        bVal = b.volatility || 0
        break
      case 'baseline':
        aVal = a.change_percent || 0
        bVal = b.change_percent || 0
        break
      case 'direction':
        aVal = a.direction || ''
        bVal = b.direction || ''
        break
    }

    if (aVal < bVal) return sortOrder.value === 'asc' ? -1 : 1
    if (aVal > bVal) return sortOrder.value === 'asc' ? 1 : -1
    return 0
  })

  pairHistory.value.events = sorted
}
</script>

<style scoped>
.loading {
  text-align: center;
  padding: 60px 20px;
  color: #e2e8f0;
}

.spinner {
  width: 50px;
  height: 50px;
  border: 4px solid #2d3748;
  border-top: 4px solid #667eea;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 20px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.welcome {
  text-align: center;
  padding: 60px 20px;
  background: #1a202c;
  border-radius: 12px;
  border: 1px solid #2d3748;
}

.welcome-icon {
  font-size: 4em;
  margin-bottom: 20px;
}

.welcome h3 {
  font-size: 1.8em;
  color: #e2e8f0;
  margin-bottom: 15px;
}

.welcome-select-container {
  display: flex;
  flex-direction: column;
  gap: 20px;
  justify-content: center;
  align-items: center;
  margin-top: 30px;
}

.welcome-symbol-select {
  padding: 12px 24px;
  font-size: 1.1em;
  border-radius: 8px;
  border: 2px solid #4a5568;
  background: #ffffff;
  color: #000000;
  cursor: pointer;
  transition: all 0.3s;
  min-width: 350px;
  max-width: 600px;
}

.welcome-symbol-select option {
  background: #ffffff;
  color: #000000;
}

.welcome-symbol-select:hover {
  border-color: #667eea;
  background: #f7fafc;
}

.welcome-symbol-select:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.2);
}

.pair-history-results {
  display: flex;
  flex-direction: column;
  gap: 30px;
}

.pair-header-card {
  background: #1a202c;
  padding: 25px;
  border-radius: 12px;
  border: 1px solid #2d3748;
  text-align: center;
}

.pair-header-with-selector {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
  justify-content: center;
  margin-bottom: 10px;
}

.pair-header-with-selector h3 {
  margin: 0;
}

.pair-header-card h3 {
  color: #e2e8f0;
  font-size: 2em;
  margin-bottom: 10px;
}

.pair-header-card p {
  color: #a0aec0;
}

.inline-select {
  padding: 8px 12px;
  border: 1px solid #4a5568;
  background: #1c2128;
  color: #000000;
  border-radius: 6px;
  font-size: 0.95em;
  cursor: pointer;
  transition: all 0.2s;
}

.inline-select:hover {
  border-color: #58a6ff;
  background: #262d38;
  color: #000000;
}

.inline-select:focus {
  outline: none;
  border-color: #58a6ff;
  box-shadow: 0 0 8px rgba(88, 166, 255, 0.3);
  color: #000000;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
}

.stat-card {
  background: linear-gradient(135deg, #2d3748 0%, #1a202c 100%);
  padding: 20px;
  border-radius: 10px;
  text-align: center;
  border: 1px solid #4a5568;
  transition: transform 0.2s;
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0,0,0,0.4);
}

.stat-icon {
  font-size: 2.5em;
  margin-bottom: 10px;
}

.stat-value {
  font-size: 2em;
  font-weight: bold;
  color: #e2e8f0;
  margin: 10px 0;
}

.stat-label {
  color: #a0aec0;
  font-size: 0.95em;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.history-table-container {
  background: #1a202c;
  padding: 25px;
  border-radius: 12px;
  border: 1px solid #2d3748;
}

.history-table-container h3 {
  color: #e2e8f0;
  margin-bottom: 20px;
}

.history-table {
  width: 100%;
  border-collapse: collapse;
}

.history-table thead {
  background: #2d3748;
}

.history-table th {
  padding: 12px;
  text-align: left;
  font-weight: 600;
  color: #e2e8f0;
  border-bottom: 2px solid #4a5568;
  cursor: pointer !important;
}

.history-table th.sortable {
  cursor: pointer !important;
  user-select: none;
  transition: background-color 0.2s ease;
}

.history-table th:hover {
  background-color: rgba(255, 255, 255, 0.05) !important;
  cursor: pointer !important;
}

.history-table td {
  padding: 12px;
  border-bottom: 1px solid #2d3748;
  color: #e2e8f0;
}

.history-table tbody tr:hover {
  background: #2d3748;
}

.date-cell {
  font-size: 0.95em;
}

.event-name {
  font-weight: 600;
}

.impact-badge {
  padding: 4px 8px;
  border-radius: 4px;
  font-weight: 600;
  font-size: 0.9em;
}

.impact-high {
  background: #dc2626;
  color: white;
}

.impact-medium {
  background: #f59e0b;
  color: white;
}

.impact-low {
  background: #3b82f6;
  color: white;
}

.volatility {
  font-weight: 600;
  color: #818cf8;
}

.percentage-change {
  font-weight: 700;
}

.change-extreme {
  color: #dc2626;
}

.change-very-high {
  color: #f59e0b;
}

.change-high {
  color: #10b981;
}

.change-medium {
  color: #3b82f6;
}

.direction-badge {
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 0.9em;
  font-weight: 600;
}

.dir-HAUSSIER {
  background: #065f46;
  color: #6ee7b7;
}

.dir-BAISSIER {
  background: #7f1d1d;
  color: #fca5a5;
}

.dir-NEUTRE {
  background: #4a5568;
  color: #e2e8f0;
}

.top-events-card {
  background: #1a202c;
  padding: 25px;
  border-radius: 12px;
  border: 1px solid #2d3748;
}

.top-events-card h3 {
  color: #e2e8f0;
  margin-bottom: 20px;
}

.top-events-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.top-event-item {
  background: #2d3748;
  padding: 15px;
  border-radius: 8px;
  display: flex;
  gap: 10px;
  align-items: center;
  color: #e2e8f0;
}

.top-event-item .rank {
  font-weight: 700;
  color: #667eea;
  font-size: 1.2em;
}

.top-event-item .event-name {
  flex: 1;
  font-weight: 600;
}

.top-event-item .event-date {
  color: #a0aec0;
  font-size: 0.9em;
}

.top-event-item .event-volatility {
  font-weight: 700;
  color: #818cf8;
}
</style>
