#!/bin/bash
# install_deps_fedora.sh - Installe les dépendances système pour Tauri sur Fedora

echo "🔧 Installation des dépendances Tauri pour Fedora..."

sudo dnf install -y \
  webkit2gtk4.1-devel \
  openssl-devel \
  curl \
  wget \
  file \
  libappindicator-gtk3-devel \
  librsvg2-devel \
  gtk3-devel \
  atk-devel \
  gdk-pixbuf2-devel \
  pango-devel \
  cairo-devel \
  libsoup3-devel

echo "✅ Dépendances installées !"
echo ""
echo "Vous pouvez maintenant compiler avec :"
echo "  cd src-tauri && cargo build"
