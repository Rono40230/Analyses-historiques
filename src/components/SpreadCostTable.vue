<script setup lang="ts">
// Composant d'affichage des coûts de trading (Spread/Slippage) en conditions de News Trading
</script>

<template>
  <div class="spread-cost-table">
    <div class="info-box">
      <h4 class="text-warning">⚠️ Conditions "News Trading"</h4>
      <p>
        Pour le <strong>News Trading</strong> (trading d'annonces économiques), les conditions de marché sont radicalement différentes des conditions normales. 
        Les fournisseurs de liquidité retirent leurs ordres juste avant l'annonce, créant des "trous" de cotation.
      </p>
      <p>
        Les valeurs ci-dessous représentent les conditions typiques durant la <strong>première seconde</strong> d'une annonce majeure (NFP, CPI, FOMC).
        Ces coûts "invisibles" doivent être déduits de vos backtests pour obtenir une espérance de gain réaliste.
      </p>
    </div>

    <div class="table-container">
      <table>
        <thead>
          <tr>
            <th>Classe d'Actif</th>
            <th>Paires / Symboles</th>
            <th>Spread News (Pips)</th>
            <th>Slippage (Pips)</th>
            <th>Coût Total / Trade</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td class="category">Majors (Liquides)</td>
            <td class="symbol">EURUSD, USDJPY</td>
            <td class="value">2.0 - 3.0</td>
            <td class="value">1.0</td>
            <td class="total">~3.5 Pips</td>
          </tr>
          <tr>
            <td class="category">Majors (Volatiles)</td>
            <td class="symbol">GBPUSD, AUDUSD</td>
            <td class="value">3.0 - 5.0</td>
            <td class="value">2.0</td>
            <td class="total">~6.0 Pips</td>
          </tr>
          <tr>
            <td class="category">Minors / Crosses</td>
            <td class="symbol">GBPJPY, EURJPY</td>
            <td class="value">5.0 - 8.0</td>
            <td class="value">3.0</td>
            <td class="total">~9.5 Pips</td>
          </tr>
          <tr>
            <td class="category">Or (Gold)</td>
            <td class="symbol">XAUUSD</td>
            <td class="value">3.0 - 5.0</td>
            <td class="value">2.0</td>
            <td class="total">~6.0 Pips</td>
          </tr>
          <tr>
            <td class="category">Indices US</td>
            <td class="symbol">US30, NAS100</td>
            <td class="value">5.0 - 10.0 pts</td>
            <td class="value">5.0 pts</td>
            <td class="total">~12.5 Pts</td>
          </tr>
          <tr>
            <td class="category">Indices EU</td>
            <td class="symbol">DAX40 (GER40)</td>
            <td class="value">4.0 - 8.0 pts</td>
            <td class="value">3.0 pts</td>
            <td class="total">~9.0 Pts</td>
          </tr>
          <tr>
            <td class="category">Crypto</td>
            <td class="symbol">BTCUSD</td>
            <td class="value">$30 - $50</td>
            <td class="value">$20</td>
            <td class="total">~$60</td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="formula-explanation">
      <h5>🧮 Impact sur le P&L (Profit & Loss)</h5>
      <div class="formula-grid">
        <div class="formula-item">
          <span class="label">Achat (Long) :</span>
          <span class="math">Prix Entrée = Buy Stop + Slippage + Spread</span>
        </div>
        <div class="formula-item">
          <span class="label">Vente (Short) :</span>
          <span class="math">Prix Entrée = Sell Stop - Slippage</span>
        </div>
        <div class="formula-item">
          <span class="label">Coût Whipsaw :</span>
          <span class="math text-danger">2x Spread + 2x Slippage + Perte SL</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.spread-cost-table {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.info-box {
  background: rgba(234, 179, 8, 0.1);
  border: 1px solid rgba(234, 179, 8, 0.2);
  border-radius: 6px;
  padding: 1rem;
}

.info-box h4 {
  color: #fbbf24;
  margin: 0 0 0.5rem 0;
  font-size: 0.95rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.info-box p {
  margin: 0;
  font-size: 0.85rem;
  color: #d1d5db;
  line-height: 1.5;
}

.info-box p + p {
  margin-top: 0.75rem;
}

.table-container {
  border: 1px solid #374151;
  border-radius: 6px;
  overflow: hidden;
}

table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.85rem;
}

th {
  background: #1f2937;
  color: #9ca3af;
  font-weight: 600;
  text-align: left;
  padding: 0.75rem 1rem;
  border-bottom: 1px solid #374151;
}

td {
  padding: 0.75rem 1rem;
  border-bottom: 1px solid #374151;
  color: #e5e7eb;
}

tr:last-child td {
  border-bottom: none;
}

tr:hover td {
  background: rgba(255, 255, 255, 0.02);
}

.category {
  color: #60a5fa;
  font-weight: 500;
}

.symbol {
  font-family: 'JetBrains Mono', monospace;
  color: #9ca3af;
}

.value {
  color: #d1d5db;
}

.total {
  color: #f87171;
  font-weight: 600;
}

.formula-explanation {
  background: #1f2937;
  border-radius: 6px;
  padding: 1rem;
}

.formula-explanation h5 {
  margin: 0 0 1rem 0;
  color: #9ca3af;
  font-size: 0.9rem;
}

.formula-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1rem;
}

.formula-item {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.label {
  font-size: 0.75rem;
  color: #6b7280;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.math {
  font-family: 'JetBrains Mono', monospace;
  font-size: 0.85rem;
  color: #e5e7eb;
  background: #111827;
  padding: 0.5rem;
  border-radius: 4px;
  border: 1px solid #374151;
}

.text-warning { color: #fbbf24; }
.text-danger { color: #f87171; }
</style>
