/**
 * Composable pour le formatage de l'affichage des statuts
 * Les calculs sont dans useMetricsCalculations.ts
 */

import { 
  STATUS_TEXTS, QUALITY_RECOMMENDATIONS, NOISE_STATUS_TEXTS, 
  DIRECTION_STATUS_TEXTS, getStatusClass, getNoiseStatus, 
  getDirectionStrengthStatus 
} from './metricsStatusHelpers'
import { useMetricsCalculations } from './useMetricsCalculations'

export function useMetricsFormatting() {
  const { formatNumber, calculateExactTime, getScoreSeverity, getQualityRecommendation } = useMetricsCalculations()

  const getStatusText = (metrics: any): string => {
    const classe = getStatusClass(metrics)
    return STATUS_TEXTS[classe as keyof typeof STATUS_TEXTS] || STATUS_TEXTS.unknown
  }

  const getNoiseStatusText = (value: number): string => {
    const status = getNoiseStatus(value)
    return NOISE_STATUS_TEXTS[status as keyof typeof NOISE_STATUS_TEXTS]
  }

  const getImbalanceStatusText = (value: number): string => {
    const distance = Math.abs(value - 1.0)
    if (distance > 1.0) return '✅ Tendance marquee'
    if (distance > 0.5) return '🔵 Moderee'
    return '❌ Equilibree'
  }

  const getDirectionStrengthStatusText = (value: number): string => {
    const strengthPercent = value * 100
    if (strengthPercent >= 20) return DIRECTION_STATUS_TEXTS.strong
    if (strengthPercent >= 10) return DIRECTION_STATUS_TEXTS.good
    if (strengthPercent >= 5) return DIRECTION_STATUS_TEXTS.weak
    return DIRECTION_STATUS_TEXTS.none
  }

  return {
    formatNumber,
    calculateExactTime,
    getStatusClass,
    getStatusText,
    getNoiseStatus,
    getNoiseStatusText,
    getImbalanceStatusText,
    getDirectionStrengthStatus,
    getDirectionStrengthStatusText,
    getScoreSeverity,
    getQualityRecommendation
  }
}

