#!/bin/bash
echo "🧹 Nettoyage du code mort..."
find src-tauri/src -name "*.rs" -exec cp {} {}.bk \; 2>/dev/null
cargo fix --allow-dirty --allow-staged 2>/dev/null
if cargo check 2>&1 | grep -q "dead_code"; then echo "❌ Code mort détecté:"; cargo check 2>&1 | grep "dead_code"; find src-tauri/src -name "*.rs.bk" -exec sh -c 'mv "$1" "${1%.bk}"' _ {} \; 2>/dev/null; exit 1; else rm -f src-tauri/src/**/*.rs.bk 2>/dev/null; echo "✅ Code mort nettoyé"; fi; exit 0
