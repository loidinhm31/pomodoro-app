use leptos::prelude::*;
use wasm_bindgen::prelude::*;

use crate::types::{TimerState, SessionType};
use crate::utils::{setInterval, clearInterval};
use crate::console_log;

#[derive(Clone)]
pub struct TimerController {
    pub timer_state: RwSignal<TimerState>,
    pub session_type: RwSignal<SessionType>,
    pub time_remaining: RwSignal<u32>,
    pub completed_sessions: RwSignal<u32>,
    pub interval_id: RwSignal<Option<i32>>,
}

impl TimerController {
    pub fn new() -> Self {
        Self {
            timer_state: RwSignal::new(TimerState::Stopped),
            session_type: RwSignal::new(SessionType::Work),
            time_remaining: RwSignal::new(25 * 60), // 25 minutes in seconds
            completed_sessions: RwSignal::new(0u32),
            interval_id: RwSignal::new(None::<i32>),
        }
    }

    pub fn start_timer(&self) {
        // Set initial time if starting fresh
        if self.timer_state.get() == TimerState::Stopped {
            self.time_remaining.set(self.session_type.get().duration_minutes() * 60);
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
    }

    pub fn complete_session(&self) {
        console_log!("Timer completed!");

        // Stop the current timer
        if let Some(id) = self.interval_id.get() {
            clearInterval(id);
            self.interval_id.set(None);
        }
        self.timer_state.set(TimerState::Stopped);

        // Update completed sessions
        let current_sessions = self.completed_sessions.get();
        self.completed_sessions.set(current_sessions + 1);

        // Auto-switch to next session type
        let next_session = self.session_type.get().next_session(current_sessions);
        self.session_type.set(next_session);
        self.time_remaining.set(next_session.duration_minutes() * 60);
    }

    pub fn set_session_type(&self, session_type: SessionType) {
        if self.timer_state.get() == TimerState::Stopped {
            self.session_type.set(session_type);
            self.time_remaining.set(session_type.duration_minutes() * 60);
        }
    }
}