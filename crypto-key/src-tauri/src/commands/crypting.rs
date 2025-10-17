use tauri::AppHandle;

#[tauri::command]
pub fn symmetric_encrypt(app_handle: AppHandle, input_path: String, output_path: String) -> Result<String, String> {
    crate::crypto::crypting::symmetric_encrypt(&app_handle, input_path, output_path)
}

#[tauri::command]
pub fn symmetric_decrypt(app_handle: AppHandle, input_path: String, output_path: String) -> Result<String, String> {
    crate::crypto::crypting::symmetric_decrypt(&app_handle, &input_path, &output_path)
}

#[tauri::command]
pub fn asymmetric_encrypt(app_handle: AppHandle, input_path: String, output_path: String) -> Result<String, String> {
    crate::crypto::crypting::asymmetric_encrypt(&app_handle, &input_path, &output_path)
}

#[tauri::command]
pub fn asymmetric_decrypt(app_handle: AppHandle, input_path: String, output_path: String) -> Result<String, String> {
    crate::crypto::crypting::asymmetric_decrypt(&app_handle, &input_path, &output_path)
}
