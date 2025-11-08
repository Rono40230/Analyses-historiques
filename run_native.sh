#!/bin/bash
# Script pour exécuter l'application Tauri depuis VSCode Flatpak
# en utilisant l'environnement natif Fedora

set -e

echo "🚀 Lancement de l'application Tauri sur l'hôte Fedora..."

cd "$(dirname "$0")/src-tauri"

# Lance sur l'hôte via flatpak-spawn
flatpak-spawn --host bash -c "cd '$PWD' && cargo run"
