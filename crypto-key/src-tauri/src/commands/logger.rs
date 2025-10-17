use tauri::AppHandle;

#[tauri::command]
pub fn read_logs_command(app_handle: AppHandle) -> Result<String, String> {
    crate::logger::logger::read_logs(&app_handle)
}
