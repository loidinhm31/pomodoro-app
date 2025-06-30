use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::types::{TimerState, SessionType, NewSession, SessionStats, save_session_to_db, get_session_stats_from_db};
use crate::utils::{setInterval, clearInterval, get_current_iso_time};
use crate::console_log;

#[derive(Clone)]
pub struct TimerController {
    pub timer_state: RwSignal<TimerState>,
    pub session_type: RwSignal<SessionType>,
    pub time_remaining: RwSignal<u32>,
    pub completed_sessions: RwSignal<u32>,
    pub interval_id: RwSignal<Option<i32>>,
    pub session_stats: RwSignal<Option<SessionStats>>,
    pub session_start_time: RwSignal<Option<String>>,
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
            loading: RwSignal::new(false),
        };

        // Load initial stats from database
        controller.load_session_stats();

        controller
    }

    pub fn start_timer(&self) {
        // Set initial time if starting fresh
        if self.timer_state.get() == TimerState::Stopped {
            self.time_remaining.set(self.session_type.get().duration_minutes() * 60);
            self.session_start_time.set(Some(get_current_iso_time()));
        }

        self.timer_state.set(TimerState::Running);

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
    }

    pub fn stop_timer(&self) {
        self.timer_state.set(TimerState::Stopped);
        if let Some(id) = self.interval_id.get() {
            clearInterval(id);
            self.interval_id.set(None);
        }
        self.time_remaining.set(self.session_type.get().duration_minutes() * 60);
        self.session_start_time.set(None);
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
        };

        self.save_session(new_session);

        // Auto-switch to next session type
        let current_sessions = self.completed_sessions.get();
        let next_session = session_type.next_session(current_sessions);
        self.session_type.set(next_session);
        self.time_remaining.set(next_session.duration_minutes() * 60);
        self.session_start_time.set(None);
    }

    pub fn set_session_type(&self, session_type: SessionType) {
        if self.timer_state.get() == TimerState::Stopped {
            self.session_type.set(session_type);
            self.time_remaining.set(session_type.duration_minutes() * 60);
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
                },
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
                },
                Err(e) => {
                    console_log!("Error loading session stats: {}", e);
                }
            }

            controller.loading.set(false);
        });
    }
}