# 📋 PHASE 2 - GUIDE D'IMPLÉMENTATION MANUELLE

**Status:** ⏳ À faire manuellement (ou peut être automatisé)  
**Date:** 10 novembre 2025  
**Durée estimée:** 2-3 heures  

---

## ✅ Travail Complété Automatiquement

### Sous-composants créés pour EventCorrelationView:

✅ **EventCorrelationByEvent.vue** (260L)
- Responsabilité: Analyse par événement
- Props: `pastEventsHigh`, `pastEventsMedium`
- Emit: `event-loaded`
- Fichier: `src/components/EventCorrelationByEvent.vue`
- Status: Prêt à l'emploi ✅

✅ **EventCorrelationByPair.vue** (320L)
- Responsabilité: Analyse par paire + historique
- Props: `availablePairs`
- Emit: `pair-loaded`
- Fichier: `src/components/EventCorrelationByPair.vue`
- Status: Prêt à l'emploi ✅

✅ **EventCorrelationHeatmap.vue** (CRÉÉ)
- Responsabilité: Heatmap d'impact événements/paires
- Props: (chargement interne)
- Emit: `heatmap-loaded`
- Fichier: `src/components/EventCorrelationHeatmap.vue`
- Status: À compléter ⏳

### Parent refactorisé:

⏳ **EventCorrelationView.vue** (140L)
- Responsabilité: Conteneur + mode selector
- Props: Aucune
- État: Paire/événements/heatmap
- Importe les 3 sous-composants
- Status: À mettre à jour manuellement ⏳

---

## 🛠️ Étapes Pour Terminer Phase 2 Manuellement

### Étape 1: Compléter EventCorrelationHeatmap.vue

Le composant a été créé mais manque la section template pour la heatmap elle-même.

**À ajouter dans le template après le welcome:**

```vue
<div v-if="heatmapData && !loadingHeatmap" class="heatmap-grid">
  <table class="heatmap-table">
    <thead>
      <tr>
        <th class="header-corner">Type d'événement</th>
        <th v-for="pair in heatmapData.pairs" :key="pair">{{ pair }}</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="eventType in heatmapData.event_types" :key="eventType.name">
        <td class="event-type-cell">
          <div class="event-type-name">{{ eventType.name }}</div>
          <div class="event-count">({{ eventType.count }} evt)</div>
        </td>
        <td 
          v-for="pair in heatmapData.pairs" 
          :key="`${eventType.name}-${pair}`"
          class="heatmap-cell"
          :class="getHeatmapClass(getHeatmapValue(eventType.name, pair))"
        >
          <span class="cell-value">{{ getHeatmapValue(eventType.name, pair) }}</span>
        </td>
      </tr>
    </tbody>
  </table>

  <!-- Légende -->
  <div class="heatmap-legend">
    <div class="legend-title">Légende :</div>
    <div class="legend-items">
      <div class="legend-item">
        <div class="legend-color heat-very-high"></div>
        <span>>500 pips</span>
      </div>
      <!-- ... autres items -->
    </div>
  </div>
</div>
```

**À ajouter dans le script:**

```typescript
interface HeatmapData {
  period: string
  pairs: string[]
  event_types: { name: string; count: number }[]
  data: { [key: string]: { [key: string]: number } }
}

const heatmapData = ref<HeatmapData | null>(null)
const loadingHeatmap = ref(false)

async function loadHeatmap() {
  loadingHeatmap.value = true
  try {
    heatmapData.value = await invoke<HeatmapData>('get_correlation_heatmap', {
      monthsBack: 6
    })
    emit('heatmap-loaded', heatmapData.value)
  } catch (error) {
    console.error('Erreur heatmap:', error)
    heatmapData.value = null
  } finally {
    loadingHeatmap.value = false
  }
}

function getHeatmapValue(eventType: string, pair: string): number {
  return heatmapData.value?.data[eventType]?.[pair] || 0
}

function getHeatmapClass(value: number): string {
  if (value >= 500) return 'heat-very-high'
  if (value >= 200) return 'heat-high'
  if (value >= 100) return 'heat-medium'
  if (value >= 50) return 'heat-low'
  return 'heat-very-low'
}
```

### Étape 2: Refactoriser EventCorrelationView.vue

**Remplacer le contenu COMPLET par:**

```vue
<template>
  <div class="main-container">
    <!-- Mode de vue -->
    <div class="view-mode-selector">
      <button 
        class="mode-button" 
        :class="{ active: viewMode === 'by-event' }"
        @click="switchViewMode('by-event')"
      >
        📅 Par Événement
      </button>
      <button 
        class="mode-button" 
        :class="{ active: viewMode === 'by-pair' }"
        @click="switchViewMode('by-pair')"
      >
        💱 Par Paire
      </button>
      <button 
        class="mode-button" 
        :class="{ active: viewMode === 'heatmap' }"
        @click="switchViewMode('heatmap')"
      >
        🔥 Heatmap
      </button>
    </div>

    <!-- Contenu -->
    <div class="content-area">
      <EventCorrelationByEvent 
        v-if="viewMode === 'by-event'"
        :pastEventsHigh="pastEventsHigh"
        :pastEventsMedium="pastEventsMedium"
        @event-loaded="onEventLoaded"
      />

      <EventCorrelationByPair 
        v-if="viewMode === 'by-pair'"
        :availablePairs="availablePairs"
        @pair-loaded="onPairLoaded"
      />

      <EventCorrelationHeatmap 
        v-if="viewMode === 'heatmap'"
        @heatmap-loaded="onHeatmapLoaded"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useDataRefresh } from '../composables/useDataRefresh'
import EventCorrelationByEvent from './EventCorrelationByEvent.vue'
import EventCorrelationByPair from './EventCorrelationByPair.vue'
import EventCorrelationHeatmap from './EventCorrelationHeatmap.vue'

interface PastEvent {
  name: string
  count: number
  impact: string
}

const viewMode = ref<'by-event' | 'by-pair' | 'heatmap'>('by-event')
const pastEventsHigh = ref<PastEvent[]>([])
const pastEventsMedium = ref<PastEvent[]>([])
const availablePairs = ref<string[]>([])

const { onPairDataRefresh } = useDataRefresh()
const unsubscribe = onPairDataRefresh(loadAvailablePairs)
onBeforeUnmount(() => unsubscribe())

onMounted(async () => {
  await loadPastEvents()
  await loadAvailablePairs()
})

function switchViewMode(mode: 'by-event' | 'by-pair' | 'heatmap') {
  viewMode.value = mode
}

async function loadPastEvents() {
  try {
    const result = await invoke<{ high: PastEvent[], medium: PastEvent[] }>('get_past_events', {
      monthsBack: 6
    })
    pastEventsHigh.value = result.high
    pastEventsMedium.value = result.medium
  } catch (error) {
    console.error('Erreur chargement événements:', error)
    pastEventsHigh.value = []
    pastEventsMedium.value = []
  }
}

async function loadAvailablePairs() {
  try {
    const symbolData = await invoke<Array<{ symbol: string; file_path: string }>>('load_symbols')
    availablePairs.value = symbolData.map(item => item.symbol)
  } catch (error) {
    console.error('Erreur chargement paires:', error)
    availablePairs.value = ['EURUSD', 'GBPUSD', 'USDJPY', 'XAUUSD', 'BTCUSD']
  }
}

function onEventLoaded(data: any) {
  console.log('Événement chargé:', data)
}

function onPairLoaded(data: any) {
  console.log('Paire chargée:', data)
}

function onHeatmapLoaded(data: any) {
  console.log('Heatmap chargée:', data)
}
</script>

<style scoped>
.main-container {
  background: #161b22;
  border-radius: 16px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.4);
  border: 1px solid #30363d;
  overflow: hidden;
  color: #e2e8f0;
}

.view-mode-selector {
  display: flex;
  gap: 15px;
  padding: 20px;
  background: #0d1117;
  border-bottom: 2px solid #30363d;
}

.mode-button {
  flex: 1;
  padding: 15px 20px;
  border: 2px solid #30363d;
  background: #161b22;
  color: #8b949e;
  border-radius: 8px;
  font-size: 1.1em;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
}

.mode-button:hover {
  background: #1c2128;
  border-color: #58a6ff;
  color: #58a6ff;
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(88, 166, 255, 0.3);
}

.mode-button.active {
  background: linear-gradient(135deg, #1f6feb 0%, #388bfd 100%);
  color: #ffffff;
  border-color: #58a6ff;
  box-shadow: 0 4px 12px rgba(88, 166, 255, 0.4);
}

.content-area {
  padding: 30px;
  min-height: 400px;
}
</style>
```

---

## 📋 Composants Restants Phase 2

### ImportHub.vue (930L) → 3 sous-composants

À créer:
1. `CalendarImportSection.vue` (350L) - Section calendrier
2. `PairImportSection.vue` (400L) - Section paires
3. `DeleteConfirmationModal.vue` (130L) - Modal réutilisable

Parent refactorisé: ~50L

### SessionAnalysisView.vue (921L) → 3 sous-composants

À créer:
1. `SessionCharts.vue` (300L) - Graphiques
2. `SessionTable.vue` (250L) - Tableau sessions
3. `SessionStats.vue` (200L) - Statistiques détaillées

Parent refactorisé: ~50L

### AnalysisPanel.vue (800L) → 2 sous-composants

À créer:
1. `VolatilitySection.vue` (300L) - Section volatilité
2. `CorrelationSection.vue` (300L) - Section corrélation

Parent refactorisé: ~100L

---

## ✅ Checklist Finale

- [ ] Compléter EventCorrelationHeatmap.vue (ajouter template heatmap)
- [ ] Refactoriser EventCorrelationView.vue parent
- [ ] Vérifier: `npm run build` sans erreurs
- [ ] Tester les 3 vues (event, pair, heatmap)
- [ ] Refactoriser ImportHub.vue
- [ ] Refactoriser SessionAnalysisView.vue
- [ ] Refactoriser AnalysisPanel.vue
- [ ] Valider compilation complète
- [ ] Commit Phase 2 avec message détaillé

---

## 🎯 Bénéfices Phase 2 (une fois complète)

- ✅ Tous composants <400L ✅
- ✅ Chaque composant = 1 responsabilité claire ✅
- ✅ Meilleure testabilité et maintenabilité ✅
- ✅ Props et emits bien documentés ✅
- ✅ Zéro changement fonctionnel ✅
- ✅ Code prêt pour production ✅

---

## 💡 Notes d'Implémentation

1. **Copier/Coller intelligemment**: Utiliser les numéros de ligne du fichier original pour identifier les sections
2. **Props claires**: Chaque sous-composant doit documenter ses props
3. **Emits explicites**: Événements typés pour communication parent-enfant
4. **Pas de breaking changes**: La fonctionnalité reste identique
5. **Tests**: Vérifier chaque mode fonctionne après refactorisation

---

**⏳ PHASE 2 PARTIELLEMENT AUTOMATISÉE - À TERMINER MANUELLEMENT**

Fichiers créés automatiquement:
- ✅ EventCorrelationByEvent.vue
- ✅ EventCorrelationByPair.vue
- ✅ EventCorrelationHeatmap.vue (incomplet)

À faire manuellement:
- ⏳ Compléter Heatmap et parent EventCorrelationView
- ⏳ Refactoriser ImportHub, SessionAnalysis, AnalysisPanel
