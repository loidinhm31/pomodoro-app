use crate::components::camera_recorder::CameraController;
use crate::types::PermissionState;
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
    let permission_check_in_progress = RwSignal::new(false);

    // Function to save settings whenever they change
    let save_settings = {
        let controller = controller.clone();
        move || {
            controller.save_settings();
        }
    };

    // Function to check permissions on component mount
    Effect::new({
        let controller = controller.clone();
        move |_| {
            if settings.get().enabled {
                let controller_clone = controller.clone();
                spawn_local(async move {
                    controller_clone.check_camera_permissions().await;
                });
            }
        }
    });

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
                            } else {
                                // Check permissions when enabling
                                let controller_clone = controller_ref.clone();
                                spawn_local(async move {
                                    controller_clone.check_camera_permissions().await;
                                });
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
                            // Permission Status and Camera Status
                            <div class="space-y-3">
                                // Permission Status
                                <div class={
                                    let permission_state = controller_for_enabled.permission_state.get();
                                    format!(
                                        "p-3 rounded border {}",
                                        match permission_state {
                                            PermissionState::Granted => "bg-green-50 dark:bg-green-900/20 border-green-200 dark:border-green-800",
                                            PermissionState::Denied => "bg-red-50 dark:bg-red-900/20 border-red-200 dark:border-red-800",
                                            PermissionState::Prompt => "bg-yellow-50 dark:bg-yellow-900/20 border-yellow-200 dark:border-yellow-800",
                                            PermissionState::Unknown => "bg-gray-50 dark:bg-gray-700 border-gray-200 dark:border-gray-600",
                                        }
                                    )
                                }>
                                    <div class="flex items-center justify-between">
                                        <div>
                                            <span class={
                                                let permission_state = controller_for_enabled.permission_state.get();
                                                format!(
                                                    "text-sm font-medium {}",
                                                    match permission_state {
                                                        PermissionState::Granted => "text-green-800 dark:text-green-200",
                                                        PermissionState::Denied => "text-red-800 dark:text-red-200",
                                                        PermissionState::Prompt => "text-yellow-800 dark:text-yellow-200",
                                                        PermissionState::Unknown => "text-gray-800 dark:text-gray-200",
                                                    }
                                                )
                                            }>
                                                "Camera Permission"
                                            </span>
                                            <p class={
                                                let permission_state = controller_for_enabled.permission_state.get();
                                                format!(
                                                    "text-xs mt-1 {}",
                                                    match permission_state {
                                                        PermissionState::Granted => "text-green-700 dark:text-green-300",
                                                        PermissionState::Denied => "text-red-700 dark:text-red-300",
                                                        PermissionState::Prompt => "text-yellow-700 dark:text-yellow-300",
                                                        PermissionState::Unknown => "text-gray-600 dark:text-gray-400",
                                                    }
                                                )
                                            }>
                                                {
                                                    let permission_state = controller_for_enabled.permission_state.get();
                                                    match permission_state {
                                                        PermissionState::Granted => "✅ Camera access granted",
                                                        PermissionState::Denied => "❌ Camera access denied - click 'Fix Permissions' below",
                                                        PermissionState::Prompt => "⏳ Camera permission will be requested when needed",
                                                        PermissionState::Unknown => "❓ Camera permission status unknown",
                                                    }
                                                }
                                            </p>
                                        </div>
                                        
                                        // Permission actions
                                        {
                                            let permission_state = controller_for_enabled.permission_state.get();
                                            let controller_permission = controller_for_enabled.clone();
                                            
                                            match permission_state {
                                                PermissionState::Denied => {
                                                    view! {
                                                        <div class="flex flex-col space-y-2">
                                                            <button
                                                                class="px-3 py-1 bg-red-500 hover:bg-red-600 text-white text-xs rounded transition-colors"
                                                                on:click={
                                                                    let controller_retry = controller_permission.clone();
                                                                    move |_| {
                                                                        let controller_clone = controller_retry.clone();
                                                                        spawn_local(async move {
                                                                            match controller_clone.retry_camera_access().await {
                                                                                Ok(_) => console_log!("Camera access retry successful"),
                                                                                Err(e) => console_log!("Camera access retry failed: {}", e),
                                                                            }
                                                                        });
                                                                    }
                                                                }
                                                            >
                                                                "Retry Access"
                                                            </button>
                                                        </div>
                                                    }.into_any()
                                                },
                                                PermissionState::Unknown => {
                                                    view! {
                                                        <button
                                                            class="px-3 py-1 bg-blue-500 hover:bg-blue-600 text-white text-xs rounded transition-colors disabled:opacity-50"
                                                            disabled=move || permission_check_in_progress.get()
                                                            on:click={
                                                                let controller_check = controller_permission.clone();
                                                                move |_| {
                                                                    permission_check_in_progress.set(true);
                                                                    let controller_clone = controller_check.clone();
                                                                    spawn_local(async move {
                                                                        controller_clone.check_camera_permissions().await;
                                                                        permission_check_in_progress.set(false);
                                                                    });
                                                                }
                                                            }
                                                        >
                                                            {move || if permission_check_in_progress.get() { "Checking..." } else { "Check Permission" }}
                                                        </button>
                                                    }.into_any()
                                                },
                                                _ => view! { <div></div> }.into_any()
                                            }
                                        }
                                    </div>
                                </div>

                                // Permission Help Text (shown when denied)
                                {
                                    let permission_state = controller_for_enabled.permission_state.get();
                                    if permission_state == PermissionState::Denied {
                                        let help_text = controller_for_enabled.get_permission_help_text();
                                        view! {
                                            <div class="p-3 bg-blue-50 dark:bg-blue-900/20 rounded border border-blue-200 dark:border-blue-800">
                                                <h6 class="text-sm font-medium text-blue-800 dark:text-blue-200 mb-2">
                                                    "🛠️ How to Fix Camera Permissions"
                                                </h6>
                                                <div class="text-xs text-blue-700 dark:text-blue-300 space-y-2">
                                                    <p><strong>"Method 1 (Recommended):"</strong></p>
                                                    <p>"1. Look for a camera icon 📹 in your browser's address bar"</p>
                                                    <p>"2. Click it and select 'Allow' for camera access"</p>
                                                    <p>"3. Refresh this page and try again"</p>
                                                    
                                                    <p class="pt-2"><strong>"Method 2 (Browser Settings):"</strong></p>
                                                    <p>{help_text}</p>
                                                    
                                                    <p class="pt-2"><strong>"Method 3 (Reset):"</strong></p>
                                                    <p>"Clear your browser's site data for this page and allow permissions when prompted again"</p>
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <div></div> }.into_any()
                                    }
                                }

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

                                    // Error display
                                    {move || {
                                        if let crate::types::CameraState::Error(error_msg) = controller_for_enabled.camera_state.get() {
                                            view! {
                                                <div class="mt-2 p-2 bg-red-100 dark:bg-red-900/20 rounded text-xs text-red-700 dark:text-red-300">
                                                    {error_msg}
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }
                                    }}

                                    // Test camera button
                                    <button
                                        class={
                                            let permission_state = controller_for_enabled.permission_state.get();
                                            let camera_state = controller_for_enabled.camera_state.get();
                                            
                                            if permission_state == PermissionState::Denied {
                                                "mt-2 px-3 py-1 bg-gray-400 text-white text-xs rounded cursor-not-allowed".to_string()
                                            } else if test_recording.get() || camera_state == crate::types::CameraState::Initializing {
                                                "mt-2 px-3 py-1 bg-blue-400 text-white text-xs rounded cursor-wait".to_string()
                                            } else {
                                                "mt-2 px-3 py-1 bg-blue-500 hover:bg-blue-600 text-white text-xs rounded transition-colors".to_string()
                                            }
                                        }
                                        disabled=move || {
                                            test_recording.get() 
                                            || controller_for_enabled.permission_state.get() == PermissionState::Denied
                                            || controller_for_enabled.camera_state.get() == crate::types::CameraState::Initializing
                                        }
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
                                        {move || {
                                            let permission_state = controller_for_enabled.permission_state.get();
                                            if permission_state == PermissionState::Denied {
                                                "Permission Denied"
                                            } else if test_recording.get() {
                                                "Testing..."
                                            } else {
                                                "Test Camera"
                                            }
                                        }}
                                    </button>
                                </div>
                            </div>

                            // Recording Behavior (only show if permission is granted or unknown)
                            {
                                let permission_state = controller_for_enabled.permission_state.get();
                                if permission_state != PermissionState::Denied {
                                    view! {
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
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            }

                            // Video Quality Settings (only show if permission is granted or unknown)
                            {
                                let permission_state = controller_for_enabled.permission_state.get();
                                if permission_state != PermissionState::Denied {
                                    view! {
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
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            }

                            // Settings persistence notice
                            <div class="p-3 bg-green-50 dark:bg-green-900/20 rounded border border-green-200 dark:border-green-800">
                                <h5 class="text-sm font-medium text-green-800 dark:text-green-200 mb-2">
                                    "Settings Saved"
                                </h5>
                                <p class="text-xs text-green-700 dark:text-green-300">
                                    "Your camera settings are automatically saved and will be restored when you restart the app."
                                </p>
                            </div>

                            // Video Cleanup Notice
                            <div class="p-3 bg-purple-50 dark:bg-purple-900/20 rounded border border-purple-200 dark:border-purple-800">
                                <h5 class="text-sm font-medium text-purple-800 dark:text-purple-200 mb-2">
                                    "Video Storage"
                                </h5>
                                <p class="text-xs text-purple-700 dark:text-purple-300">
                                    "Videos are stored locally on your device. You can manually clean up old videos from your file system if needed."
                                </p>
                            </div>
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