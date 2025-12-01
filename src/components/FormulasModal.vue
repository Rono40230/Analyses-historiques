<template>
  <div v-if="isOpen" class="formulas-overlay" @click.self="close">
    <div class="formulas-container">
      <div class="formulas-header">
        <div class="formulas-title">
          <span class="formulas-title-icon">🧮</span>
          <h2 class="formulas-title-text">Formules & Calculs</h2>
        </div>
        <button class="formulas-close" @click="close">✕</button>
      </div>

      <div class="formulas-body">
        <nav class="formulas-sidebar">
          <div class="formulas-search">
            <input v-model="searchQuery" type="text" placeholder="Chercher une formule..." />
          </div>

          <div class="formulas-categories">
            <button
              :class="['formulas-category-btn', { active: selectedCategory === 'all' }]"
              @click="selectCategory('all')"
            >
              <span class="formulas-category-emoji">📋</span>
              <span class="formulas-category-label">Toutes les formules</span>
              <span class="formulas-category-count">({{ allFormules.length }})</span>
            </button>

            <div class="formulas-separator"></div>

            <button
              v-for="cat in categories"
              :key="cat.id"
              :class="['formulas-category-btn', { active: selectedCategory === cat.id }]"
              @click="selectCategory(cat.id)"
            >
              <span class="formulas-category-emoji">{{ cat.emoji }}</span>
              <span class="formulas-category-label">{{ cat.titre }}</span>
              <span class="formulas-category-count">({{ cat.formules.length }})</span>
            </button>
          </div>
        </nav>

        <div class="formulas-content">
          <FormuleDetailPanel
            :formule="formuleSélectionnée"
            :prev-id="formulePrecedente"
            :next-id="formuleSuivante"
            :position="formulasTriees.findIndex(f => f.id === selectedFormuleId) + 1"
            :total="formulasTriees.length"
            @copy="copierFormule"
            @prev="selectedFormuleId = formulePrecedente || selectedFormuleId"
            @next="selectedFormuleId = formuleSuivante || selectedFormuleId"
          />
        </div>
      </div>

      <div class="formulas-footer">
        <button class="formulas-btn formulas-btn-export" @click="exporterPDF">📥 Exporter PDF</button>
        <button class="formulas-btn formulas-btn-close" @click="close">Fermer</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import '../styles/formulas-modal.css'
import FormuleDetailPanel from './FormuleDetailPanel.vue'
import { useFormulasLogic } from '../composables/useFormulasLogic'

interface Props {
  isOpen: boolean
}

defineProps<Props>()
const emit = defineEmits<{ close: [] }>()

const {
  searchQuery,
  selectedCategory,
  selectedFormuleId,
  allFormules,
  selectCategory,
  formulasTriees,
  formuleSélectionnée,
  formulePrecedente,
  formuleSuivante,
  copierFormule,
  categories
} = useFormulasLogic()

const messageExport = ref('')

const close = () => emit('close')

const exporterPDF = () => {
  const contenuHTML = genererHTMLFormules()
  const largeur = window.innerWidth
  const hauteur = window.innerHeight
  
  const fenetre = window.open('', '', `width=${largeur},height=${hauteur}`)
  if (!fenetre) {
    messageExport.value = 'Impossible d\'ouvrir la fenêtre d\'impression. Vérifiez les paramètres de popup.'
    setTimeout(() => { messageExport.value = '' }, 3000)
    return
  }

  fenetre.document.write(contenuHTML)
  fenetre.document.close()
  
  setTimeout(() => {
    fenetre.print()
  }, 250)
}

const genererHTMLFormules = (): string => {
  const listeFormules = selectedCategory.value === 'all' 
    ? allFormules.value 
    : (categories.value.find(c => c.id === selectedCategory.value)?.formules || [])
      .map(id => allFormules.value.find(f => f.id === id))
      .filter(Boolean)

  let html = `
    <!DOCTYPE html>
    <html lang="fr">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Formules & Calculs - Straddle Trading</title>
      <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; color: #333; line-height: 1.6; }
        h1 { text-align: center; margin: 30px 0; color: #2c5aa0; border-bottom: 3px solid #2c5aa0; padding-bottom: 20px; }
        h2 { color: #2c5aa0; margin-top: 20px; margin-bottom: 10px; font-size: 1.3em; }
        h4 { color: #333; margin-top: 15px; margin-bottom: 8px; font-size: 1em; }
        p { color: #666; margin-bottom: 10px; }
        .formule { page-break-inside: avoid; margin-bottom: 25px; border: 1px solid #ddd; padding: 20px; border-radius: 5px; background: #fff; }
        .definition { color: #666; font-style: italic; margin-bottom: 15px; }
        .formula-box { background: #f5f5f5; padding: 12px; border-radius: 3px; font-family: 'Courier New', monospace; font-size: 0.95em; overflow-x: auto; border-left: 4px solid #2c5aa0; margin: 10px 0; }
        ul { margin-left: 25px; color: #666; }
        li { margin-bottom: 5px; }
        .output-box { background: #f0f8ff; padding: 12px; border-left: 4px solid #2c5aa0; margin: 10px 0; }
        .output-box p { margin: 5px 0; }
        .exemple-box { background: #f0fff0; padding: 12px; border-left: 4px solid #10b981; margin: 10px 0; color: #666; }
        .notes-box { background: #fff8f0; padding: 12px; border-left: 4px solid #d97706; margin: 10px 0; }
        .footer { margin-top: 50px; padding-top: 20px; border-top: 2px solid #ddd; text-align: center; color: #999; font-size: 12px; }
        @media print {
          body { background: white; }
          .formule { page-break-inside: avoid; }
          h1 { page-break-after: avoid; }
        }
      </style>
    </head>
    <body>
      <h1>📋 Formules & Calculs - Straddle Trading</h1>
  `

  listeFormules.forEach((formule) => {
    html += `
      <div class="formule">
        <h2>${formule.titre}</h2>
        <p class="definition">${formule.definition}</p>
        
        <h4>🧮 Formule:</h4>
        <div class="formula-box">${formule.formule.replace(/</g, '&lt;').replace(/>/g, '&gt;')}</div>
        
        <h4>📥 Inputs:</h4>
        <ul>
          ${formule.inputs.map(input => `<li>${input}</li>`).join('')}
        </ul>
        
        <h4>📤 Output:</h4>
        <div class="output-box">
          <p><strong>Type:</strong> ${formule.output.type}</p>
          <p><strong>Range:</strong> ${formule.output.range}</p>
          <p><strong>Unité:</strong> ${formule.output.unite}</p>
        </div>
        
        <h4>📊 Exemple Concret:</h4>
        <div class="exemple-box">${formule.exemple}</div>
        
        ${formule.notes.length > 0 ? `
        <h4>⚠️ Notes & Limitations:</h4>
        <div class="notes-box">
          <ul>
            ${formule.notes.map(note => `<li>${note}</li>`).join('')}
          </ul>
        </div>
        ` : ''}
      </div>
    `
  })

  html += `
    <div class="footer">
      <p>Généré depuis <strong>Analyses-historiques</strong> - Volatility Analyzer for Straddle Trading</p>
      <p>Date: ${new Date().toLocaleDateString('fr-FR')} à ${new Date().toLocaleTimeString('fr-FR')}</p>
    </div>
    </body>
    </html>
  `

  return html
}
</script>

