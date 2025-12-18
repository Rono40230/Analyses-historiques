import { jsPDF } from 'jspdf'
import autoTable from 'jspdf-autotable'
import type { ArchivedAnalysisData } from '../../../composables/useMetricsModalLoad'

export async function generateDangerReport(doc: jsPDF, dataList: ArchivedAnalysisData[], startY: number = 20) {
  doc.setFontSize(16)
  doc.text('Zones de Danger (Blacklist)', 14, startY)
  doc.setFontSize(10)
  doc.text('Configurations à éviter (Ratio de bruit > 0.8 ou Volatilité faible)', 14, startY + 6)
  
  const rows: any[] = []

  dataList.forEach(data => {
    data.analysisResult.stats_15min.forEach(stat => {
      if (stat.noise_ratio_mean > 0.8 || stat.volatility_mean < 10) {
        let reason = ''
        if (stat.noise_ratio_mean > 0.8) reason = 'Bruit excessif'
        else if (stat.volatility_mean < 10) reason = 'Volatilité insuffisante'
        
        rows.push([
          data.analysisResult.symbol,
          `${String(stat.hour).padStart(2, '0')}:${String(stat.quarter * 15).padStart(2, '0')}`,
          reason,
          stat.noise_ratio_mean.toFixed(2),
          'ÉVITER'
        ])
      }
    })
  })

  const worstRows = rows.slice(0, 30)

  autoTable(doc, {
    startY: startY + 10,
    head: [['Paire', 'Heure', 'Raison', 'Ratio Bruit', 'Action']],
    body: worstRows,
    theme: 'grid',
    headStyles: { fillColor: [192, 57, 43] },
  })
}
