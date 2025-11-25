#!/bin/bash
# analyze-dead-code.sh - Analyse le code mort SANS RIEN SUPPRIMER
# Génère un rapport pour validation manuelle

echo "🔍 ANALYSE DU CODE MORT (LECTURE SEULE)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "⚠️  Ce script NE SUPPRIME RIEN - Il génère seulement un rapport"
echo ""

REPORT_FILE="RAPPORT_CODE_MORT.md"

# Créer le rapport
cat > "$REPORT_FILE" << 'HEADER'
# 🧹 RAPPORT D'ANALYSE DU CODE MORT

Date: $(date +%Y-%m-%d\ %H:%M:%S)

⚠️ **ATTENTION** : Ce rapport identifie du code POTENTIELLEMENT mort.
Chaque élément doit être **validé manuellement** avant suppression.

---

## 📊 RÉSUMÉ

HEADER

echo "📊 Génération du rapport..."
echo ""

# ═══════════════════════════════════════════════════════════════
# 1. COMPOSANTS VUE POTENTIELLEMENT INUTILISÉS
# ═══════════════════════════════════════════════════════════════

echo "1️⃣  Analyse des composants Vue..."

cat >> "$REPORT_FILE" << 'SECTION1'

## 1️⃣ COMPOSANTS VUE POTENTIELLEMENT INUTILISÉS

### Méthodologie :
- Recherche des imports et usages de chaque composant
- ⚠️ ATTENTION : Peut avoir des faux positifs si :
  - Composant utilisé dynamiquement (component :is="...")
  - Composant importé dans router
  - Composant utilisé dans template sans import explicite

SECTION1

UNUSED_COMPONENTS=0
TOTAL_COMPONENTS=0

if [ -d "src/components" ]; then
    while IFS= read -r comp_file; do
        ((TOTAL_COMPONENTS++))
        comp_name=$(basename "$comp_file" .vue)
        
        # Chercher les usages (import ou balise)
        USAGE_COUNT=$(grep -r "$comp_name" src/ --include="*.vue" --include="*.ts" --include="*.js" 2>/dev/null | \
                      grep -v "src/components/$comp_name.vue" | \
                      grep -v "node_modules" | \
                      wc -l)
        
        if [ "$USAGE_COUNT" -eq 0 ]; then
            ((UNUSED_COMPONENTS++))
            
            # Vérifier la taille du fichier
            LINES=$(wc -l < "$comp_file")
            
            echo "### ❌ \`$comp_name.vue\` ($LINES lignes)" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
            echo "**Usages trouvés :** 0" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
            echo "**Action suggérée :**" >> "$REPORT_FILE"
            
            # Vérifier si c'est un fichier OLD
            if [[ "$comp_name" == *"_OLD"* ]] || [[ "$comp_name" == *"Old"* ]]; then
                echo "- 🔴 **SUPPRIMER** (fichier marqué OLD)" >> "$REPORT_FILE"
            else
                echo "- ⚠️ **VÉRIFIER MANUELLEMENT** (peut être utilisé dynamiquement)" >> "$REPORT_FILE"
            fi
            
            echo "" >> "$REPORT_FILE"
            echo "**Commande pour vérifier :**" >> "$REPORT_FILE"
            echo "\`\`\`bash" >> "$REPORT_FILE"
            echo "grep -r \"$comp_name\" src/ --include=\"*.vue\" --include=\"*.ts\" --include=\"*.js\"" >> "$REPORT_FILE"
            echo "\`\`\`" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
            echo "---" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
        fi
    done < <(find src/components -name "*.vue" -type f 2>/dev/null)
fi

echo "**Total composants :** $TOTAL_COMPONENTS" >> "$REPORT_FILE"
echo "**Potentiellement inutilisés :** $UNUSED_COMPONENTS" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# ═══════════════════════════════════════════════════════════════
# 2. STORES PINIA POTENTIELLEMENT INUTILISÉS
# ═══════════════════════════════════════════════════════════════

echo "2️⃣  Analyse des stores Pinia..."

cat >> "$REPORT_FILE" << 'SECTION2'

## 2️⃣ STORES PINIA POTENTIELLEMENT INUTILISÉS

### Méthodologie :
- Recherche des imports de chaque store
- ⚠️ Stores de données (Translations, Schedules) exclus

SECTION2

UNUSED_STORES=0
TOTAL_STORES=0

if [ -d "src/stores" ]; then
    while IFS= read -r store_file; do
        store_name=$(basename "$store_file" .ts)
        
        # Exclure les stores de données
        if [[ "$store_name" == *"Translation"* ]] || [[ "$store_name" == *"Schedule"* ]]; then
            continue
        fi
        
        ((TOTAL_STORES++))
        
        # Chercher les imports
        USAGE_COUNT=$(grep -r "from.*stores.*$store_name" src/ --include="*.vue" --include="*.ts" --include="*.js" 2>/dev/null | \
                      grep -v "src/stores/$store_name" | \
                      wc -l)
        
        if [ "$USAGE_COUNT" -eq 0 ]; then
            ((UNUSED_STORES++))
            LINES=$(wc -l < "$store_file")
            
            echo "### ❌ \`$store_name.ts\` ($LINES lignes)" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
            echo "**Imports trouvés :** 0" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
            echo "**Action suggérée :** ⚠️ **VÉRIFIER MANUELLEMENT**" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
            echo "---" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
        fi
    done < <(find src/stores -name "*.ts" -type f 2>/dev/null)
fi

echo "**Total stores :** $TOTAL_STORES" >> "$REPORT_FILE"
echo "**Potentiellement inutilisés :** $UNUSED_STORES" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# ═══════════════════════════════════════════════════════════════
# 3. FICHIERS UTILS POTENTIELLEMENT INUTILISÉS
# ═══════════════════════════════════════════════════════════════

echo "3️⃣  Analyse des fichiers utils..."

cat >> "$REPORT_FILE" << 'SECTION3'

## 3️⃣ FICHIERS UTILS POTENTIELLEMENT INUTILISÉS

### Méthodologie :
- Recherche des imports de chaque fichier utils
- ⚠️ Fichiers de données (Translations, Schedules) exclus

SECTION3

UNUSED_UTILS=0
TOTAL_UTILS=0

if [ -d "src/utils" ]; then
    while IFS= read -r util_file; do
        util_name=$(basename "$util_file" .ts)
        
        # Exclure les fichiers de données
        if [[ "$util_name" == *"Translation"* ]] || [[ "$util_name" == *"Schedule"* ]]; then
            continue
        fi
        
        ((TOTAL_UTILS++))
        
        # Chercher les imports
        USAGE_COUNT=$(grep -r "from.*utils.*$util_name" src/ --include="*.vue" --include="*.ts" --include="*.js" 2>/dev/null | \
                      grep -v "src/utils/$util_name" | \
                      wc -l)
        
        if [ "$USAGE_COUNT" -eq 0 ]; then
            ((UNUSED_UTILS++))
            LINES=$(wc -l < "$util_file")
            
            echo "### ❌ \`$util_name.ts\` ($LINES lignes)" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
            echo "**Imports trouvés :** 0" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
            echo "**Action suggérée :** ⚠️ **VÉRIFIER MANUELLEMENT**" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
            echo "---" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
        fi
    done < <(find src/utils -name "*.ts" -o -name "*.js" -type f 2>/dev/null)
fi

echo "**Total utils :** $TOTAL_UTILS" >> "$REPORT_FILE"
echo "**Potentiellement inutilisés :** $UNUSED_UTILS" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# ═══════════════════════════════════════════════════════════════
# 4. RÉSUMÉ ET RECOMMANDATIONS
# ═══════════════════════════════════════════════════════════════

cat >> "$REPORT_FILE" << SUMMARY

---

## 📊 RÉSUMÉ GLOBAL

| Catégorie | Total | Potentiellement inutilisés | % |
|-----------|-------|---------------------------|---|
| Composants Vue | $TOTAL_COMPONENTS | $UNUSED_COMPONENTS | $(( TOTAL_COMPONENTS > 0 ? UNUSED_COMPONENTS * 100 / TOTAL_COMPONENTS : 0 ))% |
| Stores Pinia | $TOTAL_STORES | $UNUSED_STORES | $(( TOTAL_STORES > 0 ? UNUSED_STORES * 100 / TOTAL_STORES : 0 ))% |
| Fichiers Utils | $TOTAL_UTILS | $UNUSED_UTILS | $(( TOTAL_UTILS > 0 ? UNUSED_UTILS * 100 / TOTAL_UTILS : 0 ))% |

---

## 🎯 PLAN D'ACTION RECOMMANDÉ

### ÉTAPE 1 : VALIDATION MANUELLE

Pour chaque fichier marqué ❌, vérifier :

1. **Recherche globale :**
   \`\`\`bash
   grep -r "NomDuFichier" src/ --include="*.vue" --include="*.ts" --include="*.js"
   \`\`\`

2. **Vérifier le router :**
   \`\`\`bash
   grep -r "NomDuFichier" src/router/ 2>/dev/null
   \`\`\`

3. **Vérifier les imports dynamiques :**
   \`\`\`bash
   grep -r "component.*is.*NomDuFichier" src/ --include="*.vue"
   \`\`\`

### ÉTAPE 2 : SUPPRESSION SÉCURISÉE

**Pour les fichiers OLD (haute confiance) :**
\`\`\`bash
# Exemple : AnalysisPanel_OLD.vue
git rm src/components/AnalysisPanel_OLD.vue
git commit -m "chore: suppression fichier obsolète AnalysisPanel_OLD.vue"
\`\`\`

**Pour les autres (moyenne confiance) :**
1. Créer une branche : \`git checkout -b cleanup/remove-unused-components\`
2. Supprimer le fichier
3. Tester l'application complètement
4. Si OK → commit, sinon → git restore

### ÉTAPE 3 : DÉTECTION FINE (ESLint)

Pour détecter les imports/variables inutilisés DANS les fichiers :

\`\`\`bash
# Installer ESLint + plugin
npm install --save-dev eslint eslint-plugin-unused-imports

# Lancer ESLint
npx eslint src/ --ext .vue,.ts,.js
\`\`\`

---

## ⚠️ AVERTISSEMENTS

1. **NE PAS supprimer automatiquement** - Toujours vérifier manuellement
2. **Tester après chaque suppression** - Lancer l'app et tester les fonctionnalités
3. **Utiliser git** - Commit après chaque suppression pour pouvoir revenir en arrière
4. **Faux positifs possibles** :
   - Composants utilisés dynamiquement
   - Imports dans fichiers de configuration
   - Code utilisé uniquement en dev/test

---

## 📝 CHECKLIST DE VALIDATION

Avant de supprimer un fichier, cocher :

- [ ] Recherche globale effectuée (grep)
- [ ] Vérifié dans router
- [ ] Vérifié imports dynamiques
- [ ] Testé l'application sans le fichier
- [ ] Commit créé pour pouvoir revenir en arrière
- [ ] Tests passent (si applicables)

---

*Rapport généré le $(date +%Y-%m-%d\ %H:%M:%S)*

SUMMARY

# ═══════════════════════════════════════════════════════════════
# AFFICHAGE RÉSUMÉ
# ═══════════════════════════════════════════════════════════════

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ RAPPORT GÉNÉRÉ : $REPORT_FILE"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📊 RÉSUMÉ :"
echo "  - Composants Vue : $UNUSED_COMPONENTS/$TOTAL_COMPONENTS potentiellement inutilisés"
echo "  - Stores Pinia : $UNUSED_STORES/$TOTAL_STORES potentiellement inutilisés"
echo "  - Fichiers Utils : $UNUSED_UTILS/$TOTAL_UTILS potentiellement inutilisés"
echo ""
echo "⚠️  IMPORTANT :"
echo "  - Ce rapport identifie du code POTENTIELLEMENT mort"
echo "  - Chaque fichier doit être validé MANUELLEMENT"
echo "  - Suivre le plan d'action dans le rapport"
echo ""
echo "📖 Lire le rapport complet : $REPORT_FILE"
echo ""

exit 0
