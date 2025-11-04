//! Commandes Tauri pour la gestion de la configuration

use crate::services::ConfigService;

/// Obtient le fichier calendrier sélectionné
#[tauri::command]
pub async fn get_selected_calendar_file() -> Result<Option<String>, String> {
    ConfigService::get_selected_calendar_file()
        .map_err(|e| format!("Erreur lors de la récupération de la configuration: {}", e))
}

/// Définit le fichier calendrier sélectionné
#[tauri::command]
pub async fn set_selected_calendar_file(filename: String) -> Result<(), String> {
    ConfigService::set_selected_calendar_file(filename)
        .map_err(|e| format!("Erreur lors de la sauvegarde de la configuration: {}", e))
}
