import { jsPDF } from 'jspdf'
import autoTable from 'jspdf-autotable'
import type { ArchivedAnalysisData } from '../../../composables/useMetricsModalLoad'
import { formatPointsWithPips } from '../../pipConverter'

export async function generateBidiReport(doc: jsPDF, dataList: ArchivedAnalysisData[], startY: number = 20) {
  doc.setFontSize(16)
  doc.text('Fiche Paramètres Bidi', 14, startY)
  doc.setFontSize(10)
  doc.text('Paramètres optimaux (Source: Trading Plan validé)', 14, startY + 6)
  
  const rows: any[] = []

  dataList.forEach(data => {
    const plan = data.tradingPlan
    const slice = data.sliceAnalyses && data.sliceAnalyses.length > 0 ? data.sliceAnalyses[0].slice : null
    
    if (plan && slice) {
      const symbol = data.analysisResult.symbol
      rows.push([
        symbol,
        slice.startTime,
        plan.offset ? formatPointsWithPips(symbol, plan.offset) : 'N/A',
        plan.tp ? formatPointsWithPips(symbol, plan.tp) : 'N/A',
        plan.sl ? formatPointsWithPips(symbol, plan.sl) : 'N/A',
        plan.duration ? `${plan.duration}m` : 'N/A'
      ])
    } else {
      rows.push([
        data.analysisResult.symbol,
        'N/A', 'N/A', 'N/A', 'N/A', 'N/A'
      ])
    }
  })

  autoTable(doc, {
    startY: startY + 10,
    head: [['Paire', 'Heure', 'Offset', 'TP', 'SL', 'Durée']],
    body: rows,
    theme: 'grid',
    headStyles: { fillColor: [41, 128, 185] },
    styles: { fontSize: 10, cellPadding: 3 },
    columnStyles: {
      0: { fontStyle: 'bold' },
      2: { halign: 'right' },
      3: { halign: 'right' },
      4: { halign: 'right' },
      5: { halign: 'center' }
    }
  })
}
