use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    fn clearInterval(id: i32);
}

// Macro for easier console logging
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimerState {
    Stopped,
    Running,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SessionType {
    Work,
    ShortBreak,
    LongBreak,
}

impl SessionType {
    pub fn duration_minutes(&self) -> u32 {
        match self {
            SessionType::Work => 25,
            SessionType::ShortBreak => 5,
            SessionType::LongBreak => 15,
        }
    }

    pub fn color_class(&self) -> &'static str {
        match self {
            SessionType::Work => "bg-red-500",
            SessionType::ShortBreak => "bg-green-500",
            SessionType::LongBreak => "bg-blue-500",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            SessionType::Work => "Work",
            SessionType::ShortBreak => "Short Break",
            SessionType::LongBreak => "Long Break",
        }
    }
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    // Timer state
    let (timer_state, set_timer_state) = signal(TimerState::Stopped);
    let (session_type, set_session_type) = signal(SessionType::Work);
    let (time_remaining, set_time_remaining) = signal(25 * 60); // 25 minutes in seconds
    let (completed_sessions, set_completed_sessions) = signal(0u32);
    let (interval_id, set_interval_id) = signal(None::<i32>);

    // Demo greeting functionality (can be removed later)
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    // Timer completion effect
    Effect::new(move |_| {
        if time_remaining.get() == 0 && timer_state.get() == TimerState::Running {
            console_log!("Timer completed!");

            // Stop the current timer
            if let Some(id) = interval_id.get() {
                clearInterval(id);
                set_interval_id.set(None);
            }
            set_timer_state.set(TimerState::Stopped);

            // Update completed sessions
            let current_sessions = completed_sessions.get();
            set_completed_sessions.set(current_sessions + 1);

            // Auto-switch to next session type
            let next_session = match session_type.get() {
                SessionType::Work => {
                    if (current_sessions + 1) % 4 == 0 {
                        SessionType::LongBreak
                    } else {
                        SessionType::ShortBreak
                    }
                },
                SessionType::ShortBreak | SessionType::LongBreak => SessionType::Work,
            };

            set_session_type.set(next_session);
            set_time_remaining.set(next_session.duration_minutes() * 60);
        }
    });

    // Timer control functions
    let start_timer = move || {
        // Set initial time if starting fresh
        if timer_state.get() == TimerState::Stopped {
            set_time_remaining.set(session_type.get().duration_minutes() * 60);
        }

        set_timer_state.set(TimerState::Running);

        // Create timer interval
        let timer_closure = Closure::wrap(Box::new(move || {
            let current = time_remaining.get();
            if current > 0 {
                set_time_remaining.set(current - 1);
            }
        }) as Box<dyn FnMut()>);

        let id = setInterval(&timer_closure, 1000);
        timer_closure.forget();
        set_interval_id.set(Some(id));
    };

    let pause_timer = move || {
        set_timer_state.set(TimerState::Paused);
        if let Some(id) = interval_id.get() {
            clearInterval(id);
            set_interval_id.set(None);
        }
    };

    let stop_timer = move || {
        set_timer_state.set(TimerState::Stopped);
        if let Some(id) = interval_id.get() {
            clearInterval(id);
            set_interval_id.set(None);
        }
        set_time_remaining.set(session_type.get().duration_minutes() * 60);
    };

    // Helper functions
    let format_time = move || {
        let total_seconds = time_remaining.get();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{:02}:{:02}", minutes, seconds)
    };

    let progress_percentage = move || {
        let total_duration = session_type.get().duration_minutes() * 60;
        let elapsed = total_duration - time_remaining.get();
        (elapsed as f64 / total_duration as f64) * 100.0
    };

    // Demo greeting functionality
    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    view! {
        <main class="min-h-screen bg-gray-100 dark:bg-gray-900 flex flex-col items-center justify-center p-4">
            <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8 w-full max-w-md">
                // Session Type Header
                <div class="text-center mb-6">
                    <h1 class="text-3xl font-bold text-gray-800 dark:text-white mb-2">
                        "Pomodoro Timer"
                    </h1>
                    <div class={move || format!("inline-block px-4 py-2 rounded-full text-white font-semibold {}", session_type.get().color_class())}>
                        {move || session_type.get().name()}
                    </div>
                </div>

                // Timer Display
                <div class="text-center mb-8">
                    <div class="text-6xl font-mono font-bold text-gray-800 dark:text-white mb-4">
                        {format_time}
                    </div>
                    
                    // Progress Bar
                    <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                        <div 
                            class={move || format!("h-2 rounded-full transition-all duration-1000 {}", session_type.get().color_class())}
                            style:width=move || format!("{}%", progress_percentage())
                        ></div>
                    </div>
                </div>

                // Timer Controls
                <div class="flex justify-center space-x-4 mb-6">
                    {move || match timer_state.get() {
                        TimerState::Stopped => view! {
                            <button
                                on:click=move |_| start_timer()
                                class="bg-green-500 hover:bg-green-600 text-white font-bold py-3 px-6 rounded-lg transition-colors"
                            >
                                "Start"
                            </button>
                        }.into_any(),
                        TimerState::Running => view! {
                            <button
                                on:click=move |_| pause_timer()
                                class="bg-yellow-500 hover:bg-yellow-600 text-white font-bold py-3 px-6 rounded-lg transition-colors"
                            >
                                "Pause"
                            </button>
                            <button
                                on:click=move |_| stop_timer()
                                class="bg-red-500 hover:bg-red-600 text-white font-bold py-3 px-6 rounded-lg transition-colors"
                            >
                                "Stop"
                            </button>
                        }.into_any(),
                        TimerState::Paused => view! {
                            <button
                                on:click=move |_| start_timer()
                                class="bg-green-500 hover:bg-green-600 text-white font-bold py-3 px-6 rounded-lg transition-colors"
                            >
                                "Resume"
                            </button>
                            <button
                                on:click=move |_| stop_timer()
                                class="bg-red-500 hover:bg-red-600 text-white font-bold py-3 px-6 rounded-lg transition-colors"
                            >
                                "Stop"
                            </button>
                        }.into_any(),
                    }}
                </div>

                // Session Info
                <div class="text-center text-gray-600 dark:text-gray-400">
                    <p class="text-lg">
                        "Sessions completed: " <span class="font-bold text-gray-800 dark:text-white">{completed_sessions}</span>
                    </p>
                </div>

                // Session Type Selector
                <div class="mt-6 flex justify-center space-x-2">
                    <button
                        on:click=move |_| {
                            if timer_state.get() == TimerState::Stopped {
                                set_session_type.set(SessionType::Work);
                                set_time_remaining.set(SessionType::Work.duration_minutes() * 60);
                            }
                        }
                        class={move || format!(
                            "px-3 py-1 rounded text-sm font-medium transition-colors {}",
                            if session_type.get() == SessionType::Work {
                                "bg-red-500 text-white"
                            } else {
                                "bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600"
                            }
                        )}
                        disabled=move || timer_state.get() != TimerState::Stopped
                    >
                        "Work (25m)"
                    </button>
                    <button
                        on:click=move |_| {
                            if timer_state.get() == TimerState::Stopped {
                                set_session_type.set(SessionType::ShortBreak);
                                set_time_remaining.set(SessionType::ShortBreak.duration_minutes() * 60);
                            }
                        }
                        class={move || format!(
                            "px-3 py-1 rounded text-sm font-medium transition-colors {}",
                            if session_type.get() == SessionType::ShortBreak {
                                "bg-green-500 text-white"
                            } else {
                                "bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600"
                            }
                        )}
                        disabled=move || timer_state.get() != TimerState::Stopped
                    >
                        "Short Break (5m)"
                    </button>
                    <button
                        on:click=move |_| {
                            if timer_state.get() == TimerState::Stopped {
                                set_session_type.set(SessionType::LongBreak);
                                set_time_remaining.set(SessionType::LongBreak.duration_minutes() * 60);
                            }
                        }
                        class={move || format!(
                            "px-3 py-1 rounded text-sm font-medium transition-colors {}",
                            if session_type.get() == SessionType::LongBreak {
                                "bg-blue-500 text-white"
                            } else {
                                "bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600"
                            }
                        )}
                        disabled=move || timer_state.get() != TimerState::Stopped
                    >
                        "Long Break (15m)"
                    </button>
                </div>
            </div>

            // Demo Section (can be removed later)
            <div class="mt-8 bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 w-full max-w-md">
                <h2 class="text-xl font-bold text-gray-800 dark:text-white mb-4 text-center">
                    "Tauri Demo"
                </h2>
                <form class="flex space-x-2" on:submit=greet>
                    <input
                        id="greet-input"
                        placeholder="Enter a name..."
                        on:input=update_name
                        class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-800 dark:text-white"
                    />
                    <button 
                        type="submit"
                        class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded transition-colors"
                    >
                        "Greet"
                    </button>
                </form>
                <p class="mt-4 text-gray-600 dark:text-gray-400 text-center">
                    { move || greet_msg.get() }
                </p>
            </div>
        </main>
    }
}