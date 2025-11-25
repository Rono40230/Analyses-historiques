<template>
  <div class="error-container">
    <div class="error-icon">
      ❌
    </div>
    <h3>Oups, une erreur est survenue</h3>
    <p>{{ error }}</p>
    <button
      class="retry-btn"
      @click="$emit('retry')"
    >
      Réessayer
    </button>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  error: string
}>()

defineEmits<{
  (e: 'retry'): void
}>()
</script>

<style scoped>
.error-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  text-align: center;
}

.error-icon {
  font-size: 48px;
  margin-bottom: 20px;
  background: rgba(239, 68, 68, 0.1);
  width: 80px;
  height: 80px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.error-container h3 {
  font-size: 20px;
  color: #ef4444;
  margin-bottom: 12px;
}

.error-container p {
  color: #a0aec0;
  margin-bottom: 24px;
  max-width: 400px;
}

.retry-btn {
  background: #ef4444;
  color: #fff;
  border: none;
  padding: 10px 24px;
  border-radius: 6px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.retry-btn:hover {
  background: #dc2626;
  transform: translateY(-2px);
}
</style>
