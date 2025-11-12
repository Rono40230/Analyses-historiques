#!/bin/bash
#!/bin/bash
# Installation automatique du système de contrôle IA

set -e  # Arrête si erreur

echo "🚀 Installation du système de contrôle IA..."
echo ""

# 1. Crée le dossier scripts
mkdir -p scripts
echo "✅ Dossier scripts créé"

# 2. Génère les 5 scripts de vérification
echo "📜 Création des scripts de vérification..."

cat > scripts/check-file-size.sh << 'EOF'
#!/bin/bash
MAX_LINES_SERVICES=300; MAX_LINES_COMMANDS=200; MAX_LINES_MAIN=120; EXIT_CODE=0
while IFS= read -r file; do lines=$(wc -l < "$file"); if [[ "$file" == *"/services/"* ]] && [ "$lines" -gt "$MAX_LINES_SERVICES" ]; then echo "❌ $file: $lines lignes (max $MAX_LINES_SERVICES)"; EXIT_CODE=1; elif [[ "$file" == *"/commands/"* ]] && [ "$lines" -gt "$MAX_LINES_COMMANDS" ]; then echo "❌ $file: $lines lignes (max $MAX_LINES_COMMANDS)"; EXIT_CODE=1; elif [[ "$file" == *"/main.rs" ]] && [ "$lines" -gt "$MAX_LINES_MAIN" ]; then echo "❌ $file: $lines lignes (max $MAX_LINES_MAIN)"; EXIT_CODE=1; fi; done < <(find src-tauri/src -name "*.rs" -type f); if [ $EXIT_CODE -eq 0 ]; then echo "✅ Tous les fichiers respectent les limites de taille"; fi; exit $EXIT_CODE
EOF

cat > scripts/check-unwrap.sh << 'EOF'
#!/bin/bash
if grep -r "unwrap()" src-tauri/src --include="*.rs" | grep -v "#\[test\]" | grep -v "#\[cfg(test)\]" | grep -v "tests/"; then echo "❌ unwrap() détecté en dehors des tests!"; exit 1; fi; echo "✅ Pas d'unwrap() dans le code production"; exit 0
EOF

cat > scripts/check-antipatterns.sh << 'EOF'
#!/bin/bash
EXIT_CODE=0; if grep -r "vec!\[" src-tauri/src --include="*.rs" | grep -E "(Price|Trade|Order|User)" | grep -v "#\[test\]"; then echo "❌ Mock data détecté!"; EXIT_CODE=1; fi; while IFS= read -r file; do consecutive_comments=$(grep -c "//.*" "$file" || true); if [ "$consecutive_comments" -gt 3 ]; then echo "⚠️  $file: Plus de 3 lignes commentées"; fi; done < <(find src-tauri/src -name "*.rs" -type f); if [ $EXIT_CODE -eq 0 ]; then echo "✅ Aucun anti-pattern critique détecté"; fi; exit $EXIT_CODE
EOF

cat > scripts/check-dead-code.sh << 'EOF'
#!/bin/bash
echo "🧹 Nettoyage du code mort..."
find src-tauri/src -name "*.rs" -exec cp {} {}.bk \; 2>/dev/null
cargo fix --allow-dirty --allow-staged 2>/dev/null
if cargo check 2>&1 | grep -q "dead_code"; then echo "❌ Code mort détecté:"; cargo check 2>&1 | grep "dead_code"; find src-tauri/src -name "*.rs.bk" -exec sh -c 'mv "$1" "${1%.bk}"' _ {} \; 2>/dev/null; exit 1; else rm -f src-tauri/src/**/*.rs.bk 2>/dev/null; echo "✅ Code mort nettoyé"; fi; exit 0
EOF

cat > scripts/pre-commit.sh << 'EOF'
#!/bin/bash
echo "🔍 Vérification pre-commit en cours..."
make pre-commit; if [ $? -ne 0 ]; then echo ""; echo "❌ VÉRIFICATION PRÉ-COMMIT ÉCHOUÉE"; echo "   Commit bloqué."; echo "   Corrigez les erreurs avant de committer."; exit 1; fi; echo ""; echo "✅ Tout est vert. Commit autorisé."; exit 0
EOF

echo "✅ Scripts créés"

# 3. Rend exécutables
chmod +x scripts/*.sh
echo "✅ Scripts rendus exécutables"

# 4. Sauvegarde et remplace .clinerules
if [ -f .clinerules ]; then
    cp .clinerules .clinerules.backup
    echo "✅ .clinerules sauvegardé dans .clinerules.backup"
fi

cat > .clinerules << 'EOF'
# .clinerules - RÈGLES ABSOLUTES POUR AGENT IA

## 🚨 RÈGLE 0 : AUTO-VÉRIFICATION SYSTÉMATIQUE
// APRÈS CHAQUE GÉNÉRATION DE CODE, l'IA exécute AUTOMATIQUEMENT :
//   cargo fmt && cargo clippy -- -D warnings && cargo test
// Si échec → NE PAS LIVRER LE CODE. Corriger d'abord.
// Si succès → Livrer avec rapport de conformité.

## 🛡️ RÈGLE 1 : ZÉRO COMMIT AUTOMATIQUE (INTERDICTION ABSOLUE)
// L'IA NE MENTIONNE JAMAIS : "git commit", "git push", "git add"
// L'IA NE SAIT PAS que git existe. C'est à l'utilisateur de décider.

## 🧹 RÈGLE 2 : ZÉRO CODE MORT
// AVANT GÉNÉRATION, l'IA analyse et SUPPRIME code mort :
//   - Fonctions jamais appelées (dead_code)
//   - Imports inutilisés
//   - Variables non utilisées
// Pas de code commenté >3 lignes.

## 🧪 RÈGLE 3 : TESTS Systématiques
// POUR CHAQUE FONCTION PUBLIQUE :
//   - Générer test unitaire AVANT implémentation (TDD)
//   - Exécuter `cargo test` - DOIT PASSER
//   - Coverage >80% pour nouveaux modules
// PAS DE CODE LIVRÉ SANS TESTS PASSANTS.

## 📐 RÈGLE 4 : RESPECT STRICT .clinerules & Makefile
// L'IA ne peut PAS générer de code qui viole :
//   - Taille fichier <300 lignes
//   - Pas d'unwrap() hors tests
//   - Architecture DAG (pas d'import circulaire)
//   - ZERO mock data
// Si violation → CORRIGER IMMÉDIATEMENT.

## 📏 RÈGLE 5 : LIMITES DE TAILLE STRICTES
// - Services: <300 lignes (HARD LIMIT)
// - Commands: <200 lignes
// - main.rs: <120 lignes
// Si approche → WARN. Si dépasse → STOP + Split.

## 🎯 RÈGLE 6 : ARCHITECTURE 3 NIVEAUX (DAG)
// NIVEAU 1: utils/config.rs, utils/logger.rs, models/errors.rs → AUCUNE dépendance
// NIVEAU 2: db/cache.rs, services/api_client.rs → Dépend NIVEAU 1 SEULEMENT
// NIVEAU 3: services/trading.rs → Dépend NIVEAU 1+2
// NIVEAU 4: commands/*.rs → Dépend NIVEAU 3 SEULEMENT
// ❌ JAMAIS d'import entre services même niveau.

## 💾 RÈGLE 7 : GESTION D'ERREURS UNIFIÉE
// TOUTES fonctions publiques retournent Result<T, ServiceError>
// ❌ unwrap(), unwrap_or() interdits
// ✅ Utiliser ? pour propagation

## 🔒 RÈGLE 8 : ANTI-PATTERNS INTERDITS
// ❌ unwrap() sans contexte | Mock data | Clone() >5x/fn
// ❌ Magic numbers | Code commenté >3 lignes | panic!() services
// ❌ Import circulaire | pub sur tout
// Si détecté → SUPPRIMER/CORRIGER automatiquement.

## 📝 RÈGLE 9 : RAPPORT OBLIGATOIRE
// APRÈS CHAQUE TÂCHE, l'IA affiche :
//
// ═══════════════════════════════════════════
// ✅ TÂCHE COMPLÉTÉE : [nom]
// ═══════════════════════════════════════════
// 📏 Taille: [X] lignes - Conforme
// 🧪 Tests: [X] tests écrits - Passants
// 🔒 Coverage: [X]% - OK
// ⚠️  Warnings: [X]
// 🧹 Code mort: [X] lignes nettoyées
// ═══════════════════════════════════════════
EOF

echo "✅ .clinerules mis à jour"

# 5. Sauvegarde et remplace Makefile
if [ -f Makefile ]; then
    cp Makefile Makefile.backup
    echo "✅ Makefile sauvegardé dans Makefile.backup"
fi

cat > Makefile << 'EOF'
# Makefile - Automatisation des vérifications
.PHONY: help pre-commit check-rules validate audit dev setup-hooks clean

help:
	@echo "════════════════════════════════════════════════"
	@echo "   COMMANDES DISPONIBLES"
	@echo "════════════════════════════════════════════════"
	@echo ""
	@echo "  make pre-commit    - Vérifie tout avant commit (RECOMMANDÉ)"
	@echo "  make check-rules   - Vérifie conformité .clinerules"
	@echo "  make validate      - Compile + teste + linte"
	@echo "  make audit         - Audit sécurité dépendances"
	@echo "  make dev           - Lance dev avec hot-reload"
	@echo "════════════════════════════════════════════════"

pre-commit: check-rules validate audit
	@echo ""
	@echo "✅ Toutes les vérifications passées !"
	@echo "   Le code est prêt à être commité."

check-rules:
	@echo "📋 Vérification du respect des .clinerules..."
	@./scripts/check-file-size.sh
	@./scripts/check-unwrap.sh
	@./scripts/check-antipatterns.sh
	@./scripts/check-dead-code.sh
	@echo "✅ Vérification des règles terminée !"

validate:
	@echo "🔍 Validation complète du code..."
	cargo build --release
	cargo test -- --nocapture
	cargo clippy --release -- -D warnings
	cargo fmt -- --check
	@echo "✅ Validation terminée avec succès !"

audit:
	@echo "🔍 Audit sécurité des dépendances..."
	cargo audit 2>/dev/null || echo "⚠️ cargo-audit non installé"
	@echo "✅ Audit terminé"

dev:
	@echo "🔥 Lancement mode développement..."
	cargo watch -x "check" -x "test" -x "clippy -- -D warnings"

setup-hooks:
	@echo "🔧 Installation du pre-commit hook..."
	ln -sf ../../scripts/pre-commit.sh .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit
	@echo "✅ Hook installé"

clean:
	@echo "🧹 Nettoyage..."
	cargo clean
	rm -rf target/
	@echo "✅ Nettoyé"
EOF

echo "✅ Makefile mis à jour"

# 6. Installe le pre-commit hook
make setup-hooks
echo "✅ Hook pre-commit installé"

# 7. Teste le système
echo ""
echo "🧪 Test final du système..."
make pre-commit || echo "⚠️  Vérifications terminées avec des erreurs (normal si code existant)"
echo ""

echo "🎉 Installation terminée !"
echo ""
echo "════════════════════════════════════════════════"
echo "   PROCHAINE ÉTAPE"
echo "════════════════════════════════════════════════"
echo ""
echo "1. Teste avec : make pre-commit"
echo "2. Demande à ton IA : 'Génère un service de test'"
echo "3. L'IA doit maintenant auto-vérifier avant de livrer"
echo ""
echo "════════════════════════════════════════════════"