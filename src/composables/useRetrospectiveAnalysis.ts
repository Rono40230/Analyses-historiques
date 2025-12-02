import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// Types from Tauri command results
export interface PeakDelayData { peak_delay_minutes: number; peak_atr: number; event_minute: number; confidence: number }
export interface DecayProfileData { peak_atr: number; decay_rate_pips_per_minute: number; decay_speed: string; recommended_timeout_minutes: number }
export interface EntryTimingData { entry_offset_minutes: number; win_rate: number; whipsaw_rate: number; avg_profit_pips: number; sample_size: number; quality_score: number; is_best: boolean }
export interface DirectionalBiasData { up_wins_count: number; down_wins_count: number; whipsaw_count: number; bias_value: number; asymmetry_percent: number; classification: string; confidence_level: string }
export interface WhipsawRootCauseData { early_count: number; early_percentage: number; early_avg_loss_pips: number; late_count: number; late_percentage: number; late_avg_loss_pips: number; total_whipsaws: number; dominant_type: string }

export function useRetrospectiveAnalysis() {
  const peakDelayLoading = ref(false), peakDelayError = ref<string | null>(null), peakDelayResults = ref<PeakDelayData | null>(null)
  const decayLoading = ref(false), decayError = ref<string | null>(null), decayResults = ref<DecayProfileData | null>(null)
  const entryTimingLoading = ref(false), entryTimingError = ref<string | null>(null), entryTimingResults = ref<EntryTimingData[]>([])
  const biasLoading = ref(false), biasError = ref<string | null>(null), biasResults = ref<DirectionalBiasData | null>(null)
  const whipsawLoading = ref(false), whipsawError = ref<string | null>(null), whipsawResults = ref<WhipsawRootCauseData | null>(null)

  const analyzePeakDelay = async (candles: any[]) => {
    peakDelayLoading.value = true; peakDelayError.value = null
    try { peakDelayResults.value = await invoke<PeakDelayData>('analyze_peak_delay', { candles }) }
    catch (e) { peakDelayError.value = String(e); peakDelayResults.value = null }
    finally { peakDelayLoading.value = false }
  }

  const analyzeDecayProfile = async (candles: any[]) => {
    decayLoading.value = true; decayError.value = null
    try { decayResults.value = await invoke<DecayProfileData>('analyze_decay_profile', { candles }) }
    catch (e) { decayError.value = String(e); decayResults.value = null }
    finally { decayLoading.value = false }
  }

  const analyzeEntryTiming = async (win_rates: [number, number, number, number]) => {
    entryTimingLoading.value = true; entryTimingError.value = null
    try { entryTimingResults.value = await invoke<EntryTimingData[]>('analyze_entry_timing', { win_rate_t_minus_10: win_rates[0], win_rate_t_minus_5: win_rates[1], win_rate_t_0: win_rates[2], win_rate_t_plus_3: win_rates[3] }) }
    catch (e) { entryTimingError.value = String(e); entryTimingResults.value = [] }
    finally { entryTimingLoading.value = false }
  }

  const analyzeDirectionalBias = async (up_wins: number, down_wins: number, whipsaws: number) => {
    biasLoading.value = true; biasError.value = null
    try { biasResults.value = await invoke<DirectionalBiasData>('analyze_directional_bias', { up_wins, down_wins, whipsaws }) }
    catch (e) { biasError.value = String(e); biasResults.value = null }
    finally { biasLoading.value = false }
  }

  const analyzeWhipsawRootCause = async (early_whipsaws: number, early_avg_loss: number, late_whipsaws: number, late_avg_loss: number) => {
    whipsawLoading.value = true; whipsawError.value = null
    try { whipsawResults.value = await invoke<WhipsawRootCauseData>('analyze_whipsaw_root_cause', { early_whipsaws, early_avg_loss, late_whipsaws, late_avg_loss }) }
    catch (e) { whipsawError.value = String(e); whipsawResults.value = null }
    finally { whipsawLoading.value = false }
  }

  return { peakDelayLoading, peakDelayError, peakDelayResults, analyzePeakDelay, decayLoading, decayError, decayResults, analyzeDecayProfile, entryTimingLoading, entryTimingError, entryTimingResults, analyzeEntryTiming, biasLoading, biasError, biasResults, analyzeDirectionalBias, whipsawLoading, whipsawError, whipsawResults, analyzeWhipsawRootCause }
}
