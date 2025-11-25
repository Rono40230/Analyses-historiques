<template>
  <div class="loading-container">
    <div class="ai-brain">
      <div class="brain-pulse" />
      <div class="brain-core">
        🧠
      </div>
    </div>
    <h3 class="loading-title">
      {{ loadingStep }}
    </h3>
    <div class="progress-bar">
      <div
        class="progress-fill"
        :style="{ width: progress + '%' }"
      />
    </div>
    <div class="loading-log">
      <p
        v-for="(log, index) in logs"
        :key="index"
        class="log-item"
      >
        {{ log }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  loadingStep: string
  progress: number
  logs: string[]
}>()
</script>

<style scoped>
.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  text-align: center;
}

.ai-brain {
  position: relative;
  width: 80px;
  height: 80px;
  margin-bottom: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.brain-core {
  font-size: 40px;
  z-index: 2;
  position: relative;
}

.brain-pulse {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  border-radius: 50%;
  background: rgba(78, 205, 196, 0.2);
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0% { transform: scale(0.8); opacity: 0.5; }
  50% { transform: scale(1.2); opacity: 0.2; }
  100% { transform: scale(0.8); opacity: 0.5; }
}

.loading-title {
  font-size: 18px;
  color: #4ecdc4;
  margin-bottom: 20px;
  font-weight: 600;
}

.progress-bar {
  width: 100%;
  max-width: 400px;
  height: 6px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 3px;
  overflow: hidden;
  margin-bottom: 20px;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #4ecdc4, #2d9ca6);
  transition: width 0.3s ease;
}

.loading-log {
  height: 100px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  opacity: 0.7;
}

.log-item {
  font-size: 12px;
  color: #a0aec0;
  margin: 0;
}
</style>
