// Helper functions pour straddleCalculators.ts
// Contient les logiques de scoring et calcul détaillés

import type { Stats15Min } from '../stores/volatility'

/**
 * Estime le prix basé sur la moyenne ATR
 */
export function estimatePrice(slice: Stats15Min): number {
  if (slice.atr_mean > 1000) return 100000 // Crypto
  if (slice.atr_mean > 10) return 10000    // Indices
  return 1.0                                // Forex
}

/**
 * Calcule les métriques de volatilité normalisées
 */
export function calculateVolatilityMetrics(slice: Stats15Min, estimatedPrice: number) {
  const atrPoints = slice.atr_mean
  const atrPercent = (atrPoints / estimatedPrice) * 100
  const rangePercent = (slice.range_mean / estimatedPrice) * 100
  
  return {
    atrPoints,
    atrPercent,
    rangePercent,
    bodyRange: slice.body_range_mean,
    noiseRatio: slice.noise_ratio_mean,
    volumeImbalance: slice.volume_imbalance_mean * 100,
    breakoutPercent: slice.breakout_percentage
  }
}

/**
 * Détermine le coefficient de Stop Loss adaptatif
 */
export function calculateSlCoefficient(atrPercent: number): number {
  if (atrPercent < 1.0) return 0.8    // Volatilité très faible
  if (atrPercent < 1.5) return 1.0    // Volatilité normale
  if (atrPercent < 2.5) return 1.2    // Volatilité élevée
  return 1.5                           // Volatilité extrême
}

/**
 * Détermine le coefficient de Trailing Stop adaptatif
 */
export function calculateTsCoefficient(atrPercent: number, tpPoints: number, slPoints: number): number {
  const tsRatio = 0.6 + 0.4 * (atrPercent / 2.0)  // Plage: 0.6 à 1.0
  const tpRatio = tpPoints / slPoints
  return Math.max(tsRatio, tpRatio * 0.95)        // Jamais < TP initial
}

/**
 * Calcule les moyennes gain/loss basées sur les probabilités
 */
export function calculateRiskMetrics(
  tpPoints: number,
  slPoints: number,
  winProb: number
) {
  const avgGainPoints = tpPoints * winProb
  // Loss partielle: 50% du SL est perdu en moyenne (plus réaliste)
  const avgLossPoints = slPoints * (1 - winProb) * 0.5
  
  return {
    avgGainPoints,
    avgLossPoints,
    avgGainR: (avgGainPoints - avgLossPoints) / slPoints,
    avgLossR: Math.max(0, avgLossPoints / slPoints)
  }
}
