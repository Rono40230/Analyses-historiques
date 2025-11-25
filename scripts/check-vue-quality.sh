#!/bin/bash
# check-vue-quality.sh - Vérification qualité code Vue/TypeScript
# Règle 10.5 : Interdictions strictes en production

set -e

EXIT_CODE=0

echo "🔍 Vérification qualité Frontend (Vue.js/TypeScript)..."
echo ""

# 1. Vérifier console.log
echo "1️⃣  Recherche de console.log()..."
if grep -r "console\.log" src/ --include="*.vue" --include="*.ts" --include="*.tsx" 2>/dev/null | grep -v "node_modules" > /dev/null; then
    echo "   ❌ ERREUR: console.log() détecté en production!"
    grep -r "console\.log" src/ --include="*.vue" --include="*.ts" --include="*.tsx" 2>/dev/null | grep -v "node_modules" | head -5
    EXIT_CODE=1
else
    echo "   ✅ Pas de console.log()"
fi
echo ""

# 2. Vérifier console.error/warn/debug
echo "2️⃣  Recherche de console.error/warn/debug()..."
if grep -r "console\.\(error\|warn\|debug\)" src/ --include="*.vue" --include="*.ts" --include="*.tsx" 2>/dev/null | grep -v "node_modules" > /dev/null; then
    echo "   ❌ ERREUR: console.error/warn/debug() détecté!"
    grep -r "console\.\(error\|warn\|debug\)" src/ --include="*.vue" --include="*.ts" --include="*.tsx" 2>/dev/null | grep -v "node_modules" | head -5
    EXIT_CODE=1
else
    echo "   ✅ Pas de console.error/warn/debug()"
fi
echo ""

# 3. Vérifier debugger
echo "3️⃣  Recherche de debugger..."
if grep -r "debugger" src/ --include="*.vue" --include="*.ts" --include="*.tsx" 2>/dev/null | grep -v "node_modules" > /dev/null; then
    echo "   ❌ ERREUR: debugger détecté!"
    grep -r "debugger" src/ --include="*.vue" --include="*.ts" --include="*.tsx" 2>/dev/null | grep -v "node_modules"
    EXIT_CODE=1
else
    echo "   ✅ Pas de debugger"
fi
echo ""

# 4. Vérifier alert()
echo "4️⃣  Recherche de alert()..."
if grep -r "alert(" src/ --include="*.vue" --include="*.ts" --include="*.tsx" 2>/dev/null | grep -v "node_modules" > /dev/null; then
    echo "   ❌ ERREUR: alert() détecté (utiliser notifications UI)!"
    grep -r "alert(" src/ --include="*.vue" --include="*.ts" --include="*.tsx" 2>/dev/null | grep -v "node_modules"
    EXIT_CODE=1
else
    echo "   ✅ Pas de alert()"
fi
echo ""

# 5. Vérifier types 'any' excessifs (avertissement)
echo "5️⃣  Recherche de types 'any'..."
ANY_COUNT=$(grep -r ": any" src/ --include="*.vue" --include="*.ts" --include="*.tsx" 2>/dev/null | grep -v "node_modules" | wc -l)
if [ "$ANY_COUNT" -gt 0 ]; then
    echo "   ⚠️  AVERTISSEMENT: $ANY_COUNT types 'any' trouvés (à typer explicitement)"
    grep -r ": any" src/ --include="*.vue" --include="*.ts" --include="*.tsx" 2>/dev/null | grep -v "node_modules" | head -3
else
    echo "   ✅ Pas de types 'any'"
fi
echo ""

# 6. Vérifier code mort (imports inutilisés)
echo "6️⃣  Recherche d'imports inutilisés..."
UNUSED_IMPORTS=$(grep -r "^import " src/ --include="*.vue" --include="*.ts" --include="*.tsx" 2>/dev/null | wc -l)
# Note: Vérification superficielle - un vrai audit nécessiterait ESLint
echo "   ℹ️  Imports totaux: $UNUSED_IMPORTS (vérification détaillée: voir ESLint)"
echo ""

echo "✅ Vérification qualité Frontend terminée"

exit $EXIT_CODE
