import { ref } from 'vue'
import { jsPDF } from 'jspdf'
import { fetchArchivedData } from '../utils/pdf/dataFetcher'
import { 
  generateBidiReport, 
  generateRankingReport, 
  generateDangerReport, 
  generateIdentityReport 
} from '../utils/pdf/reportGenerators'

export interface ExportFilters {
  periodStart: string
  periodEnd: string
  pairs: string[]
}

export function usePdfExport() {
  const isGenerating = ref(false)
  const progress = ref(0)
  const error = ref<string | null>(null)

  async function generatePdf(reportTypes: string[], filters: ExportFilters) {
    isGenerating.value = true
    progress.value = 0
    error.value = null

    try {
      // 1. Récupération des données depuis les archives
      const archivedDataList = await fetchArchivedData(filters.pairs, (p) => {
        progress.value = p
      })
      
      if (archivedDataList.length === 0) {
        throw new Error("Aucune archive correspondante trouvée. Veuillez d'abord lancer et sauvegarder les analyses pour ce calendrier.")
      }

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
      doc.text(`Paires analysées: ${archivedDataList.length}`, 14, 48)

      let yPos = 60
      const totalReports = reportTypes.length
      let currentReport = 0

      for (const type of reportTypes) {
        if (pageAdded) {
          doc.addPage()
          yPos = 20
        }
        
        switch (type) {
          case 'bidi':
            await generateBidiReport(doc, archivedDataList)
            break
          case 'ranking':
            await generateRankingReport(doc, archivedDataList)
            break
          case 'danger':
            await generateDangerReport(doc, archivedDataList)
            break
          case 'identity':
            await generateIdentityReport(doc, archivedDataList)
            break
        }
        
        pageAdded = true
        currentReport++
        progress.value = 50 + ((currentReport / totalReports) * 50)
      }

      doc.save(`analyse_export_${new Date().toISOString().split('T')[0]}.pdf`)
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
