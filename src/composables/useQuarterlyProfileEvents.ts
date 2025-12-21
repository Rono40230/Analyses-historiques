import { computed, Ref } from 'vue'
import { eventTranslations } from '../utils/eventTranslations'
import { 
  obtenirMinuteEvenement as obtenirMinuteEvenementUtil, 
} from '../components/charts/quarterlyProfileChartUtils'

export interface EventProps {
  events?: Array<{
    time: string
    name: string
    impact: string
    currency: string
    frequency: number
  }>
  optimalEntry?: number
  hour?: number
  quarter?: number
}

export function useQuarterlyProfileEvents(
  props: EventProps,
  filterMode: Ref<'none' | 'all' | 'peak'>
) {
  const obtenirMinuteEvenement = (timeStr: string) => obtenirMinuteEvenementUtil(timeStr, props.hour, props.quarter)

  const processedEvents = computed(() => {
    if (!props.events || filterMode.value === 'none') return []

    let eventsToProcess = props.events

    if (filterMode.value === 'peak') {
      if (props.optimalEntry !== undefined) {
        eventsToProcess = props.events.filter(e => {
          const eventMinute = obtenirMinuteEvenement(e.time)
          return Math.abs(eventMinute - props.optimalEntry!) <= 5
        })
      } else {
        return []
      }
    }

    const groups = new Map<string, {
      time: string,
      frequency: number,
      impacts: string[],
      names: string[],
      currencies: string[]
    }>()

    for (const event of eventsToProcess) {
      const key = `${event.time}-${event.frequency}`
      if (!groups.has(key)) {
        groups.set(key, {
          time: event.time,
          frequency: event.frequency,
          impacts: [event.impact],
          names: [event.name],
          currencies: [event.currency]
        })
      } else {
        const g = groups.get(key)!
        g.impacts.push(event.impact)
        g.names.push(event.name)
        g.currencies.push(event.currency)
      }
    }

    const displayEvents = Array.from(groups.values()).map(g => {
      let maxImpact = 'Low'
      if (g.impacts.some(i => i.toUpperCase() === 'HIGH')) maxImpact = 'High'
      else if (g.impacts.some(i => i.toUpperCase() === 'MEDIUM')) maxImpact = 'Medium'
      
      const eventList = g.names.map((n, i) => {
        const translation = eventTranslations[n]
        if (translation) {
          return `• ${n} (${translation.fr}) ${translation.flag}`
        }
        return `• ${n} (${g.currencies[i]})`
      }).join('\n')
      const tooltip = `${g.time} [${maxImpact}]\n${eventList}`
      
      return {
        time: g.time,
        frequency: g.frequency,
        impact: maxImpact,
        tooltip: tooltip,
        stackIndex: 0
      }
    })

    displayEvents.sort((a, b) => {
      const timeDiff = a.time.localeCompare(b.time)
      if (timeDiff !== 0) return timeDiff
      return b.frequency - a.frequency
    })

    const timeStack = new Map<string, number>()
    for (const event of displayEvents) {
      const count = timeStack.get(event.time) || 0
      event.stackIndex = count
      timeStack.set(event.time, count + 1)
    }

    return displayEvents
  })

  return {
    processedEvents,
    obtenirMinuteEvenement
  }
}
