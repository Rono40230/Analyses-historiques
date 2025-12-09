<template>
  <div class="graph-section">
    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; gap: 15px; flex-wrap: wrap;">
      <div style="display: flex; align-items: center; gap: 20px; flex: 1;">
        <h3>📊 Impact de l'événement {{ eventLabel ? eventLabel : eventType }}{{ pair ? ' sur la volatilité de ' + pair : '' }}</h3>
        <!-- Conclusion rapide en haut -->
        <div class="quick-conclusion">
          <span v-if="volatilityIncreasePercent > 0" class="conclusion-positive">
            ✅ Événement génère {{ (volatilityIncreasePercent).toFixed(1) }}% de volatilité {{ noiseQualityAfter === 'clean' ? 'directionnelle' : 'avec bruit' }}
          </span>
          <span v-else class="conclusion-negative">
            ⚠️ Événement peu corrélé à la volatilité sur {{ pair }}
          </span>
        </div>
      </div>
      <button v-if="!isArchiveMode" class="btn-archive" @click="$emit('archive')">💾 Archiver</button>
    </div>

    <!-- Graphique 2 courbes comparatives -->
    <div class="graph-container">
      <svg viewBox="0 0 1000 450" class="graph">
        <defs>
          <linearGradient id="beforeGradient" x1="0%" y1="0%" x2="0%" y2="100%">
            <stop offset="0%" style="stop-color:#58a6ff;stop-opacity:0.3" />
            <stop offset="100%" style="stop-color:#58a6ff;stop-opacity:0" />
          </linearGradient>
          <linearGradient id="afterGradient" x1="0%" y1="0%" x2="0%" y2="100%">
            <stop offset="0%" style="stop-color:#f85149;stop-opacity:0.3" />
            <stop offset="100%" style="stop-color:#f85149;stop-opacity:0" />
          </linearGradient>
        </defs>

        <!-- Axes -->
        <line :x1="svgMargins.left" :y1="yAxisBaseline" :x2="svgMargins.right" :y2="yAxisBaseline" stroke="#4a5568" stroke-width="2" />
        <line :x1="svgMargins.left" y1="50" :x2="svgMargins.left" y2="380" stroke="#4a5568" stroke-width="2" />

        <!-- Grille horizontale -->
        <line :x1="svgMargins.left" :y1="yMidLine" :x2="svgMargins.right" :y2="yMidLine" stroke="#2d3748" stroke-width="1" stroke-dasharray="5,5" />

        <!-- Ligne séparatrice T0 (événement) -->
        <line :x1="svgMargins.t0" y1="50" :x2="svgMargins.t0" y2="380" stroke="#fbbf24" stroke-width="2.5" stroke-dasharray="4,4" opacity="0.8" />
        <text :x="svgMargins.t0" y="35" font-size="12" text-anchor="middle" fill="#fbbf24" font-weight="bold">T0 (Événement)</text>
        <text :x="svgMargins.right - 5" y="35" font-size="11" text-anchor="end" fill="#8b949e" font-style="italic">GMT+0</text>

        <!-- Ligne verticale pour le meilleur moment (entrée optimale) -->
        <line v-if="props.meilleurMoment > 0" :x1="bestMomentX" y1="50" :x2="bestMomentX" y2="380" stroke="#10b981" stroke-width="2" stroke-dasharray="6,3" opacity="0.7" />
        <text v-if="props.meilleurMoment > 0" :x="bestMomentX" y="45" font-size="11" text-anchor="middle" fill="#10b981" font-weight="600">{{ props.meilleurMoment.toFixed(0) }}min avant</text>

        <!-- Labels Y (ATR) -->
        <text :x="svgMargins.labelY" :y="yAxisBaseline + 5" font-size="12" text-anchor="end" fill="#8b949e">{{ minAtrLabel }}</text>
        <text :x="svgMargins.labelY" :y="yMidLine + 5" font-size="12" text-anchor="end" fill="#8b949e">{{ midAtrLabel }}</text>
        <text :x="svgMargins.labelY" y="55" font-size="12" text-anchor="end" fill="#8b949e">{{ maxAtrLabel }}</text>

        <!-- Marqueurs X: AVANT (-30 à 0) -->
        <template v-for="minute in [-30, -20, -10, 0]" :key="`tick-before-${minute}`">
          <line :x1="getXPositionBefore(minute)" :y1="yAxisBaseline + 5" :x2="getXPositionBefore(minute)" :y2="yAxisBaseline + 12" stroke="#4a5568" stroke-width="1.5" />
          <text :x="getXPositionBefore(minute)" :y="yAxisBaseline + 28" font-size="10" text-anchor="middle" fill="#8b949e">{{ getTimeLabel(minute) }}</text>
        </template>

        <!-- Marqueurs X: APRÈS (0 à 90, tous les 15 min) -->
        <template v-for="minute in [0, 15, 30, 45, 60, 75, 90]" :key="`tick-after-${minute}`">
          <line :x1="getXPositionAfter(minute)" :y1="yAxisBaseline + 5" :x2="getXPositionAfter(minute)" :y2="yAxisBaseline + 12" stroke="#4a5568" stroke-width="1.5" />
          <text :x="getXPositionAfter(minute)" :y="yAxisBaseline + 28" font-size="10" text-anchor="middle" fill="#8b949e">{{ getTimeLabel(minute) }}</text>
        </template>

        <!-- Courbe AVANT (T-30 à T0) en bleu -->
        <template v-if="atrTimelineBefore && atrTimelineBefore.length > 1">
          <path :d="curvePathBefore" fill="url(#beforeGradient)" stroke="none" />
          <polyline :points="beforePointsString" fill="none" stroke="#58a6ff" stroke-width="2.5" stroke-linejoin="round" stroke-linecap="round" />
          <!-- Labels ATR AVANT -->
          <text x="130" y="100" font-size="13" fill="#58a6ff" font-weight="bold">ATR Avant: {{ ceilValue(atrTimelineBefore[Math.floor(atrTimelineBefore.length / 2)]) }}</text>
        </template>

        <!-- Courbe APRÈS (T0 à T+90) en rouge/orange -->
        <template v-if="atrTimelineAfter && atrTimelineAfter.length > 1">
          <path :d="curvePathAfter" fill="url(#afterGradient)" stroke="none" />
          <polyline :points="afterPointsString" fill="none" stroke="#f85149" stroke-width="2.5" stroke-linejoin="round" stroke-linecap="round" />
          <!-- Labels ATR APRÈS -->
          <text x="600" y="100" font-size="13" fill="#f85149" font-weight="bold">ATR Après: {{ ceilValue(atrTimelineAfter[Math.floor(atrTimelineAfter.length / 2)]) }}</text>
        </template>
      </svg>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from 'vue'

interface Props {
  atrTimelineBefore?: number[]
  atrTimelineAfter?: number[]
  bodyTimelineBefore?: number[]
  bodyTimelineAfter?: number[]
  noiseRatioBefore: number
  noiseRatioDuring: number
  noiseRatioAfter: number
  volatilityIncreasePercent: number
  eventCount: number
  eventType: string
  pair: string
  eventDatetime?: string
  timezoneOffset?: string
  isArchiveMode?: boolean
  eventLabel?: string
  meilleurMoment?: number
}

const props = withDefaults(defineProps<Props>(), {
  atrTimelineBefore: () => [],
  atrTimelineAfter: () => [],
  bodyTimelineBefore: () => [],
  bodyTimelineAfter: () => [],
  noiseRatioBefore: 0,
  noiseRatioDuring: 0,
  noiseRatioAfter: 0,
  volatilityIncreasePercent: 0,
  eventCount: 0,
  eventType: '',
  pair: '',
  eventDatetime: '',
  timezoneOffset: 'UTC+0',
  isArchiveMode: false,
  eventLabel: '',
  meilleurMoment: 0
})

defineEmits<{ archive: [] }>()

// Variables réactives pour adapter les marges SVG à la taille de l'écran
const screenWidth = ref<number>(typeof window !== 'undefined' ? window.innerWidth : 1024)

const svgMargins = computed(() => {
  if (screenWidth.value > 1024) return { left: 20, right: 999, t0: 445, labelY: 15 }
  if (screenWidth.value > 768) return { left: 40, right: 980, t0: 435, labelY: 30 }
  if (screenWidth.value > 480) return { left: 60, right: 960, t0: 425, labelY: 50 }
  return { left: 80, right: 940, t0: 415, labelY: 70 }
})

const handleResize = () => { screenWidth.value = window.innerWidth }

onMounted(() => { window.addEventListener('resize', handleResize) })
onUnmounted(() => { window.removeEventListener('resize', handleResize) })

// Fonctions pour formater l'heure
function parseEventDateTime(): Date {
  if (!props.eventDatetime) return new Date()
  try { return new Date(props.eventDatetime) }
  catch { return new Date() }
}

function getTimeLabel(minuteOffset: number): string {
  const eventDate = parseEventDateTime()
  const adjustedDate = new Date(eventDate.getTime() + minuteOffset * 60000)
  const minutes = adjustedDate.getMinutes()
  const roundedMinutes = Math.round(minutes / 5) * 5
  adjustedDate.setMinutes(roundedMinutes)
  return adjustedDate.toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit', hour12: false })
}

function ceilValue(value: number): number {
  return Math.ceil(value)
}

// Calcul des extrema ATR
const allAtrValues = computed(() => [...(props.atrTimelineBefore || []), ...(props.atrTimelineAfter || [])])
const minAtr = computed(() => Math.min(...allAtrValues.value, 0))
const maxAtr = computed(() => Math.max(...allAtrValues.value, 0.001))

const minAtrLabel = computed(() => Math.ceil(minAtr.value).toString())
const maxAtrLabel = computed(() => Math.ceil(maxAtr.value).toString())
const midAtrLabel = computed(() => Math.ceil((minAtr.value + maxAtr.value) / 2).toString())

const yAxisBaseline = 380
const yMidLine = 215
const yTopLine = 50

const mapAtrToY = (atrValue: number): number => {
  if (maxAtr.value === minAtr.value) return yAxisBaseline
  const ratio = (atrValue - minAtr.value) / (maxAtr.value - minAtr.value)
  return yAxisBaseline - ratio * (yAxisBaseline - yTopLine)
}

const getXPositionBefore = (minute: number): number => {
  const normalized = (minute + 30) / 30
  const leftMargin = svgMargins.value.left
  const t0Pos = svgMargins.value.t0
  const beforeWidth = t0Pos - leftMargin
  return leftMargin + normalized * beforeWidth
}

const getXPositionAfter = (minute: number): number => {
  const normalized = minute / 90
  const t0Pos = svgMargins.value.t0
  const rightMargin = svgMargins.value.right
  const afterWidth = rightMargin - t0Pos
  return t0Pos + normalized * afterWidth
}

const bestMomentX = computed(() => {
  const t0Pos = svgMargins.value.t0
  if (props.meilleurMoment <= 0) return t0Pos
  const minuteOffset = -props.meilleurMoment
  return getXPositionBefore(minuteOffset)
})

const beforePointsString = computed(() => {
  if (!props.atrTimelineBefore || props.atrTimelineBefore.length === 0) return ''
  const points = props.atrTimelineBefore.map((atr, idx) => {
    const minute = -30 + idx
    const x = getXPositionBefore(minute)
    const y = mapAtrToY(atr)
    return `${x},${y}`
  })
  if (props.atrTimelineAfter && props.atrTimelineAfter.length > 0) {
    const firstAfter = props.atrTimelineAfter[0]
    const x = getXPositionAfter(0)
    const y = mapAtrToY(firstAfter)
    points.push(`${x},${y}`)
  }
  return points.join(' ')
})

const afterPointsString = computed(() => {
  if (!props.atrTimelineAfter || props.atrTimelineAfter.length === 0) return ''
  return props.atrTimelineAfter.map((atr, idx) => {
    const x = getXPositionAfter(idx)
    const y = mapAtrToY(atr)
    return `${x},${y}`
  }).join(' ')
})

const curvePathBefore = computed(() => {
  if (!props.atrTimelineBefore || props.atrTimelineBefore.length === 0) return ''
  const points = props.atrTimelineBefore.map((atr, idx) => {
    const minute = -30 + idx
    const x = getXPositionBefore(minute)
    const y = mapAtrToY(atr)
    return `${x},${y}`
  })
  if (props.atrTimelineAfter && props.atrTimelineAfter.length > 0) {
    const firstAfter = props.atrTimelineAfter[0]
    const x = getXPositionAfter(0)
    const y = mapAtrToY(firstAfter)
    points.push(`${x},${y}`)
  }
  points.push(`${getXPositionAfter(0)},${yAxisBaseline}`)
  points.push(`${getXPositionBefore(-30)},${yAxisBaseline}`)
  return `M${points.map(p => p).join('L')}Z`
})

const curvePathAfter = computed(() => {
  if (!props.atrTimelineAfter || props.atrTimelineAfter.length === 0) return ''
  const points = props.atrTimelineAfter.map((atr, idx) => {
    const x = getXPositionAfter(idx)
    const y = mapAtrToY(atr)
    return `${x},${y}`
  })
  points.push(`${getXPositionAfter(90)},${yAxisBaseline}`)
  points.push(`${getXPositionAfter(0)},${yAxisBaseline}`)
  return `M${points.map(p => p).join('L')}Z`
})

const noiseQualityAfter = computed(() => {
  return props.noiseRatioAfter < 1.5 ? 'clean' : props.noiseRatioAfter < 2.5 ? 'mixed' : 'choppy'
})
</script>

<style scoped>
.graph-section {
  background: #161b22;
  padding: 15px;
  border-radius: 8px;
  border: 1px solid #30363d;
  flex-shrink: 1;
  height: auto;
  min-height: 0;
}

.graph-section h3 {
  margin: 0;
  color: #58a6ff;
  font-size: 1em;
  white-space: nowrap;
}

.quick-conclusion {
  font-size: 0.85em;
  font-weight: 600;
  margin: 0;
  white-space: nowrap;
}

.quick-conclusion .conclusion-positive {
  color: #3fb950;
}

.quick-conclusion .conclusion-negative {
  color: #f85149;
}

.graph-container {
  width: 100%;
  background: #0d1117;
  border-radius: 6px;
  padding: 10px;
  aspect-ratio: 2 / 1;
  min-height: 350px;
  max-height: 500px;
  border: 1px solid #30363d;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 12px;
}

.graph {
  width: 100%;
  height: 100%;
}

.btn-archive {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 6px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 0.9em;
  flex-shrink: 0;
}

.btn-archive:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.btn-archive:active {
  transform: translateY(0);
}

@media (max-width: 1024px) {
  .graph-container {
    min-height: 320px;
    padding: 8px;
  }
}

@media (max-width: 768px) {
  .graph-section {
    padding: 12px;
  }

  .graph-section h3 {
    font-size: 0.9em;
  }

  .graph-container {
    aspect-ratio: 1.5 / 1;
    min-height: 280px;
    max-height: 400px;
    padding: 6px;
    margin-bottom: 10px;
  }

  .btn-archive {
    padding: 6px 12px;
    font-size: 0.8em;
  }
}

@media (max-width: 480px) {
  .graph-section {
    padding: 10px;
  }

  .graph-section h3 {
    font-size: 0.8em;
  }

  .graph-container {
    aspect-ratio: 4 / 3;
    min-height: 240px;
    max-height: 300px;
    padding: 4px;
    margin-bottom: 8px;
  }

  .btn-archive {
    padding: 6px 10px;
    font-size: 0.75em;
  }
}
</style>
