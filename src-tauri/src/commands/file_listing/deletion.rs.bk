use std::fs;

#[tauri::command]
pub async fn delete_calendar_file(file_path: String) -> Result<(), String> {
    tracing::info!("🗑️  Deleting calendar file: {}", file_path);
    fs::remove_file(&file_path).map_err(|e| format!("Failed to delete file: {}", e))?;
    tracing::info!("✅ File deleted successfully");
    Ok(())
}

#[tauri::command]
pub async fn delete_pair_files(file_paths: Vec<String>) -> Result<usize, String> {
    tracing::info!("🗑️  Deleting {} pair files", file_paths.len());
    let mut deleted = 0;
    for path in file_paths {
        if let Err(e) = fs::remove_file(&path) {
            tracing::warn!("Failed to delete {}: {}", path, e);
        } else {
            deleted += 1;
        }
    }
    tracing::info!("✅ Deleted {} files", deleted);
    Ok(deleted)
}
