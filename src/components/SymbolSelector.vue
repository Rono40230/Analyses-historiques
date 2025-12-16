<template>
  <div class="symbol-selector-container">
    <select 
      id="symbol-select"
      v-model="selectedSymbol" 
      :disabled="loading"
      class="symbol-select"
      @change="onSymbolChange"
    >
      <option value="">
        -- Choisir --
      </option>
      <option 
        v-for="symbol in symbols" 
        :key="symbol.symbol" 
        :value="symbol.symbol"
      >
        {{ symbol.symbol }}
      </option>
    </select>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from 'vue'
import { useVolatilityStore } from '../stores/volatility'
import { useDataRefresh } from '../composables/useDataRefresh'
import { storeToRefs } from 'pinia'

const store = useVolatilityStore()
const { symbols, loading } = storeToRefs(store)
const selectedSymbol = ref('')

const props = defineProps<{
  modelValue?: string
}>()

const { onPairDataRefresh } = useDataRefresh()

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'symbolSelected', value: string): void
}>()

watch(() => props.modelValue, (newVal) => {
  if (newVal !== undefined) {
    selectedSymbol.value = newVal
  }
}, { immediate: true })

onMounted(() => {
  store.loadSymbols()
  
  // S'abonner aux événements de rafraîchissement
  const unsubscribe = onPairDataRefresh(() => {
    store.loadSymbols()
  })
  
  // Se désabonner au démontage
  onBeforeUnmount(unsubscribe)
})

function onSymbolChange() {
  emit('update:modelValue', selectedSymbol.value)
  emit('symbolSelected', selectedSymbol.value)
}
</script>

<style scoped>
.symbol-selector-container {
  width: 100%;
}

.symbol-select {
  width: 100%;
  padding: 0 12px;
  height: 42px;
  font-size: 0.9rem;
  border: 1px solid #ccc;
  border-radius: 6px;
  background: #ffffff;
  color: #000000;
  cursor: pointer;
  transition: all 0.3s;
  box-sizing: border-box;
}

.symbol-select:hover:not(:disabled) {
  border-color: #a0aec0;
}

.symbol-select:focus {
  outline: none;
  border-color: #4299e1;
  box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.5);
}

.symbol-select:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}
</style>
