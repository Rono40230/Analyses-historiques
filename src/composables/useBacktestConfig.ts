import { ref, watch, onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import { useBacktestStore, StrategyMode, BacktestType } from '../stores/backtest'
import { useVolatilityStore } from '../stores/volatility'
import { invoke } from '@tauri-apps/api/core'
import { eventTranslations } from '../utils/eventTranslations'

export function useBacktestConfig(props: { backtestType: BacktestType }) {
  const backtestStore = useBacktestStore()
  const volatilityStore = useVolatilityStore()
  const { config, mode, loading } = storeToRefs(backtestStore)
  const { symbols } = storeToRefs(volatilityStore)

  const selectedSymbol = ref('')
  const selectedEvent = ref('')
  const selectedTime = ref('15:30')
  const startDate = ref('2020-01-01')
  const endDate = ref(new Date().toISOString().split('T')[0])

  const availableEvents = ref<{name: string, label: string, count: number}[]>([])

  // Watcher pour gérer le mode Simultané (Offset forcé à 0)
  watch(mode, (newMode) => {
    if (newMode === StrategyMode.Simultane) {
      config.value.offset_pips = 0
    }
  })

  // Watcher pour mettre à jour la valeur du point quand le symbole change
  watch(selectedSymbol, async (newSymbol) => {
    if (newSymbol) {
      await backtestStore.mettreAJourProprietesSymbole(newSymbol)
    }
  })

  onMounted(async () => {
    // Charger les types d'événements disponibles
    try {
      const response = await invoke<{ types: { name: string, count: number }[] }>('get_event_types')
      availableEvents.value = response.types.map(e => {
        const translation = eventTranslations[e.name]
        const label = translation 
          ? `${e.name} (${translation.fr}) ${translation.flag}` 
          : e.name
        return { name: e.name, label, count: e.count }
      })
    } catch (e) {
      // Fallback mock data if command not ready
      availableEvents.value = [
        { name: 'Non-Farm Employment Change', label: 'Non-Farm Employment Change', count: 0 },
        { name: 'CPI', label: 'CPI', count: 0 },
        { name: 'Interest Rate Decision', label: 'Interest Rate Decision', count: 0 }
      ]
    }
  })

  async function lancerBacktest() {
    if (!selectedSymbol.value) return

    if (props.backtestType === BacktestType.Event) {
      if (!selectedEvent.value) return
      await backtestStore.runBacktest(selectedSymbol.value, selectedEvent.value)
    } else {
      if (!selectedTime.value || !startDate.value || !endDate.value) return
      await backtestStore.runBacktestTime(
        selectedSymbol.value, 
        selectedTime.value,
        startDate.value,
        endDate.value
      )
    }
  }

  return {
    config,
    mode,
    loading,
    symbols,
    selectedSymbol,
    selectedEvent,
    selectedTime,
    startDate,
    endDate,
    availableEvents,
    lancerBacktest,
    StrategyMode
  }
}
