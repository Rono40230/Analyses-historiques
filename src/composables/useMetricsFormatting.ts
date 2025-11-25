/**
 * Composable pour le formatage des métriques et l'affichage des statuts
 * Extrait de MetricsAnalysisModal.vue pour réutilisation
 */

import { 
  STATUS_TEXTS, QUALITY_RECOMMENDATIONS, NOISE_STATUS_TEXTS, 
  DIRECTION_STATUS_TEXTS, getStatusClass, getNoiseStatus, 
  getDirectionStrengthStatus 
} from './metricsStatusHelpers'

export function useMetricsFormatting() {

    const formatNumber = (value: number, decimals: number): string => {
        if (value === undefined || value === null) return '0'
        return value.toFixed(decimals)
    }

    const calculateExactTime = (timeRange: string, offset: number): string => {
        if (!timeRange) return '-'
        const startTimeStr = timeRange.split('-')[0]
        const timeMatch = startTimeStr.match(/(\d+):(\d+)/)
        if (!timeMatch) return timeRange

        let hours = parseInt(timeMatch[1], 10)
        let minutes = parseInt(timeMatch[2], 10)
        minutes += offset

        while (minutes < 0) { minutes += 60; hours -= 1 }
        while (minutes >= 60) { minutes -= 60; hours += 1 }
        if (hours < 0) hours += 24
        if (hours >= 24) hours -= 24

        return `${hours.toString().padStart(2, '0')}h${minutes.toString().padStart(2, '0')}`
    }

    const getStatusText = (metrics: any): string => {
        const classe = getStatusClass(metrics)
        return STATUS_TEXTS[classe as keyof typeof STATUS_TEXTS] || STATUS_TEXTS.unknown
    }

    const getMetricClass = (value: number, threshold1: number, threshold2: number): string => {
        if (value >= threshold2) return 'excellent'
        if (value >= threshold1) return 'good'
        return 'poor'
    }

    const getMetricStatus = (value: number, threshold: number): string => {
        return value >= threshold ? 'ok' : 'low'
    }

    const getMetricStatusText = (value: number, threshold: number): string => {
        return value >= threshold ? '✅ OK' : '❌ TOO LOW'
    }

    const getNoiseStatusText = (value: number): string => {
        const status = getNoiseStatus(value)
        return NOISE_STATUS_TEXTS[status as keyof typeof NOISE_STATUS_TEXTS]
    }

    const getImbalanceClass = (value: number): string => {
        const distance = Math.abs(value - 1.0)
        if (distance > 1.0) return 'excellent'
        if (distance > 0.5) return 'good'
        return 'poor'
    }

    const getImbalanceStatus = (value: number): string => {
        return Math.abs(value - 1.0) > 1.0 ? 'ok' : 'low'
    }

    const getImbalanceStatusText = (value: number): string => {
        const distance = Math.abs(value - 1.0)
        if (distance > 1.0) return '✅ Tendance marquee'
        if (distance > 0.5) return '🔵 Moderee'
        return '❌ Equilibree'
    }

    const getDirectionStrengthClass = (value: number): string => {
        const strengthPercent = value * 100
        if (strengthPercent >= 20) return 'excellent'
        if (strengthPercent >= 10) return 'good'
        if (strengthPercent >= 5) return 'acceptable'
        return 'poor'
    }

    const getDirectionStrengthStatusText = (value: number): string => {
        const strengthPercent = value * 100
        if (strengthPercent >= 20) return DIRECTION_STATUS_TEXTS.strong
        if (strengthPercent >= 10) return DIRECTION_STATUS_TEXTS.good
        if (strengthPercent >= 5) return DIRECTION_STATUS_TEXTS.weak
        return DIRECTION_STATUS_TEXTS.none
    }

    const getScoreSeverity = (score: number): string => {
        if (score >= 75) return 'excellent'
        if (score >= 50) return 'good'
        if (score >= 25) return 'acceptable'
        return 'poor'
    }

    const getRankClass = (rank: number): string => `rank-${rank}`

    const getQualityScoreClass = (score: number): string => {
        if (score >= 7.0) return 'high-quality'
        if (score >= 5.0) return 'medium-quality'
        if (score >= 3.0) return 'low-quality'
        return 'avoid-quality'
    }

    const getQualityRecommendation = (score: number): string => {
        if (score >= 7.0) return QUALITY_RECOMMENDATIONS.high
        if (score >= 5.0) return QUALITY_RECOMMENDATIONS.medium
        if (score >= 3.0) return QUALITY_RECOMMENDATIONS.low
        return QUALITY_RECOMMENDATIONS.avoid
    }

    const getMovementQualityKey = (analysis: any, analysisData: any): string => {
        if (!analysisData || !analysis.slice.stats.events || analysis.slice.stats.events.length === 0) return ''
        const symbol = analysisData.symbol || 'UNKNOWN'
        const eventName = analysis.slice.stats.events[0].event_name
        return `${symbol}_${eventName}`
    }

    return {
        formatNumber,
        calculateExactTime,
        getStatusClass,
        getStatusText,
        getMetricClass,
        getMetricStatus,
        getMetricStatusText,
        getNoiseStatus,
        getNoiseStatusText,
        getImbalanceClass,
        getImbalanceStatus,
        getImbalanceStatusText,
        getDirectionStrengthClass,
        getDirectionStrengthStatus,
        getDirectionStrengthStatusText,
        getScoreSeverity,
        getRankClass,
        getQualityScoreClass,
        getQualityRecommendation,
        getMovementQualityKey
    }
}
