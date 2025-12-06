import type { NormalizedArchive, RawArchive } from './useArchiveStatistics'

// ============================================================================
// PARSEURS D'ARCHIVES PAR TYPE
// ============================================================================

/**
 * Parser une archive selon son type
 */
export function parseArchiveByType(raw: RawArchive): NormalizedArchive | NormalizedArchive[] | null {
  const type = raw.archive_type

  if (type === 'Volatilité') {
    return parseVolatilityArchive(raw)
  } else if (type === 'Métriques Rétrospectives') {
    return parseRetrospectiveArchive(raw)
  } else if (type === 'Heatmap') {
    return parseHeatmapArchive(raw)
  }

  return null
}

/**
 * Parser type "Volatilité" (4 archives)
 */
function parseVolatilityArchive(raw: RawArchive): NormalizedArchive | null {
  try {
    const data = JSON.parse(raw.data_json)
    const symbol = data.symbol || data.analysisResult?.symbol

    if (!symbol) return null

    const meanVol = data.global_metrics?.mean_volatility || data.analysisResult?.global_metrics?.mean_volatility || 0

    return {
      id: String(raw.id),
      type: 'Volatilité',
      pair: symbol,
      eventType: 'Non spécifié',
      peakAtr: meanVol * 1.5 || 20,
      peakDelay: 3,
      decayTimeout: 18,
      confidence: data.confidence_score || data.analysisResult?.confidence_score || 0.5,
      timestamp: raw.created_at
    }
  } catch {
    return null
  }
}

/**
 * Parser type "Métriques Rétrospectives" (20 archives)
 */
function parseRetrospectiveArchive(raw: RawArchive): NormalizedArchive | null {
  try {
    const data = JSON.parse(raw.data_json)
    const analysisResult = data.analysisResult || data
    const symbol = analysisResult.symbol || ''

    if (!symbol) return null

    const peakDelay = data.peakDelayResults?.peak_delay_minutes || 3.2
    const decayTimeout = data.decayResults?.recommended_timeout_minutes || 18.5
    const peakAtr = data.peakDelayResults?.peak_atr || 40
    const confidence = (data.peakDelayResults?.confidence || 0.75) * 100

    let eventType = 'Non spécifié'
    if (raw.title) {
      const match = raw.title.match(/NFP|CPI|BCE|FED|Inflation|PPI|BOE/)
      if (match) eventType = match[0]
    }

    return {
      id: String(raw.id),
      type: 'Métriques Rétrospectives',
      pair: symbol,
      eventType,
      peakAtr,
      peakDelay,
      decayTimeout,
      confidence,
      eventCount: data.peakDelayResults?.event_count || 1,
      timestamp: raw.created_at
    }
  } catch {
    return null
  }
}

/**
 * Parser type "Heatmap" (1 archive)
 */
function parseHeatmapArchive(raw: RawArchive): NormalizedArchive[] {
  const results: NormalizedArchive[] = []

  try {
    const data = JSON.parse(raw.data_json)
    const pairs = data.pairs || []
    const events = data.events || []
    const impactMatrix = data.impactMatrix || []
    const volatilityData = data.volatilityData || {}

    for (let i = 0; i < pairs.length; i++) {
      const pair = pairs[i]
      const pairVolData = volatilityData[pair] || {}

      for (let j = 0; j < events.length; j++) {
        const event = events[j]
        const impact = impactMatrix[i]?.[j] || 0
        const eventData = pairVolData[event] || {}

        results.push({
          id: `${raw.id}-${i}-${j}`,
          type: 'Heatmap',
          pair,
          eventType: event,
          peakAtr: eventData.volatility_peak || impact / 10 || 30,
          peakDelay: eventData.avg_peak_time_minutes || 3.2,
          decayTimeout: 18,
          confidence: (impact / 100) * 0.8 + 0.2,
          impactScore: impact,
          timestamp: raw.created_at
        })
      }
    }
  } catch {
    // Retourner tableau vide si parse échoue
  }

  return results
}
