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
   * Charge les VRAIES candles filtrées pour un quarter spécifique
   */
  const loadCandlesForQuarter = async (
    symbol: string,
    hour: number,
    quarter: number
  ): Promise<any[]> => {
    try {
      const response = await invoke<any>('get_candles_for_quarter', {
        symbol,
        hour,
        quarter,
      })
      if (response.candle_count > 0) {
      }
      return response.candles || []
    } catch (err) {
      console.error('❌ ERREUR get_candles_for_quarter:', err)
      return []
    }
  }

  /**
   * Analyse complète avec VRAIES candles filtrées pour un quarter
   */
  const analyzeStraddleMetrics = async (
    symbol: string,
    hour: number,
    quarter: number
  ) => {
    try {
      isLoading.value = true
      error.value = null

      // S'assurer que la paire est chargée AVANT de demander les candles
      try {
        await invoke<string>('load_pair_candles', { symbol })
      } catch (preloadErr) {
        console.warn(`⚠️ Préchargement ${symbol} échoué (peut-être déjà chargée):`, preloadErr)
      }

      // Charger les candles filtrées pour ce quarter depuis la DB
      const candles = await loadCandlesForQuarter(symbol, hour, quarter)

      if (candles.length === 0) {
        console.warn('⚠️ Pas de candles pour ce quarter - valeurs par défaut')
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

      // Appeler la command avec les VRAIES candles filtrées
      const result = await invoke<StraddleMetricsResponse>('analyze_straddle_metrics', {
        symbol,
        hour,
        candles,
      })

      offsetOptimal.value = result.offset_optimal
      winRate.value = result.win_rate
      whipsawAnalysis.value = result.whipsaw


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
    loadCandlesForQuarter,

    // Computed
    winRateColor,
  }
}
