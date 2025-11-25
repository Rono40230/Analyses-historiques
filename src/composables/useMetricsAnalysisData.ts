import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { AnalysisResult } from '../stores/volatility'
import type { SliceAnalysis } from '../utils/straddleAnalysis'
import { analyzeTop3Slices, calculateBidiParameters } from '../utils/straddleAnalysis'

interface MovementQuality {
  id?: number | null
  symbol: string
  event_type: string
  directional_move_rate: number
  whipsaw_rate: number
  avg_pips_moved: number
  success_rate: number
  quality_score: number
  sample_size: number
  created_at: number
  updated_at: number
}

interface EntryOffsetMetrics {
  minutes_before_event: number
  sample_count: number
  winning_entries: number
  losing_entries: number
  win_rate: number
  avg_pips_gained: number
  avg_pips_lost: number
  max_pips_gained: number
  max_pips_lost: number
  profit_factor: number
}

export interface VolatilityDuration {
  peak_duration_minutes: number
  volatility_half_life_minutes: number
  recommended_trade_expiration_minutes: number
  confidence_score: number
  sample_size: number
}

export function useMetricsAnalysisData() {
  const analysisData = ref<any>(null)
  const sliceAnalyses = ref<SliceAnalysis[] | null>(null)
  const movementQualities = ref<Record<string, MovementQuality>>({})
  const volatilityDuration = ref<VolatilityDuration | null>(null)
  const tradingPlan = ref<any>(null)
  const entryWindowAnalysis = ref<any>({ optimal_offset: 0, optimal_win_rate: 0 })

  async function loadMovementQuality(symbol: string, eventName: string) {
    try {
      const result = await invoke('get_movement_quality', { symbol, event_type: eventName })
      if (result) {
        movementQualities.value[eventName] = result as MovementQuality
      }
    } catch (err) {
      // Erreur silencieuse
    }
  }

  async function loadEntryWindowAnalysis(symbol: string, eventName: string) {
    try {
      const result = await invoke('get_entry_window_analysis', { symbol, event_type: eventName })
      if (result) {
        entryWindowAnalysis.value = result
      }
    } catch (err) {
      // Erreur silencieuse
    }
  }

  async function updateAnalysis(analysisResult: AnalysisResult) {
    const result = analysisResult
    analysisData.value = {
      globalMetrics: result.global_metrics,
      symbol: result.symbol,
      confidence: Math.round(result.confidence_score),
      strategy: 'SCALPING STANDARD',
      bestHours: result.best_hours.slice(0, 3).join(', ')
    }

    if (result.stats_15min && result.stats_15min.length > 0) {
      sliceAnalyses.value = analyzeTop3Slices(result.stats_15min)

      if (sliceAnalyses.value && sliceAnalyses.value.length > 0) {
        const bestSlice = sliceAnalyses.value[0]
        tradingPlan.value = calculateBidiParameters(bestSlice.slice, sliceAnalyses.value.map(a => a.slice))

        try {
          const stats15min = bestSlice.slice.stats
          volatilityDuration.value = await invoke('analyze_volatility_duration', {
            stats: {
              hour: stats15min.hour,
              quarter: stats15min.quarter,
              candle_count: stats15min.candle_count,
              atr_mean: stats15min.atr_mean,
              atr_max: stats15min.atr_max,
              volatility_mean: stats15min.volatility_mean,
              range_mean: stats15min.range_mean,
              body_range_mean: stats15min.body_range_mean,
              shadow_ratio_mean: stats15min.shadow_ratio_mean,
              tick_quality_mean: stats15min.tick_quality_mean,
              volume_imbalance_mean: stats15min.volume_imbalance_mean,
              noise_ratio_mean: stats15min.noise_ratio_mean,
              breakout_percentage: stats15min.breakout_percentage,
              events: stats15min.events || []
            }
          })
        } catch (error) {
          const atr = bestSlice.slice.stats.atr_mean
          const peakDuration = atr > 0.002 ? 120 : atr > 0.0015 ? 150 : atr > 0.001 ? 180 : 240
          const halfLife = atr > 0.002 ? 45 : atr > 0.0015 ? 60 : atr > 0.001 ? 75 : 90
          volatilityDuration.value = {
            peak_duration_minutes: peakDuration,
            volatility_half_life_minutes: halfLife,
            recommended_trade_expiration_minutes: Math.max(peakDuration, halfLife * 2),
            confidence_score: 50,
            sample_size: bestSlice.slice.stats.candle_count
          }
        }

        if (sliceAnalyses.value && sliceAnalyses.value.length > 0) {
          for (const analysis of sliceAnalyses.value) {
            if (analysis.slice.stats.events && analysis.slice.stats.events.length > 0) {
              const eventName = analysis.slice.stats.events[0].event_name
              await loadMovementQuality(result.symbol, eventName)
            }
          }

          const firstEvent = sliceAnalyses.value[0].slice.stats.events[0]
          if (firstEvent) {
            await loadEntryWindowAnalysis(result.symbol, firstEvent.event_name)
          }
        }
      }
    }
  }

  return {
    analysisData,
    sliceAnalyses,
    movementQualities,
    volatilityDuration,
    tradingPlan,
    entryWindowAnalysis,
    updateAnalysis
  }
}
