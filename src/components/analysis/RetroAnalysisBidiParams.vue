<template>
  <div class="bidi-section">
    <h4>⚙️ PARAMÈTRES BIDI STRADDLE DIRECTIONNEL</h4>
    <div class="bidi-grid">
      <div class="bidi-param">
        <div class="bidi-label">Moment de placement</div>
        <div class="bidi-value">{{ meilleurMoment > 0 ? Math.round(meilleurMoment) : '—' }} <span class="bidi-unit">min avant</span></div>
        <div class="bidi-description">Quand placer les ordres en attente</div>
      </div>
      
      <div class="bidi-param buy-param">
        <div class="bidi-label">BUY STOP</div>
        <div class="bidi-value">+{{ offset > 0 ? formatPointsWithPips(pair || 'EURUSD', offset, 0) : '—' }}</div>
        <div class="bidi-description">Entrée Achat (au-dessus du prix)</div>
      </div>

      <div class="bidi-param sell-param">
        <div class="bidi-label">SELL STOP</div>
        <div class="bidi-value">-{{ offset > 0 ? formatPointsWithPips(pair || 'EURUSD', offset, 0) : '—' }}</div>
        <div class="bidi-description">Entrée Vente (en-dessous du prix)</div>
      </div>

      <div class="bidi-param">
        <div class="bidi-label">Stop Loss</div>
        <div class="bidi-value">{{ stopLoss > 0 ? formatPointsWithPips(pair || 'EURUSD', stopLoss, 0) : '—' }}</div>
        <div class="bidi-description">Distance d'arrêt (Risque)</div>
      </div>
      <div class="bidi-param">
        <div class="bidi-label">Trailing Stop</div>
        <div class="bidi-value">{{ trailingStop > 0 ? formatPointsWithPips(pair || 'EURUSD', trailingStop, 1) : '—' }}</div>
        <div class="bidi-description">Stop dynamique adapté au noise</div>
      </div>
      <div class="bidi-param">
        <div class="bidi-label">Timeout</div>
        <div class="bidi-value">{{ timeout || '60' }} <span class="bidi-unit">min</span></div>
        <div class="bidi-description">Durée maximale du trade</div>
      </div>
    </div>
  </div>

  <div class="bidi-section simultaneous-section">
    <h4>⚙️ PARAMÈTRES BIDI STRADDLE SIMULTANÉ</h4>
    <div class="bidi-grid">
      <div class="bidi-param">
        <div class="bidi-label">Moment de placement</div>
        <div class="bidi-value">{{ meilleurMoment > 0 ? Math.round(meilleurMoment) : '—' }} <span class="bidi-unit">min avant</span></div>
        <div class="bidi-description">Identique au directionnel</div>
      </div>
      
      <div class="bidi-param">
        <div class="bidi-label">Offset</div>
        <div class="bidi-value">{{ offset > 0 ? formatPointsWithPips(pair || 'EURUSD', offset, 0) : '—' }}</div>
        <div class="bidi-description">Identique au directionnel</div>
      </div>

      <div class="bidi-param recovery-param">
        <div class="bidi-label">SL RECOVERY</div>
        <div class="bidi-value">{{ stopLossRecovery > 0 ? formatPointsWithPips(pair || 'EURUSD', stopLossRecovery, 0) : '—' }}</div>
        <div class="bidi-description">Pour couvrir le retournement</div>
      </div>

      <div class="bidi-param hedge-param">
        <div class="bidi-label">ZONE DE HEDGE</div>
        <div class="bidi-value">{{ offset > 0 ? formatPointsWithPips(pair || 'EURUSD', offset * 2, 0) : '—' }}</div>
        <div class="bidi-description">Risque si les 2 déclenchés</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatPointsWithPips } from '../../utils/pipConverter'

interface Props {
  meilleurMoment?: number
  stopLoss?: number
  trailingStop?: number
  timeout?: number
  offset?: number
  stopLossRecovery?: number
  pair: string
}

withDefaults(defineProps<Props>(), {
  meilleurMoment: 0,
  stopLoss: 0,
  trailingStop: 0,
  timeout: 60,
  offset: 0,
  stopLossRecovery: 0,
  pair: 'EURUSD'
})
</script>

<style scoped>
.bidi-section {
  margin-top: 10px;
  margin-bottom: 0;
  padding: 15px;
  background: linear-gradient(135deg, rgba(45, 90, 123, 0.15) 0%, rgba(78, 205, 196, 0.08) 100%);
  border: 1px solid #2d5a7b;
  border-radius: 8px;
}

.bidi-section h4 {
  margin: 0 0 15px 0;
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 1px;
  color: #58a6ff;
  border-bottom: 1px solid rgba(88, 166, 255, 0.2);
  padding-bottom: 8px;
}

.bidi-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 15px;
  align-items: start;
}

.bidi-param {
  background: rgba(255, 255, 255, 0.03);
  padding: 10px;
  border: 1px solid #3a5a78;
  border-radius: 6px;
  transition: all 0.2s;
}

.buy-param {
  border-color: rgba(74, 222, 128, 0.3);
  background: rgba(74, 222, 128, 0.05);
}

.buy-param .bidi-label {
  color: #4ade80;
}

.sell-param {
  border-color: rgba(248, 113, 113, 0.3);
  background: rgba(248, 113, 113, 0.05);
}

.sell-param .bidi-label {
  color: #f87171;
}

.simultaneous-section {
  border-color: rgba(167, 139, 250, 0.3);
  background: rgba(167, 139, 250, 0.05);
}

.simultaneous-section h4 {
  color: #a78bfa;
  border-bottom-color: rgba(167, 139, 250, 0.2);
}

.recovery-param {
  border-color: rgba(250, 204, 21, 0.3);
  background: rgba(250, 204, 21, 0.05);
}

.recovery-param .bidi-label {
  color: #facc15;
}

.hedge-param {
  border-color: rgba(244, 114, 182, 0.3);
  background: rgba(244, 114, 182, 0.05);
}

.hedge-param .bidi-label {
  color: #f472b6;
}

.bidi-param:hover {
  background: rgba(255, 255, 255, 0.05);
  transform: translateY(-2px);
}

.bidi-label {
  font-size: 0.75em;
  color: #8b949e;
  text-transform: uppercase;
  letter-spacing: 0.3px;
  margin-bottom: 5px;
}

.bidi-value {
  font-size: 1.1em;
  font-weight: 600;
  color: #58a6ff;
  margin-bottom: 3px;
  line-height: 1.2;
}

.bidi-unit {
  font-size: 0.65em;
  color: #6e8a99;
  margin-left: 3px;
}

.bidi-description {
  font-size: 0.7em;
  color: #6e8a99;
  margin-top: 2px;
}

@media (max-width: 1024px) {
  .bidi-grid { grid-template-columns: repeat(auto-fit, minmax(120px, 1fr)); gap: 8px; }
  .bidi-value { font-size: 1em; }
}

@media (max-width: 768px) {
  .bidi-section { padding: 12px; margin-top: 10px; }
  .bidi-section h4 { font-size: 11px; margin-bottom: 8px; }
  .bidi-grid { grid-template-columns: repeat(2, 1fr); gap: 8px; }
  .bidi-param { padding: 8px; }
  .bidi-label { font-size: 0.7em; }
  .bidi-value { font-size: 0.95em; }
  .bidi-description { font-size: 0.65em; }
}

@media (max-width: 480px) {
  .bidi-section { padding: 10px; margin-top: 8px; }
  .bidi-section h4 { font-size: 10px; margin-bottom: 8px; }
  .bidi-grid { grid-template-columns: 1fr; gap: 6px; }
  .bidi-param { padding: 6px; }
  .bidi-label { font-size: 0.65em; margin-bottom: 3px; }
  .bidi-value { font-size: 0.85em; margin-bottom: 2px; }
  .bidi-unit { font-size: 0.6em; }
  .bidi-description { font-size: 0.6em; }
}
</style>
