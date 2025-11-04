#!/bin/bash
# dev-watch.sh - Script de développement avec hot-reload

echo "🔥 Démarrage du mode développement avec hot-reload..."
echo ""
echo "Backend : cargo watch recompile automatiquement"
echo "Frontend : Vite HMR (Hot Module Replacement)"
echo ""

# Lance cargo watch en arrière-plan pour recompiler le backend
echo "📦 Installation de cargo-watch si nécessaire..."
cargo install cargo-watch 2>/dev/null || true

echo ""
echo "🚀 Lancement de l'application..."
npm run tauri dev
