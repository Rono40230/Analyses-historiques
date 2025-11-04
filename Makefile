# Makefile pour vérifier le respect des .clinerules
# Usage: make <commande>

.PHONY: help dev check-rules validate all

# Affiche l'aide
help:
	@echo "════════════════════════════════════════════════"
	@echo "   COMMANDES DISPONIBLES"
	@echo "════════════════════════════════════════════════"
	@echo ""
	@echo "  make dev          - Lance le mode développement avec hot-reload"
	@echo "  make check-rules  - Vérifie le respect des règles .clinerules"
	@echo "  make validate     - Valide tout le code (clippy + tests)"
	@echo "  make all          - Fait tout : check + validate + dev"
	@echo ""
	@echo "════════════════════════════════════════════════"

# Lance le développement avec hot-reload
dev:
	@echo "🔥 Lancement du mode développement avec hot-reload..."
	@echo "   Les changements CSS/HTML se rechargent automatiquement"
	@echo "   Les changements Rust recompilent automatiquement"
	@echo ""
	cargo watch -x run

# Vérifie le respect des règles
check-rules:
	@echo "📋 Vérification du respect des .clinerules..."
	@echo ""
	@echo "1️⃣  Vérification des tailles de fichiers..."
	@./scripts/check-file-size.sh
	@echo ""
	@echo "2️⃣  Vérification des anti-patterns..."
	@./scripts/check-unwrap.sh
	@echo "   ✅ Pas de unwrap() ni expect() hors tests"
	@echo ""
	@echo "   ❌ Recherche de TODO non formatés..."
	@! grep -r "TODO" src-tauri/src/ --include="*.rs" | grep -v "TODO(" || (echo "⚠️  AVERTISSEMENT: TODO trouvé sans format standard" && true)
	@echo "   ✅ Vérification TODO terminée"
	@echo ""
	@echo "✅ Vérification des règles terminée !"

# Valide le code complet
validate:
	@echo "🔍 Validation complète du code..."
	@echo ""
	@echo "1️⃣  Compilation..."
	cargo build
	@echo ""
	@echo "2️⃣  Tests..."
	cargo test
	@echo ""
	@echo "3️⃣  Clippy (détection de problèmes)..."
	cargo clippy -- -D warnings
	@echo ""
	@echo "4️⃣  Formatage..."
	cargo fmt -- --check
	@echo ""
	@echo "✅ Validation terminée avec succès !"

# Fait tout (sans lancer dev qui est infini)
all: check-rules validate

# Commande par défaut
.DEFAULT_GOAL := help
