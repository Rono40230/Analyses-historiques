<template>
  <div class="stats-section">
    <div class="stat-item">
      <span class="stat-label">Noise Ratio AVANT:</span>
      <span class="stat-value" :class="noiseQualityBefore">{{ noiseRatioBefore.toFixed(2) }}</span>
    </div>
    <div class="stat-item">
      <span class="stat-label">Noise Ratio PENDANT:</span>
      <span class="stat-value" :class="noiseQualityDuring">{{ noiseRatioDuring.toFixed(2) }}</span>
    </div>
    <div class="stat-item">
      <span class="stat-label">Noise Ratio APRÈS:</span>
      <span class="stat-value" :class="noiseQualityAfter">{{ noiseRatioAfter.toFixed(2) }}</span>
    </div>
    <div class="stat-item impact-item">
      <span class="stat-label">Impact Volatilité:</span>
      <span class="stat-value impact-value">+{{ volatilityIncreasePercent.toFixed(1) }}%</span>
    </div>
    <div class="stat-item">
      <span class="stat-label">Occurrences analysées:</span>
      <span class="stat-value">{{ eventCount }}</span>
    </div>
  </div>

  <!-- Section Paramètres Bidi Straddle -->
  <div class="bidi-section">
    <h4>⚙️ PARAMÈTRES BIDI STRADDLE</h4>
    <div class="bidi-grid">
      <div class="bidi-param">
        <div class="bidi-label">Meilleur moment</div>
        <div class="bidi-value">{{ meilleurMoment > 0 ? Math.round(meilleurMoment) : '—' }} <span class="bidi-unit">min avant</span></div>
        <div class="bidi-description">Entrée optimale avant événement</div>
      </div>
      <div class="bidi-param">
        <div class="bidi-label">Stop Loss</div>
        <div class="bidi-value">{{ stopLoss > 0 ? formatPointsWithPips(pair || 'EURUSD', stopLoss, 0) : '—' }}</div>
        <div class="bidi-description">Distance d'arrêt adapté au noise</div>
      </div>
      <div class="bidi-param">
        <div class="bidi-label">Trailing Stop</div>
        <div class="bidi-value">{{ trailingStop > 0 ? formatPointsWithPips(pair || 'EURUSD', trailingStop, 1) : '—' }}</div>
        <div class="bidi-description">Stop dynamique adapté au noise</div>
      </div>
      <div class="bidi-param">
        <div class="bidi-label">Timeout</div>
        <div class="bidi-value">{{ timeout || '60' }} <span class="bidi-unit">min</span></div>
        <div class="bidi-description">Durée maximale du trade</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { formatPointsWithPips } from '../../utils/pipConverter'

interface Props {
  noiseRatioBefore: number
  noiseRatioDuring: number
  noiseRatioAfter: number
  volatilityIncreasePercent: number
  eventCount: number
  meilleurMoment?: number
  stopLoss?: number
  trailingStop?: number
  timeout?: number
  pair: string
}

const props = withDefaults(defineProps<Props>(), {
  noiseRatioBefore: 0,
  noiseRatioDuring: 0,
  noiseRatioAfter: 0,
  volatilityIncreasePercent: 0,
  eventCount: 0,
  meilleurMoment: 0,
  stopLoss: 0,
  trailingStop: 0,
  timeout: 60,
  pair: 'EURUSD'
})

const noiseQualityBefore = computed(() => {
  return props.noiseRatioBefore < 1.5 ? 'clean' : props.noiseRatioBefore < 2.5 ? 'mixed' : 'choppy'
})

const noiseQualityDuring = computed(() => {
  return props.noiseRatioDuring < 1.5 ? 'clean' : props.noiseRatioDuring < 2.5 ? 'mixed' : 'choppy'
})

const noiseQualityAfter = computed(() => {
  return props.noiseRatioAfter < 1.5 ? 'clean' : props.noiseRatioAfter < 2.5 ? 'mixed' : 'choppy'
})
</script>

<style scoped>
.stats-section {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 12px;
  margin-bottom: 10px;
}

.stat-item {
  background: #0d1117;
  padding: 10px 12px;
  border-radius: 6px;
  border: 1px solid #30363d;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.stat-label {
  color: #8b949e;
  font-size: 0.8em;
  font-weight: 500;
  text-transform: uppercase;
}

.stat-value {
  font-weight: 600;
  font-size: 1.1em;
}

.stat-value.clean {
  color: #3fb950;
}

.stat-value.mixed {
  color: #fbbf24;
}

.stat-value.choppy {
  color: #f85149;
}

.impact-item .impact-value {
  color: #58a6ff;
  font-size: 1.15em;
}

/* === SECTION BIDI STRADDLE === */
.bidi-section {
  margin-top: 10px;
  margin-bottom: 0;
  padding: 15px;
  background: linear-gradient(135deg, rgba(45, 90, 123, 0.15) 0%, rgba(78, 205, 196, 0.08) 100%);
  border: 1px solid #2d5a7b;
  border-radius: 8px;
}

.bidi-section h4 {
  margin: 0 0 10px 0;
  font-size: 12px;
  font-weight: 600;
  color: #e0e7ff;
  text-transform: uppercase;
  letter-spacing: 0.4px;
}

.bidi-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(130px, 1fr));
  gap: 10px;
}

.bidi-param {
  padding: 10px;
  background: rgba(30, 30, 48, 0.6);
  border: 1px solid #3a5a78;
  border-radius: 6px;
  transition: all 0.2s;
}

.bidi-param:hover {
  background: rgba(45, 75, 110, 0.7);
  border-color: #4a7aaa;
}

.bidi-label {
  font-size: 0.75em;
  color: #8b949e;
  text-transform: uppercase;
  letter-spacing: 0.3px;
  margin-bottom: 5px;
}

.bidi-value {
  font-size: 1.1em;
  font-weight: 600;
  color: #58a6ff;
  margin-bottom: 3px;
  line-height: 1.2;
}

.bidi-unit {
  font-size: 0.65em;
  color: #6e8a99;
  margin-left: 3px;
}

.bidi-description {
  font-size: 0.7em;
  color: #6e8a99;
  margin-top: 2px;
}

@media (max-width: 1024px) {
  .stats-section {
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 10px;
  }

  .bidi-grid {
    grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
    gap: 8px;
  }

  .bidi-value {
    font-size: 1em;
  }
}

@media (max-width: 768px) {
  .stats-section {
    grid-template-columns: repeat(2, 1fr);
    gap: 8px;
  }

  .stat-item {
    padding: 8px;
  }

  .stat-label {
    font-size: 0.7em;
  }

  .stat-value {
    font-size: 0.95em;
  }

  .bidi-section {
    padding: 12px;
    margin-top: 10px;
  }

  .bidi-section h4 {
    font-size: 11px;
    margin-bottom: 8px;
  }

  .bidi-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: 8px;
  }

  .bidi-param {
    padding: 8px;
  }

  .bidi-label {
    font-size: 0.7em;
  }

  .bidi-value {
    font-size: 0.95em;
  }

  .bidi-description {
    font-size: 0.65em;
  }
}

@media (max-width: 480px) {
  .stats-section {
    grid-template-columns: 1fr;
    gap: 6px;
  }

  .stat-item {
    padding: 6px 8px;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
  }

  .stat-label {
    font-size: 0.65em;
  }

  .stat-value {
    font-size: 0.9em;
  }

  .bidi-section {
    padding: 10px;
    margin-top: 8px;
  }

  .bidi-section h4 {
    font-size: 10px;
    margin-bottom: 8px;
  }

  .bidi-grid {
    grid-template-columns: 1fr;
    gap: 6px;
  }

  .bidi-param {
    padding: 6px;
  }

  .bidi-label {
    font-size: 0.65em;
    margin-bottom: 3px;
  }

  .bidi-value {
    font-size: 0.85em;
    margin-bottom: 2px;
  }

  .bidi-unit {
    font-size: 0.6em;
  }

  .bidi-description {
    font-size: 0.6em;
  }
}
</style>
