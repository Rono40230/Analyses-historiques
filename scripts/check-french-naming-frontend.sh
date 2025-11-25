#!/bin/bash
# check-french-naming-frontend.sh - Vérification nommage français (Frontend)
# Règle 2 : Nommage français obligatoire pour fonctions/méthodes

set -e

EXIT_CODE=0

echo "🇫🇷 Vérification nommage français Frontend..."
echo ""

# Patterns anglais courants à éviter
ENGLISH_PATTERNS=(
    "function get"
    "function set"
    "function load"
    "function save"
    "function create"
    "function delete"
    "function update"
    "function fetch"
    "function handle"
    "function show"
    "function hide"
    "function toggle"
    "export const get"
    "export const set"
    "export const load"
    "export const save"
    "export const create"
    "export const delete"
    "export const update"
    "export const fetch"
    "export const handle"
)

echo "1️⃣  Recherche de fonctions anglaises..."
FOUND_ENGLISH=0

for pattern in "${ENGLISH_PATTERNS[@]}"; do
    if grep -ri "$pattern" src/ --include="*.ts" --include="*.tsx" --include="*.vue" 2>/dev/null | grep -v "node_modules" > /dev/null; then
        echo "   ⚠️  Pattern '$pattern' détecté"
        FOUND_ENGLISH=$((FOUND_ENGLISH + 1))
    fi
done

if [ $FOUND_ENGLISH -gt 0 ]; then
    echo "   ❌ Trouvé $FOUND_ENGLISH patterns anglais (renommer en français)"
    EXIT_CODE=1
else
    echo "   ✅ Nommage français respecté"
fi
echo ""

# 2. Vérifier les noms de variables anglaises courants
echo "2️⃣  Recherche de variables anglaises..."
ENGLISH_VARS=(
    "const data ="
    "const result ="
    "const value ="
    "const item ="
    "const list ="
    "const items ="
    "const rows ="
    "const temp"
    "const obj"
)

FOUND_VARS=0
for var in "${ENGLISH_VARS[@]}"; do
    COUNT=$(grep -r "$var" src/ --include="*.ts" --include="*.tsx" --include="*.vue" 2>/dev/null | grep -v "node_modules" | wc -l)
    if [ "$COUNT" -gt 0 ]; then
        echo "   ℹ️  Variable '$var': $COUNT occurrences (préférer français)"
        FOUND_VARS=$((FOUND_VARS + 1))
    fi
done

if [ $FOUND_VARS -gt 0 ]; then
    echo "   ⚠️  Préférer des noms explicites en français"
fi
echo ""

echo "✅ Vérification nommage français terminée"

exit $EXIT_CODE
