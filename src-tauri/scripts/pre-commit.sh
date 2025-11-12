#!/bin/bash
echo "🔍 Vérification pre-commit en cours..."
make pre-commit; if [ $? -ne 0 ]; then echo ""; echo "❌ VÉRIFICATION PRÉ-COMMIT ÉCHOUÉE"; echo "   Commit bloqué."; echo "   Corrigez les erreurs avant de committer."; exit 1; fi; echo ""; echo "✅ Tout est vert. Commit autorisé."; exit 0
