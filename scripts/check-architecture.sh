#!/bin/bash
# check-architecture.sh - Validation hiérarchie DAG (4 niveaux)

EXIT_CODE=0

echo "🏗️  Validation architecture DAG..."

# Définir les niveaux
LEVEL_1="utils|models/errors|config|logger"
LEVEL_2="db|services/api_client"
LEVEL_3="services/trading|services/cache"
LEVEL_4="commands"

# Fonction pour vérifier les imports d'un fichier
check_file_imports() {
    local file=$1
    local level=$2
    local allowed_deps=$3
    
    IMPORTS=$(grep -E "^use " "$file" | grep -oE "crate::[^:;]*::[^:;]*" | sort -u)
    
    while IFS= read -r import; do
        if [ -z "$import" ]; then continue; fi
        
        # Vérifier si l'import est autorisé
        if ! echo "$allowed_deps" | grep -qE "$import"; then
            # Vérifier que ce n'est pas un import circulaire
            if ! echo "$import" | grep -q "$(echo $file | grep -oE '[^/]*\.rs$' | sed 's/.rs//')"; then
                echo "⚠️  $file: import non autorisé vers $import"
                EXIT_CODE=1
            fi
        fi
    done <<< "$IMPORTS"
}

# Niveau 1: Pas de dépendances externes critiques
echo "  Niveau 1 (utils, models, config)..."
find src-tauri/src/utils src-tauri/src/models src-tauri/src/config -name "*.rs" 2>/dev/null | while read -r file; do
    if grep -qE "^use crate::(db|services|commands)" "$file"; then
        echo "❌ $file: NIVEAU 1 ne peut pas dépendre de niveaux supérieurs"
        EXIT_CODE=1
    fi
done

# Niveau 2: Peut dépendre de Niveau 1 seulement
echo "  Niveau 2 (db, api_client)..."
find src-tauri/src/db src-tauri/src/services -name "*api_client*" 2>/dev/null | while read -r file; do
    if grep -qE "^use crate::(services/trading|services/cache|commands)" "$file"; then
        echo "❌ $file: NIVEAU 2 ne peut dépendre que de NIVEAU 1"
        EXIT_CODE=1
    fi
done

# Niveau 3: Peut dépendre de Niveaux 1 + 2
echo "  Niveau 3 (services)..."
find src-tauri/src/services -name "*.rs" 2>/dev/null | while read -r file; do
    if grep -qE "^use crate::commands" "$file"; then
        echo "❌ $file: NIVEAU 3 ne peut pas dépendre de NIVEAU 4 (commands)"
        EXIT_CODE=1
    fi
done

# Niveau 4: Peut dépendre de tous les niveaux
echo "  Niveau 4 (commands)..."
find src-tauri/src/commands -name "*.rs" 2>/dev/null | while read -r file; do
    # Les commands peuvent tout importer
    :
done

if [ $EXIT_CODE -eq 0 ]; then
    echo "✅ Architecture DAG valide (4 niveaux respectés)"
else
    echo "❌ Architecture DAG non conforme"
fi

exit $EXIT_CODE
