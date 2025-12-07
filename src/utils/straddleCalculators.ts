// Calculateurs spécialisés pour l'analyse STRADDLE
import type { Stats15Min } from '../stores/volatility'
import type { GoldenCombo, DetectedTrap, TradingPlan } from './straddleTypes'
import {
  estimatePrice,
  calculateVolatilityMetrics,
  calculateSlCoefficient,
  calculateTsCoefficient,
  calculateRiskMetrics
} from './straddleCalculators.helpers'

/**
 * Calcule la durée optimale du trade basée sur ATR, type d'événement et heure du jour
 */
export function calculateTradeDuration(atrMean: number, eventType: string = 'AUTRE', hourOfDay: number = 12): number {
  let duration = atrMean > 0.005 ? 120 : atrMean > 0.004 ? 150 : atrMean > 0.0025 ? 180 : 240
  const eventFactors: Record<string, number> = { 'nfp': 0.5, 'cpi': 0.7, 'interest rate': 0.8, 'gdp': 1.0, 'jobless claims': 0.6, 'pce': 0.7, 'autre': 1.0 }
  let eventFactor = 1.0
  for (const [key, factor] of Object.entries(eventFactors)) {
    if (eventType.toLowerCase().includes(key)) { eventFactor = factor; break }
  }
  const hourFactor = { 8: 0.8, 13: 0.6, 14: 0.7 }[hourOfDay] ?? 1.0
  return Math.round(duration * eventFactor * hourFactor)
}

export function calculateStraddleScore(slice: Stats15Min, movementQualityScore?: number): number {
  if (slice.candle_count === 0) return 0
  
  const estimatedPriceValue = estimatePrice(slice)
  const metrics = calculateVolatilityMetrics(slice, estimatedPriceValue)
  const atrPercent = metrics.atrPercent
  const rangePercent = metrics.rangePercent
  const bodyRange = metrics.bodyRange
  const noiseRatio = metrics.noiseRatio
  const volumeImbalance = metrics.volumeImbalance
  const breakoutPercent = metrics.breakoutPercent
  
  let score = 0
  
  // Scoring Range (en %)
  if (rangePercent > 2.5) score += 60
  else if (rangePercent > 2.0) score += 50
  else if (rangePercent > 1.5) score += 40
  else if (rangePercent > 1.0) score += 20
  
  // Scoring ATR (en %)
  if (atrPercent > 2.0) score += 25
  else if (atrPercent > 1.5) score += 20
  else if (atrPercent > 1.0) score += 15
  else if (atrPercent > 0.5) score += 8
  
  // Scoring Body Range (pureté du signal)
  if (bodyRange > 45.0) score += 15
  else if (bodyRange > 35.0) score += 12
  else if (bodyRange > 25.0) score += 8
  else if (bodyRange > 15.0) score += 3
  
  // Scoring Noise Ratio (moins c'est bruité, mieux c'est)
  if (noiseRatio < 1.5) score += 10
  else if (noiseRatio < 2.0) score += 8
  else if (noiseRatio < 2.5) score += 5
  
  // Scoring Volume Imbalance (direction strength)
  if (volumeImbalance > 20) score += 10
  else if (volumeImbalance > 15) score += 7
  else if (volumeImbalance > 10) score += 5
  
  // Scoring Breakout
  if (breakoutPercent > 20) score += 5
  else if (breakoutPercent > 15) score += 3
  
  // Pondération qualité du mouvement (optionnel, poids 20%)
  if (movementQualityScore !== undefined) {
    const movementWeight = Math.min(100, Math.max(0, movementQualityScore)) * 0.2
    score = (score * 0.8) + movementWeight
  }
  
  return Math.min(score, 100)
}

export function detectGoldenCombos(slice: Stats15Min): GoldenCombo[] {
  const combos: GoldenCombo[] = []
  if (slice.range_mean > 0.0025 && slice.body_range_mean > 40 && slice.noise_ratio_mean < 2.0) {
    combos.push({
      name: 'Range Pur',
      description: 'Movement directionnel avec range excellent et peu de bruit',
      confidence: 'EXCELLENT',
      winRate: 0.65,
      avgGainR: 0.45
    })
  }
  if (slice.atr_mean > 0.002 && slice.volume_imbalance_mean > 0.2 && slice.breakout_percentage > 15) {
    combos.push({
      name: 'Volatilité Haute + Imbalance',
      description: 'Conditions haute volatilité avec imbalance volume confirmée',
      confidence: 'BON',
      winRate: 0.58,
      avgGainR: 0.35
    })
  }
  return combos
}

export function detectTraps(slice: Stats15Min): DetectedTrap[] {
  const traps: DetectedTrap[] = []
  if (slice.noise_ratio_mean > 3.0) {
    traps.push({
      name: 'Bruit Excessif',
      description: 'Ratio bruit/signal trop élevé, movements chaotiques',
      severity: 'HAUTE',
      metric: 'Noise Ratio',
      value: slice.noise_ratio_mean,
      threshold: 2.5,
      recommendation: 'Augmenter SL de 20-30%, réduire position size'
    })
  }
  if (slice.range_mean < 0.0010) {
    traps.push({
      name: 'Range Insuffisant',
      description: 'Marché trop calme, movement insuffisant pour straddle',
      severity: 'CRITIQUE',
      metric: 'Range',
      value: slice.range_mean,
      threshold: 0.0015,
      recommendation: 'SKIP ce créneau, pas d\'opportunity'
    })
  }
  return traps
}

export function calculateTradingPlan(slice: Stats15Min, estimatedPriceValue: number, confidenceScore: number): TradingPlan {
  const atrPoints = slice.atr_mean
  const metrics = calculateVolatilityMetrics(slice, estimatedPriceValue)
  const atrPercent = metrics.atrPercent
  
  // SL adaptatif basé sur volatilité
  const slCoefficient = calculateSlCoefficient(atrPercent)
  const tpCoefficient = slCoefficient * 1.67
  
  // Distances en points
  const slPoints = atrPoints * slCoefficient
  const tpPoints = atrPoints * tpCoefficient
  
  // USD
  const slUsd = Math.round(slPoints * estimatedPriceValue)
  const tpUsd = Math.round(tpPoints * estimatedPriceValue)
  
  // Risk metrics
  const winProb = confidenceScore / 100
  const riskMetrics = calculateRiskMetrics(tpPoints, slPoints, winProb)
  
  // TS adaptatif
  const tsCoeff = calculateTsCoefficient(atrPercent, tpPoints, slPoints)
  
  // Durée du trade
  const primaryEvent = slice.events?.[0]?.event_name ?? 'AUTRE'
  const tradeDurationMinutes = calculateTradeDuration(atrPoints, primaryEvent, slice.hour)
  
  return {
    entryTime: '—',
    slPips: Math.round(slPoints),
    slPoints: slPoints,
    slUsd: slUsd,
    tpPips: Math.round(tpPoints),
    tpPoints: tpPoints,
    tpUsd: tpUsd,
    tpRatio: tpPoints / slPoints,
    atrPercentage: atrPercent,
    atrPoints,
    winProbability: Math.round(winProb * 100),
    avgGainR: riskMetrics.avgGainR,
    avgLossR: riskMetrics.avgLossR,
    trailingStopCoefficient: tsCoeff,
    recommendation: confidenceScore >= 75 ? 'TRADE' : 'CAUTION',
    confidence: confidenceScore,
    riskLevel: confidenceScore >= 75 ? 'LOW' : confidenceScore >= 50 ? 'MEDIUM' : 'HIGH',
    tradeDurationMinutes
  }
}
