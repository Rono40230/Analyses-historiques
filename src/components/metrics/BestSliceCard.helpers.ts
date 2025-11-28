/**
 * Fonctions utilitaires pour BestSliceCard.vue
 * Gère les calculs de couleurs et formatage
 */

/**
 * Détermine la couleur selon le score (0-100)
 */
export function getScoreColor(score: number): string {
  if (score >= 75) return '#22c55e'    // 🟢 Excellent
  if (score >= 60) return '#3b82f6'    // 🔵 Bon
  if (score >= 45) return '#eab308'    // 🟡 Acceptable
  return '#ef4444'                      // 🔴 Faible
}

/**
 * Détermine la couleur selon la fréquence whipsaw (%)
 */
export function getWhipsawColor(whipsawPercent: number): string {
  if (whipsawPercent < 5) return '#22c55e'    // 🟢 Très Bas
  if (whipsawPercent < 10) return '#3b82f6'   // 🔵 Bas
  if (whipsawPercent < 20) return '#eab308'   // 🟡 Modéré
  if (whipsawPercent < 30) return '#f97316'   // 🟠 Élevé
  return '#ef4444'                             // 🔴 Très Élevé
}

/**
 * Calcule le score pondéré par whipsaw
 * Score Adjusted = Score Brut × (1 - whipsaw_frequency)
 */
export function calculateAdjustedScore(
  brutScore: number,
  whipsawFrequencyPercent?: number
): number {
  if (!whipsawFrequencyPercent) return brutScore
  const whipsawFactor = whipsawFrequencyPercent / 100
  return brutScore * (1 - whipsawFactor)
}

/**
 * Helper: construit la clé pour accéder une qualité de mouvement
 */
export function getMovementQualityKey(analysis: any): string {
  if (!analysis?.slice) return ''
  return `${analysis.slice.hour}-${analysis.slice.quarter}`
}
