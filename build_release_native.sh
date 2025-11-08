#!/bin/bash
# Script pour compiler l'application Tauri en mode release depuis VSCode Flatpak
# en utilisant l'environnement natif Fedora

set -e

echo "🔨 Compilation RELEASE de l'application Tauri sur l'hôte Fedora..."

cd "$(dirname "$0")/src-tauri"

# Compile en mode release sur l'hôte via flatpak-spawn
flatpak-spawn --host bash -c "cd '$PWD' && cargo build --release"

echo "✅ Compilation RELEASE terminée !"
echo ""
echo "Pour exécuter l'application :"
echo "  flatpak-spawn --host bash -c \"cd '$(pwd)' && cargo run --release\""
