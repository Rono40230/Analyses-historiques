# 📋 PHASE 2 - PLAN DE REFACTORISATION COMPOSANTS VUE

**Status:** ⏳ À faire dans prochain sprint  
**Priorité:** 🟡 MOYENNE (non-bloquant)  
**Effort estimé:** 2-3 heures

---

## 🎯 Objectif

Réduire la taille et la complexité des 4 composants Vue les plus volumineux en les splitant en sous-composants.

---

## 📊 Composants à Refactoriser

### 1. EventCorrelationView.vue (1643L) 🔴

**Responsabilités identifiées:**
- Mode de vue principal (parent, ~50L)
- Vue "By Event" (~130L)
- Vue "By Pair" (~123L)
- Vue "Heatmap" (~1200L)

**Plan de split:**

```
EventCorrelationView.vue (50L parent)
├── EventCorrelationByEvent.vue (130L)
├── EventCorrelationByPair.vue (123L)
└── EventCorrelationHeatmap.vue (1200L) → besoin de split
    ├── HeatmapContent.vue (400L)
    ├── HeatmapLegend.vue (200L)
    └── HeatmapControls.vue (150L)
```

**Méthode d'extraction:**
1. Copier template section 31-160 dans EventCorrelationByEvent.vue
2. Copier template section 160-283 dans EventCorrelationByPair.vue
3. Copier template section 283-EOF dans EventCorrelationHeatmap.vue
4. Extraire les fonctions utilisées par chaque composant
5. Passer les props/emits appropriées
6. Supprimer le template du parent et mettre les composants à la place

### 2. ImportHub.vue (930L) 🟠

**Responsabilités identifiées:**
- Conteneur principal (~50L)
- Section calendriers (~350L)
- Section paires (~400L)
- Modal suppression (~130L)

**Plan de split:**

```
ImportHub.vue (50L parent + modal)
├── CalendarImportSection.vue (350L)
├── PairImportSection.vue (400L)
└── DeleteConfirmationModal.vue (130L)
```

**Bénéfices:**
- Chaque section isolée et testable
- Modal réutilisable ailleurs
- Logique d'import isolée par type

### 3. SessionAnalysisView.vue (921L) 🟠

**Responsabilités identifiées:**
- Vue principale avec graphiques
- Tableau de sessions
- Statistiques détaillées

**Plan de split:**

```
SessionAnalysisView.vue
├── SessionCharts.vue (300L)
├── SessionTable.vue (250L)
└── SessionStats.vue (200L)
```

### 4. AnalysisPanel.vue (800L) 🟡

**Responsabilités identifiées:**
- Vue principale
- Section volatilité
- Section corrélation

**Plan de split:**

```
AnalysisPanel.vue
├── VolatilitySection.vue (300L)
└── CorrelationSection.vue (300L)
```

---

## 🔧 Checklist Refactorisation

### EventCorrelationView.vue

- [ ] Créer EventCorrelationByEvent.vue
  - [ ] Copier template (lignes 31-160)
  - [ ] Extraire fonctions: loadEventImpact, sortEventVolatility
  - [ ] Passer props: pastEventsHigh, pastEventsMedium, eventImpact, loadingEvent
  - [ ] Définir events: @load-event

- [ ] Créer EventCorrelationByPair.vue
  - [ ] Copier template (lignes 160-283)
  - [ ] Extraire fonctions: loadPairHistory, sortTable
  - [ ] Passer props: availablePairs, pairHistory, loadingPair

- [ ] Créer EventCorrelationHeatmap.vue
  - [ ] Copier template (lignes 283-EOF)
  - [ ] Extraire fonctions: getHeatmapValue, getHeatmapClass
  - [ ] Passer props: heatmapData, loadingHeatmap

- [ ] Simplifier parent EventCorrelationView.vue
  - [ ] Garder: mode selector, state management
  - [ ] Utiliser: <EventCorrelationByEvent />, <EventCorrelationByPair />, <EventCorrelationHeatmap />

### ImportHub.vue

- [ ] Créer CalendarImportSection.vue
  - [ ] Template: section "Calendriers"
  - [ ] Fonctions: selectCalendarFiles, confirmDeleteCalendar, deleteCalendar
  - [ ] Props: calendars

- [ ] Créer PairImportSection.vue
  - [ ] Template: section "Paires"
  - [ ] Fonctions: selectPairsFiles, confirmDeletePair, deletePair
  - [ ] Props: pairs

- [ ] Créer DeleteConfirmationModal.vue (RÉUTILISABLE)
  - [ ] Template: modal uniquement
  - [ ] Props: show, message, type
  - [ ] Events: @confirm, @cancel

- [ ] Simplifier ImportHub.vue
  - [ ] Garder: layout, state management global
  - [ ] Utiliser: <CalendarImportSection />, <PairImportSection />, <DeleteConfirmationModal />

### SessionAnalysisView.vue

- [ ] Analyser structure et identifier sections
- [ ] Créer 3 sous-composants
- [ ] Refactoriser état partagé

### AnalysisPanel.vue

- [ ] Analyser structure
- [ ] Créer 2 sous-composants
- [ ] Partager données via props/emits

---

## ✅ Critères d'Acceptation

Après refactorisation Phase 2:

- ✅ Tous les composants <400L
- ✅ Chaque composant = 1 responsabilité claire
- ✅ Pas de duplication logic/template
- ✅ Props et emits bien documentés
- ✅ Fonctionnalité identique (zéro changement logique)
- ✅ Tests Vue à jour

---

## 📍 Notes Importantes

1. **C'est optionnel** - Le code fonctionne correctement
2. **À faire dans prochain sprint** - Non bloquant pour production
3. **Très consommateur de temps** - 2-3 heures pour tout faire
4. **Bénéfices:** Meilleure maintenabilité, testabilité, réutilisabilité

---

## 🚀 Prochaines Étapes

1. ✅ Phase 1: COMPLÉTÉE (Rust refactorisation)
2. ⏳ Phase 2: À planifier (Vue refactorisation)
3. ⏳ Phase 3: À identifier (autres optimisations)

