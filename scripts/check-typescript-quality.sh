#!/bin/bash
# check-typescript-quality.sh - Vérification anti-patterns TypeScript
# Détecte patterns dangereux en TypeScript/Vue

set -e

EXIT_CODE=0

echo "🧹 Vérification anti-patterns TypeScript..."
echo ""

# 1. Vérifier les non-null assertions excessives (!)
echo "1️⃣  Recherche de non-null assertions (!)..."
BANG_COUNT=$(grep -r "!" src/ --include="*.vue" --include="*.ts" --include="*.tsx" 2>/dev/null | grep -E "as |^\s*.*!" | wc -l)
if [ "$BANG_COUNT" -gt 10 ]; then
    echo "   ⚠️  AVERTISSEMENT: $BANG_COUNT non-null assertions détectées (à éviter)"
else
    echo "   ✅ Nombre raisonnable de non-null assertions"
fi
echo ""

# 2. Vérifier les 'as any' (cast dangereux)
echo "2️⃣  Recherche de 'as any' (cast dangereux)..."
if grep -r " as any" src/ --include="*.vue" --include="*.ts" --include="*.tsx" 2>/dev/null | grep -v "node_modules" > /dev/null; then
    echo "   ❌ ERREUR: 'as any' détecté (utiliser types explicites)!"
    grep -r " as any" src/ --include="*.vue" --include="*.ts" --include="*.tsx" 2>/dev/null | grep -v "node_modules"
    EXIT_CODE=1
else
    echo "   ✅ Pas de 'as any'"
fi
echo ""

# 3. Vérifier TODO non formatés
echo "3️⃣  Recherche de TODO non formatés..."
if grep -r "TODO" src/ --include="*.vue" --include="*.ts" --include="*.tsx" 2>/dev/null | grep -v "TODO(" > /dev/null; then
    echo "   ⚠️  AVERTISSEMENT: TODO trouvés sans format standard (devrait être 'TODO(nom): description')"
    grep -r "TODO" src/ --include="*.vue" --include="*.ts" --include="*.tsx" 2>/dev/null | grep -v "TODO(" | head -3
else
    echo "   ✅ Tous les TODO sont formatés correctement"
fi
echo ""

# 4. Vérifier les fonctions vides/stub
echo "4️⃣  Recherche de fonctions vides..."
EMPTY_FUNC=$(grep -r "{\s*}" src/ --include="*.vue" --include="*.ts" --include="*.tsx" 2>/dev/null | wc -l)
if [ "$EMPTY_FUNC" -gt 0 ]; then
    echo "   ⚠️  AVERTISSEMENT: $EMPTY_FUNC fonctions vides trouvées (à implémenter)"
else
    echo "   ✅ Pas de fonctions vides"
fi
echo ""

# 5. Vérifier les imports circulaires (pattern dangereux)
echo "5️⃣  Analyse imports circulaires..."
# Note: Vérification simplifiée - un vrai audit nécessiterait une analyse de dépendances
echo "   ℹ️  Vérification complète via build TypeScript"
echo ""

echo "✅ Vérification anti-patterns TypeScript terminée"

exit $EXIT_CODE
