export const STATUS_TEXTS = {
  unknown: 'Unknown',
  excellent: 'Excellent',
  good: 'Good',
  acceptable: 'Acceptable',
  poor: 'Poor'
}

export const QUALITY_RECOMMENDATIONS = {
  high: 'TRADE ✅',
  medium: 'CAUTION 🔵',
  low: 'CAUTION 🔵',
  avoid: 'AVOID ❌'
}

export const NOISE_STATUS_TEXTS = {
  excellent: '✅ Signal pur',
  good: '🔵 Acceptable',
  poor: '❌ Chaotique'
}

export const DIRECTION_STATUS_TEXTS = {
  strong: '✅ Forte direction',
  good: '🔵 Bonne direction',
  weak: '🟠 Direction faible',
  none: '❌ Peu de direction'
}

interface MetricsLike {
  mean_atr?: number
}

export function getStatusClass(metrics: MetricsLike | null): string {
  if (!metrics) return 'unknown'
  const confidence = metrics.mean_atr
  if (confidence && confidence > 0.0015) return 'excellent'
  if (confidence && confidence > 0.001) return 'good'
  if (confidence && confidence > 0.0005) return 'acceptable'
  return 'poor'
}

export function getNoiseStatus(value: number): string {
  if (value < 2.0) return 'excellent'
  if (value < 3.0) return 'good'
  return 'poor'
}

export function getDirectionStrengthStatus(value: number): string {
  const strengthPercent = value * 100
  if (strengthPercent >= 20) return 'ok'
  if (strengthPercent >= 5) return 'warning'
  return 'low'
}
