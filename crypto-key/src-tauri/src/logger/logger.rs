use simplelog::*;
use time::macros::format_description;
use std::io::Write;
use tauri::AppHandle;
use tauri::Manager;

static LOG_FILE_NAME: &str = "crypto_log.txt";

pub fn init_logger(app_handle: &AppHandle) {
    if let Ok(app_dir) = app_handle.path().app_data_dir() {
        let log_dir = app_dir.join("logger");
        let _ = std::fs::create_dir_all(&log_dir);
        let log_path = log_dir.join(LOG_FILE_NAME);

        let _ = CombinedLogger::init(vec![WriteLogger::new(
            LevelFilter::Info,
            ConfigBuilder::new()
                .set_time_format_custom(format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"))
                .set_time_offset_to_local()
                .unwrap()
                .build(),
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_path)
                .unwrap(),
        )]);

        log::info!("🚀 Logger inicializiran na {:?}", log_path);
    } else {
        eprintln!("⚠️ Nije moguće dohvatiti app_data_dir za logger inicijalizaciju");
    }
}

pub fn write_log_entry(app_handle: &AppHandle, message: &str) -> Result<(), String> {
    let log_path = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Error getting app data dir: {}", e))?
        .join("logger")
        .join(LOG_FILE_NAME);

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .map_err(|e| format!("Error opening log file: {}", e))?;

    let timestamp = chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
    writeln!(file, "{} {}", timestamp, message).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn read_logs(app_handle: &AppHandle) -> Result<String, String> {
    let log_path = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Error getting log dir: {}", e))?
        .join("logger")
        .join(LOG_FILE_NAME);

    std::fs::read_to_string(&log_path)
        .map_err(|e| format!("Error reading log file: {}", e))
}
