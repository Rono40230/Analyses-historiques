import { jsPDF } from 'jspdf'
import autoTable from 'jspdf-autotable'
import type { ArchivedAnalysisData } from '../../../composables/useMetricsModalLoad'
import { formaterPointsAvecPips } from '../../pipConverter'

export async function generateIdentityReport(doc: jsPDF, dataList: ArchivedAnalysisData[], startY: number = 20) {
  doc.setFontSize(16)
  doc.text('Carte d\'Identité par Paire', 14, startY)
  
  doc.setDrawColor(200, 200, 200)
  doc.setFillColor(245, 245, 245)
  doc.rect(14, startY + 5, 182, 25, 'FD')
  
  doc.setFontSize(9)
  doc.setTextColor(80, 80, 80)
  const explanation = "Note importante : Les métriques ci-dessous (ATR Moyen, Ratio Bruit) sont des moyennes globales calculées sur l'ensemble de l'historique (Macro). Elles diffèrent des métriques affichées dans l'interface d'analyse qui sont spécifiques à un créneau horaire et une configuration d'événement donnés (Micro)."
  const splitText = doc.splitTextToSize(explanation, 175)
  doc.text(splitText, 18, startY + 12)
  
  doc.setTextColor(0, 0, 0)
  let yPos = startY + 40

  dataList.forEach((data, index) => {
    if (index > 0 && index % 3 === 0) {
      doc.addPage()
      yPos = 30
    }

    const res = data.analysisResult
    const metrics = res.global_metrics
    
    const bestSlice = data.sliceAnalyses && data.sliceAnalyses.length > 0 ? data.sliceAnalyses[0] : null
    const bestTime = bestSlice ? bestSlice.slice.startTime : 'N/A'
    const confidence = data.whipsawAnalysis ? (100 - data.whipsawAnalysis.whipsaw_frequency_percentage) : res.confidence_score

    doc.setFontSize(14)
    doc.setTextColor(41, 128, 185)
    doc.text(`• ${res.symbol}`, 14, yPos)
    doc.setTextColor(0, 0, 0)
    doc.setFontSize(10)
    
    const info = [
      [`Volatilité Moyenne: ${formaterPointsAvecPips(res.symbol, metrics.mean_volatility)}`, `Meilleur Moment: ${bestTime}`],
      [`ATR Moyen: ${formaterPointsAvecPips(res.symbol, metrics.mean_atr)}`, `Ratio Bruit Moyen: ${metrics.mean_noise_ratio.toFixed(2)}`],
      [`Bougies Analysées: ${metrics.total_candles}`, `Confiance: ${confidence.toFixed(1)}/100`]
    ]

    autoTable(doc, {
      startY: yPos + 5,
      body: info,
      theme: 'plain',
      styles: { cellPadding: 1, fontSize: 10 },
      margin: { left: 14 }
    })

    // @ts-ignore
    yPos = doc.lastAutoTable.finalY + 15
  })
}
