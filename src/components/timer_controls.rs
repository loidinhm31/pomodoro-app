use crate::components::CameraController;
use crate::theme::ThemeController;
use crate::timer::TimerController;
use crate::types::TimerState;
use leptos::prelude::*;

#[component]
pub fn TimerControls(
    timer_controller: TimerController,
    camera_controller: CameraController,
    theme_controller: ThemeController,
) -> impl IntoView {
    let start_timer = {
        let timer_controller = timer_controller.clone();
        let camera_controller = camera_controller.clone();

        move |_| {
            // Start timer with camera
            timer_controller.start_timer_with_camera(Some(&camera_controller));
        }
    };

    // Pause function
    let pause_timer = {
        let timer_controller = timer_controller.clone();
        let camera_controller = camera_controller.clone();

        move |_| {
            timer_controller.pause_timer_with_camera(Some(&camera_controller));
        }
    };

    // Stop function
    let stop_timer = {
        let timer_controller = timer_controller.clone();
        let camera_controller = camera_controller.clone();

        move |_| {
            timer_controller.stop_timer_with_camera(Some(&camera_controller));
        }
    };

    // Get theme-aware button colors
    let get_button_color = {
        let theme_controller = theme_controller.clone();
        move |button_type: &str| {
            let theme = theme_controller.get_current_theme();
            match button_type {
                "start" | "resume" => theme.work_color().to_string(),
                "pause" => theme.short_break_color().to_string(),
                "stop" => theme.long_break_color().to_string(),
                _ => "#6B7280".to_string(),
            }
        }
    };

    view! {
        <div class="flex flex-col items-center space-y-4 mb-6">
            // Main Control Buttons
            <div class="flex justify-center space-x-4">
                {
                    let timer_controller_buttons = timer_controller.clone();
                    let get_button_color = get_button_color.clone();
                    move || {
                        match timer_controller_buttons.timer_state.get() {
                            TimerState::Stopped => {
                                let start_color = get_button_color("start");
                                view! {
                                    <button
                                        on:click=start_timer.clone()
                                        class="px-8 py-4 text-white font-bold rounded-xl transition-all duration-200 transform hover:scale-105 hover:shadow-lg flex items-center space-x-2"
                                        style=move || format!("background-color: {}", start_color.clone())
                                    >
                                        <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
                                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd"></path>
                                        </svg>
                                        <span class="text-lg">"Start"</span>
                                    </button>
                                }.into_any()
                            },
                            TimerState::Running => {
                                let pause_color = get_button_color("pause");
                                let stop_color = get_button_color("stop");
                                view! {
                                    <div class="flex space-x-4">
                                        <button
                                            on:click=pause_timer.clone()
                                            class="px-6 py-4 text-white font-bold rounded-xl transition-all duration-200 transform hover:scale-105 hover:shadow-lg flex items-center space-x-2"
                                            style=move || format!("background-color: {}", pause_color.clone())
                                        >
                                            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zM7 8a1 1 0 012 0v4a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v4a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd"></path>
                                            </svg>
                                            <span>"Pause"</span>
                                        </button>
                                        <button
                                            on:click=stop_timer.clone()
                                            class="px-6 py-4 text-white font-bold rounded-xl transition-all duration-200 transform hover:scale-105 hover:shadow-lg flex items-center space-x-2"
                                            style=move || format!("background-color: {}", stop_color.clone())
                                        >
                                            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8 7a1 1 0 00-1 1v4a1 1 0 001 1h4a1 1 0 001-1V8a1 1 0 00-1-1H8z" clip-rule="evenodd"></path>
                                            </svg>
                                            <span>"Stop"</span>
                                        </button>
                                    </div>
                                }.into_any()
                            },
                            TimerState::Paused => {
                                let resume_color = get_button_color("resume");
                                let stop_color = get_button_color("stop");
                                view! {
                                    <div class="flex space-x-4">
                                        <button
                                            on:click=start_timer.clone()
                                            class="px-6 py-4 text-white font-bold rounded-xl transition-all duration-200 transform hover:scale-105 hover:shadow-lg flex items-center space-x-2"
                                            style=move || format!("background-color: {}", resume_color.clone())
                                        >
                                            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd"></path>
                                            </svg>
                                            <span>"Resume"</span>
                                        </button>
                                        <button
                                            on:click=stop_timer.clone()
                                            class="px-6 py-4 text-white font-bold rounded-xl transition-all duration-200 transform hover:scale-105 hover:shadow-lg flex items-center space-x-2"
                                            style=move || format!("background-color: {}", stop_color.clone())
                                        >
                                            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8 7a1 1 0 00-1 1v4a1 1 0 001 1h4a1 1 0 001-1V8a1 1 0 00-1-1H8z" clip-rule="evenodd"></path>
                                            </svg>
                                            <span>"Stop"</span>
                                        </button>
                                    </div>
                                }.into_any()
                            },
                        }
                    }
                }
            </div>

            // Quick Control Buttons
            <div class="flex justify-center space-x-3">
                // Quick Session Type Switch (when stopped)
                {
                    let timer_controller_switch = timer_controller.clone();
                    move || {
                        if timer_controller_switch.timer_state.get() == TimerState::Stopped {
                            view! {
                                <button
                                    class="px-3 py-2 rounded-lg text-sm font-medium transition-all duration-200 border-2 text-gray-600 dark:text-gray-400 border-gray-300 dark:border-gray-600 bg-transparent hover:bg-gray-50 dark:hover:bg-gray-700"
                                    on:click={
                                        let timer_controller_switch = timer_controller_switch.clone();
                                        move |_| {
                                            let current = timer_controller_switch.session_type.get();
                                            let next = match current {
                                                crate::types::SessionType::Work => crate::types::SessionType::ShortBreak,
                                                crate::types::SessionType::ShortBreak => crate::types::SessionType::LongBreak,
                                                crate::types::SessionType::LongBreak => crate::types::SessionType::Work,
                                            };
                                            timer_controller_switch.set_session_type(next);
                                        }
                                    }
                                    title="Quick switch session type"
                                >
                                    <div class="flex items-center space-x-1">
                                        <span>"🔄"</span>
                                        <span class="hidden sm:inline">"Switch"</span>
                                    </div>
                                </button>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }
                }

                // Reset button (when stopped)
                {
                    let timer_controller_reset = timer_controller.clone();
                    move || {
                        if timer_controller_reset.timer_state.get() == TimerState::Stopped {
                            view! {
                                <button
                                    class="px-3 py-2 rounded-lg text-sm font-medium transition-all duration-200 border-2 text-orange-600 dark:text-orange-400 border-orange-300 dark:border-orange-600 bg-transparent hover:bg-orange-50 dark:hover:bg-orange-900/20"
                                    on:click={
                                        let timer_controller_reset = timer_controller_reset.clone();
                                        move |_| {
                                            timer_controller_reset.reset_work_sessions();
                                        }
                                    }
                                    title="Reset work session cycle"
                                >
                                    <div class="flex items-center space-x-1">
                                        <span>"↻"</span>
                                        <span class="hidden sm:inline">"Reset"</span>
                                    </div>
                                </button>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }
                }
            </div>

            // Status Information
            <div class="text-center">
                {
                    let timer_controller_status = timer_controller.clone();
                    move || {
                        let state = timer_controller_status.timer_state.get();
                        let session_type = timer_controller_status.session_type.get();
                        
                        match state {
                            TimerState::Running => {
                                view! {
                                    <div class="flex items-center justify-center space-x-2 text-sm">
                                        <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                                        <span class="text-gray-600 dark:text-gray-400">
                                            "Focus time! " {session_type.name()} " session in progress"
                                        </span>
                                    </div>
                                }.into_any()
                            },
                            TimerState::Paused => {
                                view! {
                                    <div class="flex items-center justify-center space-x-2 text-sm">
                                        <div class="w-2 h-2 bg-yellow-500 rounded-full"></div>
                                        <span class="text-gray-600 dark:text-gray-400">
                                            "Paused - " {session_type.name()} " session"
                                        </span>
                                    </div>
                                }.into_any()
                            },
                            TimerState::Stopped => {
                                view! {
                                    <div class="flex items-center justify-center space-x-2 text-sm">
                                        <div class="w-2 h-2 bg-gray-500 rounded-full"></div>
                                        <span class="text-gray-600 dark:text-gray-400">
                                            "Ready to start " {session_type.name()} " session"
                                        </span>
                                    </div>
                                }.into_any()
                            },
                        }
                    }
                }
            </div>

            // Keyboard shortcuts hint
            <div class="text-xs text-gray-500 dark:text-gray-400 text-center">
                <p>"💡 Tip: Use keyboard shortcuts for quick control"</p>
                <p class="mt-1">"Space: Start/Pause • Esc: Stop • Tab: Switch session"</p>
            </div>
        </div>
    }
}