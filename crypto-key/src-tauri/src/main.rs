#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod crypto;
mod logger;

use crypto::{keys, signature, crypting};

#[tauri::command]
fn generate_keys(app_handle: tauri::AppHandle) -> Result<String, String> {
    keys::generate_and_save(app_handle)
}

#[tauri::command]
fn calculate_digest_and_save(app_handle: tauri::AppHandle, input_path: String) -> Result<String, String> {
    signature::calculate_digest_and_save(&app_handle, &input_path)
}

#[tauri::command]
fn digitally_sign(app_handle: tauri::AppHandle, file_path: String) -> Result<String, String> {
    signature::digitally_sign(&app_handle, &file_path)
}

#[tauri::command]
fn verify_signature(app_handle: tauri::AppHandle, file_path: String, signature_path: String) -> Result<bool, String> {
    signature::verify_signature(&app_handle, &file_path, &signature_path)
}

#[tauri::command]
fn symmetric_encrypt(app_handle: tauri::AppHandle, input_path: String, output_path: String) -> Result<String, String> {
    crypting::symmetric_encrypt(&app_handle, input_path, output_path)
}

#[tauri::command]
fn symmetric_decrypt(app_handle: tauri::AppHandle, input_path: String, output_path: String) -> Result<String, String> {
    crypting::symmetric_decrypt(&app_handle, &input_path, &output_path)
}

#[tauri::command]
fn asymmetric_encrypt(app_handle: tauri::AppHandle, input_path: String, output_path: String) -> Result<String, String> {
    crypting::asymmetric_encrypt(&app_handle, &input_path, &output_path)
}

#[tauri::command]
fn asymmetric_decrypt(app_handle: tauri::AppHandle, input_path: String, output_path: String) -> Result<String, String> {
    crypting::asymmetric_decrypt(&app_handle, &input_path, &output_path)
}

#[tauri::command]
fn list_signatures_cmd(app_handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    signature::list_signatures(&app_handle)
}

#[tauri::command]
fn read_logs_command(app_handle: tauri::AppHandle) -> Result<String, String> {
    logger::logger::read_logs(&app_handle)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            logger::logger::init_logger(&app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            generate_keys,
            calculate_digest_and_save,
            digitally_sign,
            verify_signature,
            symmetric_encrypt,
            symmetric_decrypt,
            asymmetric_encrypt,
            asymmetric_decrypt,
            list_signatures_cmd,
            read_logs_command
        ])
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("Error while running the Tauri application.");
}
