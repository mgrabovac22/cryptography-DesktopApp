use tauri::AppHandle;

#[tauri::command]
pub fn generate_keys(app_handle: AppHandle) -> Result<String, String> {
    crate::crypto::keys::generate_and_save(app_handle)
}

#[tauri::command]
pub fn get_private_key(app_handle: AppHandle) -> Result<String, String> {
    crate::crypto::keys::load_private_key_for_display(&app_handle)
}

#[tauri::command]
pub fn get_public_key(app_handle: AppHandle) -> Result<String, String> {
    crate::crypto::keys::load_public_key_for_display(&app_handle)
}

#[tauri::command]
pub fn get_secret_key(app_handle: AppHandle) -> Result<String, String> {
    crate::crypto::keys::load_secret_key_for_display(&app_handle)
}
