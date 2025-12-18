export interface ConversionRow {
  symbol: string
  priceFormat: string
  pipValue: string
  lotValue: string
  formula: string
  example: string
}

export interface ConversionCategory {
  name: string
  rows: ConversionRow[]
}

export const conversionData: ConversionCategory[] = [
  {
    name: "💵 PAIRES FOREX",
    rows: [
      { symbol: "EUR/USD", priceFormat: "1.10538", pipValue: "10 points", lotValue: "$10", formula: "Points ÷ 10 = Pips", example: "100 pts = 10 pips" },
      { symbol: "GBP/USD", priceFormat: "1.27345", pipValue: "10 points", lotValue: "$10", formula: "Points ÷ 10 = Pips", example: "50 pts = 5 pips" },
      { symbol: "USD/CAD", priceFormat: "1.39782", pipValue: "10 points", lotValue: "$10", formula: "Points ÷ 10 = Pips", example: "150 pts = 15 pips" },
      { symbol: "USD/JPY", priceFormat: "149.538", pipValue: "10 points", lotValue: "$10", formula: "Points ÷ 10 = Pips", example: "80 pts = 8 pips" },
      { symbol: "GBP/JPY", priceFormat: "190.456", pipValue: "10 points", lotValue: "Variable*", formula: "Points ÷ 10 = Pips", example: "120 pts = 12 pips" },
      { symbol: "CAD/JPY", priceFormat: "107.891", pipValue: "10 points", lotValue: "Variable*", formula: "Points ÷ 10 = Pips", example: "90 pts = 9 pips" }
    ]
  },
  {
    name: "🥇 MÉTAUX PRÉCIEUX",
    rows: [
      { symbol: "XAU/USD (Or)", priceFormat: "2045.38", pipValue: "10 points", lotValue: "$10", formula: "Points ÷ 10 = Pips", example: "50 pts = 5 pips ($0.50)" },
      { symbol: "XAG/USD (Argent)", priceFormat: "24.538", pipValue: "10 points", lotValue: "$50", formula: "Points ÷ 10 = Pips", example: "30 pts = 3 pips ($0.03)" }
    ]
  },
  {
    name: "📈 INDICES",
    rows: [
      { symbol: "USATEC (NASDAQ-100)", priceFormat: "16789.45", pipValue: "1 point = 1 pip", lotValue: "$1", formula: "Points = Pips", example: "50 pts = 50 pips" },
      { symbol: "USAIDX (S&P 500)", priceFormat: "4567.89", pipValue: "1 point = 1 pip", lotValue: "$1", formula: "Points = Pips", example: "25 pts = 25 pips" },
      { symbol: "DEUIDX (DAX 40)", priceFormat: "17234.56", pipValue: "1 point = 1 pip", lotValue: "€1", formula: "Points = Pips", example: "30 pts = 30 pips" }
    ]
  },
  {
    name: "₿ CRYPTO",
    rows: [
      { symbol: "BTC/USD", priceFormat: "42567.89", pipValue: "1 point = 1 point", lotValue: "$1", formula: "Points = Pips", example: "100 pts = 100 pips" }
    ]
  }
]
