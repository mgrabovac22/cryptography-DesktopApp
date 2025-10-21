use simplelog::*;
use time::macros::format_description;
use std::io::Write;
use tauri::{AppHandle};
use tauri::Manager;
use std::collections::HashMap;

#[derive(serde::Serialize, Debug)]
pub struct LogEntry {
    timestamp: String,
    level: String,
    event: String,
    details: HashMap<String, String>,
}

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
                .set_level_padding(simplelog::LevelPadding::Right)
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

pub fn get_formatted_logs(app_handle: AppHandle) -> Result<Vec<LogEntry>, String> {
    let log_content = read_logs(&app_handle)?;
    let mut entries = Vec::new();

    for line in log_content.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }

        let mut timestamp = String::from("N/A");
        let mut level = String::from("N/A");
        let mut message_content = trimmed_line.to_string();

        if trimmed_line.starts_with('[') {
            if let Some(time_end) = trimmed_line.find(']') {
                timestamp = trimmed_line[1..time_end].trim().to_string();
                
                let after_time = &trimmed_line[time_end + 1..].trim();
                message_content = after_time.to_string();
                
                if message_content.starts_with("EVENT:") {
                    level = "INFO".to_string();
                }
            }
        } else if trimmed_line.len() >= 19 && trimmed_line.chars().nth(4) == Some('-') {
            timestamp = trimmed_line[..19].to_string();
            let after_time = &trimmed_line[19..].trim();
            
            if let Some(level_start) = after_time.find('[') {
                if let Some(level_end) = after_time[level_start..].find(']') {
                    let raw_level = &after_time[level_start + 1..level_start + level_end];
                    level = raw_level.trim_matches(|c| c == '[' || c == ']' || c == ' ').split_once(' ').map(|(l, _)| l.to_string()).unwrap_or(raw_level.trim().to_string());
                    message_content = after_time[level_start + level_end + 1..].trim().to_string();
                } else {
                    message_content = after_time.to_string();
                }
            } else {
                message_content = after_time.to_string();
            }
        }

        let event_name: String;
        let mut details_map = HashMap::new();
        
        if message_content.starts_with("EVENT:") {
            let event_and_details = message_content.trim_start_matches("EVENT:").trim();
            let mut parts_iter = event_and_details.splitn(2, "; ");
            
            event_name = parts_iter.next().unwrap_or(event_and_details).trim().to_string();
            
            if let Some(raw_details) = parts_iter.next() {
                for detail in raw_details.split(';') {
                    if let Some((key, value)) = detail.split_once(':') {
                        details_map.insert(key.trim().to_string(), value.trim().to_string());
                    }
                }
            }
        } else if message_content.contains("Logger inicializiran na") {
            event_name = "Logger Initialization".to_string();
            if let Some((_, path_raw)) = message_content.split_once('"') {
                 if let Some((path_val, _)) = path_raw.split_once('"') {
                    details_map.insert("Path".to_string(), path_val.to_string());
                 }
            }
            if level == "N/A" { level = "INFO".to_string(); }
        } else {
            event_name = message_content.clone();
        }

        let final_level = level.to_uppercase().trim_matches(|c| c == '[' || c == ']' || c == ' ').to_string();

        entries.push(LogEntry {
            timestamp,
            level: final_level,
            event: event_name,
            details: details_map,
        });
    }

    Ok(entries)
}
