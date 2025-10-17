use tauri::AppHandle;

#[tauri::command]
pub fn calculate_digest_and_save(app_handle: AppHandle, input_path: String) -> Result<String, String> {
    crate::crypto::signature::calculate_digest_and_save(&app_handle, &input_path)
}

#[tauri::command]
pub fn digitally_sign(app_handle: AppHandle, file_path: String) -> Result<String, String> {
    crate::crypto::signature::digitally_sign(&app_handle, &file_path)
}

#[tauri::command]
pub fn verify_signature(app_handle: AppHandle, file_path: String, signature_path: String) -> Result<bool, String> {
    crate::crypto::signature::verify_signature(&app_handle, &file_path, &signature_path)
}

#[tauri::command]
pub fn list_signatures_cmd(app_handle: AppHandle) -> Result<Vec<String>, String> {
    crate::crypto::signature::list_signatures(&app_handle)
}
