#!/bin/bash
# check-frontend-quality.sh - Audit complet de la qualité du code Frontend
# Vérifie: console.log, any types, code mort, imports circulaires, etc.

echo "🎨 AUDIT QUALITÉ FRONTEND (Vue.js/TypeScript)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

EXIT_CODE=0
VIOLATIONS=0
WARNINGS=0

# Couleurs
RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# ═══════════════════════════════════════════════════════════════
# 1. CONSOLE.LOG EN PRODUCTION (CRITIQUE)
# ═══════════════════════════════════════════════════════════════

echo -e "${BLUE}1️⃣  Vérification console.log() en production${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

CONSOLE_LOGS=$(grep -rn "console\.log" src/ --include="*.vue" --include="*.ts" --include="*.js" 2>/dev/null || true)
CONSOLE_COUNT=$(echo "$CONSOLE_LOGS" | grep -c "console\.log" || echo "0")

if [ "$CONSOLE_COUNT" -gt 0 ]; then
    echo -e "${RED}❌ $CONSOLE_COUNT console.log() trouvés en production !${NC}"
    echo ""
    echo "Fichiers concernés:"
    echo "$CONSOLE_LOGS" | head -20
    if [ "$CONSOLE_COUNT" -gt 20 ]; then
        echo "... et $((CONSOLE_COUNT - 20)) autres"
    fi
    echo ""
    ((VIOLATIONS += CONSOLE_COUNT))
    EXIT_CODE=1
else
    echo -e "${GREEN}✅ Aucun console.log() trouvé${NC}"
fi
echo ""

# ═══════════════════════════════════════════════════════════════
# 2. DEBUGGER STATEMENTS
# ═══════════════════════════════════════════════════════════════

echo -e "${BLUE}2️⃣  Vérification debugger statements${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

DEBUGGERS=$(grep -rn "debugger" src/ --include="*.vue" --include="*.ts" --include="*.js" 2>/dev/null || true)
DEBUGGER_COUNT=$(echo "$DEBUGGERS" | grep -c "debugger" || echo "0")

if [ "$DEBUGGER_COUNT" -gt 0 ]; then
    echo -e "${RED}❌ $DEBUGGER_COUNT debugger statement(s) trouvé(s) !${NC}"
    echo "$DEBUGGERS"
    echo ""
    ((VIOLATIONS += DEBUGGER_COUNT))
    EXIT_CODE=1
else
    echo -e "${GREEN}✅ Aucun debugger statement${NC}"
fi
echo ""

# ═══════════════════════════════════════════════════════════════
# 3. TYPES ANY EN TYPESCRIPT
# ═══════════════════════════════════════════════════════════════

echo -e "${BLUE}3️⃣  Vérification types 'any' en TypeScript${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

ANY_TYPES=$(grep -rn ": any" src/ --include="*.ts" --include="*.vue" 2>/dev/null | grep -v "node_modules" || true)
ANY_COUNT=$(echo "$ANY_TYPES" | grep -c ": any" || echo "0")

if [ "$ANY_COUNT" -gt 0 ]; then
    echo -e "${YELLOW}⚠️  $ANY_COUNT type(s) 'any' trouvé(s) (devrait être typé)${NC}"
    echo ""
    echo "Occurrences:"
    echo "$ANY_TYPES" | head -10
    if [ "$ANY_COUNT" -gt 10 ]; then
        echo "... et $((ANY_COUNT - 10)) autres"
    fi
    echo ""
    ((WARNINGS += ANY_COUNT))
else
    echo -e "${GREEN}✅ Tous les types sont définis (pas de 'any')${NC}"
fi
echo ""

# ═══════════════════════════════════════════════════════════════
# 4. IMPORTS INUTILISÉS (CODE MORT)
# ═══════════════════════════════════════════════════════════════

echo -e "${BLUE}4️⃣  Vérification imports inutilisés${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Vérifier si ESLint est disponible
if command -v npx &> /dev/null; then
    if [ -f "package.json" ] && grep -q "eslint" package.json; then
        echo "Utilisation d'ESLint pour détecter les imports inutilisés..."
        UNUSED_IMPORTS=$(npx eslint src/ --format=compact 2>/dev/null | grep "is defined but never used" || true)
        UNUSED_COUNT=$(echo "$UNUSED_IMPORTS" | grep -c "is defined but never used" || echo "0")
        
        if [ "$UNUSED_COUNT" -gt 0 ]; then
            echo -e "${YELLOW}⚠️  $UNUSED_COUNT import(s)/variable(s) inutilisé(s)${NC}"
            echo "$UNUSED_IMPORTS" | head -10
            if [ "$UNUSED_COUNT" -gt 10 ]; then
                echo "... et $((UNUSED_COUNT - 10)) autres"
            fi
            ((WARNINGS += UNUSED_COUNT))
        else
            echo -e "${GREEN}✅ Aucun import inutilisé détecté${NC}"
        fi
    else
        echo -e "${YELLOW}⚠️  ESLint non configuré, vérification ignorée${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  npm/npx non disponible, vérification ignorée${NC}"
fi
echo ""

# ═══════════════════════════════════════════════════════════════
# 5. TODO NON FORMATÉS
# ═══════════════════════════════════════════════════════════════

echo -e "${BLUE}5️⃣  Vérification TODO formatés${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

BAD_TODOS=$(grep -rn "TODO" src/ --include="*.vue" --include="*.ts" --include="*.js" 2>/dev/null | grep -v "TODO(" || true)
TODO_COUNT=$(echo "$BAD_TODOS" | grep -c "TODO" || echo "0")

if [ "$TODO_COUNT" -gt 0 ]; then
    echo -e "${YELLOW}⚠️  $TODO_COUNT TODO non formaté(s) (devrait être 'TODO(nom): description')${NC}"
    echo "$BAD_TODOS" | head -5
    if [ "$TODO_COUNT" -gt 5 ]; then
        echo "... et $((TODO_COUNT - 5)) autres"
    fi
    echo ""
    ((WARNINGS += TODO_COUNT))
else
    echo -e "${GREEN}✅ Tous les TODO sont bien formatés${NC}"
fi
echo ""

# ═══════════════════════════════════════════════════════════════
# 6. TRY/CATCH VIDES
# ═══════════════════════════════════════════════════════════════

echo -e "${BLUE}6️⃣  Vérification try/catch vides${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Rechercher les catch vides ou qui ne font que console.log
EMPTY_CATCH=$(grep -rn "catch.*{" src/ --include="*.vue" --include="*.ts" --include="*.js" -A 2 2>/dev/null | grep -B 1 "^\s*}$" || true)
EMPTY_CATCH_COUNT=$(echo "$EMPTY_CATCH" | grep -c "catch" || echo "0")

if [ "$EMPTY_CATCH_COUNT" -gt 0 ]; then
    echo -e "${YELLOW}⚠️  Possibles catch vides ou mal gérés détectés${NC}"
    echo "Vérification manuelle recommandée"
    ((WARNINGS += EMPTY_CATCH_COUNT))
else
    echo -e "${GREEN}✅ Pas de catch vide évident détecté${NC}"
fi
echo ""

# ═══════════════════════════════════════════════════════════════
# 7. COMPOSANTS VUE NON UTILISÉS
# ═══════════════════════════════════════════════════════════════

echo -e "${BLUE}7️⃣  Vérification composants Vue potentiellement inutilisés${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Liste tous les composants
COMPONENTS=$(find src/components -name "*.vue" -type f 2>/dev/null | xargs -I {} basename {} .vue)
UNUSED_COMPONENTS=""
UNUSED_COMP_COUNT=0

for comp in $COMPONENTS; do
    # Chercher les usages du composant (import ou balise)
    USAGE_COUNT=$(grep -r "$comp" src/ --include="*.vue" --include="*.ts" --include="*.js" 2>/dev/null | grep -v "src/components/$comp.vue" | wc -l)
    
    if [ "$USAGE_COUNT" -eq 0 ]; then
        UNUSED_COMPONENTS="$UNUSED_COMPONENTS\n  - $comp.vue"
        ((UNUSED_COMP_COUNT++))
    fi
done

if [ "$UNUSED_COMP_COUNT" -gt 0 ]; then
    echo -e "${YELLOW}⚠️  $UNUSED_COMP_COUNT composant(s) potentiellement inutilisé(s):${NC}"
    echo -e "$UNUSED_COMPONENTS"
    echo ""
    echo "Note: Vérification manuelle recommandée (peut être utilisé dynamiquement)"
    ((WARNINGS += UNUSED_COMP_COUNT))
else
    echo -e "${GREEN}✅ Tous les composants semblent utilisés${NC}"
fi
echo ""

# ═══════════════════════════════════════════════════════════════
# 8. ALERT() EN PRODUCTION
# ═══════════════════════════════════════════════════════════════

echo -e "${BLUE}8️⃣  Vérification alert() en production${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

ALERTS=$(grep -rn "alert(" src/ --include="*.vue" --include="*.ts" --include="*.js" 2>/dev/null || true)
ALERT_COUNT=$(echo "$ALERTS" | grep -c "alert(" || echo "0")

if [ "$ALERT_COUNT" -gt 0 ]; then
    echo -e "${RED}❌ $ALERT_COUNT alert() trouvé(s) en production !${NC}"
    echo "$ALERTS"
    echo ""
    ((VIOLATIONS += ALERT_COUNT))
    EXIT_CODE=1
else
    echo -e "${GREEN}✅ Aucun alert() trouvé${NC}"
fi
echo ""

# ═══════════════════════════════════════════════════════════════
# RÉSUMÉ
# ═══════════════════════════════════════════════════════════════

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 RÉSUMÉ DE L'AUDIT FRONTEND"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ $VIOLATIONS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✅ Code frontend impeccable !${NC}"
    echo "   Aucune violation, aucun warning"
elif [ $VIOLATIONS -eq 0 ]; then
    echo -e "${YELLOW}⚠️  $WARNINGS warning(s) détecté(s)${NC}"
    echo -e "${GREEN}✅ Aucune violation critique${NC}"
    echo ""
    echo "Recommandation: Corriger les warnings pour un code optimal"
else
    echo -e "${RED}❌ $VIOLATIONS violation(s) critique(s) détectée(s)${NC}"
    if [ $WARNINGS -gt 0 ]; then
        echo -e "${YELLOW}⚠️  $WARNINGS warning(s) supplémentaire(s)${NC}"
    fi
    echo ""
    echo "Action requise: Corriger les violations avant mise en production"
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

exit $EXIT_CODE
