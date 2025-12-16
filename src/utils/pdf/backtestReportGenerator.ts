import { jsPDF } from 'jspdf'
import autoTable from 'jspdf-autotable'
import type { ArchivedBacktestData } from './dataFetcher'

export async function generateBacktestReport(doc: jsPDF, dataList: ArchivedBacktestData[]) {
  doc.setFontSize(16)
  doc.text('Rapport de Performance Backtest', 14, 20)
  doc.setFontSize(10)
  doc.text('Simulation historique sur les événements sélectionnés', 14, 26)
  
  let yPos = 35

  for (const data of dataList) {
    const res = data.result
    const cfg = data.config
    
    // Nouvelle page si nécessaire
    if (yPos > 250) {
      doc.addPage()
      yPos = 20
    }

    // En-tête de section
    doc.setFontSize(14)
    doc.setTextColor(41, 128, 185)
    doc.text(`• ${res.symbol} - ${res.event_name}`, 14, yPos)
    doc.setTextColor(0, 0, 0)
    doc.setFontSize(10)
    yPos += 8

    // Paramètres utilisés
    doc.setFontSize(9)
    doc.setTextColor(100)
    doc.text(`Mode: ${data.mode} | Offset: ${cfg.offset_pips} | SL: ${cfg.stop_loss_pips} | TP: ${cfg.trailing_stop_pips} (TS) | Spread: ${cfg.spread_pips}`, 14, yPos)
    doc.setTextColor(0)
    doc.setFontSize(10)
    yPos += 8

    // Tableau des métriques
    const metrics = [
      ['Win Rate', `${res.win_rate_percent.toFixed(1)}%`],
      ['Profit Factor', res.profit_factor.toFixed(2)],
      ['Total Trades', res.total_trades.toString()],
      ['Total Pips', res.total_pips.toFixed(1)],
      ['Max Drawdown', `${res.max_drawdown_pips.toFixed(1)} pips`],
      ['Avg Pips/Trade', res.average_pips_per_trade.toFixed(1)]
    ]

    autoTable(doc, {
      startY: yPos,
      head: [['Métrique', 'Valeur', 'Métrique', 'Valeur']],
      body: [
        [metrics[0][0], metrics[0][1], metrics[3][0], metrics[3][1]],
        [metrics[1][0], metrics[1][1], metrics[4][0], metrics[4][1]],
        [metrics[2][0], metrics[2][1], metrics[5][0], metrics[5][1]]
      ],
      theme: 'grid',
      headStyles: { fillColor: [52, 73, 94] },
      styles: { fontSize: 10, cellPadding: 2 },
      margin: { left: 14, right: 14 }
    })

    // @ts-ignore
    yPos = doc.lastAutoTable.finalY + 10

    // Petit tableau des 5 derniers trades
    const lastTrades = res.trades.slice(-5).reverse().map((t: any) => [
      new Date(t.event_date).toLocaleDateString(),
      t.outcome,
      t.pips_net.toFixed(1),
      `${t.duration_minutes}m`
    ])

    if (lastTrades.length > 0) {
      doc.text('Derniers trades:', 14, yPos)
      yPos += 5
      
      autoTable(doc, {
        startY: yPos,
        head: [['Date', 'Résultat', 'Pips', 'Durée']],
        body: lastTrades,
        theme: 'plain',
        styles: { fontSize: 8, cellPadding: 1 },
        margin: { left: 14, right: 100 } // Tableau compact à gauche
      })
      
      // @ts-ignore
      yPos = doc.lastAutoTable.finalY + 15
    } else {
      yPos += 10
    }
  }
}
