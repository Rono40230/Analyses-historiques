<template>
  <div class="modal-overlay" @click="closeModal">
    <div class="modal-container" @click.stop>
      <!-- Header -->
      <div class="modal-header">
        <div class="header-title">
          <span class="icon">⚙️</span>
          <span>Paramètres Bidi</span>
        </div>
        <div class="header-context">
          Créneau optimisé | Score {{ sliceScore }}/100
        </div>
        <button class="close-btn" @click="closeModal">✕</button>
      </div>

      <!-- Parameters Container -->
      <div class="parameters-container">
        <!-- 1️⃣ EVENT TIME - HEURE EXACTE -->
        <div class="parameter-card eventtime-card">
          <div class="card-header">
            <span class="param-icon">📍</span>
            <span class="param-title">HEURE DE DÉCLENCHEMENT</span>
          </div>
          <div class="param-content">
            <div class="param-value eventtime-value">{{ params.eventTime }}</div>
            <div class="param-explanation">{{ params.eventTimeExplanation }}</div>
            <div class="action-hint">→ Entre cette heure dans Bidi</div>
          </div>
        </div>

        <!-- 2️⃣ STOP LOSS - DISTANCE EN POINTS -->
        <div class="parameter-card stoploss-card">
          <div class="card-header">
            <span class="param-icon">🛑</span>
            <span class="param-title">POSITIONNEMENT STOP LOSS</span>
          </div>
          <div class="param-content">
            <!-- Distance en points concrets -->
            <div class="value-block">
              <div class="label">Distance SL</div>
              <div class="value-display">
                <span class="value-main">{{ params.stopLossPoints }}</span>
                <span class="value-unit">points</span>
              </div>
              <div class="value-alt">({{ params.stopLossLevelPercent }}% de l'ATR)</div>
            </div>

            <!-- Explication -->
            <div class="explanation-block">
              {{ params.stopLossExplanation }}
            </div>

            <!-- Calculatrice visuelle -->
            <div class="calculator-block">
              <div class="calc-title">📐 Formule SL: R ± (ATR × %)</div>
              <div class="calc-row">
                <span class="calc-label">ATR moyen:</span>
                <span class="calc-value">{{ params.atrMeanForCalculation }}</span>
                <span class="calc-unit">points</span>
              </div>
              <div class="calc-row">
                <span class="calc-label">{{ params.stopLossLevelPercent }}% × {{ params.atrMeanForCalculation }}pts:</span>
                <span class="calc-value">{{ params.stopLossPoints }}</span>
                <span class="calc-unit">points</span>
              </div>
              <div class="calc-note">→ SL = R ± {{ params.stopLossPoints }}pts (R = prix entrée)</div>
            </div>
          </div>
        </div>

        <!-- 3️⃣ ATR MULTIPLIER - AGRESSIVITÉ TRAILING -->
        <div class="parameter-card atr-card">
          <div class="card-header">
            <span class="param-icon">📈</span>
            <span class="param-title">AGRESSIVITÉ DU TRAILING STOP</span>
          </div>
          <div class="param-content">
            <!-- Multiplier et profil -->
            <div class="value-block">
              <div class="label">Multiplier</div>
              <div class="value-display">
                <span class="value-main">{{ params.atrMultiplier }}</span>
              </div>
              <div class="profile-badge" :class="`profile-${getProfileClass(params.atrMultiplierProfile)}`">
                {{ params.atrMultiplierProfile }}
              </div>
            </div>

            <!-- Explication -->
            <div class="explanation-block">
              {{ params.atrMultiplierExplanation }}
            </div>

            <!-- Profils disponibles -->
            <div class="profiles-block">
              <div class="profiles-title">Profils d'agressivité:</div>
              <div class="profiles-list">
                <div class="profile-option agg-aggressive">
                  <span class="profile-mult">1.5</span>
                  <span class="profile-desc">Agressif (serré, rapide)</span>
                </div>
                <div class="profile-option agg-normal" :class="{ active: params.atrMultiplier === 2.0 }">
                  <span class="profile-mult">2.0</span>
                  <span class="profile-desc">Normal (équilibré) ✓</span>
                </div>
                <div class="profile-option agg-generous" :class="{ active: params.atrMultiplier === 2.5 }">
                  <span class="profile-mult">2.5</span>
                  <span class="profile-desc">Généreux (suit trends)</span>
                </div>
                <div class="profile-option agg-very-generous">
                  <span class="profile-mult">3.0</span>
                  <span class="profile-desc">Très généreux (lent)</span>
                </div>
              </div>
            </div>

            <!-- Calculatrice visuelle -->
            <div class="calculator-block">
              <div class="calc-title">📐 Trailing Step:</div>
              <div class="calc-row">
                <span class="calc-label">{{ params.atrMultiplier }} × ATR:</span>
                <span class="calc-value">{{ params.trailingStepPoints }}</span>
                <span class="calc-unit">points/tick (irréversible)</span>
              </div>
              <div class="calc-note">À chaque tick: TSL monte de {{ params.trailingStepPoints }}pts et ne redescend jamais</div>
            </div>
          </div>
        </div>
      </div>

      <!-- Constants Footer -->
      <div class="constants-footer">
        <div class="constant-note">
          <span class="icon">🔐</span>
          <span>Constantes (ne pas modifier):</span>
          <span class="value">RiskPercent: 1% | TradeExpiration: 300min</span>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="action-buttons">
        <button class="btn btn-primary" @click="copyParameters">
          <span class="btn-icon">📋</span>
          <span>Copier les 3 paramètres</span>
        </button>
        <button class="btn btn-secondary" @click="downloadParameters">
          <span class="btn-icon">⬇️</span>
          <span>Télécharger CSV</span>
        </button>
        <button class="btn btn-close" @click="closeModal">Fermer</button>
      </div>

      <!-- Toast Notification -->
      <div v-if="toastMessage" class="toast-notification" :class="toastType">
        {{ toastMessage }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { BidiParameters } from '../utils/straddleAnalysis'

interface Props {
  params: BidiParameters
  sliceScore: number
}

const props = defineProps<Props>()
const emit = defineEmits<{
  close: []
}>()

const toastMessage = ref('')
const toastType = ref<'success' | 'error'>('success')

const closeModal = () => {
  emit('close')
}

/**
 * Retourne la classe CSS pour le profil d'agressivité
 */
const getProfileClass = (profile: string): string => {
  switch (profile) {
    case 'Agressif':
      return 'aggressive'
    case 'Normal':
      return 'normal'
    case 'Généreux':
      return 'generous'
    case 'Très Généreux':
      return 'very-generous'
    default:
      return 'normal'
  }
}

/**
 * Copie les 3 paramètres dans le clipboard
 */
const copyParameters = async () => {
  const text = `⚙️ PARAMÈTRES BIDI OPTIMISÉS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📍 HEURE DE DÉCLENCHEMENT
${props.params.eventTime}

🛑 STOP LOSS LEVEL PERCENT
${props.params.stopLossLevelPercent}%
= ${props.params.stopLossPoints} points

📈 ATR MULTIPLIER
${props.params.atrMultiplier}
= ${props.params.trailingStepPoints} points/tick (${props.params.atrMultiplierProfile})

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🔐 Constantes:
RiskPercent: 1.0 (1% par jambe = 2% straddle)
TradeExpiration: 300 minutes`

  try {
    await navigator.clipboard.writeText(text)
    showToast('✅ Paramètres copiés !', 'success')
    setTimeout(() => {
      toastMessage.value = ''
    }, 3000)
  } catch (err) {
    showToast('❌ Erreur lors de la copie', 'error')
    console.error('Failed to copy:', err)
  }
}

/**
 * Télécharge les paramètres en CSV
 */
const downloadParameters = () => {
  const csv = `Paramètre,Valeur,Unité,Explication
EventTime,${props.params.eventTime},HH:MM:SS,${props.params.eventTimeExplanation}
StopLossLevelPercent,${props.params.stopLossLevelPercent},%,${props.params.stopLossExplanation}
StopLossPoints,${props.params.stopLossPoints},points,Distance SL en points concrets
ATRMultiplier,${props.params.atrMultiplier},,${props.params.atrMultiplierExplanation}
TrailingStepPoints,${props.params.trailingStepPoints},points/tick,Augmentation TSL à chaque tick
RiskPercent,${props.params.riskPercent},%,CONSTANTE - Ne pas modifier
TradeExpiration,${props.params.tradeExpiration},minutes,CONSTANTE - Ne pas modifier`

  const blob = new Blob([csv], { type: 'text/csv;charset=utf-8;' })
  const link = document.createElement('a')
  link.href = URL.createObjectURL(blob)
  link.download = `bidi-parameters-${new Date().toISOString().split('T')[0]}.csv`
  link.click()

  showToast('✅ Fichier téléchargé !', 'success')
  setTimeout(() => {
    toastMessage.value = ''
  }, 3000)
}

/**
 * Affiche un message toast
 */
const showToast = (message: string, type: 'success' | 'error') => {
  toastMessage.value = message
  toastType.value = type
}
</script>

<style scoped lang="css">
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1001;
  backdrop-filter: blur(2px);
}

.modal-container {
  background: linear-gradient(135deg, #1a1f26 0%, #0d1117 100%);
  border: 1px solid #30363d;
  border-radius: 12px;
  max-width: 850px;
  width: 95%;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
  animation: modalSlideIn 0.3s ease-out;
  display: flex;
  flex-direction: column;
}

@keyframes modalSlideIn {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Header */
.modal-header {
  padding: 20px 24px;
  border-bottom: 1px solid #30363d;
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: linear-gradient(135deg, rgba(255, 193, 7, 0.1) 0%, rgba(33, 38, 45, 0) 100%);
  flex-shrink: 0;
}

.header-title {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 18px;
  font-weight: 600;
  color: #e6edf3;
}

.header-title .icon {
  font-size: 24px;
}

.header-context {
  font-size: 12px;
  color: #8b949e;
  font-weight: 500;
}

.close-btn {
  background: none;
  border: none;
  color: #8b949e;
  cursor: pointer;
  font-size: 24px;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  transition: all 0.2s;
}

.close-btn:hover {
  background: rgba(139, 148, 158, 0.1);
  color: #e6edf3;
}

/* Parameters Container */
.parameters-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 24px;
  flex: 1;
  overflow-y: auto;
  background: #0d1117;
}

.parameter-card {
  background: linear-gradient(135deg, #2a3236 0%, #21262d 100%);
  border: 1px solid #30363d;
  border-left: 5px solid #ffc107;
  border-radius: 8px;
  padding: 20px;
  transition: all 0.3s;
}

.parameter-card:hover {
  border-color: #58a6ff;
  box-shadow: 0 4px 12px rgba(88, 166, 255, 0.1);
}

.card-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid #30363d;
}

.param-icon {
  font-size: 20px;
}

.param-title {
  font-weight: 700;
  color: #e6edf3;
  font-size: 14px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.param-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

/* Value Blocks */
.value-block {
  background: rgba(88, 166, 255, 0.05);
  border: 1px solid rgba(88, 166, 255, 0.2);
  border-radius: 6px;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.value-block .label {
  font-size: 11px;
  color: #8b949e;
  text-transform: uppercase;
  font-weight: 600;
}

.value-display {
  display: flex;
  align-items: baseline;
  gap: 8px;
}

.value-main {
  font-size: 28px;
  font-weight: 700;
  color: #58a6ff;
  font-family: 'Monaco', monospace;
}

.value-unit {
  font-size: 14px;
  color: #8b949e;
  font-weight: 500;
}

.value-alt {
  font-size: 12px;
  color: #8b949e;
  margin-top: 4px;
}

.eventtime-value {
  font-size: 24px;
  font-weight: 700;
  color: #79c0ff;
  font-family: 'Monaco', monospace;
  padding: 12px;
  background: rgba(121, 192, 255, 0.1);
  border-radius: 4px;
  text-align: center;
}

/* EventTime specific */
.eventtime-card .param-content {
  gap: 8px;
}

.action-hint {
  font-size: 12px;
  color: #fbbf24;
  background: rgba(251, 191, 36, 0.1);
  padding: 8px;
  border-radius: 4px;
  text-align: center;
  font-weight: 600;
}

/* Explanation */
.explanation-block {
  font-size: 13px;
  color: #cbd5e0;
  line-height: 1.5;
  background: rgba(0, 0, 0, 0.2);
  padding: 10px;
  border-radius: 4px;
}

/* Calculator */
.calculator-block {
  background: rgba(251, 191, 36, 0.08);
  border: 1px solid rgba(251, 191, 36, 0.2);
  border-radius: 6px;
  padding: 10px;
}

.calc-title {
  font-size: 12px;
  font-weight: 600;
  color: #fbbf24;
  margin-bottom: 8px;
  text-transform: uppercase;
}

.calc-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 12px;
  margin-bottom: 4px;
  font-family: 'Monaco', monospace;
}

.calc-label {
  color: #8b949e;
}

.calc-value {
  font-weight: 700;
  color: #fbbf24;
}

.calc-unit {
  color: #8b949e;
  font-size: 11px;
}

.calc-note {
  font-size: 11px;
  color: #8b949e;
  margin-top: 6px;
  padding-top: 6px;
  border-top: 1px solid rgba(251, 191, 36, 0.2);
  font-style: italic;
}

/* Profile Badge */
.profile-badge {
  display: inline-block;
  padding: 4px 12px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
  width: fit-content;
  margin-top: 4px;
}

.profile-badge.profile-aggressive {
  background: rgba(239, 68, 68, 0.2);
  color: #ff7875;
}

.profile-badge.profile-normal {
  background: rgba(59, 130, 246, 0.2);
  color: #93c5fd;
}

.profile-badge.profile-generous {
  background: rgba(34, 197, 94, 0.2);
  color: #86efac;
}

.profile-badge.profile-very-generous {
  background: rgba(251, 191, 36, 0.2);
  color: #fbbf24;
}

/* Profiles List */
.profiles-block {
  margin-top: 8px;
}

.profiles-title {
  font-size: 12px;
  font-weight: 600;
  color: #8b949e;
  text-transform: uppercase;
  margin-bottom: 8px;
}

.profiles-list {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 8px;
}

.profile-option {
  background: rgba(45, 55, 72, 0.4);
  border: 1px solid #30363d;
  border-radius: 4px;
  padding: 8px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  align-items: center;
  font-size: 12px;
  cursor: default;
  transition: all 0.2s;
}

.profile-option.active {
  border-color: #fbbf24;
  background: rgba(251, 191, 36, 0.1);
}

.profile-mult {
  font-weight: 700;
  font-size: 14px;
  color: #e6edf3;
  font-family: 'Monaco', monospace;
}

.profile-desc {
  font-size: 11px;
  color: #8b949e;
  text-align: center;
}

.agg-aggressive {
  border-color: rgba(239, 68, 68, 0.3);
}

.agg-normal {
  border-color: rgba(59, 130, 246, 0.3);
}

.agg-generous {
  border-color: rgba(34, 197, 94, 0.3);
}

.agg-very-generous {
  border-color: rgba(251, 191, 36, 0.3);
}

/* Constants Footer */
.constants-footer {
  padding: 12px 24px;
  border-top: 1px solid #30363d;
  background: rgba(0, 0, 0, 0.2);
  flex-shrink: 0;
}

.constant-note {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 11px;
  color: #8b949e;
  font-weight: 500;
}

.constant-note .icon {
  font-size: 14px;
}

.constant-note .value {
  margin-left: auto;
  color: #6c757d;
  font-family: 'Monaco', monospace;
  font-size: 10px;
}

/* Action Buttons */
.action-buttons {
  display: flex;
  gap: 12px;
  padding: 16px 24px;
  border-top: 1px solid #30363d;
  background: #0d1117;
  justify-content: flex-end;
  flex-wrap: wrap;
  flex-shrink: 0;
}

.btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-icon {
  font-size: 16px;
}

.btn-primary {
  background: linear-gradient(135deg, #ffc107 0%, #ffb300 100%);
  color: #0d1117;
  border: 1px solid #ffb300;
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(255, 193, 7, 0.3);
  background: linear-gradient(135deg, #ffb300 0%, #ffa500 100%);
}

.btn-secondary {
  background: #21262d;
  color: #58a6ff;
  border: 1px solid #30363d;
}

.btn-secondary:hover {
  background: #30363d;
  border-color: #58a6ff;
}

.btn-close {
  background: #21262d;
  color: #8b949e;
  border: 1px solid #30363d;
}

.btn-close:hover {
  background: #30363d;
  color: #e6edf3;
}

/* Toast Notification */
.toast-notification {
  position: fixed;
  bottom: 24px;
  right: 24px;
  padding: 12px 20px;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  animation: toastSlideIn 0.3s ease-out;
  z-index: 1002;
}

@keyframes toastSlideIn {
  from {
    opacity: 0;
    transform: translateX(20px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

.toast-notification.success {
  background: rgba(63, 185, 80, 0.9);
  color: #ffffff;
  border: 1px solid #3fb950;
}

.toast-notification.error {
  background: rgba(248, 81, 73, 0.9);
  color: #ffffff;
  border: 1px solid #f85149;
}

/* Scrollbar */
.parameters-container::-webkit-scrollbar {
  width: 6px;
}

.parameters-container::-webkit-scrollbar-track {
  background: transparent;
}

.parameters-container::-webkit-scrollbar-thumb {
  background: rgba(139, 148, 158, 0.3);
  border-radius: 3px;
}

.parameters-container::-webkit-scrollbar-thumb:hover {
  background: rgba(139, 148, 158, 0.5);
}

/* Responsive */
@media (max-width: 768px) {
  .modal-container {
    max-width: 100%;
    border-radius: 0;
    max-height: 100vh;
  }

  .parameters-container {
    gap: 12px;
    padding: 16px;
  }

  .parameter-card {
    padding: 16px;
  }

  .action-buttons {
    flex-direction: column;
  }

  .btn {
    width: 100%;
    justify-content: center;
  }

  .value-main {
    font-size: 24px;
  }

  .profiles-list {
    grid-template-columns: repeat(2, 1fr);
  }
}
</style>
