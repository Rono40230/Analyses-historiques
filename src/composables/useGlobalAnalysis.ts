import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface GlobalAnalysisResult { total_analyses: number; total_days_analyzed: number; global_stats: any; best_pairs: any[]; golden_hours: any[]; tradable_events: any[]; pair_straddle_rates: any[]; optimal_time_windows: any[]; generated_at: string }

export function useGlobalAnalysis() {
  const loading = ref(false)
  const result = ref<GlobalAnalysisResult | null>(null)
  const error = ref<string | null>(null)
  const loadingStep = ref('Initialisation...')
  const progress = ref(0)
  const logs = ref<string[]>([])
  const startDate = ref('')
  const endDate = ref('')
  const selectedPairs = ref<string[]>([])
  const availablePairs = ref<string[]>([
    'EURUSD', 'GBPUSD', 'USDJPY', 'AUDUSD', 'USDCAD', 'USDCHF', 'NZDUSD',
    'EURGBP', 'EURJPY', 'GBPJPY', 'AUDJPY', 'XAUUSD', 'USAIDX', 'DEUIDX'
  ])

  const sortedGoldenHours = computed(() => {
    if (!result.value) return []
    return [...result.value.golden_hours]
      .sort((a, b) => b.reliability - a.reliability)
      .slice(0, 8)
      .sort((a, b) => a.hour - b.hour)
  })

  const bestHour = computed(() => {
    if (!result.value || result.value.golden_hours.length === 0) return '?'
    const best = [...result.value.golden_hours].sort((a, b) => b.reliability - a.reliability)[0]
    return best.hour
  })

  const bestHourReliability = computed(() => {
    if (!result.value || result.value.golden_hours.length === 0) return '0'
    const best = [...result.value.golden_hours].sort((a, b) => b.reliability - a.reliability)[0]
    return best.reliability.toFixed(0)
  })

  const bestPair = computed(() => {
    if (!result.value || result.value.best_pairs.length === 0) return '?'
    return result.value.best_pairs[0].symbol
  })

  function addLog(message: string) {
    logs.value.unshift(message)
    if (logs.value.length > 5) logs.value.pop()
  }

  async function runAnalysis(animate = true) {
    loading.value = true
    error.value = null
    result.value = null
    progress.value = 0
    logs.value = []

    if (animate) {
      const steps = [
        { msg: 'Lecture des archives...', p: 10 },
        { msg: 'Désérialisation des données JSON...', p: 30 },
        { msg: 'Agrégation des métriques de volatilité...', p: 50 },
        { msg: 'Calcul des corrélations croisées...', p: 70 },
        { msg: 'Identification des Golden Hours...', p: 90 },
        { msg: 'Génération du rapport IA...', p: 100 }
      ]

      for (const step of steps) {
        loadingStep.value = step.msg
        addLog(step.msg)
        progress.value = step.p
        await new Promise(resolve => setTimeout(resolve, 300))
      }
    }

    try {
      const filters = {
        start_date: startDate.value || null,
        end_date: endDate.value || null,
        pairs: selectedPairs.value.length > 0 ? selectedPairs.value : null
      }
      const data = await invoke<GlobalAnalysisResult>('analyze_all_archives', { filters })
      result.value = data
    } catch (e: any) {
      error.value = typeof e === 'string' ? e : "Erreur inconnue lors de l'analyse"
    } finally {
      loading.value = false
    }
  }

  return {
    loading,
    result,
    error,
    loadingStep,
    progress,
    logs,
    startDate,
    endDate,
    selectedPairs,
    availablePairs,
    sortedGoldenHours,
    bestHour,
    bestHourReliability,
    bestPair,
    runAnalysis
  }
}
