<script setup lang="ts">
import { computed } from 'vue'

interface EventAction {
  id: string
  offset: number
  tp: number
  sl: number
  offset_simultaneous: number
  tp_simultaneous: number
  sl_simultaneous: number
}

const props = defineProps<{
  event: EventAction
  hasArchive: boolean
}>()

const emit = defineEmits<{
  (e: 'update', id: string, field: string, value: number): void
}>()

const hasSimultaneous = computed(() => 
  props.hasArchive && (props.event.sl_simultaneous || props.event.tp_simultaneous)
)
</script>

<template>
  <div v-if="hasArchive">
    <div class="action-params">
      <div class="param-group">
        <label>Offset</label>
        <div class="input-wrapper">
          <input 
            type="number" 
            :value="event.offset"
            @input="emit('update', event.id, 'offset', Number(($event.target as HTMLInputElement).value))"
            step="0.1"
          >
          <span class="unit">pips</span>
        </div>
      </div>

      <div class="param-group">
        <label>TP</label>
        <div class="input-wrapper">
          <input 
            type="number" 
            :value="event.tp"
            @input="emit('update', event.id, 'tp', Number(($event.target as HTMLInputElement).value))"
            step="1"
          >
          <span class="unit">pts</span>
        </div>
      </div>

      <div class="param-group">
        <label>SL</label>
        <div class="input-wrapper">
          <input 
            type="number" 
            :value="event.sl"
            @input="emit('update', event.id, 'sl', Number(($event.target as HTMLInputElement).value))"
            step="1"
          >
          <span class="unit">pts</span>
        </div>
      </div>
    </div>

    <!-- Section Simultané (si disponible) -->
    <div v-if="hasSimultaneous" class="simultaneous-section">
      <div class="simultaneous-label">Simultané</div>
      <div class="action-params simultaneous-params">
        <div class="param-group">
          <label>TS</label>
          <div class="input-wrapper">
            <span class="static-value">{{ event.tp_simultaneous }}</span>
            <span class="unit">pts</span>
          </div>
        </div>
        <div class="param-group">
          <label>SL</label>
          <div class="input-wrapper">
            <span class="static-value">{{ event.sl_simultaneous }}</span>
            <span class="unit">pts</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.action-params {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 0.5rem;
  background: var(--bg-secondary);
  padding: 0.5rem;
  border-radius: 6px;
}

.param-group {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.param-group label {
  font-size: 0.7rem;
  color: var(--text-secondary);
  text-transform: uppercase;
}

.input-wrapper {
  display: flex;
  align-items: baseline;
  gap: 0.25rem;
}

.input-wrapper input {
  width: 100%;
  background: transparent;
  border: none;
  border-bottom: 1px solid var(--border-color);
  color: var(--text-primary);
  font-weight: 600;
  font-size: 0.9rem;
  padding: 0 0 2px 0;
  text-align: right;
}

.input-wrapper input:focus {
  outline: none;
  border-bottom-color: var(--primary-color);
}

.unit {
  font-size: 0.7rem;
  color: var(--text-secondary);
}

.simultaneous-section {
  margin-top: 8px;
  border-top: 1px solid var(--border-color);
  padding-top: 8px;
}

.simultaneous-label {
  font-size: 0.75em;
  color: var(--text-secondary);
  margin-bottom: 4px;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.simultaneous-params {
  grid-template-columns: repeat(2, 1fr);
  background: rgba(15, 23, 42, 0.5);
}

.static-value {
  font-weight: bold;
  font-size: 0.9rem;
  color: var(--text-primary);
  text-align: right;
  width: 100%;
}
</style>
