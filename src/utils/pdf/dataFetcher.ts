import { invoke } from '@tauri-apps/api/core'
import type { Archive } from '../../stores/archiveStore'
import type { ArchivedAnalysisData } from '../../composables/useMetricsModalLoad'

interface CalendarMetadata {
  id: number
  name: string
  start_date?: string
  end_date?: string
}

export async function fetchArchivedData(pairs: string[], onProgress: (progress: number) => void): Promise<ArchivedAnalysisData[]> {
  const results: ArchivedAnalysisData[] = []
  const calendarId = parseInt(localStorage.getItem('activeCalendarId') || '0', 10)
  
  if (!calendarId) throw new Error("Aucun calendrier sélectionné")

  // 1. Récupérer les métadonnées du calendrier pour avoir les dates
  const calendars = await invoke<CalendarMetadata[]>('get_calendars_metadata')
  const selectedCalendar = calendars.find(c => c.id === calendarId)
  
  if (!selectedCalendar || !selectedCalendar.start_date || !selectedCalendar.end_date) {
    throw new Error("Calendrier invalide ou sans dates définies")
  }

  const calStart = new Date(selectedCalendar.start_date).getTime()
  const calEnd = new Date(selectedCalendar.end_date).getTime()

  // 2. Récupérer toutes les archives
  const archives = await invoke<Archive[]>('list_archives')
  
  // 3. Filtrer et parser
  const total = pairs.length
  let current = 0

  for (const symbol of pairs) {
    // Trouver l'archive la plus récente pour ce symbole et cette période
    // On cherche une archive dont la période correspond à peu près au calendrier
    // et qui contient le symbole dans son titre ou ses données
    
    // Note: Idéalement, l'archive devrait avoir un champ 'symbol' explicite.
    // Ici on va parser le JSON pour vérifier le symbole.
    
    let bestArchive: ArchivedAnalysisData | null = null
    let bestArchiveDate = 0

    for (const archive of archives) {
      // Vérifier si la période correspond (à 24h près)
      const archStart = new Date(archive.period_start).getTime()
      const archEnd = new Date(archive.period_end).getTime()
      
      const isPeriodMatch = Math.abs(archStart - calStart) < 86400000 && 
                            Math.abs(archEnd - calEnd) < 86400000
      
      if (!isPeriodMatch) continue

      try {
        const data = JSON.parse(archive.data_json) as ArchivedAnalysisData
        
        // Vérifier si c'est bien une archive d'analyse complète
        if (data.analysisResult && data.analysisResult.symbol === symbol) {
          const archiveTime = new Date(archive.created_at).getTime()
          if (archiveTime > bestArchiveDate) {
            bestArchive = data
            bestArchiveDate = archiveTime
          }
        }
      } catch (e) {
        // Ignorer les archives mal formées
      }
    }

    if (bestArchive) {
      results.push(bestArchive)
    } else {
      if (!archive) {
      // Aucune archive trouvée pour ce symbole
      continue
    }
    }

    current++
    onProgress((current / total) * 50)
  }
  
  return results
}

export interface ArchivedBacktestData {
  result: any // BacktestResult
  config: any // BacktestConfig
  mode: any // StrategyMode
}

export async function fetchBacktestArchivedData(pairs: string[], onProgress: (progress: number) => void): Promise<ArchivedBacktestData[]> {
  const results: ArchivedBacktestData[] = []
  const calendarId = parseInt(localStorage.getItem('activeCalendarId') || '0', 10)
  
  if (!calendarId) throw new Error("Aucun calendrier sélectionné")

  // 1. Récupérer les métadonnées du calendrier
  const calendars = await invoke<CalendarMetadata[]>('get_calendars_metadata')
  const selectedCalendar = calendars.find(c => c.id === calendarId)
  
  if (!selectedCalendar || !selectedCalendar.start_date || !selectedCalendar.end_date) {
    throw new Error("Calendrier invalide ou sans dates définies")
  }

  const calStart = new Date(selectedCalendar.start_date).getTime()
  const calEnd = new Date(selectedCalendar.end_date).getTime()

  // 2. Récupérer toutes les archives
  const archives = await invoke<Archive[]>('list_archives')
  
  // 3. Filtrer pour les Backtests
  const total = pairs.length
  let current = 0

  for (const symbol of pairs) {
    let bestArchive: ArchivedBacktestData | null = null
    let bestArchiveDate = 0

    for (const archive of archives) {
      if (archive.archive_type !== 'Backtest') continue

      // Vérifier si la période correspond (à 24h près)
      const archStart = new Date(archive.period_start).getTime()
      const archEnd = new Date(archive.period_end).getTime()
      
      // Pour les backtests, la période peut être plus large ou incluse
      // On vérifie si l'intersection des périodes n'est pas vide
      const overlap = Math.max(0, Math.min(calEnd, archEnd) - Math.max(calStart, archStart))
      const isRelevant = overlap > 0
      
      if (!isRelevant) continue

      try {
        const data = JSON.parse(archive.data_json) as ArchivedBacktestData
        
        if (data.result && data.result.symbol === symbol) {
          const archiveTime = new Date(archive.created_at).getTime()
          if (archiveTime > bestArchiveDate) {
            bestArchive = data
            bestArchiveDate = archiveTime
          }
        }
      } catch (e) {
        // Ignorer
      }
    }

    if (bestArchive) {
      results.push(bestArchive)
    }

    current++
    onProgress((current / total) * 50)
  }
  
  return results
}
