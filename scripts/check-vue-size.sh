#!/bin/bash
# check-vue-size.sh - Vérification des tailles fichiers Vue/TypeScript Frontend
# Règle 15 : Limites strictes

set -e

EXIT_CODE=0

MAX_LINES_VUE=250
MAX_LINES_VUE_COMPLEX=300
MAX_LINES_STORE=200
MAX_LINES_STORE_DATA=500
MAX_LINES_COMPOSABLE=150
MAX_LINES_UTILS=200

# Fichiers pré-existants exclus de l'audit (resteront hors limites)
EXCLUDED_FILES=(
    "src/components/HourlyTable.vue"
    "src/stores/eventTranslations.ts"
    "src/utils/eventTranslations.ts"
)

# Fonction pour vérifier si un fichier est exlcu
is_excluded() {
    local file="$1"
    for excluded in "${EXCLUDED_FILES[@]}"; do
        if [[ "$file" == "$excluded" ]]; then
            return 0
        fi
    done
    return 1
}

echo "📏 Vérification des tailles fichiers Frontend..."
echo ""

# 1. Composants Vue
echo "1️⃣  Composants Vue..."
while IFS= read -r file; do
    lines=$(wc -l < "$file")
    
    # Ignorer les fichiers exclus
    if is_excluded "$file"; then
        echo "   ⊘ $file: $lines lignes (EXCLU - pré-existant)"
        continue
    fi
    
    # Déterminer la limite selon la complexité (tables/modals)
    if [[ "$file" == *"Table"* ]] || [[ "$file" == *"Modal"* ]]; then
        limit=$MAX_LINES_VUE_COMPLEX
    else
        limit=$MAX_LINES_VUE
    fi
    
    if [ "$lines" -gt "$limit" ]; then
        echo "   ❌ $file: $lines lignes (max $limit)"
        EXIT_CODE=1
    fi
done < <(find src/components -name "*.vue" -type f 2>/dev/null)

echo "   ✅ Vérification composants terminée"
echo ""

# 2. Stores Pinia
echo "2️⃣  Stores Pinia..."
while IFS= read -r file; do
    lines=$(wc -l < "$file")
    
    # Ignorer les fichiers exclus
    if is_excluded "$file"; then
        echo "   ⊘ $file: $lines lignes (EXCLU - pré-existant)"
        continue
    fi
    
    # Déterminer la limite selon le type (données statiques tolèrent plus)
    if [[ "$file" == *"translations"* ]] || [[ "$file" == *"schedules"* ]]; then
        limit=$MAX_LINES_STORE_DATA
    else
        limit=$MAX_LINES_STORE
    fi
    
    if [ "$lines" -gt "$limit" ]; then
        echo "   ❌ $file: $lines lignes (max $limit)"
        EXIT_CODE=1
    fi
done < <(find src/stores -name "*.ts" -type f 2>/dev/null)

echo "   ✅ Vérification stores terminée"
echo ""

# 3. Composables
echo "3️⃣  Composables..."
while IFS= read -r file; do
    lines=$(wc -l < "$file")
    
    # Ignorer les fichiers exclus
    if is_excluded "$file"; then
        echo "   ⊘ $file: $lines lignes (EXCLU - pré-existant)"
        continue
    fi
    
    if [ "$lines" -gt "$MAX_LINES_COMPOSABLE" ]; then
        echo "   ❌ $file: $lines lignes (max $MAX_LINES_COMPOSABLE)"
        EXIT_CODE=1
    fi
done < <(find src/composables -name "*.ts" -type f 2>/dev/null)

echo "   ✅ Vérification composables terminée"
echo ""

# 4. Utils/Helpers
echo "4️⃣  Utils et Helpers..."
while IFS= read -r file; do
    lines=$(wc -l < "$file")
    
    # Ignorer les fichiers exclus
    if is_excluded "$file"; then
        echo "   ⊘ $file: $lines lignes (EXCLU - pré-existant)"
        continue
    fi
    
    if [ "$lines" -gt "$MAX_LINES_UTILS" ]; then
        echo "   ❌ $file: $lines lignes (max $MAX_LINES_UTILS)"
        EXIT_CODE=1
    fi
done < <(find src/utils -name "*.ts" -type f 2>/dev/null)

echo "   ✅ Vérification utils terminée"
echo ""

if [ $EXIT_CODE -eq 0 ]; then
    echo "✅ Tous les fichiers Frontend respectent les limites de taille"
else
    echo "❌ Certains fichiers dépassent les limites (à refactoriser)"
fi

exit $EXIT_CODE
