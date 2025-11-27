// Color determination functions for BidiParametersSection
// Centralize all color logic in one place

/**
 * Détermine la couleur du Winrate (0-100%)
 */
export function getWinrateColor(winrate: number): string {
  if (winrate >= 65) return '#10b981'   // Green - Excellent
  if (winrate >= 55) return '#3b82f6'   // Blue - Good
  if (winrate >= 50) return '#eab308'   // Orange - Acceptable
  return '#ef4444'                      // Red - Poor
}

/**
 * Détermine la couleur du Stop Loss
 * Basé sur le ratio SL/ATR
 */
export function getSlColor(slPoints: number, atrPoints: number): string {
  if (atrPoints === 0) return '#888888'
  
  const ratio = slPoints / atrPoints
  
  // 🟢 Excellent: SL ≤ 0.8 × ATR (très serré, confiant)
  if (ratio <= 0.8) return '#10b981'
  // 🔵 Good: 0.8 < SL ≤ 1.0 × ATR (normal)
  if (ratio <= 1.0) return '#3b82f6'
  // 🟡 Acceptable: 1.0 < SL ≤ 1.3 × ATR (prudent)
  if (ratio <= 1.3) return '#eab308'
  // 🔴 Poor: SL > 1.3 × ATR (très large, risqué)
  return '#ef4444'
}

/**
 * Détermine la couleur du Trailing Stop coefficient
 * (non utilisée actuellement - TS est calculé optimalement)
 */
export function getTsColor(tsCoefficient: number): string {
  if (tsCoefficient <= 0.8) return '#10b981'   // Tight
  if (tsCoefficient <= 1.0) return '#3b82f6'   // Normal
  if (tsCoefficient <= 1.3) return '#eab308'   // Wide
  return '#ef4444'                             // Very Wide
}

/**
 * Détermine la couleur de l'offset du Meilleur Moment
 * (en minutes d'offset)
 */
export function getOffsetColor(offset: number): string {
  if (offset === 0) return '#10b981'           // Exact moment
  if (offset <= 2) return '#3b82f6'            // Very close
  if (offset <= 5) return '#eab308'            // Acceptable delay
  return '#ef4444'                             // Too far
}

/**
 * Détermine la couleur du Timeout (durée en minutes)
 * (non utilisée actuellement - Timeout est calculé simplement)
 */
export function getTimeoutColor(durationMinutes: number): string {
  if (durationMinutes >= 30) return '#10b981'  // Long duration
  if (durationMinutes >= 20) return '#3b82f6'  // Medium duration
  if (durationMinutes >= 10) return '#eab308'  // Short duration
  return '#ef4444'                             // Very short
}
