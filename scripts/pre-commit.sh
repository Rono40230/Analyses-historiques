#!/bin/bash
# pre-commit.sh - Hook de vérification avant commit
set -e

echo "🔍 Vérification pre-commit en cours..."
echo ""

# Changer vers le répertoire racine du projet
cd "$(git rev-parse --show-toplevel)"

# Exécuter le make pre-commit qui fait tous les vérifications
if make pre-commit; then
    echo ""
    echo "✅ Tout est vert. Commit autorisé."
    exit 0
else
    echo ""
    echo "❌ VÉRIFICATION PRÉ-COMMIT ÉCHOUÉE"
    echo "   Commit bloqué."
    echo "   Corrigez les erreurs avant de committer."
    exit 1
fi

