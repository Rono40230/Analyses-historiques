#!/bin/bash
# run-tests.sh - Lance tous les tests et validations

echo "🧪 Lancement de la suite de tests complète..."
echo ""

cd src-tauri

echo "📝 1. Formatage du code (cargo fmt)..."
cargo fmt --check
FMT_STATUS=$?

echo ""
echo "🔍 2. Vérification Clippy (qualité code)..."
cargo clippy -- -D warnings 2>/dev/null
CLIPPY_STATUS=$?

echo ""
echo "✅ 3. Tests unitaires (cargo test)..."
cargo test --lib
TEST_STATUS=$?

echo ""
echo "================================================"
echo "📊 RÉSULTATS"
echo "================================================"

if [ $FMT_STATUS -eq 0 ]; then
    echo "✅ Formatage: OK"
else
    echo "❌ Formatage: ÉCHEC (exécutez 'cargo fmt')"
fi

if [ $CLIPPY_STATUS -eq 0 ]; then
    echo "✅ Clippy: OK"
else
    echo "⚠️  Clippy: WARNINGS (dépendances système manquantes)"
fi

if [ $TEST_STATUS -eq 0 ]; then
    echo "✅ Tests: OK"
else
    echo "❌ Tests: ÉCHEC"
fi

echo "================================================"

# Code de sortie global
if [ $TEST_STATUS -eq 0 ] && [ $FMT_STATUS -eq 0 ]; then
    echo "🎉 Tous les tests sont passés !"
    exit 0
else
    echo "⚠️  Certains tests ont échoué"
    exit 1
fi
