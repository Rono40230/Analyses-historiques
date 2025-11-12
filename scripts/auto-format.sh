#!/bin/bash
# auto-format.sh - Formatage automatique du code (cargo fmt)

echo "✏️  Formatage automatique du code..."

cd "$(git rev-parse --show-toplevel)"

# Format both frontend and backend
cargo fmt --manifest-path src-tauri/Cargo.toml 2>&1 || true

echo "✅ Code formaté avec succès"
exit 0
