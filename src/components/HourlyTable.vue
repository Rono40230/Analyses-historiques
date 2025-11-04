<template>
  <div v-if="stats.length > 0" class="hourly-table">
    <h3>📅 Statistiques par Heure UTC</h3>
    
    <div class="table-container">
      <table>
        <thead>
          <tr>
            <th>Heure</th>
            <th>Bougies</th>
            <th>ATR Moyen</th>
            <th>Volatilité %</th>
            <th>Body Range %</th>
            <th>Tick Quality</th>
            <th>Noise Ratio</th>
            <th>Breakouts %</th>
            <th>Score Qualité</th>
          </tr>
        </thead>
        <tbody>
          <tr 
            v-for="stat in stats" 
            :key="stat.hour"
            :class="{ 'best-hour': isBestHour(stat.hour) }"
          >
            <td class="hour-cell">
              {{ formatHour(stat.hour) }}
              <span v-if="isBestHour(stat.hour)" class="star">⭐</span>
            </td>
            <td>{{ stat.candle_count }}</td>
            <td>{{ formatNumber(stat.atr_mean, 5) }}</td>
            <td>{{ (stat.volatility_mean * 100).toFixed(2) }}</td>
            <td>{{ stat.body_range_mean.toFixed(1) }}</td>
            <td>{{ formatNumber(stat.tick_quality_mean, 5) }}</td>
            <td>{{ stat.noise_ratio_mean.toFixed(2) }}</td>
            <td>{{ stat.breakout_percentage.toFixed(1) }}</td>
            <td>
              <div class="quality-score" :class="getQualityClass(stat)">
                {{ getQualityScore(stat).toFixed(0) }}
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { HourlyStats } from '../stores/volatility'

const props = defineProps<{
  stats: HourlyStats[]
  bestHours: number[]
}>()

function formatHour(hour: number): string {
  return `${hour.toString().padStart(2, '0')}:00`
}

function formatNumber(num: number, decimals: number): string {
  return num.toFixed(decimals)
}

function isBestHour(hour: number): boolean {
  return props.bestHours.includes(hour)
}

function getQualityScore(stat: HourlyStats): number {
  if (stat.candle_count === 0) return 0
  
  let score = 0
  
  // ATR (25 points)
  if (stat.atr_mean > 0.001) score += 25
  else if (stat.atr_mean > 0.0005) score += 15
  else if (stat.atr_mean > 0.0001) score += 5
  
  // Body Range (25 points)
  if (stat.body_range_mean > 50) score += 25
  else if (stat.body_range_mean > 30) score += 15
  else if (stat.body_range_mean > 10) score += 5
  
  // Tick Quality (20 points)
  if (stat.tick_quality_mean > 0.001) score += 20
  else if (stat.tick_quality_mean > 0.0005) score += 10
  
  // Noise Ratio (20 points)
  if (stat.noise_ratio_mean < 2.0) score += 20
  else if (stat.noise_ratio_mean < 3.0) score += 10
  
  // Volatilité (10 points)
  if (stat.volatility_mean < 0.15) score += 10
  
  return Math.min(score, 100)
}

function getQualityClass(stat: HourlyStats): string {
  const score = getQualityScore(stat)
  if (score >= 80) return 'excellent'
  if (score >= 60) return 'good'
  if (score >= 40) return 'fair'
  return 'poor'
}
</script>

<style scoped>
.hourly-table {
  background: #161b22;
  border-radius: 12px;
  padding: 2rem;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
  border: 1px solid #30363d;
}

.hourly-table h3 {
  margin: 0 0 1.5rem 0;
  color: #e6edf3;
}

.table-container {
  overflow-x: auto;
}

table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.9rem;
}

thead {
  background: linear-gradient(135deg, #1f6feb 0%, #388bfd 100%);
  color: white;
}

th {
  padding: 1rem;
  text-align: left;
  font-weight: bold;
  white-space: nowrap;
}

td {
  padding: 0.75rem 1rem;
  border-bottom: 1px solid #30363d;
  color: #e6edf3;
}

tbody tr:hover {
  background: #0d1117;
}

.best-hour {
  background: #2d2715 !important;
  font-weight: bold;
}

.best-hour:hover {
  background: #3d3715 !important;
}

.hour-cell {
  font-weight: bold;
  color: #58a6ff;
}

.star {
  margin-left: 0.5rem;
}

.quality-score {
  display: inline-block;
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
  font-weight: bold;
  color: white;
  min-width: 40px;
  text-align: center;
}

.quality-score.excellent {
  background: #22c55e;
}

.quality-score.good {
  background: #3b82f6;
}

.quality-score.fair {
  background: #f59e0b;
}

.quality-score.poor {
  background: #ef4444;
}
</style>
