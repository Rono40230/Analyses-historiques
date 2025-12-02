/**
 * Composable pour les calculs de formatage (extraction de useMetricsFormatting)
 */

export function useMetricsCalculations() {
  const formatNumber = (value: number, decimals: number): string => {
    if (value === undefined || value === null) return '0'
    return value.toFixed(decimals)
  }

  const calculateExactTime = (timeRange: string, offset: number): string => {
    if (!timeRange) return '-'
    
    const parts = timeRange.split('-')
    const startTimeStr = parts[0]
    const endTimeStr = parts[1] || startTimeStr
    
    const startMatch = startTimeStr.match(/(\d+):(\d+)/)
    const endMatch = endTimeStr.match(/(\d+):(\d+)/)
    
    if (!startMatch || !endMatch) return timeRange

    let startHours = parseInt(startMatch[1], 10)
    let startMinutes = parseInt(startMatch[2], 10)
    let endHours = parseInt(endMatch[1], 10)
    let endMinutes = parseInt(endMatch[2], 10)
    
    let resultHours = startHours
    let resultMinutes = startMinutes + offset

    while (resultMinutes < 0) { resultMinutes += 60; resultHours -= 1 }
    while (resultMinutes >= 60) { resultMinutes -= 60; resultHours += 1 }
    if (resultHours < 0) resultHours += 24
    if (resultHours >= 24) resultHours -= 24

    const resultTotalMinutes = resultHours * 60 + resultMinutes
    const endTotalMinutes = endHours * 60 + endMinutes
    
    if (resultTotalMinutes > endTotalMinutes) {
      resultHours = endHours
      resultMinutes = endMinutes
    }

    return `${resultHours.toString().padStart(2, '0')}h${resultMinutes.toString().padStart(2, '0')}`
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

  const getImbalanceClass = (value: number): string => {
    const distance = Math.abs(value - 1.0)
    if (distance > 1.0) return 'excellent'
    if (distance > 0.5) return 'good'
    return 'poor'
  }

  const getImbalanceStatus = (value: number): string => {
    return Math.abs(value - 1.0) > 1.0 ? 'ok' : 'low'
  }

  const getDirectionStrengthClass = (value: number): string => {
    const strengthPercent = value * 100
    if (strengthPercent >= 20) return 'excellent'
    if (strengthPercent >= 10) return 'good'
    if (strengthPercent >= 5) return 'acceptable'
    return 'poor'
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

  interface AnalysisLike {
    slice?: { stats?: { events?: Array<{ event_name: string }> } }
  }

  interface AnalysisDataLike {
    symbol?: string
  }

  const getMovementQualityKey = (analysis: AnalysisLike, analysisData: AnalysisDataLike): string => {
    if (!analysisData || !analysis.slice?.stats?.events || analysis.slice.stats.events.length === 0) return ''
    const symbol = analysisData.symbol || 'UNKNOWN'
    const eventName = analysis.slice.stats.events[0].event_name
    return `${symbol}_${eventName}`
  }

  return {
    formatNumber,
    calculateExactTime,
    getMetricClass,
    getMetricStatus,
    getMetricStatusText,
    getImbalanceClass,
    getImbalanceStatus,
    getDirectionStrengthClass,
    getScoreSeverity,
    getRankClass,
    getQualityScoreClass,
    getMovementQualityKey
  }
}
