#!/bin/bash
# Script pour compiler l'application Tauri depuis VSCode Flatpak
# en utilisant l'environnement natif Fedora

set -e

echo "🔨 Compilation de l'application Tauri sur l'hôte Fedora..."

cd "$(dirname "$0")/src-tauri"

# Compile sur l'hôte via flatpak-spawn
flatpak-spawn --host bash -c "cd '$PWD' && cargo build"

echo "✅ Compilation terminée !"
echo ""
echo "Pour exécuter l'application :"
echo "  ./run_native.sh"
