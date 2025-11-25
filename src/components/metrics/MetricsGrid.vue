<template>
  <div class="metrics-section">
    <h4>METRIQUES</h4>
    <div class="metrics-grid">
      <MetricItemCard
        v-for="metric in getMetrics()"
        :key="metric.label"
        :metric="metric"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import MetricItemCard from './MetricItemCard.vue'
import type { SliceAnalysis } from '../../utils/straddleAnalysis'

interface Props {
  analysis: SliceAnalysis
  analysisData: any
}

interface MetricConfig {
  label: string
  value15: number
  valueGlobal: number
  goodThreshold: number
  excellentThreshold: number
  definition: string
  usage: string
  scoring: string
  suffix?: string
  decimals?: number
}

const props = defineProps<Props>()

const getMetrics = computed(() => () => {
  const stats = props.analysis.slice.stats
  const globals = props.analysisData?.globalMetrics || {}

  return [
    {
      label: 'ATR Moyen',
      value15: stats.atr_mean,
      valueGlobal: globals.mean_atr ?? 0,
      goodThreshold: 0.001,
      excellentThreshold: 0.002,
      definition: 'Average True Range sur 14 périodes : mesure de volatilité moyenne du créneau horaire.',
      usage: 'Score >0.002 = Excellent | 0.001-0.002 = Bon | <0.001 = Mauvais.',
      scoring: 'Détermine largeur SL/TP. Plus ATR élevé = plus grande opportunité scalping.',
      decimals: 5
    },
    {
      label: 'True Range',
      value15: stats.range_mean,
      valueGlobal: globals.mean_range ?? 0,
      goodThreshold: 0.0015,
      excellentThreshold: 0.0025,
      definition: 'Max(High-Low, |High-Close[t-1]|, |Low-Close[t-1]|) : mouvement total exploitable.',
      usage: 'Score >2.5% = Excellent | 1.5-2.5% = Bon | <1.5% = Faible.',
      scoring: 'Capture les gaps contrairement au Range. Combine avec ATR pour breakouts.',
      decimals: 5
    },
    {
      label: 'Volatilité %',
      value15: stats.volatility_mean * 100,
      valueGlobal: (globals.mean_volatility ?? 0) * 100,
      goodThreshold: 15,
      excellentThreshold: 30,
      definition: 'Ratio ATR / Close exprimé en pourcentage.',
      usage: '>30% = Exceptionnellement volatil | 15-30% = Bon | <15% = Faible.',
      scoring: 'Formula: (ATR / Close) × 100.',
      suffix: '%',
      decimals: 1
    },
    {
      label: 'Body Range %',
      value15: stats.body_range_mean,
      valueGlobal: globals.mean_body_range ?? 0,
      goodThreshold: 25,
      excellentThreshold: 45,
      definition: 'Pourcentage du range représenté par le body (Close-Open) : pureté du signal.',
      usage: '>45% = Signal Très Pur | 25-45% = Acceptable | <25% = Bruité.',
      scoring: 'Formula: (|Close - Open| / Range) × 100. Corps fort = pression directionnelle claire.',
      suffix: '%',
      decimals: 1
    },
    {
      label: 'Tick Quality',
      value15: stats.tick_quality_mean,
      valueGlobal: globals.mean_tick_quality ?? 0,
      goodThreshold: 0.0005,
      excellentThreshold: 0.001,
      definition: 'Mesure la douceur du pricing : variance des ticks.',
      usage: '>0.001 = Excellent (mouvements lisses) | <0.0005 = Mauvais (bruit).',
      scoring: 'Standard deviation des mouvements de tick.',
      decimals: 5
    },
    {
      label: 'Noise Ratio',
      value15: stats.noise_ratio_mean,
      valueGlobal: globals.mean_noise_ratio ?? 0,
      goodThreshold: 2.0,
      excellentThreshold: 1.5,
      definition: 'Ratio wicks/body : mesure du bruit vs vraie direction.',
      usage: '<2.0 = Signal Excellent | 2.0-2.5 = Acceptable | >2.5 = Très Bruité.',
      scoring: 'Bas = direction confirmée. Élevé = beaucoup de rejets.',
      decimals: 2
    },
    {
      label: 'Direction Strength',
      value15: stats.volume_imbalance_mean * 100,
      valueGlobal: (globals.mean_volume_imbalance ?? 0) * 100,
      goodThreshold: 10,
      excellentThreshold: 20,
      definition: 'Puissance du mouvement directionnel.',
      usage: '>20% = Excellent | 10-20% = Bon | 5-10% = Moyen | <5% = Faible.',
      scoring: 'Combine directionnalité + cassures identifiées.',
      suffix: '%',
      decimals: 1
    },
    {
      label: 'Breakout %',
      value15: stats.breakout_percentage,
      valueGlobal: globals.mean_breakout_percentage ?? 0,
      goodThreshold: 10,
      excellentThreshold: 20,
      definition: 'Pourcentage de cassures de niveaux clés.',
      usage: '>15% = Breakouts Fréquents | <10% = Peu de breakouts (range-bound).',
      scoring: 'Formula: (Breakout_events / Total_periods) × 100.',
      suffix: '%',
      decimals: 1
    }
  ] as MetricConfig[]
})
</script>

<style scoped>
.metrics-section { background: #0d1117; padding: 20px; border-radius: 8px; margin-bottom: 20px; border: 1px solid #30363d; }
.metrics-section h4 { margin: 0 0 15px 0; color: #e2e8f0; }
.metrics-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 15px; }
</style>
