<template>
  <!-- Composant supprimé - Plus utilisé -->
</template>

<script setup lang="ts">
// Composant vide - Opportunité Straddle retirée du design
</script>

<style scoped>
/* Aucun style */
</style>
  const favorable = countFavorableConditions()
  if (favorable >= 4) return 'strong'
  if (favorable >= 3) return 'moderate'
  return 'weak'
}

function getVerdictText() {
  const level = getVerdictLevel()
  if (level === 'strong') return '🟢 OPPORTUNITÉ FORTE'
  if (level === 'moderate') return '🟡 OPPORTUNITÉ MODÉRÉE'
  return '🔴 OPPORTUNITÉ FAIBLE'
}

function getVerdictDescription() {
  const favorable = countFavorableConditions()
  const level = getVerdictLevel()
  
  if (level === 'strong') {
    return `${favorable}/5 conditions favorables détectées. Cette période présente un bon potentiel Straddle.`
  }
  if (level === 'moderate') {
    return `${favorable}/5 conditions favorables détectées. Straddle viable mais avec conditions limitantes.`
  }
  return `${favorable}/5 conditions favorables détectées. Straddle déconseillé sans ajustements.`
}

function getRecommendation() {
  const level = getVerdictLevel()
  const price = getEstimatedPrice()
  const volPercent = (props.analysis.slice.stats.atr_mean / price) * 100
  const bodyRange = props.analysis.slice.stats.body_range_mean
  const duration = props.volatilityDuration?.peak_duration_minutes || 0
  const whipsawFreq = props.whipsawAnalysis?.whipsaw_frequency_percentage || 0
  
  if (level === 'strong') {
    return `Profiter de cette période volatile. Adapter offset et TP/SL à la volatilité observée (${volPercent.toFixed(2)}%).`
  }
  if (level === 'moderate') {
    if (bodyRange < BODY_RANGE_THRESHOLD_MEDIUM && hasSignificantWhipsaws()) {
      return `Possible mais gérer les whipsaws (${whipsawFreq.toFixed(1)}% détectés). Réduire l'offset et utiliser SL strict.`
    }
    if (bodyRange < BODY_RANGE_THRESHOLD_MEDIUM) {
      return `Possible mais gérer le bruit. Réduire l'offset et augmenter la confiance en signal pur.`
    }
    if (duration < DURATION_THRESHOLD_LONG) {
      return `Période trop courte. Envisager plusieurs micro-trades ou augmenter la durée de holding.`
    }
    return `Réduire le risque: offset conservateur, TP/SL serrés, gestion stricte des whipsaws (${whipsawFreq.toFixed(1)}%).`
  }
  if (hasSignificantWhipsaws()) {
    return `Whipsaws détectés (${whipsawFreq.toFixed(1)}%). Attendre une meilleure configuration avec moins de bruit avant de trader.`
  }
  return `Attendre une meilleure configuration. Volatilité insuffisante ou conditions trop bruités pour un Straddle fiable.`
}

function getQualityScore() {
  const key = getMovementQualityKey(props.analysis, props.analysisData)
  return (props.movementQualities[key]?.quality_score || 0).toFixed(1)
}

// Format des valeurs pour affichage
function getAtrPercent(): string {
  const price = getEstimatedPrice()
  const atrPercent = (props.analysis.slice.stats.atr_mean / price) * 100
  return atrPercent.toFixed(2)
}

function getRangePercent(): string {
  const price = getEstimatedPrice()
  const rangePercent = (props.analysis.slice.stats.range_mean / price) * 100
  return rangePercent.toFixed(2)
}

// Données réelles du whipsaw
function getWhipsawRealData(): string {
  if (!props.whipsawAnalysis) return 'Données du whipsaw indisponibles'
  const count = props.whipsawAnalysis.whipsaw_count || 0
  const percentage = props.whipsawAnalysis.whipsaw_frequency_percentage || 0
  return `${count} whipsaws détectés (${percentage.toFixed(1)}%)`
}

function getWhipsawPercent(): string {
  if (!props.whipsawAnalysis) return '0.0'
  return (props.whipsawAnalysis.whipsaw_frequency_percentage || 0).toFixed(1)
}

function hasSignificantWhipsaws(): boolean {
  if (!props.whipsawAnalysis) return false
  return props.whipsawAnalysis.whipsaw_frequency_percentage >= 20
}
</script>

<style scoped>
.observations-section {
  margin-top: 20px;
  padding: 20px;
  background: #1a1a2e;
  border: 1px solid #16213e;
  border-radius: 8px;
}

.observations-section h4 {
  margin: 0 0 15px 0;
  font-size: 14px;
  font-weight: 600;
  color: #e0e7ff;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

/* Verdict Box */
.verdict-box {
  padding: 15px;
  border-radius: 6px;
  margin-bottom: 15px;
  border-left: 4px solid;
}

.verdict-box.verdict-strong {
  background: rgba(76, 175, 80, 0.1);
  border-left-color: #4caf50;
}

.verdict-box.verdict-moderate {
  background: rgba(255, 193, 7, 0.1);
  border-left-color: #ffc107;
}

.verdict-box.verdict-weak {
  background: rgba(244, 67, 54, 0.1);
  border-left-color: #f44336;
}

.verdict-title {
  font-size: 15px;
  font-weight: bold;
  color: #fff;
  margin-bottom: 5px;
}

.verdict-description {
  font-size: 12px;
  color: #cbd5e0;
  line-height: 1.4;
}

/* Conditions Groups */
.conditions-group {
  margin-bottom: 15px;
}

.conditions-group h5 {
  margin: 0 0 8px 0;
  font-size: 12px;
  font-weight: 600;
  color: #e0e7ff;
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.conditions-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.conditions-list li {
  padding: 6px 0 6px 20px;
  font-size: 12px;
  color: #cbd5e0;
  line-height: 1.4;
  position: relative;
}

.conditions-list li:before {
  content: '•';
  position: absolute;
  left: 0;
}

.no-conditions {
  padding: 6px 0;
  font-size: 12px;
  color: #888;
  font-style: italic;
}

/* Recommendation Box */
.recommendation-box {
  padding: 12px;
  border-radius: 6px;
  font-size: 12px;
  line-height: 1.5;
  border-left: 3px solid;
}

.recommendation-box.recommendation-strong {
  background: rgba(76, 175, 80, 0.1);
  border-left-color: #4caf50;
  color: #a8d5a8;
}

.recommendation-box.recommendation-moderate {
  background: rgba(255, 193, 7, 0.1);
  border-left-color: #ffc107;
  color: #ffd666;
}

.recommendation-box.recommendation-weak {
  background: rgba(244, 67, 54, 0.1);
  border-left-color: #f44336;
  color: #ef9a9a;
}
</style>
