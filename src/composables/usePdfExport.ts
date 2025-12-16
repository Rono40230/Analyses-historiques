import { ref } from 'vue'
import { jsPDF } from 'jspdf'
import { fetchArchivedData, fetchBacktestArchivedData } from '../utils/pdf/dataFetcher'
import { 
  generateBidiReport, 
  generateRankingReport, 
  generateDangerReport, 
  generateIdentityReport
} from '../utils/pdf/reportGenerators'
import { generateBacktestReport } from '../utils/pdf/backtestReportGenerator'

export interface ExportFilters {
  periodStart: string
  periodEnd: string
  pairs: string[]
}

export function usePdfExport() {
  const isGenerating = ref(false)
  const progress = ref(0)
  const error = ref<string | null>(null)

  async function generatePdf(reportTypes: string[], filters: ExportFilters, preview = false) {
    isGenerating.value = true
    progress.value = 0
    error.value = null

    try {
      const doc = new jsPDF()
      let pageAdded = false
      
      // Titre global
      doc.setFontSize(20)
      doc.text('Rapport d\'Analyse Historique', 14, 22)
      doc.setFontSize(10)
      doc.text(`Généré le ${new Date().toLocaleDateString('fr-FR')}`, 14, 30)
      doc.text('Source: Archives validées', 14, 36)
      
      // Filtres appliqués
      doc.text(`Période: ${filters.periodStart} au ${filters.periodEnd}`, 14, 42)
      doc.text(`Paires analysées: ${filters.pairs.length}`, 14, 48)

      let yPos = 60
      const totalReports = reportTypes.length
      let currentReport = 0

      // 1. Récupération des données d'analyse (si nécessaire)
      const analysisReports = ['bidi', 'ranking', 'danger', 'identity']
      const needsAnalysisData = reportTypes.some(r => analysisReports.includes(r))
      
      let archivedDataList: any[] = []
      if (needsAnalysisData) {
        archivedDataList = await fetchArchivedData(filters.pairs, (p) => {
          progress.value = p * 0.5 // 50% du progrès pour le fetch
        })
        
        if (archivedDataList.length === 0 && !reportTypes.includes('backtest')) {
          throw new Error("Aucune archive d'analyse trouvée.")
        }
      }

      // 2. Récupération des données Backtest (si nécessaire)
      let backtestDataList: any[] = []
      if (reportTypes.includes('backtest')) {
        backtestDataList = await fetchBacktestArchivedData(filters.pairs, (p) => {
           // Ajuster le progrès si on fetch aussi l'analyse
           const base = needsAnalysisData ? 50 : 0
           const scale = needsAnalysisData ? 0.25 : 0.5
           progress.value = base + (p * scale)
        })

        if (backtestDataList.length === 0 && reportTypes.length === 1) {
           throw new Error("Aucune archive de backtest trouvée.")
        }
      }

      // 3. Génération des rapports
      for (const type of reportTypes) {
        if (pageAdded) {
          doc.addPage()
          yPos = 20
        }
        
        switch (type) {
          case 'bidi':
            if (archivedDataList.length > 0) await generateBidiReport(doc, archivedDataList)
            break
          case 'ranking':
            if (archivedDataList.length > 0) await generateRankingReport(doc, archivedDataList)
            break
          case 'danger':
            if (archivedDataList.length > 0) await generateDangerReport(doc, archivedDataList)
            break
          case 'identity':
            if (archivedDataList.length > 0) await generateIdentityReport(doc, archivedDataList)
            break
          case 'backtest':
            if (backtestDataList.length > 0) await generateBacktestReport(doc, backtestDataList)
            break
        }
        
        pageAdded = true
        currentReport++
        // Mise à jour du progrès restant
        const startGen = needsAnalysisData || reportTypes.includes('backtest') ? 75 : 0
        progress.value = startGen + ((currentReport / totalReports) * 25)
      }

      if (preview) {
        return doc.output('bloburl')
      } else {
        doc.save(`analyse_export_${new Date().toISOString().split('T')[0]}.pdf`)
        return true
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      return false
    } finally {
      isGenerating.value = false
    }
  }

  return {
    generatePdf,
    isGenerating,
    progress,
    error
  }
}
