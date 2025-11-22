# 🧰 Antigravity Starter Kit

Ce dossier contient tout ce dont vous avez besoin pour démarrer un nouveau projet avec Antigravity en mode "Vibe Coding" tout en gardant vos standards de qualité élevés.

## 📂 Contenu du Kit

### 1. `.clinerules` (Le Cerveau)
Définit votre identité de codeur, vos préférences (Vue.js, Rust, Français) et vos règles strictes (pas de unwrap, tailles de fichiers).
> **Action** : À copier à la racine du nouveau projet.

### 2. `SYSTEM_PROMPT.md` (Le Gardien)
Définit le protocole de sécurité : Phase 1 (Code) -> Phase 2 (Validation). Il force l'IA à vérifier son travail avant de commiter.
> **Action** : À copier à la racine.

### 3. `.agent/workflows/` (L'Automatisation)
Contient vos commandes magiques comme `/validate`.
> **Action** : Copier tout le dossier `.agent` à la racine.

### 4. `scripts/` (Les Outils)
Vos scripts de validation automatique (Impact Detection, Quality Check).
> **Action** : Copier le dossier `scripts` à la racine.

---

## 🚀 Procédure de Démarrage (New Project)

1. **Créer le projet** (ex: `npm create tauri-app`)
2. **Copier le Starter Kit** :
   ```bash
   cp -r ~/Antigravity_Template/.clinerules .
   cp -r ~/Antigravity_Template/SYSTEM_PROMPT.md .
   cp -r ~/Antigravity_Template/.agent .
   cp -r ~/Antigravity_Template/scripts .
   ```
3. **Initialiser** :
   ```bash
   chmod +x scripts/impact-detection/*.sh
   ./scripts/impact-detection/init-impact-system.sh
   ```
4. **Coder !**
   Dites simplement à Antigravity : "Lis le SYSTEM_PROMPT et on attaque".

---

## 🔄 Mises à jour
Si vous modifiez vos règles dans un projet, pensez à les reporter dans ce dossier Template pour que les futurs projets en profitent.
