use crate::components::CameraController;
use crate::console_log;
use crate::types::{
    generate_session_id, get_session_stats_from_db, save_session_to_db, NewSession, SessionStats,
    SessionType, TimerState,
};
use crate::utils::{clearInterval, get_current_iso_time, setInterval};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone)]
pub struct TimerController {
    pub timer_state: RwSignal<TimerState>,
    pub session_type: RwSignal<SessionType>,
    pub time_remaining: RwSignal<u32>,
    pub completed_sessions: RwSignal<u32>,
    pub interval_id: RwSignal<Option<i32>>,
    pub session_stats: RwSignal<Option<SessionStats>>,
    pub session_start_time: RwSignal<Option<String>>,
    pub current_session_id: RwSignal<Option<String>>,
    pub loading: RwSignal<bool>,
}

impl TimerController {
    pub fn new() -> Self {
        let controller = Self {
            timer_state: RwSignal::new(TimerState::Stopped),
            session_type: RwSignal::new(SessionType::Work),
            time_remaining: RwSignal::new(25 * 60), // 25 minutes in seconds
            completed_sessions: RwSignal::new(0u32),
            interval_id: RwSignal::new(None::<i32>),
            session_stats: RwSignal::new(None::<SessionStats>),
            session_start_time: RwSignal::new(None::<String>),
            current_session_id: RwSignal::new(None::<String>),
            loading: RwSignal::new(false),
        };

        // Load initial stats from database
        controller.load_session_stats();

        controller
    }

    pub fn start_timer(&self) {
        // Set initial time if starting fresh
        if self.timer_state.get() == TimerState::Stopped {
            self.time_remaining
                .set(self.session_type.get().duration_minutes() * 60);
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
        self.time_remaining
            .set(self.session_type.get().duration_minutes() * 60);
        self.session_start_time.set(None);
        self.current_session_id.set(None);
        console_log!("Timer stopped");
    }

    pub fn complete_session(&self) {
        console_log!("Timer completed!");

        // Stop the current timer
        if let Some(id) = self.interval_id.get() {
            clearInterval(id);
            self.interval_id.set(None);
        }
        self.timer_state.set(TimerState::Stopped);

        // Calculate session data
        let session_type = self.session_type.get();
        let planned_duration = session_type.duration_minutes() * 60;
        let actual_duration = planned_duration - self.time_remaining.get();
        let end_time = get_current_iso_time();
        let start_time = self.session_start_time.get().unwrap_or_else(|| {
            // Fallback: calculate start time based on duration
            let now = js_sys::Date::new_0();
            let start_ms = now.get_time() - (actual_duration as f64 * 1000.0);
            js_sys::Date::new(&start_ms.into()).to_iso_string().into()
        });

        // Save session to database
        let new_session = NewSession {
            session_type: session_type.to_string(),
            planned_duration,
            actual_duration,
            start_time,
            end_time,
            completed: true,
            video_path: None,
        };

        self.save_session(new_session);

        // Auto-switch to next session type
        let current_sessions = self.completed_sessions.get();
        let next_session = session_type.next_session(current_sessions);
        self.session_type.set(next_session);
        self.time_remaining
            .set(next_session.duration_minutes() * 60);
        self.session_start_time.set(None);
        self.current_session_id.set(None);

        console_log!("Switched to next session: {:?}", next_session);
    }

    pub fn set_session_type(&self, session_type: SessionType) {
        if self.timer_state.get() == TimerState::Stopped {
            self.session_type.set(session_type);
            self.time_remaining
                .set(session_type.duration_minutes() * 60);
            console_log!("Session type changed to: {:?}", session_type);
        }
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
                    controller.completed_sessions.set(stats.work_sessions);
                    controller.session_stats.set(Some(stats));
                }
                Err(e) => {
                    console_log!("Error loading session stats: {}", e);
                }
            }

            controller.loading.set(false);
        });
    }

    pub fn complete_session_with_camera(&self, camera_controller: Option<&CameraController>) {
        console_log!("Timer completed with camera integration!");

        // Stop the current timer
        if let Some(id) = self.interval_id.get() {
            clearInterval(id);
            self.interval_id.set(None);
        }
        self.timer_state.set(TimerState::Stopped);

        // Calculate session data
        let session_type = self.session_type.get();
        let planned_duration = session_type.duration_minutes() * 60;
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

                // Save session with video path
                let new_session = NewSession {
                    session_type: session_type.to_string(),
                    planned_duration,
                    actual_duration,
                    start_time,
                    end_time,
                    completed: true,
                    video_path,
                };

                controller.save_session(new_session);
            });
        } else {
            // Save session without video
            let new_session = NewSession {
                session_type: session_type.to_string(),
                planned_duration,
                actual_duration,
                start_time,
                end_time,
                completed: true,
                video_path: None,
            };

            self.save_session(new_session);
        }

        // Auto-switch to next session type
        let current_sessions = self.completed_sessions.get();
        let next_session = session_type.next_session(current_sessions);
        self.session_type.set(next_session);
        self.time_remaining
            .set(next_session.duration_minutes() * 60);
        self.session_start_time.set(None);
        self.current_session_id.set(None);

        console_log!("Switched to next session: {:?}", next_session);
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
