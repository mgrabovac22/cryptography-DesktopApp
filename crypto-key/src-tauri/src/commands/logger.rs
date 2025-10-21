use tauri::AppHandle;

#[tauri::command]
pub fn read_logs_command(app_handle: AppHandle) -> Result<String, String> {
    crate::logger::logger::read_logs(&app_handle)
}

#[tauri::command]
pub fn get_formatted_logs_command(app_handle: AppHandle) -> Result<Vec<crate::logger::logger::LogEntry>, String> {
    crate::logger::logger::get_formatted_logs(app_handle.clone())
}
