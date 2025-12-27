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

            <template v-for="cat in categories" :key="cat.id">
              <div v-if="cat.id === 'conversion'" class="formulas-separator"></div>
              <button
                :class="['formulas-category-btn', { active: selectedCategory === cat.id }]"
                @click="selectCategory(cat.id)"
              >
                <span class="formulas-category-emoji">{{ cat.emoji }}</span>
                <span class="formulas-category-label">{{ cat.titre }}</span>
                <span v-if="cat.id !== 'conversion' && cat.id !== 'spread_cost'" class="formulas-category-count">({{ cat.formules.length }})</span>
              </button>
            </template>
          </div>
        </nav>

        <div class="formulas-content">
          <ConversionTable v-if="selectedCategory === 'conversion'" />
          <SpreadCostTable v-else-if="selectedCategory === 'spread_cost'" />
          <FormuleDetailPanel
            v-else
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
        <button 
          v-if="selectedCategory !== 'conversion' && selectedCategory !== 'spread_cost'"
          class="formulas-btn formulas-btn-export" 
          @click="exporterPDF"
        >
          📥 Exporter PDF
        </button>
        <button class="formulas-btn formulas-btn-close" @click="close">Fermer</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { save } from '@tauri-apps/plugin-dialog'
import '../styles/formulas-modal.css'
import FormuleDetailPanel from './FormuleDetailPanel.vue'
import ConversionTable from './ConversionTable.vue'
import SpreadCostTable from './SpreadCostTable.vue'
import { useFormulasLogic } from '../composables/useFormulasLogic'
import type { Formule } from '../data/formules'

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
const exportEnCours = ref(false)

const close = () => emit('close')

const exporterPDF = async () => {
  if (exportEnCours.value) return
  exportEnCours.value = true
  messageExport.value = 'Préparation de l\'export...'

  const listeFormules = selectedCategory.value === 'all' 
    ? allFormules.value 
    : (categories.value.find(c => c.id === selectedCategory.value)?.formules || [])
      .map(id => allFormules.value.find(f => f.id === id))
      .filter((f): f is Formule => f !== undefined)

  // Convertir au format attendu par la commande Rust
  const formulesData = listeFormules.map(f => ({
    titre: f.titre,
    definition: f.definition,
    formule: f.formule,
    inputs: f.inputs,
    output: f.output,
    exemple: f.exemple,
    notes: f.notes
  }))

  // Dialogue de sélection de fichier
  const filePath = await save({
    defaultPath: `Formules_Straddle_${new Date().toISOString().split('T')[0]}.pdf`,
    filters: [
      { name: 'PDF', extensions: ['pdf'] }
    ]
  })

  if (!filePath) {
    messageExport.value = ''
    exportEnCours.value = false
    return
  }

  try {
    messageExport.value = 'Génération du PDF...'
    const resultat = await invoke<string>('exporter_formules_pdf', {
      formules: formulesData,
      fichierSortie: filePath
    })

    messageExport.value = '✅ PDF exporté avec succès!'
    setTimeout(() => {
      messageExport.value = ''
    }, 3000)
  } catch (erreur) {
    messageExport.value = `❌ Erreur: ${String(erreur)}`
    setTimeout(() => {
      messageExport.value = ''
    }, 5000)
  } finally {
    exportEnCours.value = false
  }
}
</script>

