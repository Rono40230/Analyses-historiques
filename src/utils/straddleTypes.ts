// Types et interfaces pour l'analyse STRADDLE
import type { Stats15Min } from '../stores/volatility'

export interface Slice15minWithScore {
  hour: number; quarter: number; startTime: string; stats: Stats15Min; straddleScore: number
}

export interface GoldenCombo {
  name: string; description: string; confidence: 'JACKPOT' | 'EXCELLENT' | 'BON' | 'MOYEN' | 'FAIBLE'; winRate: number; avgGainR: number
}

export interface DetectedTrap {
  name: string; description: string; severity: 'CRITIQUE' | 'HAUTE' | 'MOYENNE' | 'BASSE'; metric: string; value: number; threshold: number; recommendation: string
}

export interface TradingPlan {
  entryTime: string; slPips: number; slPoints: number; slUsd: number; tpPips: number; tpPoints: number; tpUsd: number; tpRatio: number
  atrPercentage: number; atrPoints: number; winProbability: number; avgGainR: number; avgLossR: number; tradeExpiration?: number
  trailingStopCoefficient: number; recommendation: string; confidence: number; riskLevel: string; tradeDurationMinutes?: number
}

export interface SliceAnalysis {
  rank: 1 | 2 | 3; slice: Slice15minWithScore; combos: GoldenCombo[]; traps: DetectedTrap[]; tradingPlan: TradingPlan
}

export interface BidiParameters {
  entryTime: string; slPips: number; tpPips: number; winRate: number; avgGain: number; tradeExpiration: number; bestHourReliability: number
}
