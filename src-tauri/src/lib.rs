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

#[tauri::command]
async fn open_video_file(path: String) -> Result<String, String> {
    let path_buf = PathBuf::from(&path);

    // Check if file exists
    if !path_buf.exists() {
        return Err(format!("Video file not found: {}", path));
    }

    // Try to open the file with the default application
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let result = Command::new("cmd")
            .args(&["/C", "start", "", &path])
            .status();

        match result {
            Ok(status) if status.success() => Ok("File opened successfully".to_string()),
            Ok(_) => Err("Failed to open video file".to_string()),
            Err(e) => Err(format!("Error opening video file: {}", e)),
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let result = Command::new("open").arg(&path).status();

        match result {
            Ok(status) if status.success() => Ok("File opened successfully".to_string()),
            Ok(_) => Err("Failed to open video file".to_string()),
            Err(e) => Err(format!("Error opening video file: {}", e)),
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        let result = Command::new("xdg-open").arg(&path).status();

        match result {
            Ok(status) if status.success() => Ok("File opened successfully".to_string()),
            Ok(_) => Err("Failed to open video file".to_string()),
            Err(e) => Err(format!("Error opening video file: {}", e)),
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("Opening files is not supported on this platform".to_string())
    }
}

#[tauri::command]
async fn reveal_in_explorer(path: String) -> Result<String, String> {
    let path_buf = PathBuf::from(&path);

    // Check if file exists
    if !path_buf.exists() {
        return Err(format!("Video file not found: {}", path));
    }

    // Try to reveal the file in the file explorer
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let result = Command::new("explorer").args(&["/select,", &path]).status();

        match result {
            Ok(status) if status.success() => Ok("File revealed in explorer".to_string()),
            Ok(_) => Err("Failed to reveal file in explorer".to_string()),
            Err(e) => Err(format!("Error revealing file: {}", e)),
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let result = Command::new("open").args(&["-R", &path]).status();

        match result {
            Ok(status) if status.success() => Ok("File revealed in Finder".to_string()),
            Ok(_) => Err("Failed to reveal file in Finder".to_string()),
            Err(e) => Err(format!("Error revealing file: {}", e)),
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;

        // Try to use the file manager to show the file
        // First try with dbus (works with most modern file managers)
        let result = Command::new("dbus-send")
            .args(&[
                "--session",
                "--dest=org.freedesktop.FileManager1",
                "--type=method_call",
                "/org/freedesktop/FileManager1",
                "org.freedesktop.FileManager1.ShowItems",
                &format!("array:string:file://{}", path),
                "string:",
            ])
            .status();

        if result.is_ok() && result.unwrap().success() {
            return Ok("File revealed in file manager".to_string());
        }

        // Fallback: open the parent directory
        if let Some(parent) = path_buf.parent() {
            let result = Command::new("xdg-open").arg(parent).status();

            match result {
                Ok(status) if status.success() => Ok("Parent directory opened".to_string()),
                Ok(_) => Err("Failed to open parent directory".to_string()),
                Err(e) => Err(format!("Error opening directory: {}", e)),
            }
        } else {
            Err("Cannot determine parent directory".to_string())
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("Revealing files is not supported on this platform".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_videos_dir,
            save_video_file,
            list_video_files,
            open_video_file,
            reveal_in_explorer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
