use crate::components::CameraController;
use crate::console_log;
use crate::task::TaskController;
use crate::types::{
    complete_work_session_with_task, generate_session_id, get_session_stats_from_db,
    save_session_to_db, NewSession, SessionStats, SessionType, TimerSettings, TimerState,
};
use crate::utils::{clearInterval, get_current_iso_time, setInterval};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone)]
pub struct TimerController {
    pub timer_state: RwSignal<TimerState>,
    pub session_type: RwSignal<SessionType>,
    pub time_remaining: RwSignal<u32>,
    pub completed_work_sessions: RwSignal<u32>, // Historical total for stats
    pub current_cycle_work_sessions: RwSignal<u32>, // Current cycle for break timing
    pub interval_id: RwSignal<Option<i32>>,
    pub session_stats: RwSignal<Option<SessionStats>>,
    pub session_start_time: RwSignal<Option<String>>,
    pub current_session_id: RwSignal<Option<String>>,
    pub loading: RwSignal<bool>,
    pub timer_settings: RwSignal<TimerSettings>,
}

impl TimerController {
    pub fn new() -> Self {
        let settings = TimerSettings::load_from_storage();

        let controller = Self {
            timer_state: RwSignal::new(TimerState::Stopped),
            session_type: RwSignal::new(SessionType::Work),
            time_remaining: RwSignal::new(settings.work_duration_minutes * 60),
            completed_work_sessions: RwSignal::new(0u32), // Historical total
            current_cycle_work_sessions: RwSignal::new(0u32), // Current cycle
            interval_id: RwSignal::new(None::<i32>),
            session_stats: RwSignal::new(None::<SessionStats>),
            session_start_time: RwSignal::new(None::<String>),
            current_session_id: RwSignal::new(None::<String>),
            loading: RwSignal::new(false),
            timer_settings: RwSignal::new(settings),
        };

        // Load initial stats from database
        controller.load_session_stats();

        controller
    }

    pub fn update_timer_settings(&self, new_settings: TimerSettings) {
        // Save to storage
        new_settings.save_to_storage();

        // Update signal
        self.timer_settings.set(new_settings.clone());

        // If timer is stopped, update time remaining for current session
        if self.timer_state.get() == TimerState::Stopped {
            let current_duration = self.session_type.get().duration_minutes(&new_settings) * 60;
            self.time_remaining.set(current_duration);
        }

        console_log!("Timer settings updated");
    }

    pub fn start_timer(&self) {
        // Set initial time if starting fresh
        if self.timer_state.get() == TimerState::Stopped {
            let settings = self.timer_settings.get();
            self.time_remaining
                .set(self.session_type.get().duration_minutes(&settings) * 60);
            self.session_start_time.set(Some(get_current_iso_time()));
            self.current_session_id.set(Some(generate_session_id()));
        }

        self.timer_state.set(TimerState::Running);
        console_log!("Timer started for {:?} session", self.session_type.get());

        // Create timer interval
        let time_remaining = self.time_remaining;
        let timer_closure = Closure::wrap(Box::new(move || {
            let current = time_remaining.get();
            if current > 0 {
                time_remaining.set(current - 1);
            }
        }) as Box<dyn FnMut()>);

        let id = setInterval(&timer_closure, 1000);
        timer_closure.forget();
        self.interval_id.set(Some(id));
    }

    pub fn pause_timer(&self) {
        self.timer_state.set(TimerState::Paused);
        if let Some(id) = self.interval_id.get() {
            clearInterval(id);
            self.interval_id.set(None);
        }
        console_log!("Timer paused");
    }

    pub fn stop_timer(&self) {
        self.timer_state.set(TimerState::Stopped);
        if let Some(id) = self.interval_id.get() {
            clearInterval(id);
            self.interval_id.set(None);
        }
        let settings = self.timer_settings.get();
        self.time_remaining
            .set(self.session_type.get().duration_minutes(&settings) * 60);
        self.session_start_time.set(None);
        self.current_session_id.set(None);
        console_log!("Timer stopped");
    }

    pub fn set_session_type(&self, session_type: SessionType) {
        if self.timer_state.get() == TimerState::Stopped {
            self.session_type.set(session_type);
            let settings = self.timer_settings.get();
            self.time_remaining
                .set(session_type.duration_minutes(&settings) * 60);
            console_log!("Session type changed to: {:?}", session_type);
        }
    }

    pub fn reset_work_sessions(&self) {
        self.current_cycle_work_sessions.set(0);
        console_log!("Work session cycle count reset");
    }

    pub fn get_next_session_info(&self) -> (SessionType, String) {
        let current_cycle_sessions = self.current_cycle_work_sessions.get();
        let settings = self.timer_settings.get();

        // If current session is work, we need to predict what happens after it completes
        let hypothetical_work_sessions = if self.session_type.get() == SessionType::Work {
            current_cycle_sessions + 1
        } else {
            current_cycle_sessions
        };

        let next_session = self
            .session_type
            .get()
            .next_session(hypothetical_work_sessions, &settings);

        let description = match next_session {
            SessionType::Work => "Back to work!".to_string(),
            SessionType::ShortBreak => {
                format!(
                    "Short break after {} work session(s)",
                    hypothetical_work_sessions
                )
            }
            SessionType::LongBreak => {
                format!(
                    "Long break after {} work sessions!",
                    hypothetical_work_sessions
                )
            }
        };

        (next_session, description)
    }

    fn should_auto_start_session(&self, session_type: SessionType) -> bool {
        let settings = self.timer_settings.get();
        match session_type {
            SessionType::Work => settings.auto_start_work,
            SessionType::ShortBreak | SessionType::LongBreak => settings.auto_start_breaks,
        }
    }

    fn save_session_with_task_tracking(&self, session: NewSession, focus_time_seconds: u32) {
        let controller = self.clone();
        spawn_local(async move {
            controller.loading.set(true);

            // Use the new task-aware session saving function
            match complete_work_session_with_task(session, focus_time_seconds).await {
                Ok(_) => {
                    console_log!("Session with task tracking saved successfully!");
                    controller.load_session_stats();
                }
                Err(e) => {
                    console_log!("Error saving session with task tracking: {}", e);
                }
            }

            controller.loading.set(false);
        });
    }

    fn save_session(&self, session: NewSession) {
        let controller = self.clone();
        spawn_local(async move {
            controller.loading.set(true);

            match save_session_to_db(session).await {
                Ok(_) => {
                    console_log!("Session saved successfully!");
                    controller.load_session_stats();
                }
                Err(e) => {
                    console_log!("Error saving session: {}", e);
                }
            }

            controller.loading.set(false);
        });
    }

    pub fn load_session_stats(&self) {
        let controller = self.clone();
        spawn_local(async move {
            controller.loading.set(true);

            match get_session_stats_from_db().await {
                Ok(stats) => {
                    // Update historical stats for display
                    controller.completed_work_sessions.set(stats.work_sessions);
                    controller.session_stats.set(Some(stats));

                    // Note: We don't update current_cycle_work_sessions here
                    // as it should only be managed by session completion and reset
                }
                Err(e) => {
                    console_log!("Error loading session stats: {}", e);
                }
            }

            controller.loading.set(false);
        });
    }

    async fn send_session_notification(&self, session_type: SessionType) {
        let session_type_str = session_type.to_string();
        let settings = self.timer_settings.get();
        let duration_minutes = session_type.duration_minutes(&settings);

        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
            "sessionType": session_type_str,
            "durationMinutes": duration_minutes
        }))
        .unwrap_or(JsValue::NULL);

        let result = invoke("session_completed_notification", args).await;

        // Try to deserialize as Result<String, String> to check for errors
        match serde_wasm_bindgen::from_value::<Result<String, String>>(result) {
            Ok(Ok(_)) => {
                console_log!("Session completion notification sent successfully");
            }
            Ok(Err(e)) => {
                console_log!("Failed to send session notification: {}", e);
                // Fallback to web notification if Tauri fails
                self.send_web_notification(session_type).await;
            }
            Err(_) => {
                console_log!("Unexpected response from session notification command");
                // Fallback to web notification if Tauri fails
                self.send_web_notification(session_type).await;
            }
        }
    }

    // Fallback web notification for when Tauri notifications fail
    async fn send_web_notification(&self, session_type: SessionType) {
        // Try to use Web Notification API as fallback
        if let Some(window) = web_sys::window() {
            // Try browser notification first
            if let Ok(notification_constructor) =
                js_sys::Reflect::get(&window, &"Notification".into())
            {
                // Check permission
                let permission =
                    js_sys::Reflect::get(&notification_constructor, &"permission".into())
                        .unwrap_or_else(|_| "denied".into());

                let mut permission_str = permission.as_string().unwrap_or("denied".to_string());

                // Request permission if needed
                if permission_str == "default" {
                    if let Ok(request_permission_fn) =
                        js_sys::Reflect::get(&notification_constructor, &"requestPermission".into())
                    {
                        if let Ok(function) = request_permission_fn.dyn_into::<js_sys::Function>() {
                            let permission_promise = function.call0(&notification_constructor);
                            if let Ok(promise_value) = permission_promise {
                                if let Ok(promise) = promise_value.dyn_into::<js_sys::Promise>() {
                                    let permission_result =
                                        wasm_bindgen_futures::JsFuture::from(promise)
                                            .await
                                            .unwrap_or_else(|_| "denied".into());

                                    permission_str = permission_result
                                        .as_string()
                                        .unwrap_or("denied".to_string());
                                }
                            }
                        }
                    }
                }

                // Create notification if permission is granted
                if permission_str == "granted" {
                    let title = "Pomodoro Timer";
                    let body = match session_type {
                        SessionType::Work => "Work session completed! Time for a break!",
                        SessionType::ShortBreak => "Short break completed! Back to work!",
                        SessionType::LongBreak => "Long break completed! Ready for focused work!",
                    };

                    let options = js_sys::Object::new();
                    js_sys::Reflect::set(&options, &"body".into(), &body.into()).unwrap();
                    js_sys::Reflect::set(&options, &"icon".into(), &"/public/tauri.svg".into())
                        .unwrap();

                    let args = js_sys::Array::new();
                    args.push(&title.into());
                    args.push(&options);

                    let _notification_instance =
                        js_sys::Reflect::construct(&notification_constructor.into(), &args);

                    console_log!("Web notification sent as fallback");
                    return; // Successfully sent notification, no need for audio fallback
                }
            }
        }

        // Audio beep fallback if notifications fail
        self.play_audio_beep().await;
    }

    async fn play_audio_beep(&self) {
        if let Some(window) = web_sys::window() {
            // Try to create audio context
            if let Ok(audio_context_constructor) =
                js_sys::Reflect::get(&window, &"AudioContext".into())
            {
                if let Ok(audio_context) = js_sys::Reflect::construct(
                    &audio_context_constructor.into(),
                    &js_sys::Array::new(),
                ) {
                    // Try to get a proper AudioContext
                    if let Ok(ctx) = audio_context.dyn_into::<web_sys::AudioContext>() {
                        if let (Ok(oscillator), Ok(gain_node)) =
                            (ctx.create_oscillator(), ctx.create_gain())
                        {
                            oscillator.frequency().set_value(800.0);
                            oscillator.set_type(web_sys::OscillatorType::Sine);

                            gain_node.gain().set_value(0.3);

                            let _ = oscillator.connect_with_audio_node(&gain_node);
                            let _ = gain_node.connect_with_audio_node(&ctx.destination());

                            let _ = oscillator.start();

                            // Stop after 3000ms
                            let stop_time = ctx.current_time() + 3.0;
                            let _ = oscillator.stop_with_when(stop_time);

                            console_log!("Played audio beep as fallback notification");
                            return;
                        }
                    }
                }
            }
        }

        console_log!("Could not play audio beep - audio context unavailable");
    }

    pub fn complete_session_with_camera_and_tasks(
        &self,
        camera_controller: Option<&CameraController>,
        task_controller: Option<&TaskController>,
    ) {
        console_log!("Timer completed with camera and task integration!");

        // Stop the current timer
        if let Some(id) = self.interval_id.get() {
            clearInterval(id);
            self.interval_id.set(None);
        }
        self.timer_state.set(TimerState::Stopped);

        // Calculate session data
        let session_type = self.session_type.get();
        let settings = self.timer_settings.get();
        let planned_duration = session_type.duration_minutes(&settings) * 60;
        let actual_duration = planned_duration - self.time_remaining.get();
        let end_time = get_current_iso_time();
        let start_time = self.session_start_time.get().unwrap_or_else(|| {
            let now = js_sys::Date::new_0();
            let start_ms = now.get_time() - (actual_duration as f64 * 1000.0);
            js_sys::Date::new(&start_ms.into()).to_iso_string().into()
        });

        // Get current session ID
        let session_id = self
            .current_session_id
            .get()
            .unwrap_or_else(|| generate_session_id());

        // Get task information if available
        let (task_id, subtask_id) = if let Some(task_ctrl) = task_controller {
            task_ctrl.get_current_selection()
        } else {
            (None, None)
        };

        // Send notification FIRST (before any async operations)
        let controller_for_notification = self.clone();
        spawn_local(async move {
            controller_for_notification
                .send_session_notification(session_type)
                .await;
        });

        // Update work session count FIRST (if it's a work session)
        let updated_work_sessions = if session_type == SessionType::Work {
            let current_cycle_sessions = self.current_cycle_work_sessions.get();
            let new_count = current_cycle_sessions + 1;
            self.current_cycle_work_sessions.set(new_count);
            new_count
        } else {
            self.current_cycle_work_sessions.get()
        };

        // NOW determine next session type using the UPDATED count
        let next_session = session_type.next_session(updated_work_sessions, &settings);

        console_log!(
            "Current work sessions: {}, Next session: {:?}",
            updated_work_sessions,
            next_session
        );

        // Handle camera recording completion
        if let Some(camera) = camera_controller {
            let camera = camera.clone();
            let session_id_clone = session_id.clone();
            let controller = self.clone();

            spawn_local(async move {
                let video_path = if camera.is_recording.get() {
                    match camera.stop_recording_and_save(&session_id_clone).await {
                        Ok(path) => {
                            console_log!("Video recording completed successfully");
                            path
                        }
                        Err(e) => {
                            console_log!("Failed to save video: {}", e);
                            None
                        }
                    }
                } else {
                    console_log!("No active recording to save");
                    None
                };

                // Create session with task information
                let new_session = NewSession {
                    session_type: session_type.to_string(),
                    planned_duration,
                    actual_duration,
                    start_time,
                    end_time,
                    completed: true,
                    video_path,
                    task_id,
                    subtask_id,
                };

                // Save session with task tracking for work sessions
                if session_type == SessionType::Work {
                    controller.save_session_with_task_tracking(new_session, actual_duration);
                } else {
                    controller.save_session(new_session);
                }
            });
        } else {
            let new_session = NewSession {
                session_type: session_type.to_string(),
                planned_duration,
                actual_duration,
                start_time,
                end_time,
                completed: true,
                video_path: None,
                task_id,
                subtask_id,
            };

            // Save session with task tracking for work sessions
            if session_type == SessionType::Work {
                self.save_session_with_task_tracking(new_session, actual_duration);
            } else {
                self.save_session(new_session);
            }
        }

        // Reload task stats if task controller is available
        if let Some(task_ctrl) = task_controller {
            task_ctrl.load_task_stats();
        }

        // Switch to next session type
        self.session_type.set(next_session);
        self.time_remaining
            .set(next_session.duration_minutes(&settings) * 60);
        self.session_start_time.set(None);
        self.current_session_id.set(None);

        console_log!("Switched to next session: {:?}", next_session);

        // Auto-start next session if enabled
        if self.should_auto_start_session(next_session) {
            console_log!("Auto-starting next session");
            // Small delay to allow UI to update
            let controller = self.clone();
            spawn_local(async move {
                gloo_timers::future::sleep(std::time::Duration::from_millis(1000)).await;
                controller.start_timer();
            });
        }
    }

    pub fn start_timer_with_camera(&self, camera_controller: Option<&CameraController>) {
        // Start the timer first
        self.start_timer();

        // Handle camera recording for break sessions
        if let Some(camera) = camera_controller {
            let session_type = self.session_type.get();
            let settings = camera.camera_settings.get();

            // Start recording if it's a break session or if recording during work is enabled
            if settings.enabled {
                let should_record = if settings.only_during_breaks {
                    session_type == SessionType::ShortBreak
                        || session_type == SessionType::LongBreak
                } else {
                    true // Record for all sessions if not limited to breaks
                };

                if should_record {
                    if let Err(e) = camera.start_recording(session_type) {
                        console_log!("Failed to start camera recording: {}", e);
                    } else {
                        console_log!("Camera recording started for {:?} session", session_type);
                    }
                } else {
                    console_log!("Skipping camera recording for {:?} session", session_type);
                }
            }
        }
    }

    pub fn stop_timer_with_camera(&self, camera_controller: Option<&CameraController>) {
        // Stop camera recording if active
        if let Some(camera) = camera_controller {
            if camera.is_recording.get() {
                camera.stop_recording();
                console_log!("Camera recording stopped");
            }
        }

        // Stop the timer
        self.stop_timer();
    }

    pub fn pause_timer_with_camera(&self, camera_controller: Option<&CameraController>) {
        // For now, we continue recording during pause
        // In the future, we might want to pause recording too
        self.pause_timer();

        if let Some(_camera) = camera_controller {
            console_log!("Timer paused, camera recording continues");
        }
    }

    // Helper method to check if current session should be recorded
    pub fn should_record_current_session(&self, camera_controller: &CameraController) -> bool {
        let settings = camera_controller.camera_settings.get();
        if !settings.enabled {
            return false;
        }

        let session_type = self.session_type.get();
        if settings.only_during_breaks {
            session_type == SessionType::ShortBreak || session_type == SessionType::LongBreak
        } else {
            true
        }
    }
}
