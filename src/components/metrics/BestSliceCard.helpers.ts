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
 * Génère un conseil de trading global basé sur toutes les métriques
 */
export interface RecommendationData {
  decision: 'TRADER' | 'ATTENDRE' | 'PRUDENT'
  emoji: string
  advice: string
}

export function generateRecommendation(
  adjustedScore: number,
  whipsawFrequency: number,
  adjustedWinRate: number,
  volatilityConfidence?: number
): RecommendationData {
  let decision: 'TRADER' | 'ATTENDRE' | 'PRUDENT' = 'ATTENDRE'
  let emoji = '❌'
  let advice = 'Setup insuffisant. Attendre une meilleure opportunité.'

  // Décision basée sur le Score ajusté (principal indicateur)
  if (adjustedScore >= 75) {
    decision = 'TRADER'
    emoji = '✅'
    advice = 'Straddle optimal. Conditions excellentes pour trader.'
  } else if (adjustedScore >= 60) {
    decision = 'TRADER'
    emoji = '✅'
    advice = 'Setup viable. Conditions suffisantes pour trader avec prudence.'
  } else if (adjustedScore >= 45) {
    decision = 'PRUDENT'
    emoji = '⚠️'
    advice = 'Setup acceptable mais marginal. À considérer avec attention.'
  } else {
    decision = 'ATTENDRE'
    emoji = '❌'
    advice = 'Setup insuffisant. Risques trop élevés. Attendre.'
  }

  // Dégradation si whipsaw très élevé
  if (whipsawFrequency >= 30) {
    if (decision === 'TRADER') decision = 'PRUDENT'
    if (decision === 'PRUDENT') {
      advice = 'Whipsaw très élevé. À éviter dans les conditions actuelles.'
      decision = 'ATTENDRE'
    }
  }

  // Dégradation si winrate ajusté trop faible
  if (adjustedWinRate < 30) {
    advice = 'Taux de gain insuffisant. Risque économique trop élevé.'
    decision = 'ATTENDRE'
  } else if (adjustedWinRate < 40 && decision === 'TRADER') {
    decision = 'PRUDENT'
    advice = 'Taux de gain serré. À trader avec gestion de risque stricte.'
  }

  // Dégradation si confiance volatilité faible
  if (volatilityConfidence !== undefined && volatilityConfidence < 30 && decision === 'TRADER') {
    decision = 'PRUDENT'
    advice = 'Confiance de volatilité faible. Trader avec prudence.'
  }

  return {
    decision,
    emoji,
    advice
  }
}
