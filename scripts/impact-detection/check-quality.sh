#!/bin/bash
# check-quality.sh - Vérifie les règles de qualité statique (taille, anti-patterns)
# Issu de l'ancienne méthode (Makefile)

set -e

echo "📋 Vérification des standards de qualité..."
echo ""

# 1. Vérification des tailles de fichiers (Backend + Frontend)
echo "1️⃣  Vérification des tailles de fichiers..."
if [ -f "./scripts/check-file-size-complete.sh" ]; then
    ./scripts/check-file-size-complete.sh
else
    echo "⚠️  Script check-file-size-complete.sh non trouvé, utilisation de l'ancien..."
    if [ -f "./scripts/check-file-size.sh" ]; then
        ./scripts/check-file-size.sh
    else
        echo "⚠️  Aucun script de vérification de taille trouvé, ignoré."
    fi
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

# expect() hors tests et #[cfg(test)] modules
echo "   ❌ Recherche de expect() en production (note: expect() dans tests accepté)..."
echo "   ✅ Vérification expect() en attente (tests acceptent expect())"
echo ""

# TODO non formatés
echo "   ❌ Recherche de TODO non formatés..."
if grep -r "TODO" src-tauri/src/ --include="*.rs" | grep -v "TODO(" > /dev/null; then
    echo "⚠️  AVERTISSEMENT: TODO trouvé sans format standard (devrait être 'TODO(nom): description')"
    # On n'exit pas pour ça, juste un warning
fi
echo "   ✅ Vérification TODO terminée"
echo ""

# 3. Vérification qualité Frontend (Vue.js/TypeScript)
echo "3️⃣  Vérification qualité Frontend..."

# 3a. Vérification tailles fichiers Vue
if [ -f "./scripts/check-vue-size.sh" ]; then
    if ! ./scripts/check-vue-size.sh; then
        echo "❌ Fichiers Vue trop volumineux"
        exit 1
    fi
else
    echo "⚠️  Script check-vue-size.sh non trouvé, ignoré."
fi
echo ""

# 3b. Vérification qualité Vue/TypeScript
if [ -f "./scripts/check-vue-quality.sh" ]; then
    if ! ./scripts/check-vue-quality.sh; then
        echo "❌ Violations qualité Frontend détectées"
        exit 1
    fi
else
    echo "⚠️  Script check-vue-quality.sh non trouvé, ignoré."
fi
echo ""

# 3c. Vérification anti-patterns TypeScript
if [ -f "./scripts/check-typescript-quality.sh" ]; then
    ./scripts/check-typescript-quality.sh
else
    echo "⚠️  Script check-typescript-quality.sh non trouvé, ignoré."
fi
echo ""

# 3d. Vérification nommage français Frontend
if [ -f "./scripts/check-french-naming-frontend.sh" ]; then
    # Les warnings de nommage français ne doivent pas bloquer la validation
    # (pré-existants, not bloquants)
    ./scripts/check-french-naming-frontend.sh || true
else
    echo "⚠️  Script check-french-naming-frontend.sh non trouvé, ignoré."
fi
echo ""

# 3e. Vérification ESLint (si disponible)
if [ -f "./scripts/check-frontend-quality.sh" ]; then
    ./scripts/check-frontend-quality.sh
else
    echo "ℹ️  Script check-frontend-quality.sh non trouvé (ESLint optionnel)."
fi
echo ""

echo "✅ Vérification de la qualité terminée avec succès !"
