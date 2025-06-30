use crate::components::{
    CameraController, CameraRecorder, CameraSettings, SessionHistory, SessionSelector,
    SessionStats, TimerControls, TimerDisplay,
};
use crate::console_log;
use crate::timer::TimerController;
use crate::types::{CameraState, TimerState};
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum AppTab {
    Timer,
    History,
    Statistics,
    Settings,
}

#[component]
pub fn App() -> impl IntoView {
    let controller = TimerController::new();
    let camera_controller = CameraController::new();
    let active_tab = RwSignal::new(AppTab::Timer);

    // Timer completion effect with camera integration
    Effect::new({
        let controller = controller.clone();
        let camera_controller = camera_controller.clone();
        move |_| {
            if controller.time_remaining.get() == 0
                && controller.timer_state.get() == TimerState::Running
            {
                controller.complete_session_with_camera(Some(&camera_controller));
            }
        }
    });

    // Monitor session type changes to manage camera recording
    Effect::new({
        let controller = controller.clone();
        let camera_controller = camera_controller.clone();
        move |_| {
            let session_type = controller.session_type.get();
            let timer_state = controller.timer_state.get();
            let camera_settings = camera_controller.camera_settings.get();

            // If timer is running and camera is enabled, start recording for appropriate sessions
            if timer_state == TimerState::Running && camera_settings.enabled {
                let should_record = if camera_settings.only_during_breaks {
                    matches!(
                        session_type,
                        crate::types::SessionType::ShortBreak
                            | crate::types::SessionType::LongBreak
                    )
                } else {
                    true
                };

                if should_record && camera_controller.camera_state.get() == CameraState::Stopped {
                    if let Err(e) = camera_controller.start_recording(session_type) {
                        console_log!("Failed to start camera recording: {}", e);
                    }
                } else if !should_record && camera_controller.is_recording.get() {
                    camera_controller.stop_recording();
                }
            }
        }
    });

    view! {
        <main class="min-h-screen bg-gray-100 dark:bg-gray-900 flex flex-col items-center justify-start p-4">
            <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg w-full max-w-4xl">

                // Tab Navigation
                <div class="flex border-b border-gray-200 dark:border-gray-700">
                    <button
                        class={move || format!(
                            "flex-1 py-3 px-4 text-center font-medium transition-colors {}",
                            if active_tab.get() == AppTab::Timer {
                                "border-b-2 border-blue-500 text-blue-600 dark:text-blue-400"
                            } else {
                                "text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200"
                            }
                        )}
                        on:click=move |_| active_tab.set(AppTab::Timer)
                    >
                        "Timer"
                    </button>
                    <button
                        class={move || format!(
                            "flex-1 py-3 px-4 text-center font-medium transition-colors {}",
                            if active_tab.get() == AppTab::History {
                                "border-b-2 border-blue-500 text-blue-600 dark:text-blue-400"
                            } else {
                                "text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200"
                            }
                        )}
                        on:click=move |_| active_tab.set(AppTab::History)
                    >
                        "History"
                    </button>
                    <button
                        class={move || format!(
                            "flex-1 py-3 px-4 text-center font-medium transition-colors {}",
                            if active_tab.get() == AppTab::Statistics {
                                "border-b-2 border-blue-500 text-blue-600 dark:text-blue-400"
                            } else {
                                "text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200"
                            }
                        )}
                        on:click=move |_| active_tab.set(AppTab::Statistics)
                    >
                        "Stats"
                    </button>
                    <button
                        class={move || format!(
                            "flex-1 py-3 px-4 text-center font-medium transition-colors {}",
                            if active_tab.get() == AppTab::Settings {
                                "border-b-2 border-blue-500 text-blue-600 dark:text-blue-400"
                            } else {
                                "text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200"
                            }
                        )}
                        on:click=move |_| active_tab.set(AppTab::Settings)
                    >
                        "Settings"
                    </button>
                </div>

                // Tab Content
                <div class="p-6">
                    {move || {
                        match active_tab.get() {
                            AppTab::Timer => {
                                let controller_timer = controller.clone();
                                let camera_controller_timer = camera_controller.clone();
                                view! {
                                    <div class="max-w-lg mx-auto">
                                        // Session Type Header
                                        <div class="text-center mb-6">
                                            <h1 class="text-3xl font-bold text-gray-800 dark:text-white mb-2">
                                                "Pomodoro Timer"
                                            </h1>
                                            <div class={
                                                let controller_header = controller_timer.clone();
                                                move || format!("inline-block px-4 py-2 rounded-full text-white font-semibold {}", controller_header.session_type.get().color_class())
                                            }>
                                                {
                                                    let controller_header = controller_timer.clone();
                                                    move || controller_header.session_type.get().name()
                                                }
                                            </div>
                                        </div>

                                        // Timer Display
                                        <TimerDisplay controller=controller_timer.clone() />

                                        // Camera Component with session type and timer state
                                        <CameraRecorder
                                            controller=camera_controller_timer.clone()
                                            current_session_type=controller_timer.session_type
                                            timer_state=controller_timer.timer_state
                                        />

                                        // Enhanced Timer Controls with Camera Integration
                                        <TimerControls
                                            timer_controller=controller_timer.clone()
                                            camera_controller=camera_controller_timer.clone()
                                        />

                                        // Session Info
                                        <div class="text-center text-gray-600 dark:text-gray-400 mb-6">
                                            <p class="text-lg">
                                                "Sessions completed: " <span class="font-bold text-gray-800 dark:text-white">{
                                                    let controller_sessions = controller_timer.clone();
                                                    move || controller_sessions.completed_sessions.get()
                                                }</span>
                                            </p>

                                            // Show recording status
                                            {
                                                let controller_status = controller_timer.clone();
                                                let camera_controller_status = camera_controller_timer.clone();
                                                move || {
                                                    if camera_controller_status.camera_settings.get().enabled {
                                                        let should_record = controller_status.should_record_current_session(&camera_controller_status);
                                                        if should_record && controller_status.timer_state.get() == TimerState::Running {
                                                            view! {
                                                                <p class="text-sm text-red-600 dark:text-red-400 mt-1">
                                                                    "🔴 Recording session"
                                                                </p>
                                                            }.into_any()
                                                        } else if camera_controller_status.camera_settings.get().only_during_breaks {
                                                            view! {
                                                                <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
                                                                    "📹 Recording enabled for breaks"
                                                                </p>
                                                            }.into_any()
                                                        } else {
                                                            view! { <div></div> }.into_any()
                                                        }
                                                    } else {
                                                        view! { <div></div> }.into_any()
                                                    }
                                                }
                                            }
                                        </div>

                                        // Session Type Selector
                                        <SessionSelector controller=controller_timer.clone() />
                                    </div>
                                }.into_any()
                            },

                            AppTab::History => view! {
                                <div>
                                    <SessionHistory controller=controller.clone() />
                                </div>
                            }.into_any(),

                            AppTab::Statistics => view! {
                                <div class="max-w-4xl mx-auto">
                                    <SessionStats controller=controller.clone() />
                                </div>
                            }.into_any(),

                            AppTab::Settings => view! {
                                <div class="max-w-2xl mx-auto">
                                    <h3 class="text-lg font-semibold text-gray-800 dark:text-white mb-4">"Settings"</h3>
                                    <CameraSettings controller=camera_controller.clone() />
                                </div>
                            }.into_any(),
                        }
                    }}
                </div>
            </div>
        </main>
    }
}
