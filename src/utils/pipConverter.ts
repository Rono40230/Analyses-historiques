/**
 * Convertisseur Points MT5 ↔ Pips
 * 
 * MT5 utilise nativement les "points" (unité minimale).
 * Mais les traders parlent en "pips" (unité de cotation).
 * 
 * Tableau de conversion (basé sur norme MT5 officielle):
 * - Forex 5 décimales (EURUSD, GBPUSD, USDCAD, EURJPY, CADJPY, etc): 1 pip = 10 points
 * - Or (XAUUSD, XAUJPY): 1 pip = 10 points
 * - Argent (XAGUSD): 1 pip = 1000 points
 * - Indices (USA500IDXUSD): 1 pip = 1 point
 * - Crypto (BTCUSD, ETHUSD): 1 pip = 1 point
 */

type SymbolType = 'forex' | 'gold' | 'silver' | 'indices' | 'crypto'

/**
 * Déterminer le type de symbole pour obtenir le bon ratio points→pips
 */
function obtenirTypeSymbole(symbol: string): SymbolType {
  if (symbol.includes('XAU')) return 'gold'
  if (symbol.includes('XAG')) return 'silver'
  if (symbol.includes('US30') || symbol.includes('DE30') || symbol.includes('NAS100') || symbol.includes('SPX500') || symbol.includes('USA500') || symbol.includes('DEUIDX') || symbol.includes('USAIDX') || symbol.includes('USATEC')) return 'indices'
  if (symbol.includes('BTC') || symbol.includes('ETH')) return 'crypto'
  return 'forex'  // Tous les autres: EURUSD, USDJPY, CADJPY, EURJPY, etc
}

/**
 * Obtenir le nombre de points par pip
 * points_per_pip = nombre de points qu'il faut pour faire 1 pip
 */
export function obtenirPointsParPip(symbol: string): number {
  const type = obtenirTypeSymbole(symbol)
  
  switch (type) {
    case 'gold':     return 10        // 1 pip = 10 points
    case 'silver':   return 1000      // 1 pip = 1000 points
    case 'indices':  return 1         // 1 pip = 1 point
    case 'crypto':   return 1         // 1 pip = 1 point
    default:         return 10        // Forex (tous): 1 pip = 10 points
  }
}

/**
 * Obtenir la pip_value pour un symbole (valeur minimale de variation)
 * Conservé pour compatibilité, mais utilise pointsPerPip en interne
 */
export function obtenirValeurPip(symbol: string): number {
  // Retourne une valeur arbitraire pour compatibilité
  // La vraie conversion se fait via obtenirPointsParPip
  return obtenirPointsParPip(symbol)
}

/**
 * Convertir les points MT5 en pips
 * Formula: pips = points / points_per_pip
 * 
 * @param symbol Symbole de trading (ex: "EURUSD", "BTCUSD", "CADJPY")
 * @param points Valeur en points MT5
 * @returns Valeur en pips
 * 
 * Exemples:
 * - EURUSD: 100 points ÷ 10 = 10 pips
 * - CADJPY: 10 points ÷ 10 = 1 pip
 * - XAGUSD: 1000 points ÷ 1000 = 1 pip
 * - BTCUSD: 100 points ÷ 1 = 100 pips
 */
export function convertirPointsEnPips(symbol: string, points: number): number {
  const pointsPerPip = obtenirPointsParPip(symbol)
  return points / pointsPerPip
}

/**
 * Convertir les pips en points MT5
 * Formula: points = pips × points_per_pip
 * 
 * @param symbol Symbole de trading
 * @param pips Valeur en pips
 * @returns Valeur en points MT5
 */
export function convertirPipsEnPoints(symbol: string, pips: number): number {
  const pointsPerPip = obtenirPointsParPip(symbol)
  return pips * pointsPerPip
}

/**
 * Formater une valeur normalisée (Pips/Points) pour affichage
 * 
 * @param symbol Symbole de trading
 * @param normalizedValue Valeur déjà normalisée (1.0 = 1 pip ou 1 point)
 * @param _decimals Non utilisé (gardé pour compatibilité)
 * @returns Chaîne formatée
 */
export function formaterPointsAvecPips(symbol: string, normalizedValue: number | undefined, _decimals = 2): string {
  // Gérer les valeurs undefined ou NaN
  if (normalizedValue === undefined || normalizedValue === null || isNaN(normalizedValue)) {
    return 'N/A'
  }
  
  const pointsPerPip = obtenirPointsParPip(symbol)
  
  if (pointsPerPip === 1) {
    return `${normalizedValue.toFixed(1)} points (soit ${normalizedValue.toFixed(1)} pips)`
  }
  
  // Pour le Forex (1 pip = 10 points), normalizedValue est en pips
  const points = normalizedValue * 10
  return `${points.toFixed(1)} points (soit ${normalizedValue.toFixed(1)} pips)`
}

/**
 * Obtenir uniquement la conversion pips pour affichage court
 * Exemple: "15 pips"
 * 
 * @param symbol Symbole de trading
 * @param points Valeur en points
 * @param decimals Nombre de décimales (défaut: 0)
 * @returns Chaîne formatée
 */
export function formaterEnPips(symbol: string, points: number, decimals = 0): string {
  const pips = convertirPointsEnPips(symbol, points)
  return `${pips.toFixed(decimals)} pips`
}

/**
 * Déterminer si le symbole utilise une conversion (non-1:1)
 * Retourne true si pip_value !== 1.0
 */
export function aUneConversion(symbol: string): boolean {
  return obtenirValeurPip(symbol) !== 1.0
}
