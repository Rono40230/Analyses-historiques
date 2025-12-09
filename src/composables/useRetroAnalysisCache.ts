import { ref } from 'vue'

// Gestion centralisée du cache rétrospectif
const retroAnalysisCache = ref<Map<string, any>>(new Map())

export function useRetroAnalysisCache() {
  function cacheRetroAnalysis(pair: string, eventType: string, data: any) {
    const key = `${pair}_${eventType}`
    retroAnalysisCache.value.set(key, data)
  }

  function getRetroAnalysisCache(pair: string, eventType: string) {
    const key = `${pair}_${eventType}`
    return retroAnalysisCache.value.get(key) || null
  }

  function clearRetroAnalysisCache() {
    retroAnalysisCache.value.clear()
  }

  function clearRetroAnalysisCacheForPair(pair: string) {
    const keysToDelete: string[] = []
    retroAnalysisCache.value.forEach((_, key) => {
      if (key.startsWith(`${pair}_`)) {
        keysToDelete.push(key)
      }
    })
    keysToDelete.forEach(key => retroAnalysisCache.value.delete(key))
  }

  return {
    retroAnalysisCache,
    cacheRetroAnalysis,
    getRetroAnalysisCache,
    clearRetroAnalysisCache,
    clearRetroAnalysisCacheForPair,
  }
}
