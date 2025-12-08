import { computed, ref } from 'vue'

interface RetroGraphDisplayProps {
  atr5Timeline?: number[]
  peakMinute?: number
  peakAtr5?: number
  meanAtr5?: number
  stdAtr5?: number
}

export function useRetroGraphDisplay(props: RetroGraphDisplayProps) {
  // Calculer les stats ATR (min, max, mid)
  const atrStats = computed(() => {
    if (!props.atr5Timeline || props.atr5Timeline.length < 2) {
      return { min: 0, max: 100, mid: 50, minLabel: '0', midLabel: '50', maxLabel: '100' }
    }

    const minAtr = Math.min(...props.atr5Timeline)
    const maxAtr = props.peakAtr5 || Math.max(...props.atr5Timeline)

    return {
      min: minAtr,
      max: maxAtr,
      mid: (minAtr + maxAtr) / 2,
      minLabel: minAtr.toFixed(1),
      midLabel: ((minAtr + maxAtr) / 2).toFixed(1),
      maxLabel: maxAtr.toFixed(1)
    }
  })

  // Calculer les positions Y pour les labels
  const yPositions = computed(() => {
    const yStart = 320  // bottom
    const yEnd = 40    // top
    const graphHeight = yStart - yEnd

    const minAtr = atrStats.value.min
    const maxAtr = atrStats.value.max
    const range = maxAtr - minAtr || 1

    const yMin = yStart
    const yMax = yEnd
    const midAtr = (minAtr + maxAtr) / 2
    const yMid = yStart - ((midAtr - minAtr) / range) * graphHeight

    return { min: yMin, max: yMax, mid: yMid }
  })

  // Calculer la position Y du premier point
  const firstPointYPosition = computed(() => {
    if (!props.atr5Timeline || props.atr5Timeline.length < 1) return yPositions.value.min

    const minAtr = atrStats.value.min
    const maxAtr = atrStats.value.max
    const range = maxAtr - minAtr || 1
    const graphHeight = yPositions.value.min - yPositions.value.max

    const firstAtr = props.atr5Timeline[0]
    const normalized = (firstAtr - minAtr) / range
    const y = yPositions.value.min - normalized * graphHeight

    return y
  })

  // Générer le chemin SVG pour la courbe ATR (commençant au 2e point)
  const atrPathData = computed(() => {
    if (!props.atr5Timeline || props.atr5Timeline.length < 2) return ''

    const maxAtr = atrStats.value.max
    const minAtr = atrStats.value.min
    const range = maxAtr - minAtr || 1

    const xStart = 80
    const yStart = yPositions.value.min
    const graphWidth = 770
    const graphHeight = yPositions.value.min - yPositions.value.max

    // Commencer à l'indice 1 pour ignorer le premier point vide
    const points = props.atr5Timeline.slice(1).map((atr: number, i: number) => {
      const index = i + 1
      const x = xStart + (index / 120) * graphWidth
      const normalized = (atr - minAtr) / range
      const y = yStart - normalized * graphHeight
      return `${x},${y}`
    })

    return `M ${points.join(' L ')}`
  })

  // Path avec remplissage (courbe fermée jusqu'à la baseline, sans le premier point)
  const atrPathDataWithFill = computed(() => {
    if (!props.atr5Timeline || props.atr5Timeline.length < 2) return ''

    const maxAtr = atrStats.value.max
    const minAtr = atrStats.value.min
    const range = maxAtr - minAtr || 1

    const xStart = 80
    const yStart = yPositions.value.min
    const graphWidth = 770
    const graphHeight = yPositions.value.min - yPositions.value.max

    const points = props.atr5Timeline.slice(1).map((atr: number, i: number) => {
      const index = i + 1
      const x = xStart + (index / 120) * graphWidth
      const normalized = (atr - minAtr) / range
      const y = yStart - normalized * graphHeight
      return `${x},${y}`
    })

    const pathUp = points.join(' L ')
    const firstPointIndex = 1
    const firstX = xStart + (firstPointIndex / 120) * graphWidth
    const lastX = xStart + (120 / 120) * graphWidth
    const closeDown = `L ${lastX},${yStart} L ${firstX},${yStart} Z`

    return `M ${pathUp} ${closeDown}`
  })

  // Calculer la position X pour une minute donnée
  function getXPosition(minute: number): number {
    const xStart = 80
    const graphWidth = 770
    const normalizedMinute = minute + 30
    return xStart + (normalizedMinute / 120) * graphWidth
  }

  // Calculer la position X du pic
  const peakXPosition = computed(() => {
    if (props.peakMinute === undefined) return 400
    const xStart = 80
    const graphWidth = 770
    return xStart + (props.peakMinute / 120) * graphWidth
  })

  return {
    atrStats,
    yPositions,
    firstPointYPosition,
    atrPathData,
    atrPathDataWithFill,
    getXPosition,
    peakXPosition
  }
}
