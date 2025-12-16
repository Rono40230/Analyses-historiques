<template>
  <div v-if="isOpen" class="modal-overlay" @click.self="close">
    <div class="modal-content preview-modal">
      <div class="modal-header">
        <div class="header-title">👁️ Prévisualisation PDF</div>
        <button class="close-btn" @click="close">✕</button>
      </div>

      <div class="modal-section preview-container">
        <iframe v-if="pdfUrl" :src="pdfUrl" class="pdf-preview" title="Aperçu PDF"></iframe>
        <div v-else class="loading-preview">Chargement de l'aperçu...</div>
      </div>

      <div class="modal-footer">
        <button class="btn-secondary" @click="close">Fermer</button>
        <button class="btn-primary" @click="download">
          💾 Télécharger
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  isOpen: boolean
  pdfUrl: string | null
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'download'): void
}>()

function close() {
  emit('close')
}

function download() {
  emit('download')
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

.modal-content.preview-modal {
  background: #1e293b;
  border-radius: 12px;
  width: 90%;
  height: 90%;
  max-width: 1200px;
  display: flex;
  flex-direction: column;
  border: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
}

.modal-header {
  padding: 1rem 1.5rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: #f8fafc;
}

.close-btn {
  background: none;
  border: none;
  color: #94a3b8;
  font-size: 1.5rem;
  cursor: pointer;
  padding: 0.25rem;
  transition: color 0.2s;
}

.close-btn:hover {
  color: #f8fafc;
}

.modal-section.preview-container {
  flex: 1;
  padding: 0;
  overflow: hidden;
  background: #334155;
  position: relative;
}

.pdf-preview {
  width: 100%;
  height: 100%;
  border: none;
}

.loading-preview {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  color: #94a3b8;
}

.modal-footer {
  padding: 1rem 1.5rem;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
}

.btn-primary {
  background: #3b82f6;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-primary:hover {
  background: #2563eb;
}

.btn-secondary {
  background: transparent;
  border: 1px solid #475569;
  color: #cbd5e1;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary:hover {
  background: #334155;
  border-color: #64748b;
  color: #f8fafc;
}
</style>
