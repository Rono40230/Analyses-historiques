#!/bin/bash
if grep -r "unwrap()" src-tauri/src --include="*.rs" | grep -v "#\[test\]" | grep -v "#\[cfg(test)\]" | grep -v "tests/"; then echo "❌ unwrap() détecté en dehors des tests!"; exit 1; fi; echo "✅ Pas d'unwrap() dans le code production"; exit 0
