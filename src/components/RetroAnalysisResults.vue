<script setup lang="ts">
import { computed } from 'vue'
import { formatPointsWithPips } from '../utils/pipConverter'

interface Props {
  peakDelay: number
  decayTimeout: number
  peakAtr: number
  decayRate: number
  decaySpeed: string
  confidence: number
  eventCount: number
  isArchiveMode?: boolean
  eventLabel?: string
}

const props = defineProps<Props>()

defineEmits<{ archive: [] }>()

const totalDuration = computed(() => props.peakDelay + props.decayTimeout)
const maxExit = computed(() => Math.ceil(props.decayTimeout * 1.5))
</script>

<style scoped>
.results-container { display: flex; flex-direction: column; gap: 20px; width: 100%; height: 100%; overflow-y: auto; flex: 1; }
.graph-section { background: #161b22; padding: 20px; border-radius: 8px; border: 1px solid #30363d; flex-shrink: 0; height: 65%; min-height: 450px; }
.graph-section h3 { margin: 0 0 20px 0; color: #58a6ff; font-size: 1.1em; }
.graph-container { width: 100%; background: #0d1117; border-radius: 6px; padding: 10px; height: 100%; }
.graph { width: 100%; max-width: 100%; aspect-ratio: auto; }
.analysis-grid-2-cols { display: grid; grid-template-columns: repeat(2, 1fr); gap: 15px; width: 100%; flex-shrink: 0; height: 30%; min-height: 180px; }
.interpretation-block { background: #161b22; padding: 20px; border-radius: 8px; border: 1px solid #30363d; border-left: 3px solid #58a6ff; }
.interpretation-block p { margin: 0 0 15px 0; color: #58a6ff; font-weight: 600; font-size: 1em; }
.interpretation-block ul { margin: 0; padding-left: 20px; list-style: none; }
.interpretation-block li { margin: 10px 0; color: #c9d1d9; line-height: 1.5; font-size: 0.95em; }

.btn-archive { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border: none; padding: 10px 20px; border-radius: 6px; font-weight: 600; cursor: pointer; transition: all 0.2s; font-size: 0.95em; }
.btn-archive:hover { transform: translateY(-2px); box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4); }
.btn-archive:active { transform: translateY(0); }
@media (max-width: 768px) { .analysis-grid-2-cols { grid-template-columns: 1fr; } }
</style>
