# ✅ CURSEUR TRIABLE - EN-TÊTES DE COLONNE

## Modifications Appliquées

### HTML (Vue)
**Fichier**: `src/components/EventCorrelationView.vue` ligne 203-209

Avant:
```html
<th>Date</th>
<th>Événement</th>
<th>Impact</th>
<th>Volatilité</th>
<th>vs Baseline</th>
<th>Direction</th>
```

Après:
```html
<th class="sortable">Date</th>
<th class="sortable">Événement</th>
<th class="sortable">Impact</th>
<th class="sortable">Volatilité</th>
<th class="sortable">vs Baseline</th>
<th class="sortable">Direction</th>
```

### CSS
**Fichier**: `src/components/EventCorrelationView.vue` ligne 1157-1170

```css
.history-table th.sortable {
  cursor: pointer;  /* ✅ Curseur main qui change au survol */
  user-select: none;  /* Empêche la sélection de texte */
  transition: background-color 0.2s ease;
}

.history-table th.sortable:hover {
  background-color: rgba(255, 255, 255, 0.1);  /* Effet hover subtil */
}
```

---

## Résultat Visual

### Avant:
- En-têtes: texte simple, pas d'indication du tri
- Au survol: rien ne change

### Après:
- En-têtes: **curseur `pointer`** apparaît au survol 👆
- Au survol: fond s'éclaircit légèrement
- Indication claire: "cette colonne peut être triée"

---

## Détails Techniques

| Propriété CSS | Effet |
|--------------|--------|
| `cursor: pointer` | Change le curseur en main pour indiquer le tri |
| `user-select: none` | Empêche la sélection accidentelle du texte |
| `transition: 0.2s` | Animation douce du hover effect |
| `rgba(..., 0.1)` | Léger éclaircissement du fond (10% opacité) |

---

## État

✅ **Terminé** - Les en-têtes affichent maintenant un curseur pointeur pour indiquer qu'elles sont triables.

Note: Le tri réel (croissant/décroissant) dépend de l'implémentation JavaScript. 
Actuellement, c'est juste l'**indication visuelle** que les colonnes sont triables.
