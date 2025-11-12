#!/bin/bash
# check-circular-imports.sh - Détection imports circulaires (architecture DAG)

EXIT_CODE=0

echo "🔄 Vérification des imports circulaires..."

# Créer un fichier temporaire pour stocker les imports
TEMP_IMPORTS=$(mktemp)

# Parser tous les imports Rust
find src-tauri/src -name "*.rs" -type f | while read -r file; do
    # Extraire les imports et créer un graphe
    grep -E "^use |^mod " "$file" | sed "s|.*use ||; s|.*mod ||; s|;||; s| as .*||" | while read -r import; do
        if [ -n "$import" ]; then
            echo "$file -> $import" >> "$TEMP_IMPORTS"
        fi
    done
done

# Vérifier les cycles simples (A->B et B->A)
if [ -f "$TEMP_IMPORTS" ]; then
    CYCLES=$(awk -F' -> ' '{print $2" -> "$1}' "$TEMP_IMPORTS" | sort | uniq -d | wc -l)
    
    if [ "$CYCLES" -gt 0 ]; then
        echo "❌ Imports circulaires détectés:"
        awk -F' -> ' '{print $2" -> "$1}' "$TEMP_IMPORTS" | sort | uniq -d
        EXIT_CODE=1
    else
        echo "✅ Pas d'imports circulaires détectés"
        EXIT_CODE=0
    fi
fi

rm -f "$TEMP_IMPORTS"

# Double check avec cargo
echo "🔍 Vérification complète avec cargo..."
if cargo check 2>&1 | grep -iq "circular"; then
    echo "❌ Dépendance circulaire détectée"
    EXIT_CODE=1
fi

exit $EXIT_CODE
