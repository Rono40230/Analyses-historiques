<script setup lang="ts">
import { computed } from 'vue'
import { getPointsPerPip } from '../utils/pipConverter'

const props = defineProps<{
  value: number
  unit: string
  decimals?: number
  symbol?: string
}>()

const pointsPerPip = computed(() => {
  if (props.symbol) return getPointsPerPip(props.symbol)
  // Fallback historique si le symbole n'est pas fourni
  if (props.unit === 'pips') return 10
  return 1
})

const pointsValue = computed(() => {
  if (props.unit === 'pips') return props.value * pointsPerPip.value
  return props.value
})

const pipsValue = computed(() => {
  if (props.unit === 'pips') return props.value
  return props.value / pointsPerPip.value
})

const pipsDecimals = computed(() => {
  // On veut au moins assez de décimales pour voir la conversion
  // Si ratio = 10, on veut +1 décimale par rapport aux points
  // Si ratio = 1000, on veut +3 décimales
  const baseDecimals = props.decimals ?? 1
  if (pointsPerPip.value >= 1000) return Math.max(baseDecimals, 3)
  if (pointsPerPip.value >= 10) return Math.max(baseDecimals, 2)
  return baseDecimals
})

const formattedValue = computed(() => {
  const decimals = props.decimals ?? 1
  return props.value.toFixed(decimals)
})

const displayUnit = computed(() => {
  switch (props.unit) {
    case 'pips':
      return 'pips'
    case 'points':
    case 'pts':
      return 'pts'
    case '$':
      return ''
    default:
      return props.unit
  }
})

const prefix = computed(() => {
  return props.unit === '$' ? '$' : ''
})
</script>

<template>
  <span class="unit-display">
    <template v-if="unit === 'pips' || unit === 'pts' || unit === 'points'">
      {{ pointsValue.toFixed(1) }} points <span class="sub-unit">(soit {{ pipsValue.toFixed(pipsDecimals) }} pips)</span>
    </template>
    <template v-else>
      {{ prefix }}{{ formattedValue }} <span v-if="displayUnit" class="unit">{{ displayUnit }}</span>
    </template>
  </span>
</template>

<style scoped>
.unit-display {
  font-variant-numeric: tabular-nums;
  white-space: nowrap;
}
.unit {
  font-size: 0.85em;
  opacity: 0.7;
  margin-left: 2px;
}
.sub-unit {
  font-size: 0.8em;
  opacity: 0.6;
  margin-left: 4px;
  font-style: italic;
}
</style>
