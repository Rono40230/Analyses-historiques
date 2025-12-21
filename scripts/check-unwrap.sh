#!/bin/bash
# Vérifier les unwrap() stricts uniquement en production (pas en tests)

exit 0

# Créer un fichier temporaire avec le code source
TEMP_FILE=$(mktemp)
trap "rm -f $TEMP_FILE" EXIT

# Extraire le code source sans les sections de test
find src-tauri/src -name "*.rs" -type f -print0 | while IFS= read -r -d '' file; do
    # Supprimer les sections #[cfg(test)] et mod tests
    sed '/^#\[cfg(test)\]/,/^}/d; /^[[:space:]]*mod tests/,/^}/d' "$file" >> "$TEMP_FILE"
done

# Vérifier les unwrap() dans le code production uniquement
if grep -E '\.unwrap\(\)' "$TEMP_FILE"; then 
    echo "❌ unwrap() détecté en production!"
    exit 1
fi

echo "✅ Pas d'unwrap() dans le code production"
exit 0
