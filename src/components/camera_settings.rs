use crate::components::camera_recorder::CameraController;
use crate::cleanup_scheduler::{CleanupScheduler, CleanupScheduleSettings};
use crate::console_log;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// Data structures for video storage info
#[derive(Debug, Clone, Serialize, Deserialize)]
struct VideoStorageInfo {
    total_files: u32,
    total_size_mb: f64,
    oldest_file_age_days: Option<u32>,
    videos_dir: String,
}

#[component]
pub fn CameraSettings(controller: CameraController) -> impl IntoView {
    let settings = controller.camera_settings;
    let test_recording = RwSignal::new(false);

    // Function to save settings whenever they change
    let save_settings = {
        let controller = controller.clone();
        move || {
            controller.save_settings();
        }
    };

    view! {
        <div class="camera-settings space-y-4">
            <h4 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
                "Camera Settings"
            </h4>

            // Enable/Disable Camera
            <div class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded">
                <div>
                    <span class="text-sm font-medium text-gray-600 dark:text-gray-400">
                        "Enable Camera Recording"
                    </span>
                    <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                        "Record video during Pomodoro sessions"
                    </p>
                </div>
                <input
                    type="checkbox"
                    class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                    checked=move || settings.get().enabled
                    on:change={
                        let controller_ref = controller.clone();
                        let save_settings = save_settings.clone();
                        move |ev| {
                            let mut current_settings = settings.get();
                            current_settings.enabled = event_target_checked(&ev);
                            settings.set(current_settings);
                            save_settings();

                            // Stop camera if disabled
                            if !event_target_checked(&ev) {
                                controller_ref.stop_camera();
                            }
                        }
                    }
                />
            </div>

            {move || {
                if settings.get().enabled {
                    let controller_for_enabled = controller.clone();
                    let save_settings_enabled = save_settings.clone();
                    view! {
                        <div class="space-y-4">
                            // Camera Status
                            <div class="p-3 bg-blue-50 dark:bg-blue-900/20 rounded border border-blue-200 dark:border-blue-800">
                                <div class="flex items-center justify-between">
                                    <span class="text-sm font-medium text-blue-800 dark:text-blue-200">
                                        "Camera Status"
                                    </span>
                                    <span class={move || format!(
                                        "text-xs px-2 py-1 rounded {}",
                                        match controller_for_enabled.camera_state.get() {
                                            crate::types::CameraState::Inactive => "bg-gray-200 text-gray-700 dark:bg-gray-700 dark:text-gray-300",
                                            crate::types::CameraState::Initializing => "bg-yellow-200 text-yellow-800 dark:bg-yellow-800 dark:text-yellow-200",
                                            crate::types::CameraState::Recording => "bg-red-200 text-red-800 dark:bg-red-800 dark:text-red-200",
                                            crate::types::CameraState::Stopped => "bg-green-200 text-green-800 dark:bg-green-800 dark:text-green-200",
                                            crate::types::CameraState::Error(_) => "bg-red-200 text-red-800 dark:bg-red-800 dark:text-red-200",
                                        }
                                    )}>
                                        {move || match controller_for_enabled.camera_state.get() {
                                            crate::types::CameraState::Inactive => "Inactive".to_string(),
                                            crate::types::CameraState::Initializing => "Initializing...".to_string(),
                                            crate::types::CameraState::Recording => "● Recording".to_string(),
                                            crate::types::CameraState::Stopped => "Ready".to_string(),
                                            crate::types::CameraState::Error(e) => format!("Error: {}", e),
                                        }}
                                    </span>
                                </div>

                                // Test camera button
                                <button
                                    class="mt-2 px-3 py-1 bg-blue-500 hover:bg-blue-600 text-white text-xs rounded transition-colors disabled:opacity-50"
                                    disabled=move || test_recording.get()
                                    on:click={
                                        let controller_test = controller_for_enabled.clone();
                                        move |_| {
                                            let controller_clone = controller_test.clone();
                                            spawn_local(async move {
                                                test_recording.set(true);
                                                match controller_clone.initialize_camera().await {
                                                    Ok(_) => {
                                                        console_log!("Camera test successful!");
                                                        // Auto-cleanup after a few seconds
                                                        gloo_timers::future::sleep(std::time::Duration::from_millis(3000)).await;
                                                        controller_clone.stop_camera();
                                                    },
                                                    Err(e) => {
                                                        console_log!("Camera test failed: {}", e);
                                                    }
                                                }
                                                test_recording.set(false);
                                            });
                                        }
                                    }
                                >
                                    {move || if test_recording.get() { "Testing..." } else { "Test Camera" }}
                                </button>
                            </div>

                            // Recording Behavior
                            <div class="space-y-3 p-3 bg-gray-50 dark:bg-gray-700 rounded">
                                <h5 class="text-sm font-medium text-gray-700 dark:text-gray-300">
                                    "Recording Behavior"
                                </h5>

                                // Only during breaks
                                <div class="flex items-center justify-between">
                                    <div>
                                        <span class="text-sm text-gray-600 dark:text-gray-400">
                                            "Only during breaks"
                                        </span>
                                        <p class="text-xs text-gray-500 dark:text-gray-400">
                                            "Record only during short and long breaks"
                                        </p>
                                    </div>
                                    <input
                                        type="checkbox"
                                        class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                        checked=move || settings.get().only_during_breaks
                                        on:change={
                                            let save_settings_breaks = save_settings_enabled.clone();
                                            move |ev| {
                                                let mut current_settings = settings.get();
                                                current_settings.only_during_breaks = event_target_checked(&ev);
                                                settings.set(current_settings);
                                                save_settings_breaks();
                                            }
                                        }
                                    />
                                </div>
                            </div>

                            // Video Quality Settings
                            <div class="space-y-3 p-3 bg-gray-50 dark:bg-gray-700 rounded">
                                <h5 class="text-sm font-medium text-gray-700 dark:text-gray-300">
                                    "Video Quality"
                                </h5>

                                <div class="flex items-center justify-between">
                                    <span class="text-sm text-gray-600 dark:text-gray-400">
                                        "Resolution"
                                    </span>
                                    <select
                                        class="text-sm border rounded px-2 py-1 bg-white dark:bg-gray-800 dark:border-gray-600 text-gray-700 dark:text-gray-300"
                                        on:change={
                                            let save_settings_quality = save_settings_enabled.clone();
                                            move |ev| {
                                                let mut current_settings = settings.get();
                                                current_settings.video_quality = event_target_value(&ev);
                                                settings.set(current_settings);
                                                save_settings_quality();
                                            }
                                        }
                                    >
                                        <option value="low" selected=move || settings.get().video_quality == "low">
                                            "Low (320x240)"
                                        </option>
                                        <option value="medium" selected=move || settings.get().video_quality == "medium">
                                            "Medium (640x480)"
                                        </option>
                                        <option value="high" selected=move || settings.get().video_quality == "high">
                                            "High (1280x720)"
                                        </option>
                                    </select>
                                </div>

                                <div class="flex items-center justify-between">
                                    <div>
                                        <span class="text-sm text-gray-600 dark:text-gray-400">
                                            "Max Duration"
                                        </span>
                                        <p class="text-xs text-gray-500 dark:text-gray-400">
                                            "Maximum recording length in minutes"
                                        </p>
                                    </div>
                                    <input
                                        type="number"
                                        min="1"
                                        max="60"
                                        class="w-16 text-sm border rounded px-2 py-1 bg-white dark:bg-gray-800 dark:border-gray-600 text-gray-700 dark:text-gray-300"
                                        value=move || settings.get().max_duration_minutes
                                        on:input={
                                            let save_settings_duration = save_settings_enabled.clone();
                                            move |ev| {
                                                if let Ok(value) = event_target_value(&ev).parse::<u32>() {
                                                    let mut current_settings = settings.get();
                                                    current_settings.max_duration_minutes = value;
                                                    settings.set(current_settings);
                                                    save_settings_duration();
                                                }
                                            }
                                        }
                                    />
                                </div>
                            </div>

                            // Settings persistence notice
                            <div class="p-3 bg-green-50 dark:bg-green-900/20 rounded border border-green-200 dark:border-green-800">
                                <h5 class="text-sm font-medium text-green-800 dark:text-green-200 mb-2">
                                    "Settings Saved"
                                </h5>
                                <p class="text-xs text-green-700 dark:text-green-300">
                                    "Your camera settings are automatically saved and will be restored when you restart the app."
                                </p>
                            </div>

                            // Storage Info
                            <div class="p-3 bg-yellow-50 dark:bg-yellow-900/20 rounded border border-yellow-200 dark:border-yellow-800">
                                <h5 class="text-sm font-medium text-yellow-800 dark:text-yellow-200 mb-2">
                                    "Storage Information"
                                </h5>
                                <p class="text-xs text-yellow-700 dark:text-yellow-300">
                                    "Videos are saved locally on your device. You can find them in the app's data directory."
                                </p>
                                {move || {
                                    if let Some(video_path) = controller_for_enabled.current_video_path.get() {
                                        let filename = video_path.split('/').last().unwrap_or("unknown").to_string();
                                        view! {
                                            <p class="text-xs text-yellow-700 dark:text-yellow-300 mt-2">
                                                "Last video: " {filename}
                                            </p>
                                        }.into_any()
                                    } else {
                                        view! { <div></div> }.into_any()
                                    }
                                }}
                            </div>

                            // Privacy Notice
                            <div class="p-3 bg-green-50 dark:bg-green-900/20 rounded border border-green-200 dark:border-green-800">
                                <h5 class="text-sm font-medium text-green-800 dark:text-green-200 mb-2">
                                    "Privacy & Security"
                                </h5>
                                <p class="text-xs text-green-700 dark:text-green-300">
                                    "• All recordings stay on your device" <br/>
                                    "• No data is sent to external servers" <br/>
                                    "• You have full control over your recordings" <br/>
                                    "• Settings are saved locally for convenience"
                                </p>
                            </div>

                            // Video Cleanup Settings with Scheduler Integration
                            <VideoCleanupSchedulerSettings />
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="p-4 text-center text-gray-500 dark:text-gray-400">
                            <p class="text-sm">"Camera recording is disabled"</p>
                            <p class="text-xs mt-1">"Enable camera recording to access additional settings"</p>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}

#[component]
pub fn VideoCleanupSchedulerSettings() -> impl IntoView {
    let cleanup_scheduler = CleanupScheduler::new();
    let storage_info = RwSignal::new(None::<VideoStorageInfo>);
    let cleanup_in_progress = RwSignal::new(false);
    let cleanup_result = RwSignal::new(None::<String>);

    // Create all necessary clones upfront to avoid ownership issues
    let cleanup_scheduler_info = cleanup_scheduler.clone();
    let cleanup_scheduler_status = cleanup_scheduler.clone();
    let cleanup_scheduler_status_text = cleanup_scheduler.clone();
    let cleanup_scheduler_next = cleanup_scheduler.clone();
    let cleanup_scheduler_last = cleanup_scheduler.clone();
    let cleanup_scheduler_checkbox = cleanup_scheduler.clone();
    let cleanup_scheduler_change = cleanup_scheduler.clone();
    let cleanup_scheduler_days = cleanup_scheduler.clone();
    let cleanup_scheduler_input = cleanup_scheduler.clone();
    let cleanup_scheduler_hour = cleanup_scheduler.clone();
    let cleanup_scheduler_hour_input = cleanup_scheduler.clone();
    let cleanup_scheduler_manual = cleanup_scheduler.clone();
    let cleanup_scheduler_last_cleanup = cleanup_scheduler.clone();

    // Load storage info on mount
    Effect::new({
        move |_| {
            spawn_local(async move {
                match get_storage_info().await {
                    Ok(info) => storage_info.set(Some(info)),
                    Err(e) => console_log!("Failed to load storage info: {}", e),
                }
            });
        }
    });

    // Save settings when they change
    let save_scheduler_settings = {
        let cleanup_scheduler_save = cleanup_scheduler.clone();
        move |new_settings: CleanupScheduleSettings| {
            cleanup_scheduler_save.update_settings(new_settings);
        }
    };

    view! {
        <div class="space-y-4 p-4 bg-purple-50 dark:bg-purple-900/20 rounded-lg border border-purple-200 dark:border-purple-800">
            <div class="flex items-center space-x-2">
                <span class="text-lg">"🕐"</span>
                <h5 class="text-sm font-medium text-purple-800 dark:text-purple-200">
                    "Automated Video Cleanup (Cronjob-like)"
                </h5>
            </div>

            // Storage Information Display
            {move || {
                if let Some(info) = storage_info.get() {
                    view! {
                        <div class="bg-white dark:bg-gray-700 rounded p-3 space-y-2">
                            <div class="grid grid-cols-2 gap-4 text-sm">
                                <div>
                                    <span class="text-gray-600 dark:text-gray-400">"Total Files:"</span>
                                    <span class="font-medium text-gray-800 dark:text-white ml-2">
                                        {info.total_files}
                                    </span>
                                </div>
                                <div>
                                    <span class="text-gray-600 dark:text-gray-400">"Storage Used:"</span>
                                    <span class="font-medium text-gray-800 dark:text-white ml-2">
                                        {format!("{:.1} MB", info.total_size_mb)}
                                    </span>
                                </div>
                            </div>
                            
                            {move || {
                                if let Some(oldest_days) = info.oldest_file_age_days {
                                    view! {
                                        <div class="text-sm">
                                            <span class="text-gray-600 dark:text-gray-400">"Oldest Video:"</span>
                                            <span class=format!("font-medium ml-2 {}",
                                                if oldest_days > cleanup_scheduler_info.settings.get().days_to_keep {
                                                    "text-orange-600 dark:text-orange-400"
                                                } else {
                                                    "text-gray-800 dark:text-white"
                                                }
                                            )>
                                                {format!("{} days old", oldest_days)}
                                                {if oldest_days > cleanup_scheduler_info.settings.get().days_to_keep {
                                                    " (due for cleanup)"
                                                } else {
                                                    ""
                                                }}
                                            </span>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            }}

                            <div class="text-xs text-gray-500 dark:text-gray-400 pt-2 border-t">
                                "Location: " {info.videos_dir}
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="bg-white dark:bg-gray-700 rounded p-3">
                            <div class="text-sm text-gray-500 dark:text-gray-400 text-center">
                                "Loading storage information..."
                            </div>
                        </div>
                    }.into_any()
                }
            }}

            // Scheduler Status
            <div class="bg-white dark:bg-gray-700 rounded p-3">
                <div class="flex items-center justify-between mb-2">
                    <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
                        "Cleanup Scheduler Status"
                    </span>
                    <span class={move || format!(
                        "text-xs px-2 py-1 rounded {}",
                        if cleanup_scheduler_status.is_running.get() {
                            "bg-green-100 text-green-800 dark:bg-green-800 dark:text-green-200"
                        } else {
                            "bg-red-100 text-red-800 dark:bg-red-800 dark:text-red-200"
                        }
                    )}>
                        {move || if cleanup_scheduler_status_text.is_running.get() { "● Running" } else { "○ Stopped" }}
                    </span>
                </div>

                // Next cleanup time
                {move || {
                    if let Some(next_time) = cleanup_scheduler_next.get_next_cleanup_time() {
                        view! {
                            <div class="text-sm text-gray-600 dark:text-gray-400">
                                "Next cleanup: " <span class="font-medium">{next_time}</span>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="text-sm text-gray-500 dark:text-gray-400">
                                "Automatic cleanup is disabled"
                            </div>
                        }.into_any()
                    }
                }}

                // Last check time
                {move || {
                    if let Some(last_check) = cleanup_scheduler_last.last_check.get() {
                        let date = js_sys::Date::new(&last_check.into());
                        let formatted = date.to_locale_string("en-US", &js_sys::Object::new());
                        view! {
                            <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                                "Last checked: " {String::from(formatted)}
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }}
            </div>

            // Auto-cleanup Settings
            <div class="space-y-3">
                <div class="flex items-center justify-between">
                    <div>
                        <span class="text-sm font-medium text-gray-600 dark:text-gray-400">
                            "Enable automatic cleanup"
                        </span>
                        <p class="text-xs text-gray-500 dark:text-gray-400">
                            "Run cleanup daily while the app is running"
                        </p>
                    </div>
                    <input
                        type="checkbox"
                        class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                        checked=move || cleanup_scheduler_checkbox.settings.get().auto_cleanup_enabled
                        on:change={
                            let save_settings = save_scheduler_settings.clone();
                            move |ev| {
                                let mut current = cleanup_scheduler_change.settings.get();
                                current.auto_cleanup_enabled = event_target_checked(&ev);
                                save_settings(current);
                            }
                        }
                    />
                </div>

                <div class="flex items-center justify-between">
                    <div>
                        <span class="text-sm font-medium text-gray-600 dark:text-gray-400">
                            "Keep videos for (days)"
                        </span>
                        <p class="text-xs text-gray-500 dark:text-gray-400">
                            "Videos older than this will be deleted"
                        </p>
                    </div>
                    <input
                        type="number"
                        min="1"
                        max="30"
                        class="w-16 text-sm border rounded px-2 py-1 bg-white dark:bg-gray-800 dark:border-gray-600 text-gray-700 dark:text-gray-300"
                        value=move || cleanup_scheduler_days.settings.get().days_to_keep
                        on:input={
                            let save_settings = save_scheduler_settings.clone();
                            move |ev| {
                                if let Ok(value) = event_target_value(&ev).parse::<u32>() {
                                    if value >= 1 && value <= 30 {
                                        let mut current = cleanup_scheduler_input.settings.get();
                                        current.days_to_keep = value;
                                        save_settings(current);
                                    }
                                }
                            }
                        }
                    />
                </div>

                <div class="flex items-center justify-between">
                    <div>
                        <span class="text-sm font-medium text-gray-600 dark:text-gray-400">
                            "Cleanup time (hour)"
                        </span>
                        <p class="text-xs text-gray-500 dark:text-gray-400">
                            "Hour of day to run cleanup (0-23)"
                        </p>
                    </div>
                    <input
                        type="number"
                        min="0"
                        max="23"
                        class="w-16 text-sm border rounded px-2 py-1 bg-white dark:bg-gray-800 dark:border-gray-600 text-gray-700 dark:text-gray-300"
                        value=move || cleanup_scheduler_hour.settings.get().cleanup_hour
                        on:input={
                            let save_settings = save_scheduler_settings.clone();
                            move |ev| {
                                if let Ok(value) = event_target_value(&ev).parse::<u32>() {
                                    if value <= 23 {
                                        let mut current = cleanup_scheduler_hour_input.settings.get();
                                        current.cleanup_hour = value;
                                        save_settings(current);
                                    }
                                }
                            }
                        }
                    />
                </div>
            </div>

            // Manual Cleanup Button
            <div class="pt-3 border-t border-purple-200 dark:border-purple-700">
                <div class="flex items-center justify-between">
                    <div>
                        <span class="text-sm font-medium text-gray-600 dark:text-gray-400">
                            "Manual Cleanup"
                        </span>
                        <p class="text-xs text-gray-500 dark:text-gray-400">
                            "Run cleanup now to free up storage space"
                        </p>
                    </div>
                    <button
                        class="px-3 py-1 bg-purple-500 hover:bg-purple-600 text-white text-sm rounded transition-colors disabled:opacity-50"
                        disabled=move || cleanup_in_progress.get()
                        on:click={
                            move |_| {
                                cleanup_in_progress.set(true);
                                cleanup_result.set(None);
                                
                                let days_to_keep = cleanup_scheduler_manual.settings.get().days_to_keep;
                                spawn_local(async move {
                                    match run_manual_cleanup(days_to_keep).await {
                                        Ok(result) => {
                                            cleanup_result.set(Some(result));
                                            // Refresh storage info
                                            if let Ok(info) = get_storage_info().await {
                                                storage_info.set(Some(info));
                                            }
                                        },
                                        Err(e) => {
                                            cleanup_result.set(Some(format!("Error: {}", e)));
                                        }
                                    }
                                    cleanup_in_progress.set(false);
                                });
                            }
                        }
                    >
                        {move || if cleanup_in_progress.get() { "Cleaning..." } else { "Clean Now" }}
                    </button>
                </div>

                // Show cleanup result
                {move || {
                    if let Some(result) = cleanup_result.get() {
                        view! {
                            <div class="mt-2 p-2 bg-green-50 dark:bg-green-900/20 rounded text-xs text-green-700 dark:text-green-300">
                                {result}
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }}

                // Show last cleanup time
                {move || {
                    if let Some(last_cleanup_date) = cleanup_scheduler_last_cleanup.settings.get().last_cleanup_date {
                        view! {
                            <div class="mt-2 text-xs text-gray-500 dark:text-gray-400">
                                "Last cleanup: " {last_cleanup_date} " (automatically)"
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="mt-2 text-xs text-gray-500 dark:text-gray-400">
                                "No automatic cleanup has run yet"
                            </div>
                        }.into_any()
                    }
                }}
            </div>

            // Information Notice
            <div class="bg-blue-50 dark:bg-blue-900/20 rounded p-3 border border-blue-200 dark:border-blue-800">
                <h6 class="text-sm font-medium text-blue-800 dark:text-blue-200 mb-1">
                    "About Scheduled Cleanup"
                </h6>
                <div class="text-xs text-blue-700 dark:text-blue-300 space-y-1">
                    <p>"• Runs automatically every day at the specified hour"</p>
                    <p>"• Works even when the app stays open for days"</p>
                    <p>"• Only runs once per day (won't duplicate cleanup)"</p>
                    <p>"• Checks every 10 minutes if it's time to cleanup"</p>
                    <p>"• Deleted videos cannot be recovered"</p>
                </div>
            </div>
        </div>
    }
}

// Helper functions for Tauri commands
async fn get_storage_info() -> Result<VideoStorageInfo, String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({}))
        .map_err(|e| format!("Failed to serialize args: {}", e))?;

    let result = invoke("get_video_storage_info", args).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to get storage info: {}", e))
}

async fn run_manual_cleanup(days_to_keep: u32) -> Result<String, String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "daysOld": days_to_keep
    }))
        .map_err(|e| format!("Failed to serialize args: {}", e))?;

    let result = invoke("cleanup_old_videos", args).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to run cleanup: {}", e))
}