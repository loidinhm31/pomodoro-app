use crate::components::camera_recorder::CameraController;
use crate::console_log;
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn CameraSettings(controller: CameraController) -> impl IntoView {
    let settings = controller.camera_settings;
    let test_recording = RwSignal::new(false);

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
                        move |ev| {
                            let mut current_settings = settings.get();
                            current_settings.enabled = event_target_checked(&ev);
                            settings.set(current_settings);
                            
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
                                        on:change=move |ev| {
                                            let mut current_settings = settings.get();
                                            current_settings.only_during_breaks = event_target_checked(&ev);
                                            settings.set(current_settings);
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
                                        on:change=move |ev| {
                                            let mut current_settings = settings.get();
                                            current_settings.video_quality = event_target_value(&ev);
                                            settings.set(current_settings);
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
                                        on:input=move |ev| {
                                            if let Ok(value) = event_target_value(&ev).parse::<u32>() {
                                                let mut current_settings = settings.get();
                                                current_settings.max_duration_minutes = value;
                                                settings.set(current_settings);
                                            }
                                        }
                                    />
                                </div>
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
                                    "• You have full control over your recordings"
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