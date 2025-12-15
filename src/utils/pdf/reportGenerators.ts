import { jsPDF } from 'jspdf'
import autoTable from 'jspdf-autotable'
import type { ArchivedAnalysisData } from '../../composables/useMetricsModalLoad'

export async function generateBidiReport(doc: jsPDF, dataList: ArchivedAnalysisData[]) {
  doc.setFontSize(16)
  doc.text('Fiche Paramètres Bidi', 14, 20)
  doc.setFontSize(10)
  doc.text('Paramètres optimaux (Source: Trading Plan validé)', 14, 26)
  
  const rows: any[] = []

  dataList.forEach(data => {
    // Utiliser le Trading Plan validé de l'archive
    const plan = data.tradingPlan
    const slice = data.sliceAnalyses && data.sliceAnalyses.length > 0 ? data.sliceAnalyses[0].slice : null
    
    if (plan && slice) {
      rows.push([
        data.analysisResult.symbol,
        slice.startTime, // Heure formatée (ex: 14:00-14:15)
        plan.offset ? plan.offset.toFixed(1) : 'N/A',
        plan.tp ? plan.tp.toFixed(1) : 'N/A',
        plan.sl ? plan.sl.toFixed(1) : 'N/A',
        plan.duration ? `${plan.duration}m` : 'N/A'
      ])
    } else {
      // Fallback si pas de plan complet
      rows.push([
        data.analysisResult.symbol,
        'N/A', 'N/A', 'N/A', 'N/A', 'N/A'
      ])
    }
  })

  autoTable(doc, {
    startY: 30,
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

export async function generateRankingReport(doc: jsPDF, dataList: ArchivedAnalysisData[]) {
  doc.setFontSize(16)
  doc.text('Classement des Opportunités', 14, 20)
  doc.setFontSize(10)
  doc.text('Basé sur le score de straddle validé (Quarter élu uniquement)', 14, 26)
  
  const opportunities: any[] = []
  
  dataList.forEach(data => {
    // On ne prend QUE le meilleur slice (celui élu)
    if (data.sliceAnalyses && data.sliceAnalyses.length > 0) {
      const bestSlice = data.sliceAnalyses[0]
      const stats = bestSlice.slice.stats
      
      opportunities.push({
        symbol: data.analysisResult.symbol,
        time: bestSlice.slice.startTime,
        score: bestSlice.slice.straddleScore,
        volatility: stats.volatility_mean,
        noise: stats.noise_ratio_mean,
        breakout: stats.breakout_percentage
      })
    }
  })

  // Trier par score décroissant
  opportunities.sort((a, b) => b.score - a.score)

  const rows = opportunities.map((opp, index) => [
    index + 1,
    opp.symbol,
    opp.time,
    opp.score.toFixed(1),
    opp.volatility.toFixed(1),
    opp.noise.toFixed(2),
    `${opp.breakout.toFixed(1)}%`
  ])

  autoTable(doc, {
    startY: 30,
    head: [['Rang', 'Paire', 'Heure', 'Score', 'Volatilité', 'Bruit', 'Breakout']],
    body: rows,
    theme: 'striped',
    headStyles: { fillColor: [46, 204, 113] },
  })
}

export async function generateDangerReport(doc: jsPDF, dataList: ArchivedAnalysisData[]) {
  doc.setFontSize(16)
  doc.text('Zones de Danger (Blacklist)', 14, 20)
  doc.setFontSize(10)
  doc.text('Configurations à éviter (Ratio de bruit > 0.8 ou Volatilité faible)', 14, 26)
  
  const rows: any[] = []

  dataList.forEach(data => {
    // On utilise les stats brutes pour détecter les dangers sur TOUS les créneaux
    // ou juste sur le créneau élu ? Le rapport danger est souvent global.
    // Utilisons les stats 15min complètes de l'analyse result
    
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

  // Limiter aux 30 pires
  const worstRows = rows.slice(0, 30)

  autoTable(doc, {
    startY: 30,
    head: [['Paire', 'Heure', 'Raison', 'Ratio Bruit', 'Action']],
    body: worstRows,
    theme: 'grid',
    headStyles: { fillColor: [192, 57, 43] },
  })
}

export async function generateIdentityReport(doc: jsPDF, dataList: ArchivedAnalysisData[]) {
  doc.setFontSize(16)
  doc.text('Carte d\'Identité par Paire', 14, 20)
  
  let yPos = 30

  dataList.forEach((data, index) => {
    if (index > 0 && index % 3 === 0) {
      doc.addPage()
      yPos = 20
    }

    const res = data.analysisResult
    const metrics = res.global_metrics
    
    // Utiliser les données raffinées si possible
    const bestSlice = data.sliceAnalyses && data.sliceAnalyses.length > 0 ? data.sliceAnalyses[0] : null
    const bestTime = bestSlice ? bestSlice.slice.startTime : 'N/A'
    const confidence = data.whipsawAnalysis ? (100 - data.whipsawAnalysis.whipsaw_frequency_percentage) : res.confidence_score

    doc.setFontSize(14)
    doc.setTextColor(41, 128, 185)
    doc.text(`• ${res.symbol}`, 14, yPos)
    doc.setTextColor(0, 0, 0)
    doc.setFontSize(10)
    
    const info = [
      [`Volatilité Moyenne: ${metrics.mean_volatility.toFixed(1)} pips`, `Meilleur Moment: ${bestTime}`],
      [`ATR Moyen: ${metrics.mean_atr.toFixed(1)}`, `Ratio Bruit Moyen: ${metrics.mean_noise_ratio.toFixed(2)}`],
      [`Bougies Analysées: ${metrics.total_candles}`, `Confiance: ${confidence.toFixed(1)}/100`]
    ]

    autoTable(doc, {
      startY: yPos + 5,
      body: info,
      theme: 'plain',
      styles: { cellPadding: 1, fontSize: 10 },
      margin: { left: 14 }
    })

    // @ts-ignore - lastAutoTable is added by the plugin
    yPos = doc.lastAutoTable.finalY + 15
  })
}
