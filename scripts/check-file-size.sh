#!/bin/bash
# Script de vérification des tailles de fichiers selon .clinerules

echo "📏 Vérification des tailles de fichiers..."
echo ""

# Couleurs pour l'affichage
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Compteurs
errors=0
warnings=0

# Fonction de vérification
check_file() {
    local file=$1
    local max_lines=$2
    local file_type=$3
    
    if [ -f "$file" ]; then
        lines=$(wc -l < "$file")
        if [ $lines -gt $max_lines ]; then
            echo -e "${RED}❌ $file_type : $file${NC}"
            echo -e "   Taille: $lines lignes (max: $max_lines)"
            ((errors++))
        else
            echo -e "${GREEN}✅ $file_type : $file${NC}"
            echo -e "   Taille: $lines lignes (max: $max_lines)"
        fi
    fi
}

# Vérification main.rs (max 120 lignes)
echo "═══════════════════════════════════════"
echo "  FICHIER PRINCIPAL"
echo "═══════════════════════════════════════"
check_file "src-tauri/src/main.rs" 120 "Main"
echo ""

# Vérification des services (max 300 lignes)
echo "═══════════════════════════════════════"
echo "  SERVICES"
echo "═══════════════════════════════════════"
if [ -d "src-tauri/src/services" ]; then
    for service in src-tauri/src/services/*.rs; do
        if [ -f "$service" ]; then
            check_file "$service" 300 "Service"
        fi
    done
else
    echo -e "${YELLOW}⚠️  Dossier src-tauri/src/services/ n'existe pas encore${NC}"
fi
echo ""

# Vérification des commandes (max 200 lignes)
echo "═══════════════════════════════════════"
echo "  COMMANDES TAURI"
echo "═══════════════════════════════════════"
if [ -d "src-tauri/src/commands" ]; then
    for cmd in src-tauri/src/commands/*.rs; do
        if [ -f "$cmd" ]; then
            check_file "$cmd" 200 "Command"
        fi
    done
else
    echo -e "${YELLOW}⚠️  Dossier src-tauri/src/commands/ n'existe pas encore${NC}"
fi
echo ""

# Vérification des modèles (max 150 lignes)
echo "═══════════════════════════════════════"
echo "  MODÈLES"
echo "═══════════════════════════════════════"
if [ -d "src-tauri/src/models" ]; then
    for model in src-tauri/src/models/*.rs; do
        if [ -f "$model" ]; then
            check_file "$model" 150 "Model"
        fi
    done
else
    echo -e "${YELLOW}⚠️  Dossier src-tauri/src/models/ n'existe pas encore${NC}"
fi
echo ""

# Résumé
echo "═══════════════════════════════════════"
echo "  RÉSUMÉ"
echo "═══════════════════════════════════════"
if [ $errors -gt 0 ]; then
    echo -e "${RED}❌ $errors fichier(s) dépasse(nt) la limite${NC}"
    echo -e "${RED}   Action requise: refactoriser les fichiers trop grands${NC}"
    exit 1
else
    echo -e "${GREEN}✅ Tous les fichiers respectent les limites de taille${NC}"
    exit 0
fi
