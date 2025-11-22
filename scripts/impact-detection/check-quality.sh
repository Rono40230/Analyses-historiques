#!/bin/bash
# check-quality.sh - Vérifie les règles de qualité statique (taille, anti-patterns)
# Issu de l'ancienne méthode (Makefile)

set -e

echo "📋 Vérification des standards de qualité..."
echo ""

# 1. Vérification des tailles de fichiers
echo "1️⃣  Vérification des tailles de fichiers..."
if [ -f "./scripts/check-file-size.sh" ]; then
    ./scripts/check-file-size.sh
else
    echo "⚠️  Script check-file-size.sh non trouvé, ignoré."
fi
echo ""

# 2. Vérification des anti-patterns
echo "2️⃣  Vérification des anti-patterns..."

# unwrap()
echo "   ❌ Recherche de unwrap()..."
if grep -r "\.unwrap()" src-tauri/src/ --include="*.rs" > /dev/null; then
    echo "⚠️  ERREUR: unwrap() trouvé dans le code !"
    grep -r "\.unwrap()" src-tauri/src/ --include="*.rs"
    exit 1
fi
echo "   ✅ Pas de unwrap() trouvé"
echo ""

# expect() hors tests
echo "   ❌ Recherche de expect() en production..."
if grep -r "\.expect(" src-tauri/src/ --include="*.rs" | grep -v "tests/" > /dev/null; then
    echo "⚠️  ERREUR: expect() trouvé hors tests !"
    grep -r "\.expect(" src-tauri/src/ --include="*.rs" | grep -v "tests/"
    exit 1
fi
echo "   ✅ Pas de expect() hors tests"
echo ""

# TODO non formatés
echo "   ❌ Recherche de TODO non formatés..."
if grep -r "TODO" src-tauri/src/ --include="*.rs" | grep -v "TODO(" > /dev/null; then
    echo "⚠️  AVERTISSEMENT: TODO trouvé sans format standard (devrait être 'TODO(nom): description')"
    # On n'exit pas pour ça, juste un warning
fi
echo "   ✅ Vérification TODO terminée"
echo ""

echo "✅ Vérification de la qualité terminée avec succès !"
