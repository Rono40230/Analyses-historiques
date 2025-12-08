import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface RetroGraphData {
  atr5_timeline: number[]      // ATR5 glissant sur 91 points (T0 → T+90)
  peak_minute: number          // Minute où ATR5 atteint le max
  peak_atr5: number            // Valeur max d'ATR5
  mean_atr5: number            // Moyenne d'ATR5
  std_atr5: number             // Écart-type d'ATR5
  event_count: number
  event_type: string
  pair: string
}

export function useRetroAnalysisGraphData() {
  const graphData = ref<RetroGraphData | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function chargerDonnéesGraph(pair: string, eventType: string) {
    loading.value = true
    error.value = null
    try {
      graphData.value = await invoke<RetroGraphData>('analyze_volatility_profile', {
        pair,
        eventType
      })
    } catch (e) {
      error.value = typeof e === 'string' ? e : (e instanceof Error ? e.message : 'Erreur inconnue')
      graphData.value = null
    } finally {
      loading.value = false
    }
  }

  return {
    graphData,
    loading,
    error,
    chargerDonnéesGraph
  }
}
