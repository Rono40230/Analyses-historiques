// composables/useStraddleAnalysis.ts - Composable pour les calculs Straddle
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface OptimalOffset {
  offset_pips: number
  percentile_95_wicks: number
  with_margin: number
}

export interface WinRateMetric {
  total_trades: number
  wins: number
  losses: number
  whipsaws: number
  win_rate_percentage: number
}

export interface WhipsawMetric {
  total_trades: number
  whipsaw_count: number
  whipsaw_frequency_percentage: number
  risk_level: string
  risk_color: string
}

export interface StraddleMetricsResponse {
  symbol: string
  hour: number
  candle_count: number
  offset_optimal: OptimalOffset
  win_rate: WinRateMetric
  whipsaw: WhipsawMetric
}

export function useStraddleAnalysis() {
  const isLoading = ref(false)
  const offsetOptimal = ref<OptimalOffset | null>(null)
  const winRate = ref<WinRateMetric | null>(null)
  const whipsawAnalysis = ref<WhipsawMetric | null>(null)
  const error = ref<string | null>(null)

  /**
   * Charge les VRAIES candles d'une heure depuis la DB
   */
  const loadCandlesForHour = async (
    symbol: string,
    dateStr: string,
    hour: number
  ): Promise<any[]> => {
    try {
      const response = await invoke<any>('get_candles_for_hour', {
        symbol,
        dateStr, // Tauri convertit automatiquement en camelCase
        hour,
      })
      console.log(`✅ Chargé ${response.candle_count} candles pour ${symbol} ${dateStr} heure ${hour}`)
      return response.candles || []
    } catch (err) {
      console.warn('⚠️ Impossible de charger les candles:', err)
      return []
    }
  }

  /**
   * Analyse complète des métriques Straddle avec VRAIES données
   * Appelle la command Tauri qui combine les 3 calculateurs
   */
  const analyzeStraddleMetrics = async (
    symbol: string,
    dateStr: string,
    hour: number
  ) => {
    try {
      isLoading.value = true
      error.value = null

      // 1. Charger les VRAIES candles depuis la DB
      const candles = await loadCandlesForHour(symbol, dateStr, hour)

      if (candles.length === 0) {
        console.warn('⚠️ Pas de candles trouvées - utilisation de valeurs par défaut')
        offsetOptimal.value = {
          offset_pips: 0,
          percentile_95_wicks: 0,
          with_margin: 0,
        }
        winRate.value = {
          total_trades: 0,
          wins: 0,
          losses: 0,
          whipsaws: 0,
          win_rate_percentage: 0,
        }
        whipsawAnalysis.value = {
          total_trades: 0,
          whipsaw_count: 0,
          whipsaw_frequency_percentage: 0,
          risk_level: 'N/A',
          risk_color: '#6b7280',
        }
        return null
      }

      // 2. Appeler la command Tauri avec les VRAIES candles
      const result = await invoke<StraddleMetricsResponse>('analyze_straddle_metrics', {
        symbol,
        hour,
        candles,
      })

      // 3. Extraire chaque métrique
      offsetOptimal.value = result.offset_optimal
      winRate.value = result.win_rate
      whipsawAnalysis.value = result.whipsaw

      console.log('✅ TÂCHE 5 - Analyse Straddle COMPLÈTE avec VRAIES données:')
      console.log('   - Offset optimal:', offsetOptimal.value.offset_pips, 'pips')
      console.log('   - Win Rate:', winRate.value.win_rate_percentage.toFixed(1), '%')
      console.log('   - Whipsaw:', whipsawAnalysis.value.whipsaw_frequency_percentage.toFixed(1), '%')

      return result
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err)
      console.error('❌ Erreur analyse Straddle:', error.value)
      return null
    } finally {
      isLoading.value = false
    }
  }

  // Computed pour les couleurs
  const winRateColor = computed(() => {
    if (!winRate.value) return '#6b7280'
    if (winRate.value.win_rate_percentage >= 60) return '#22c55e' // Vert
    if (winRate.value.win_rate_percentage >= 40) return '#eab308' // Jaune
    return '#ef4444' // Rouge
  })

  return {
    // État
    isLoading,
    offsetOptimal,
    winRate,
    whipsawAnalysis,
    error,

    // Méthodes
    analyzeStraddleMetrics,
    loadCandlesForHour,

    // Computed
    winRateColor,
  }
}
