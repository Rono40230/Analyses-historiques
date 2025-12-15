<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  value: number
  unit: string
  decimals?: number
}>()

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
    <template v-if="unit === 'pips'">
      {{ (value * 10).toFixed(0) }} pts <span class="sub-unit">(soit {{ formattedValue }} pips)</span>
    </template>
    <template v-else-if="unit === 'pts' || unit === 'points'">
      {{ formattedValue }} pts <span class="sub-unit">(soit {{ formattedValue }} pips)</span>
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
