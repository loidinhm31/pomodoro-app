use std::path::PathBuf;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_videos_dir(app: tauri::AppHandle) -> Result<String, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;

    let videos_dir = app_data_dir.join("videos");

    // Create directory if it doesn't exist
    if !videos_dir.exists() {
        std::fs::create_dir_all(&videos_dir)
            .map_err(|e| format!("Failed to create videos directory: {}", e))?;
    }

    Ok(videos_dir.to_string_lossy().to_string())
}

#[tauri::command]
async fn save_video_file(
    app: tauri::AppHandle,
    filename: String,
    data: Vec<u8>,
) -> Result<String, String> {
    let videos_dir = get_videos_dir(app).await?;
    let file_path = PathBuf::from(videos_dir).join(filename);

    std::fs::write(&file_path, data).map_err(|e| format!("Failed to write video file: {}", e))?;

    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
async fn list_video_files(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let videos_dir = get_videos_dir(app).await?;
    let mut files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(videos_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(filename) = path.file_name() {
                        files.push(filename.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    Ok(files)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_videos_dir,
            save_video_file,
            list_video_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
