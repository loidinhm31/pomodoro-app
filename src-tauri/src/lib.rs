// src-tauri/src/lib.rs
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

// New notification commands
#[tauri::command]
async fn bring_window_to_front(app: tauri::AppHandle) -> Result<String, String> {
    if let Some(window) = app.get_webview_window("main") {
        // Bring window to front
        window.set_focus().map_err(|e| format!("Failed to set focus: {}", e))?;

        // Also try to show/restore the window if it's minimized
        window.show().map_err(|e| format!("Failed to show window: {}", e))?;
        window.unminimize().map_err(|e| format!("Failed to unminimize: {}", e))?;

        Ok("Window brought to front".to_string())
    } else {
        Err("Could not find main window".to_string())
    }
}

#[tauri::command]
async fn play_notification_sound() -> Result<String, String> {
    // Play system notification sound
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        // Use Windows system sound
        let result = Command::new("powershell")
            .args(&["-Command", "[console]::beep(800, 300)"])
            .status();

        match result {
            Ok(status) if status.success() => Ok("Notification sound played".to_string()),
            Ok(_) => Err("Failed to play notification sound".to_string()),
            Err(e) => Err(format!("Error playing sound: {}", e)),
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        // Use macOS system sound
        let result = Command::new("afplay")
            .arg("/System/Library/Sounds/Glass.aiff")
            .status();

        match result {
            Ok(status) if status.success() => Ok("Notification sound played".to_string()),
            Ok(_) => {
                // Fallback to system beep
                let beep_result = Command::new("osascript")
                    .args(&["-e", "beep"])
                    .status();

                match beep_result {
                    Ok(status) if status.success() => Ok("System beep played".to_string()),
                    _ => Err("Failed to play notification sound".to_string()),
                }
            },
            Err(e) => Err(format!("Error playing sound: {}", e)),
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        // Try paplay first (PulseAudio)
        let result = Command::new("paplay")
            .arg("/usr/share/sounds/alsa/Front_Left.wav")
            .status();

        if result.is_ok() && result.unwrap().success() {
            return Ok("Notification sound played".to_string());
        }

        // Fallback to system beep
        let beep_result = Command::new("beep").status();
        match beep_result {
            Ok(status) if status.success() => Ok("System beep played".to_string()),
            Ok(_) => {
                // Final fallback - terminal bell
                print!("\x07");
                Ok("Terminal bell played".to_string())
            },
            Err(e) => Err(format!("Error playing sound: {}", e)),
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("Playing sounds is not supported on this platform".to_string())
    }
}

#[tauri::command]
async fn show_system_notification(
    title: String,
    body: String,
    app: tauri::AppHandle,
) -> Result<String, String> {
    // Show system notification
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let script = format!(
            r#"
            Add-Type -AssemblyName System.Windows.Forms
            $notification = New-Object System.Windows.Forms.NotifyIcon
            $notification.Icon = [System.Drawing.SystemIcons]::Information
            $notification.BalloonTipTitle = "{}"
            $notification.BalloonTipText = "{}"
            $notification.BalloonTipIcon = "Info"
            $notification.Visible = $true
            $notification.ShowBalloonTip(3000)
            Start-Sleep -Seconds 4
            $notification.Dispose()
            "#,
            title.replace('"', "\"\""),
            body.replace('"', "\"\"")
        );

        let result = Command::new("powershell")
            .args(&["-Command", &script])
            .status();

        match result {
            Ok(status) if status.success() => Ok("System notification shown".to_string()),
            _ => Err("Failed to show system notification".to_string()),
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let script = format!(
            r#"display notification "{}" with title "{}""#,
            body.replace('"', "\\\""),
            title.replace('"', "\\\"")
        );

        let result = Command::new("osascript")
            .args(&["-e", &script])
            .status();

        match result {
            Ok(status) if status.success() => Ok("System notification shown".to_string()),
            _ => Err("Failed to show system notification".to_string()),
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        let result = Command::new("notify-send")
            .args(&[&title, &body])
            .status();

        match result {
            Ok(status) if status.success() => Ok("System notification shown".to_string()),
            _ => Err("Failed to show system notification".to_string()),
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("System notifications are not supported on this platform".to_string())
    }
}

#[tauri::command]
async fn session_completed_notification(
    session_type: String,
    duration_minutes: u32,
    app: tauri::AppHandle,
) -> Result<String, String> {
    // Bring window to front
    bring_window_to_front(app.clone()).await?;

    // Play notification sound
    play_notification_sound().await?;

    // Show system notification
    let title = "Pomodoro Timer".to_string();
    let body = match session_type.as_str() {
        "Work" => format!("Work session completed! ({}m)\nTime for a break!", duration_minutes),
        "ShortBreak" => format!("Short break completed! ({}m)\nBack to work!", duration_minutes),
        "LongBreak" => format!("Long break completed! ({}m)\nReady for focused work!", duration_minutes),
        _ => format!("Session completed! ({}m)", duration_minutes),
    };

    show_system_notification(title, body, app).await?;

    Ok("Session completion notification sent".to_string())
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
            reveal_in_explorer,
            bring_window_to_front,
            play_notification_sound,
            show_system_notification,
            session_completed_notification
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}