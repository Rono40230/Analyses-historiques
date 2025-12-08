<template>
  <div class="results-container">
    <div class="graph-section full-width graph-large">
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px;">
        <h3>📊 Cycle complet de volatilité (Peak + Décroissance){{ eventLabel ? ' - ' + eventLabel : '' }}</h3>
        <button v-if="!isArchiveMode" class="btn-archive" @click="$emit('archive')">💾 Archiver</button>
      </div>
      <div class="graph-container graph-container-large">
        <svg viewBox="0 0 900 400" class="graph graph-enlarged">
          <defs>
            <linearGradient id="peakGradient" x1="0%" y1="0%" x2="0%" y2="100%">
              <stop offset="0%" style="stop-color:#58a6ff;stop-opacity:0.3" />
              <stop offset="100%" style="stop-color:#58a6ff;stop-opacity:0" />
            </linearGradient>
            <linearGradient id="decayGradient" x1="0%" y1="0%" x2="0%" y2="100%">
              <stop offset="0%" style="stop-color:#f85149;stop-opacity:0.2" />
              <stop offset="100%" style="stop-color:#f85149;stop-opacity:0" />
            </linearGradient>
            <!-- Masque pour cacher la zone T-30 à T-29 -->
            <mask id="graphMask">
              <rect x="0" y="0" width="900" height="400" fill="white" />
              <!-- Cacher la zone vide de T-30 (x=80) à T-29 (x ≈ 86) -->
              <rect x="0" y="0" width="100" height="400" fill="black" />
            </mask>
          </defs>
          <!-- Axes -->
          <line x1="80" :y1="firstPointYPosition" x2="850" :y2="firstPointYPosition" stroke="#4a5568" stroke-width="2" />
          <line x1="80" :y1="yPositions.max" x2="80" :y2="yPositions.min" stroke="#4a5568" stroke-width="2" />
          
          <!-- Ligne de référence au niveau minAtr (baseline réelle) -->
          <template v-if="atr5Timeline && atr5Timeline.length > 1">
            <line x1="80" :y1="yPositions.min" x2="850" :y2="yPositions.min" stroke="#3a4a5a" stroke-width="1" stroke-dasharray="3,3" opacity="0.7" />
          </template>
          
          <!-- Grille horizontale -->
          <line x1="80" :y1="yPositions.mid" x2="850" :y2="yPositions.mid" stroke="#2d3748" stroke-width="1" stroke-dasharray="5,5" />
          
          <!-- Labels Y (ATR) -->
          <text x="65" :y="yPositions.min + 5" font-size="12" text-anchor="end" fill="#8b949e">{{ atrStats.minLabel }}</text>
          <text x="65" :y="yPositions.mid + 5" font-size="12" text-anchor="end" fill="#8b949e">{{ atrStats.midLabel }}</text>
          <text x="65" :y="yPositions.max - 5" font-size="12" text-anchor="end" fill="#8b949e">{{ atrStats.maxLabel }}</text>
          
          <!-- Graduations X (temps en minutes) -->
          <template v-if="atr5Timeline && atr5Timeline.length > 1">
            <!-- Marques tous les 15 minutes, de T-30 à T+90 -->
            <template v-for="minute in [-30, -15, 0, 15, 30, 45, 60, 75, 90]" :key="`tick-${minute}`">
              <line :x1="getXPosition(minute)" :y1="yPositions.min + 5" :x2="getXPosition(minute)" :y2="yPositions.min + 15" stroke="#4a5568" stroke-width="1.5" />
              <text :x="getXPosition(minute)" :y="yPositions.min + 30" font-size="11" text-anchor="middle" fill="#8b949e">{{ minute < 0 ? `T${minute}` : `T+${minute}` }}</text>
            </template>
          </template>
          
          <!-- Courbe ATR réelle (si données disponibles) -->
          <template v-if="atr5Timeline && atr5Timeline.length > 1">
            <!-- Zone remplie sous la courbe jusqu'à la baseline minAtr -->
            <path :d="atrPathDataWithFill" fill="url(#peakGradient)" opacity="0.15" mask="url(#graphMask)" />
            <!-- Courbe elle-même -->
            <path :d="atrPathData" stroke="#58a6ff" stroke-width="3" fill="none" stroke-linecap="round" mask="url(#graphMask)" />
            <!-- Ligne verticale du pic -->
            <line v-if="peakMinute !== undefined" :x1="peakXPosition" :y1="yPositions.max" :x2="peakXPosition" :y2="yPositions.min" stroke="#f85149" stroke-width="2" stroke-dasharray="4,4" opacity="0.8" mask="url(#graphMask)" />
            <text v-if="peakMinute !== undefined" :x="peakXPosition" :y="yPositions.max - 5" font-size="11" text-anchor="middle" fill="#f85149" font-weight="bold">Peak (T+{{ peakMinute - 30 }})</text>
          </template>
          
          <!-- Fallback: courbes statiques si pas de données -->
          <template v-else>
            <path d="M 110 280 Q 250 140 400 50 L 400 320 L 110 320 Z" fill="url(#peakGradient)" />
            <path d="M 400 50 Q 550 170 700 260 L 700 320 L 400 320 Z" fill="url(#decayGradient)" />
            <path d="M 110 280 Q 250 140 400 50" stroke="#58a6ff" stroke-width="4" fill="none" stroke-linecap="round" />
            <path d="M 400 50 Q 550 170 700 260" stroke="#f85149" stroke-width="4" fill="none" stroke-linecap="round" />
            <circle cx="110" cy="280" r="6" fill="#58a6ff" stroke="#fff" stroke-width="2" />
            <circle cx="400" cy="50" r="6" fill="#f85149" stroke="#fff" stroke-width="2" />
            <circle cx="700" cy="260" r="6" fill="#3fb950" stroke="#fff" stroke-width="2" />
            <line x1="400" y1="50" x2="400" y2="340" stroke="#f85149" stroke-width="1" stroke-dasharray="4,4" opacity="0.5" />
            <line x1="700" y1="260" x2="700" y2="340" stroke="#3fb950" stroke-width="1" stroke-dasharray="4,4" opacity="0.5" />
            <text x="110" y="355" font-size="13" text-anchor="middle" fill="#8b949e">T0</text>
            <text x="400" y="355" font-size="13" text-anchor="middle" fill="#f85149" font-weight="bold">T+{{ peakDelay }} min</text>
            <text x="700" y="355" font-size="13" text-anchor="middle" fill="#3fb950" font-weight="bold">T+{{ totalDuration }} min</text>
            <text x="200" y="30" font-size="14" fill="#58a6ff" font-weight="bold">📈 Phase Peak</text>
            <text x="500" y="30" font-size="14" fill="#f85149" font-weight="bold">📉 Phase Décroissance</text>
            <text x="740" y="290" font-size="12" fill="#3fb950">✓ Stabilisé</text>
          </template>
          
          <!-- Infos bas du graphique -->
          <text x="110" y="390" font-size="11" fill="#58a6ff">ATR5 max: {{ peakAtr5 ? Math.round(peakAtr5) : '—' }} points</text>
          <text x="700" y="390" font-size="11" fill="#3fb950">Decay: {{ decaySpeed }}</text>
        </svg>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { formatPointsWithPips } from '../utils/pipConverter'
import { useRetroGraphDisplay } from '../composables/useRetroGraphDisplay'

interface Props {
  peakDelay: number
  decayTimeout: number
  peakAtr: number
  decayRate: number
  decaySpeed: string
  confidence: number
  eventCount: number
  isArchiveMode?: boolean
  eventLabel?: string
  atr5Timeline?: number[]
  peakMinute?: number
  peakAtr5?: number
  meanAtr5?: number
  stdAtr5?: number
}

const props = defineProps<Props>()
defineEmits<{ archive: [] }>()

// Utiliser le composable pour la logique du graphique
const { atrStats, yPositions, firstPointYPosition, atrPathData, atrPathDataWithFill, getXPosition, peakXPosition } = useRetroGraphDisplay(props)

const totalDuration = computed(() => props.peakDelay + props.decayTimeout)
const maxExit = computed(() => Math.ceil(props.decayTimeout * 1.5))

</script>

<style scoped>
.results-container { display: flex; flex-direction: column; gap: 20px; width: 100%; height: 100%; overflow-y: auto; flex: 1; }
.graph-section { background: #161b22; padding: 20px; border-radius: 8px; border: 1px solid #30363d; flex-shrink: 0; height: auto; min-height: 550px; }
.graph-section h3 { margin: 0 0 20px 0; color: #58a6ff; font-size: 1.1em; }
.graph-container { width: 100%; background: #0d1117; border-radius: 6px; padding: 15px; min-height: 480px; border: 1px solid #30363d; display: flex; align-items: center; justify-content: center; }
.graph { width: 100%; height: 100%; }
.analysis-grid-2-cols { display: grid; grid-template-columns: repeat(2, 1fr); gap: 15px; width: 100%; flex-shrink: 0; height: auto; min-height: auto; }
.interpretation-block { background: #161b22; padding: 20px; border-radius: 8px; border: 1px solid #30363d; border-left: 3px solid #58a6ff; }
.interpretation-block p { margin: 0 0 15px 0; color: #58a6ff; font-weight: 600; font-size: 1em; }
.interpretation-block ul { margin: 0; padding-left: 20px; list-style: none; }
.interpretation-block li { margin: 10px 0; color: #c9d1d9; line-height: 1.5; font-size: 0.95em; }

.btn-archive { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border: none; padding: 10px 20px; border-radius: 6px; font-weight: 600; cursor: pointer; transition: all 0.2s; font-size: 0.95em; }
.btn-archive:hover { transform: translateY(-2px); box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4); }
.btn-archive:active { transform: translateY(0); }

@media (max-width: 1024px) {
  .graph-container { min-height: 420px; }
  .graph-section { min-height: 500px; }
  .graph-section h3 { font-size: 1em; }
}

@media (max-width: 768px) {
  .analysis-grid-2-cols { grid-template-columns: 1fr; }
  .graph-container { min-height: 380px; padding: 12px; }
  .graph-section { min-height: 450px; padding: 15px; }
  .graph-section h3 { font-size: 0.95em; margin-bottom: 15px; }
}

@media (max-width: 480px) {
  .graph-container { min-height: 300px; padding: 8px; }
  .graph-section { min-height: 350px; padding: 10px; }
  .graph-section h3 { font-size: 0.85em; margin-bottom: 10px; }
  .btn-archive { padding: 8px 15px; font-size: 0.85em; }
}
</style>
