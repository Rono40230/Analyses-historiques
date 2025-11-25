#!/bin/bash
# full-code-audit.sh - Audit complète du code entier (Backend + Frontend)
# Applique TOUTES les règles .clinerules sur l'intégrité complète du projet

set -e

echo ""
echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║        🔍 AUDIT COMPLET DU CODE - BACKEND + FRONTEND              ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

EXIT_CODE=0
PHASE_COUNT=1

# ═══════════════════════════════════════════════════════════════
# PHASE 1 : TAILLES DE FICHIERS (Frontend + Backend)
# ═══════════════════════════════════════════════════════════════

echo "PHASE $PHASE_COUNT/7: 📏 Tailles de fichiers (Backend + Frontend)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if ! ./scripts/check-file-size-complete.sh; then
    echo "❌ PHASE $PHASE_COUNT ÉCHOUÉE: Fichiers trop volumineux"
    EXIT_CODE=1
fi
PHASE_COUNT=$((PHASE_COUNT + 1))
echo ""

# ═══════════════════════════════════════════════════════════════
# PHASE 2 : QUALITÉ BACKEND (Rust)
# ═══════════════════════════════════════════════════════════════

echo "PHASE $PHASE_COUNT/7: 🦀 Qualité Backend (Rust) - Anti-patterns"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if ! ./scripts/check-antipatterns.sh; then
    echo "❌ PHASE $PHASE_COUNT ÉCHOUÉE: Anti-patterns Rust détectés"
    EXIT_CODE=1
fi
PHASE_COUNT=$((PHASE_COUNT + 1))
echo ""

# ═══════════════════════════════════════════════════════════════
# PHASE 3 : UNWRAP() BACKEND
# ═══════════════════════════════════════════════════════════════

echo "PHASE $PHASE_COUNT/7: 🔒 Backend - unwrap() en production"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if ! ./scripts/check-unwrap.sh; then
    echo "❌ PHASE $PHASE_COUNT ÉCHOUÉE: unwrap() détecté"
    EXIT_CODE=1
fi
PHASE_COUNT=$((PHASE_COUNT + 1))
echo ""

# ═══════════════════════════════════════════════════════════════
# PHASE 4 : CODE MORT BACKEND
# ═══════════════════════════════════════════════════════════════

echo "PHASE $PHASE_COUNT/7: 💀 Backend - Code mort"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if ! ./scripts/check-dead-code.sh; then
    echo "❌ PHASE $PHASE_COUNT ÉCHOUÉE: Code mort détecté"
    EXIT_CODE=1
fi
PHASE_COUNT=$((PHASE_COUNT + 1))
echo ""

# ═══════════════════════════════════════════════════════════════
# PHASE 5 : ARCHITECTURE BACKEND
# ═══════════════════════════════════════════════════════════════

echo "PHASE $PHASE_COUNT/7: 🏗️  Backend - Architecture DAG & Hiérarchie"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if ./scripts/check-circular-imports.sh > /dev/null 2>&1; then
    echo "   ✅ Pas d'imports circulaires"
else
    echo "   ⚠️  Avertissement imports circulaires"
fi

if ./scripts/check-architecture.sh > /dev/null 2>&1; then
    echo "   ✅ Hiérarchie respectée"
else
    echo "   ⚠️  Avertissement hiérarchie"
fi
PHASE_COUNT=$((PHASE_COUNT + 1))
echo ""

# ═══════════════════════════════════════════════════════════════
# PHASE 6 : QUALITÉ FRONTEND (Vue.js/TypeScript)
# ═══════════════════════════════════════════════════════════════

echo "PHASE $PHASE_COUNT/7: 🎨 Qualité Frontend (Vue.js/TypeScript)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ -f "./scripts/check-vue-quality.sh" ]; then
    if ! ./scripts/check-vue-quality.sh; then
        echo "❌ PHASE $PHASE_COUNT ÉCHOUÉE: Violations Frontend détectées"
        EXIT_CODE=1
    fi
else
    echo "⚠️  Script check-vue-quality.sh non trouvé, ignoré."
fi

if [ -f "./scripts/check-typescript-quality.sh" ]; then
    ./scripts/check-typescript-quality.sh
else
    echo "⚠️  Script check-typescript-quality.sh non trouvé, ignoré."
fi

if [ -f "./scripts/check-french-naming-frontend.sh" ]; then
    ./scripts/check-french-naming-frontend.sh
else
    echo "⚠️  Script check-french-naming-frontend.sh non trouvé, ignoré."
fi

PHASE_COUNT=$((PHASE_COUNT + 1))
echo ""

# ═══════════════════════════════════════════════════════════════
# PHASE 7 : NOMMAGE FRANÇAIS (Backend)
# ═══════════════════════════════════════════════════════════════

echo "PHASE $PHASE_COUNT/7: 🇫🇷 Nommage français - Backend (Rust)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ -f "./scripts/check-french-naming.sh" ]; then
    ./scripts/check-french-naming.sh
else
    echo "ℹ️  Script check-french-naming.sh non trouvé (optionnel)."
fi
echo ""

# ═══════════════════════════════════════════════════════════════
# RÉSUMÉ FINAL
# ═══════════════════════════════════════════════════════════════

echo ""
echo "╔═══════════════════════════════════════════════════════════════════╗"
if [ $EXIT_CODE -eq 0 ]; then
    echo "║    ✅ AUDIT COMPLET RÉUSSI - Code conforme à .clinerules       ║"
else
    echo "║    ❌ AUDIT ÉCHOUÉ - Violations détectées (voir ci-dessus)     ║"
fi
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

exit $EXIT_CODE
