import { eventTranslations } from '../stores/eventTranslations'

export function getDefaultPairForCurrency(currency: string): string {
  const map: Record<string, string> = {
    'USD': 'EURUSD',
    'EUR': 'EURUSD',
    'GBP': 'GBPUSD',
    'JPY': 'USDJPY',
    'CAD': 'USDCAD',
    'AUD': 'AUDUSD',
    'NZD': 'NZDUSD',
    'CHF': 'USDCHF'
  }
  return map[currency] || 'EURUSD'
}

export function getEventDisplayInfo(name: string, currency: string, hasHistory: boolean) {
  const translation = eventTranslations[name]
  
  // Drapeau : soit celui de la traduction, soit déduit de la devise
  let flag = translation?.flag
  if (!flag) {
    const currencyFlags: Record<string, string> = {
      'USD': '🇺🇸', 'EUR': '🇪🇺', 'GBP': '🇬🇧', 'JPY': '🇯🇵', 
      'CAD': '🇨🇦', 'AUD': '🇦🇺', 'NZD': '🇳🇿', 'CHF': '🇨🇭', 'CNY': '🇨🇳'
    }
    flag = currencyFlags[currency] || ''
  }

  // Titre
  let title = name
  if (hasHistory && translation?.fr) {
    title = `${name} (${translation.fr})`
  }

  return { title, flag }
}

export function getImpactColor(impact: string): string {
  switch (impact) {
    case 'High': return 'var(--danger-color)'
    case 'Medium': return 'var(--warning-color)'
    default: return 'var(--text-secondary)'
  }
}

export function getConfidenceColor(score: number): string {
  if (score >= 80) return 'var(--success-color)'
  if (score >= 50) return 'var(--warning-color)'
  return 'var(--danger-color)'
}
