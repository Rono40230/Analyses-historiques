#!/bin/bash
# Vérifie qu'il n'y a pas de .unwrap() ou .expect() en dehors des tests

set -e

FOUND_ISSUE=0

# Pour chaque fichier Rust
for file in $(find src-tauri/src -name "*.rs"); do
    # Extraire uniquement le code hors tests (avant #[cfg(test)])
    CODE_ONLY=$(sed '/#\[cfg(test)\]/,$d' "$file")
    
    # Chercher .unwrap() dans le code (hors tests)
    if echo "$CODE_ONLY" | grep -q "\.unwrap()"; then
        echo "❌ unwrap() trouvé dans $file (hors tests)"
        echo "$CODE_ONLY" | grep -n "\.unwrap()"
        FOUND_ISSUE=1
    fi
    
    # Chercher .expect() dans le code (hors tests)
    if echo "$CODE_ONLY" | grep -q "\.expect("; then
        echo "❌ expect() trouvé dans $file (hors tests)"
        echo "$CODE_ONLY" | grep -n "\.expect("
        FOUND_ISSUE=1
    fi
done

if [ $FOUND_ISSUE -eq 1 ]; then
    exit 1
fi

exit 0
