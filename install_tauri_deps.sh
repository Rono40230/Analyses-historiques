#!/bin/bash
# Script d'installation des dépendances Tauri 2.0 pour Fedora

echo "🔧 Installation des dépendances Tauri 2.0 sur Fedora..."

# Dépendances de base pour Tauri 2.0
sudo dnf install -y \
    webkit2gtk4.1 \
    webkit2gtk4.1-devel \
    openssl-devel \
    curl \
    wget \
    file \
    libappindicator-gtk3 \
    libappindicator-gtk3-devel \
    librsvg2-devel

echo "✅ Dépendances installées !"
echo ""
echo "Vérification de pkg-config..."
pkg-config --modversion webkit2gtk-4.1 || echo "⚠️ webkit2gtk-4.1 non trouvé"

echo ""
echo "Pour recompiler le projet :"
echo "  cd src-tauri"
echo "  cargo clean"
echo "  cargo build"
