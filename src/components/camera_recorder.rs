use js_sys::{Array, Object, Reflect};
use leptos::prelude::*;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{
    Blob, BlobEvent, MediaDevices, MediaRecorder,
    MediaStream, MediaStreamConstraints, Navigator
};

use crate::console_log;
pub(crate) use crate::types::{CameraSettings, CameraState, SessionType, TimerState, PermissionState};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// Thread-local storage for web-sys objects that can't be Send/Sync
thread_local! {
    static MEDIA_STREAM: RefCell<Option<MediaStream>> = RefCell::new(None);
    static MEDIA_RECORDER: RefCell<Option<MediaRecorder>> = RefCell::new(None);
    static RECORDED_CHUNKS: RefCell<Vec<Blob>> = RefCell::new(Vec::new());
}

#[derive(Clone)]
pub struct CameraController {
    pub camera_state: RwSignal<CameraState>,
    pub camera_settings: RwSignal<CameraSettings>,
    pub current_video_path: RwSignal<Option<String>>,
    pub is_recording: RwSignal<bool>,
    pub permission_state: RwSignal<PermissionState>, // Add permission state tracking
}

impl CameraController {
    pub fn new() -> Self {
        let controller = Self {
            camera_state: RwSignal::new(CameraState::Inactive),
            camera_settings: RwSignal::new(CameraSettings::default()),
            current_video_path: RwSignal::new(None),
            is_recording: RwSignal::new(false),
            permission_state: RwSignal::new(PermissionState::Unknown),
        };

        // Load saved settings
        controller.load_settings();

        controller
    }

    pub fn save_settings(&self) {
        let settings = self.camera_settings.get();
        if let Ok(settings_json) = serde_json::to_string(&settings) {
            if let Some(storage) = web_sys::window()
                .and_then(|w| w.local_storage().ok())
                .flatten()
            {
                let _ = storage.set_item("pomodoro_camera_settings", &settings_json);
                console_log!("Camera settings saved");
            }
        }
    }

    pub fn load_settings(&self) {
        if let Some(storage) = web_sys::window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
        {
            if let Ok(Some(settings_json)) = storage.get_item("pomodoro_camera_settings") {
                if let Ok(settings) = serde_json::from_str::<CameraSettings>(&settings_json) {
                    self.camera_settings.set(settings);
                    console_log!("Camera settings loaded");
                    return;
                }
            }
        }
        console_log!("Using default camera settings");
    }

    pub async fn check_camera_permissions(&self) -> PermissionState {
        if let Some(window) = web_sys::window() {
            let navigator = window.navigator();

            // Try to use the Permissions API via JavaScript reflection
            if let Ok(permissions) = js_sys::Reflect::get(&navigator, &"permissions".into()) {
                // Create permission descriptor for camera
                let descriptor = js_sys::Object::new();
                js_sys::Reflect::set(&descriptor, &"name".into(), &"camera".into()).unwrap();

                // Call permissions.query(descriptor)
                if let Ok(query_fn) = js_sys::Reflect::get(&permissions, &"query".into()) {
                    if let Ok(query_function) = query_fn.dyn_into::<js_sys::Function>() {
                        let args = js_sys::Array::new();
                        args.push(&descriptor);

                        if let Ok(promise_result) = query_function.apply(&permissions, &args) {
                            if let Ok(promise) = promise_result.dyn_into::<js_sys::Promise>() {
                                if let Ok(permission_status) = wasm_bindgen_futures::JsFuture::from(promise).await {
                                    // Get the state property
                                    if let Ok(state_value) = js_sys::Reflect::get(&permission_status, &"state".into()) {
                                        if let Some(state_str) = state_value.as_string() {
                                            let permission_state = PermissionState::from_string(&state_str);
                                            self.permission_state.set(permission_state.clone());
                                            console_log!("Camera permission state: {:?}", permission_state);
                                            return permission_state;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Fallback: assume unknown if we can't check
        let unknown_state = PermissionState::Unknown;
        self.permission_state.set(unknown_state.clone());
        console_log!("Could not check camera permissions, assuming unknown");
        unknown_state
    }

    pub async fn initialize_camera(&self) -> Result<(), String> {
        if !self.camera_settings.get().enabled {
            return Err("Camera is disabled".to_string());
        }

        self.camera_state.set(CameraState::Initializing);

        // Check permissions first
        let permission_state = self.check_camera_permissions().await;

        match permission_state {
            PermissionState::Denied => {
                self.camera_state.set(CameraState::Error("Camera permission denied. Please enable camera access in your browser settings and refresh the page.".to_string()));
                return Err("Camera permission denied. Please check browser settings.".to_string());
            },
            PermissionState::Granted => {
                console_log!("Camera permission already granted, proceeding...");
            },
            PermissionState::Prompt | PermissionState::Unknown => {
                console_log!("Camera permission will be requested...");
            },
        }

        let window = web_sys::window().ok_or("No window found")?;
        let navigator: Navigator = window.navigator();
        let media_devices: MediaDevices = navigator
            .media_devices()
            .map_err(|_| "MediaDevices not supported")?;

        // Create constraints for video recording
        let constraints = MediaStreamConstraints::new();
        let video_constraints = Object::new();

        // Set video quality based on settings
        let settings = self.camera_settings.get();
        match settings.video_quality.as_str() {
            "low" => {
                Reflect::set(&video_constraints, &"width".into(), &320.into()).unwrap();
                Reflect::set(&video_constraints, &"height".into(), &240.into()).unwrap();
            },
            "medium" => {
                Reflect::set(&video_constraints, &"width".into(), &640.into()).unwrap();
                Reflect::set(&video_constraints, &"height".into(), &480.into()).unwrap();
            },
            "high" => {
                Reflect::set(&video_constraints, &"width".into(), &1280.into()).unwrap();
                Reflect::set(&video_constraints, &"height".into(), &720.into()).unwrap();
            },
            _ => {
                Reflect::set(&video_constraints, &"width".into(), &640.into()).unwrap();
                Reflect::set(&video_constraints, &"height".into(), &480.into()).unwrap();
            }
        }

        constraints.set_video(&video_constraints);
        constraints.set_audio(&JsValue::TRUE);

        let stream_promise = media_devices
            .get_user_media_with_constraints(&constraints)
            .map_err(|_| "Failed to get user media")?;

        let stream = wasm_bindgen_futures::JsFuture::from(stream_promise)
            .await
            .map_err(|e| {
                // Provide more specific error messages
                let error_str = format!("{:?}", e);
                if error_str.contains("NotAllowedError") || error_str.contains("Permission") {
                    self.permission_state.set(PermissionState::Denied);
                    self.camera_state.set(CameraState::Error("Camera access denied. Please enable camera permissions in your browser and try again.".to_string()));
                    "Camera permission denied. Please enable camera access in your browser settings.".to_string()
                } else if error_str.contains("NotFoundError") {
                    self.camera_state.set(CameraState::Error("No camera found. Please connect a camera and try again.".to_string()));
                    "No camera device found. Please connect a camera.".to_string()
                } else if error_str.contains("NotReadableError") {
                    self.camera_state.set(CameraState::Error("Camera is being used by another application.".to_string()));
                    "Camera is busy or being used by another application.".to_string()
                } else {
                    self.camera_state.set(CameraState::Error(format!("Camera error: {}", error_str)));
                    format!("Failed to access camera: {}", error_str)
                }
            })?;

        let media_stream: MediaStream = stream.into();

        MEDIA_STREAM.with(|ms| {
            *ms.borrow_mut() = Some(media_stream);
        });

        // Update permission state to granted if we got here
        self.permission_state.set(PermissionState::Granted);
        self.camera_state.set(CameraState::Stopped);

        console_log!("Camera initialized successfully");
        Ok(())
    }

    // Provide permission help
    pub fn get_permission_help_text(&self) -> String {
        let user_agent = web_sys::window()
            .and_then(|w| w.navigator().user_agent().ok())
            .unwrap_or_default();

        if user_agent.contains("Chrome") {
            "Chrome: Click the camera icon in the address bar, or go to Settings > Privacy and security > Site Settings > Camera".to_string()
        } else if user_agent.contains("Firefox") {
            "Firefox: Click the shield icon in the address bar, or go to Preferences > Privacy & Security > Permissions > Camera".to_string()
        } else if user_agent.contains("Safari") {
            "Safari: Go to Safari > Settings > Websites > Camera, or check the address bar for permission prompts".to_string()
        } else if user_agent.contains("Edge") {
            "Edge: Click the camera icon in the address bar, or go to Settings > Site permissions > Camera".to_string()
        } else {
            "Please check your browser's privacy/security settings to enable camera access for this site".to_string()
        }
    }

    // Add method to reset camera state and retry
    pub async fn retry_camera_access(&self) -> Result<(), String> {
        console_log!("Retrying camera access...");

        // Reset state
        self.camera_state.set(CameraState::Inactive);
        self.permission_state.set(PermissionState::Unknown);

        // Stop any existing streams
        self.stop_camera();

        // Wait a bit and try again
        gloo_timers::future::sleep(std::time::Duration::from_millis(500)).await;

        // Re-check permissions and initialize
        self.initialize_camera().await
    }

    pub fn start_recording(&self, session_type: SessionType) -> Result<(), String> {
        let settings = self.camera_settings.get();

        // Check if we should record for this session type
        if settings.only_during_breaks && session_type == SessionType::Work {
            console_log!("Skipping recording for work session (only_during_breaks is enabled)");
            return Ok(());
        }

        if !settings.enabled {
            return Err("Camera is disabled".to_string());
        }

        let media_stream = MEDIA_STREAM.with(|ms| {
            ms.borrow().as_ref().cloned()
        }).ok_or("No media stream available")?;

        if self.is_recording.get() {
            console_log!("Already recording, stopping previous recording first");
            self.stop_recording();
        }

        // Create MediaRecorder
        let media_recorder = MediaRecorder::new_with_media_stream(&media_stream)
            .map_err(|e| format!("Failed to create MediaRecorder: {:?}", e))?;

        // Clear previous chunks
        RECORDED_CHUNKS.with(|chunks| {
            chunks.borrow_mut().clear();
        });

        // Set up event handlers
        let ondataavailable = Closure::wrap(Box::new(move |event: BlobEvent| {
            if let Some(blob) = event.data() {
                let blob_size = blob.size();
                if blob_size > 0.0 {
                    RECORDED_CHUNKS.with(|chunks| {
                        chunks.borrow_mut().push(blob);
                    });
                    console_log!("Recorded chunk of size: {}", blob_size);
                }
            }
        }) as Box<dyn FnMut(BlobEvent)>);

        media_recorder.set_ondataavailable(Some(ondataavailable.as_ref().unchecked_ref()));
        ondataavailable.forget();

        let camera_state = self.camera_state;
        let is_recording = self.is_recording;
        let onstop = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            console_log!("Recording stopped");
            camera_state.set(CameraState::Stopped);
            is_recording.set(false);
        }) as Box<dyn FnMut(web_sys::Event)>);

        media_recorder.set_onstop(Some(onstop.as_ref().unchecked_ref()));
        onstop.forget();

        let camera_state_error = self.camera_state;
        let is_recording_error = self.is_recording;
        let onerror = Closure::wrap(Box::new(move |event: web_sys::Event| {
            console_log!("Recording error: {:?}", event);
            camera_state_error.set(CameraState::Error("Recording error".to_string()));
            is_recording_error.set(false);
        }) as Box<dyn FnMut(web_sys::Event)>);

        media_recorder.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();

        // Start recording
        media_recorder.start_with_time_slice(1000) // Request data every 1 second
            .map_err(|e| format!("Failed to start recording: {:?}", e))?;

        MEDIA_RECORDER.with(|mr| {
            *mr.borrow_mut() = Some(media_recorder);
        });

        self.is_recording.set(true);
        self.camera_state.set(CameraState::Recording);

        console_log!("Started camera recording for {:?}", session_type);
        Ok(())
    }

    pub fn stop_recording(&self) {
        MEDIA_RECORDER.with(|mr| {
            if let Some(recorder) = mr.borrow().as_ref() {
                if self.is_recording.get() {
                    recorder.stop().unwrap_or_else(|e| {
                        console_log!("Error stopping recorder: {:?}", e);
                    });
                }
            }
        });
    }

    pub async fn stop_recording_and_save(&self, session_id: &str) -> Result<Option<String>, String> {
        if !self.is_recording.get() {
            return Ok(None);
        }

        // Stop the recording
        self.stop_recording();

        // Wait a bit for the recording to finalize and all chunks to be collected
        gloo_timers::future::sleep(std::time::Duration::from_millis(2000)).await;

        let chunks = RECORDED_CHUNKS.with(|chunks| {
            chunks.borrow().clone()
        });

        if chunks.is_empty() {
            console_log!("No video chunks recorded");
            return Ok(None);
        }

        console_log!("Processing {} video chunks", chunks.len());

        // Create a blob from all chunks
        let array = Array::new();
        for chunk in &chunks {
            array.push(chunk);
        }

        let blob_options = Object::new();
        Reflect::set(&blob_options, &"type".into(), &"video/webm".into()).unwrap();
        let final_blob = Blob::new_with_blob_sequence(&array)
            .map_err(|e| format!("Failed to create final blob: {:?}", e))?;

        console_log!("Created final blob of size: {}", final_blob.size());

        // Convert blob to Uint8Array
        let array_buffer_promise = final_blob.array_buffer();
        let array_buffer = wasm_bindgen_futures::JsFuture::from(array_buffer_promise)
            .await
            .map_err(|e| format!("Failed to convert blob to array buffer: {:?}", e))?;

        let uint8_array = js_sys::Uint8Array::new(&array_buffer);
        let video_data: Vec<u8> = uint8_array.to_vec();

        console_log!("Converted to {} bytes", video_data.len());

        // Generate filename
        let timestamp = js_sys::Date::now() as u64;
        let filename = format!("session_{}_{}.webm", session_id, timestamp);

        // Save using Tauri command
        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
            "filename": filename,
            "data": video_data
        })).map_err(|e| format!("Failed to serialize arguments: {}", e))?;

        let result = invoke("save_video_file", args).await;
        let video_path: String = serde_wasm_bindgen::from_value(result)
            .map_err(|e| format!("Failed to save video file: {}", e))?;

        self.current_video_path.set(Some(video_path.clone()));
        console_log!("Video saved to: {}", video_path);

        // Clear the chunks
        RECORDED_CHUNKS.with(|chunks| {
            chunks.borrow_mut().clear();
        });

        Ok(Some(video_path))
    }

    pub fn stop_camera(&self) {
        // Stop recording if active
        self.stop_recording();

        // Stop all media tracks
        MEDIA_STREAM.with(|ms| {
            if let Some(stream) = ms.borrow().as_ref() {
                let tracks = stream.get_tracks();
                for i in 0..tracks.length() {
                    if let Some(track) = tracks.get(i).dyn_into::<web_sys::MediaStreamTrack>().ok() {
                        track.stop();
                    }
                }
            }
            *ms.borrow_mut() = None;
        });

        MEDIA_RECORDER.with(|mr| {
            *mr.borrow_mut() = None;
        });

        RECORDED_CHUNKS.with(|chunks| {
            chunks.borrow_mut().clear();
        });

        self.is_recording.set(false);
        self.camera_state.set(CameraState::Inactive);

        console_log!("Camera stopped and cleaned up");
    }

    pub fn get_preview_stream(&self) -> Option<MediaStream> {
        MEDIA_STREAM.with(|ms| {
            ms.borrow().clone()
        })
    }
}

#[component]
pub fn CameraRecorder(
    controller: CameraController,
    current_session_type: RwSignal<SessionType>,
    timer_state: RwSignal<TimerState>,
) -> impl IntoView {
    let video_ref = NodeRef::<leptos::tachys::html::element::Video>::new();

    // Initialize camera when component mounts and settings change
    Effect::new({
        let controller = controller.clone();
        move |_| {
            let settings = controller.camera_settings.get();
            let session_type = current_session_type.get(); // Watch for session type changes

            if settings.enabled {
                // Check if we should initialize camera based on session type
                let should_initialize = if settings.only_during_breaks {
                    // Only initialize during breaks if "only_during_breaks" is enabled
                    session_type == SessionType::ShortBreak || session_type == SessionType::LongBreak
                } else {
                    // Initialize for all sessions if "only_during_breaks" is disabled
                    true
                };

                if should_initialize {
                    let controller_clone = controller.clone();
                    spawn_local(async move {
                        if let Err(e) = controller_clone.initialize_camera().await {
                            console_log!("Failed to initialize camera: {}", e);
                            controller_clone.camera_state.set(CameraState::Error(e));
                        }
                    });
                } else {
                    // Stop camera if it shouldn't be active for this session type
                    console_log!("Stopping camera for work session (only_during_breaks enabled)");
                    controller.stop_camera();
                }
            } else {
                controller.stop_camera();
            }
        }
    });

    // Connect stream to video element for preview
    Effect::new({
        let controller = controller.clone();
        move |_| {
            if let Some(video_element) = video_ref.get() {
                if let Some(stream) = controller.get_preview_stream() {
                    video_element.set_src_object(Some(&stream));
                    console_log!("Video preview connected to stream");
                }
            }
        }
    });

    // Helper function to check if we should show preview
    let should_show_preview = move || {
        let settings = controller.camera_settings.get();
        if !settings.enabled {
            return false;
        }

        let session_type = current_session_type.get();
        let timer_running = timer_state.get() == TimerState::Running;

        // Only show preview during breaks or if recording is enabled for all sessions
        if settings.only_during_breaks {
            timer_running && (session_type == SessionType::ShortBreak || session_type == SessionType::LongBreak)
        } else {
            timer_running || controller.camera_state.get() == CameraState::Recording
        }
    };

    view! {
        <div class="camera-recorder">
            {move || {
                if controller.camera_settings.get().enabled {
                    view! {
                        <div class="mb-4">
                            <div class="flex items-center justify-between mb-2">
                                <h4 class="text-sm font-medium text-gray-700 dark:text-gray-300">
                                    "Camera"
                                </h4>
                                <div class={move || format!(
                                    "px-2 py-1 rounded text-xs font-medium {}",
                                    match controller.camera_state.get() {
                                        CameraState::Inactive => "bg-gray-200 text-gray-700 dark:bg-gray-700 dark:text-gray-300",
                                        CameraState::Initializing => "bg-yellow-200 text-yellow-800 dark:bg-yellow-800 dark:text-yellow-200",
                                        CameraState::Recording => "bg-red-200 text-red-800 dark:bg-red-800 dark:text-red-200",
                                        CameraState::Stopped => "bg-green-200 text-green-800 dark:bg-green-800 dark:text-green-200",
                                        CameraState::Error(_) => "bg-red-200 text-red-800 dark:bg-red-800 dark:text-red-200",
                                    }
                                )}>
                                    {move || match controller.camera_state.get() {
                                        CameraState::Inactive => "Inactive".to_string(),
                                        CameraState::Initializing => "Initializing...".to_string(),
                                        CameraState::Recording => "● Recording".to_string(),
                                        CameraState::Stopped => "Ready".to_string(),
                                        CameraState::Error(e) => format!("Error: {}", e),
                                    }}
                                </div>
                            </div>

                            // Video preview - only show during appropriate times
                            {move || {
                                if should_show_preview() {
                                    match controller.camera_state.get() {
                                        CameraState::Recording | CameraState::Stopped => {
                                            view! {
                                                <div class="relative">
                                                    <video
                                                        node_ref=video_ref
                                                        class="w-full max-w-32 h-24 bg-gray-200 dark:bg-gray-700 rounded border object-cover"
                                                        autoplay=true
                                                        muted=true
                                                        playsinline=true
                                                    ></video>
                                                    {move || {
                                                        if controller.camera_state.get() == CameraState::Recording {
                                                            view! {
                                                                <div class="absolute top-1 right-1 bg-red-500 text-white text-xs px-1 rounded animate-pulse">
                                                                    "REC"
                                                                </div>
                                                            }.into_any()
                                                        } else {
                                                            view! { <div></div> }.into_any()
                                                        }
                                                    }}
                                                </div>
                                            }.into_any()
                                        },
                                        CameraState::Initializing => {
                                            view! {
                                                <div class="w-full max-w-32 h-24 bg-gray-200 dark:bg-gray-700 rounded border flex items-center justify-center">
                                                    <div class="text-xs text-gray-500 dark:text-gray-400 text-center">
                                                        <div class="loading-spinner inline-block w-4 h-4 border-2 border-gray-300 border-t-blue-500 rounded-full mb-1"></div>
                                                        <div>"Initializing..."</div>
                                                    </div>
                                                </div>
                                            }.into_any()
                                        },
                                        _ => {
                                            view! {
                                                <div class="w-full max-w-32 h-24 bg-gray-200 dark:bg-gray-700 rounded border flex items-center justify-center">
                                                    <span class="text-xs text-gray-500 dark:text-gray-400">"No Preview"</span>
                                                </div>
                                            }.into_any()
                                        }
                                    }
                                } else {
                                    // Show message when preview is not active
                                    let session_type = current_session_type.get();
                                    let settings = controller.camera_settings.get();
                                    let timer_running = timer_state.get() == TimerState::Running;
                                    
                                    if settings.only_during_breaks && session_type == SessionType::Work {
                                        if timer_running {
                                            view! {
                                                <div class="w-full max-w-32 h-24 bg-gray-100 dark:bg-gray-800 rounded border flex items-center justify-center">
                                                    <span class="text-xs text-gray-500 dark:text-gray-400 text-center">
                                                        "Camera disabled" <br/> "during work sessions"
                                                    </span>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <div class="w-full max-w-32 h-24 bg-gray-100 dark:bg-gray-800 rounded border flex items-center justify-center">
                                                    <span class="text-xs text-gray-500 dark:text-gray-400 text-center">
                                                        "Preview during" <br/> "breaks only"
                                                    </span>
                                                </div>
                                            }.into_any()
                                        }
                                    } else {
                                        view! {
                                            <div class="w-full max-w-32 h-24 bg-gray-200 dark:bg-gray-700 rounded border flex items-center justify-center">
                                                <span class="text-xs text-gray-500 dark:text-gray-400">"Preview Ready"</span>
                                            </div>
                                        }.into_any()
                                    }
                                }
                            }}

                            // Recording info
                            {move || {
                                if let Some(video_path) = controller.current_video_path.get() {
                                    let filename = video_path.split('/').last().unwrap_or("unknown").to_string();
                                    view! {
                                        <div class="mt-2 text-xs text-gray-500 dark:text-gray-400">
                                            "Last recording: " {filename}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            }}
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="text-xs text-gray-500 dark:text-gray-400">
                            "Camera disabled"
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}