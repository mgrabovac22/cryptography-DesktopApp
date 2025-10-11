mod crypto;

use crypto::{keys, signature, crypting};

use tauri::Manager;
use crate::crypto::keys::generate_and_save;

#[tauri::command]
fn generate_keys(app_handle: tauri::AppHandle) -> Result<String, String> {
    generate_and_save(app_handle)
}

#[tauri::command]
fn calculate_digest_and_save(path: String) -> Result<String, String> {
    signature::calculate_digest_and_save(&path) 
}

#[tauri::command]
fn digitally_sign(file_path: String) -> Result<String, String> {
    signature::digitally_sign(&file_path)
}

#[tauri::command]
fn verify_signature(file_path: String, signature_path: String) -> Result<bool, String> {
    signature::verify_signature(&file_path, &signature_path)
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
fn asymmetric_encrypt(input_path: String, output_path: String) -> Result<String, String> {
    crypting::asymmetric_encrypt(&input_path, &output_path)
}

#[tauri::command]
fn asymmetric_decrypt(input_path: String, output_path: String) -> Result<String, String> {
    crypting::asymmetric_decrypt(&input_path, &output_path)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_keys,
            calculate_digest_and_save,
            digitally_sign,
            verify_signature,
            symmetric_encrypt,
            symmetric_decrypt,
            asymmetric_encrypt,
            asymmetric_decrypt,
        ])
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("Error while running the Tauri application.");
}
