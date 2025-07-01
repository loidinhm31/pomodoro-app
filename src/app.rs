use crate::components::{
    CameraController, CameraRecorder, CameraSettings, SessionHistory, SessionSelector,
    SessionStats, TimerControls, TimerDisplay, TimerSettings, TaskSelector, TaskManager, TaskStats,
};
use crate::console_log;
use crate::timer::TimerController;
use crate::task::TaskController;
use crate::types::{CameraState, TimerState};
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum AppTab {
    Timer,
    Tasks,
    History,
    Statistics,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SettingsTab {
    Timer,
    Camera,
}

#[component]
pub fn App() -> impl IntoView {
    let timer_controller = TimerController::new();
    let camera_controller = CameraController::new();
    let task_controller = TaskController::new();
    let active_tab = RwSignal::new(AppTab::Timer);
    let active_settings_tab = RwSignal::new(SettingsTab::Timer);

    // Timer completion effect with camera and task integration
    Effect::new({
        let timer_controller = timer_controller.clone();
        let camera_controller = camera_controller.clone();
        let task_controller = task_controller.clone();
        move |_| {
            if timer_controller.time_remaining.get() == 0
                && timer_controller.timer_state.get() == TimerState::Running
            {
                timer_controller.complete_session_with_camera_and_tasks(
                    Some(&camera_controller),
                    Some(&task_controller),
                );
            }
        }
    });

    // Monitor session type changes to manage camera recording
    Effect::new({
        let timer_controller = timer_controller.clone();
        let camera_controller = camera_controller.clone();
        move |_| {
            let session_type = timer_controller.session_type.get();
            let timer_state = timer_controller.timer_state.get();
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
            <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg w-full max-w-6xl">

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
                            if active_tab.get() == AppTab::Tasks {
                                "border-b-2 border-blue-500 text-blue-600 dark:text-blue-400"
                            } else {
                                "text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200"
                            }
                        )}
                        on:click=move |_| active_tab.set(AppTab::Tasks)
                    >
                        "Tasks"
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
                    {
                        let timer_controller_clone = timer_controller.clone();
                        let camera_controller_clone = camera_controller.clone();
                        let task_controller_clone = task_controller.clone();
                        move || {
                            match active_tab.get() {
                                AppTab::Timer => {
                                    let timer_controller_timer = timer_controller_clone.clone();
                                    let camera_controller_timer = camera_controller_clone.clone();
                                    let task_controller_timer = task_controller_clone.clone();
                                view! {
                                    <div class="max-w-lg mx-auto">
                                        // Session Type Header
                                        <div class="text-center mb-6">
                                            <h1 class="text-3xl font-bold text-gray-800 dark:text-white mb-2">
                                                "Pomodoro Timer"
                                            </h1>
                                            <div class={
                                                let timer_controller_header = timer_controller_timer.clone();
                                                move || format!("inline-block px-4 py-2 rounded-full text-white font-semibold {}", timer_controller_header.session_type.get().color_class())
                                            }>
                                                {
                                                    let timer_controller_header = timer_controller_timer.clone();
                                                    move || {
                                                        let settings = timer_controller_header.timer_settings.get();
                                                        timer_controller_header.session_type.get().display_with_duration(&settings)
                                                    }
                                                }
                                            </div>
                                        </div>

                                        // Task Selection
                                        <TaskSelector task_controller=task_controller_timer.clone() />

                                        // Timer Display
                                        <TimerDisplay controller=timer_controller_timer.clone() />

                                        // Camera Component with session type and timer state
                                        <CameraRecorder
                                            controller=camera_controller_timer.clone()
                                            current_session_type=timer_controller_timer.session_type
                                            timer_state=timer_controller_timer.timer_state
                                        />

                                        // Enhanced Timer Controls with Camera Integration
                                        <TimerControls
                                            timer_controller=timer_controller_timer.clone()
                                            camera_controller=camera_controller_timer.clone()
                                        />

                                        // Session Info with Task Information
                                        <div class="text-center text-gray-600 dark:text-gray-400 mb-6">
                                            <p class="text-lg">
                                                "Work sessions completed: " <span class="font-bold text-gray-800 dark:text-white">{
                                                    let timer_controller_sessions = timer_controller_timer.clone();
                                                    move || timer_controller_sessions.current_cycle_work_sessions.get()
                                                }</span>
                                                " (cycle) / " <span class="font-bold text-gray-800 dark:text-white">{
                                                    let timer_controller_total = timer_controller_timer.clone();
                                                    move || timer_controller_total.completed_work_sessions.get()
                                                }</span>
                                                " (total)"
                                            </p>

                                            // Show active task info
                                            {
                                                let task_controller_info = task_controller_timer.clone();
                                                move || {
                                                    if let Some(task_info) = task_controller_info.get_active_task_info() {
                                                        view! {
                                                            <p class="text-sm text-blue-600 dark:text-blue-400 mt-1">
                                                                "🎯 " {task_info}
                                                            </p>
                                                        }.into_any()
                                                    } else {
                                                        view! {
                                                            <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
                                                                "No task selected - time won't be tracked"
                                                            </p>
                                                        }.into_any()
                                                    }
                                                }
                                            }

                                            // Show recording status
                                            {
                                                let timer_controller_status = timer_controller_timer.clone();
                                                let camera_controller_status = camera_controller_timer.clone();
                                                move || {
                                                    if camera_controller_status.camera_settings.get().enabled {
                                                        let should_record = timer_controller_status.should_record_current_session(&camera_controller_status);
                                                        if should_record && timer_controller_status.timer_state.get() == TimerState::Running {
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
                                        <SessionSelector controller=timer_controller_timer.clone() />
                                    </div>
                                }.into_any()
                            },

                            AppTab::Tasks => view! {
                                <div class="max-w-6xl mx-auto">
                                    <TaskManager task_controller=task_controller_clone.clone() />
                                </div>
                            }.into_any(),

                            AppTab::History => view! {
                                <div>
                                    <SessionHistory controller=timer_controller_clone.clone() />
                                </div>
                            }.into_any(),

                            AppTab::Statistics => view! {
                                <div class="max-w-6xl mx-auto">
                                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                                        // Session Statistics
                                        <div>
                                            <SessionStats controller=timer_controller_clone.clone() />
                                        </div>
                                        
                                        // Task Statistics
                                        <div>
                                            <TaskStats task_controller=task_controller_clone.clone() />
                                        </div>
                                    </div>
                                </div>
                            }.into_any(),

                            AppTab::Settings => view! {
                                <div class="max-w-4xl mx-auto">
                                    <h3 class="text-lg font-semibold text-gray-800 dark:text-white mb-6">"Settings"</h3>
                                    
                                    // Settings sub-tabs
                                    <div class="flex border-b border-gray-200 dark:border-gray-700 mb-6">
                                        <button
                                            class={move || format!(
                                                "py-2 px-4 font-medium transition-colors {}",
                                                if active_settings_tab.get() == SettingsTab::Timer {
                                                    "border-b-2 border-blue-500 text-blue-600 dark:text-blue-400"
                                                } else {
                                                    "text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200"
                                                }
                                            )}
                                            on:click=move |_| active_settings_tab.set(SettingsTab::Timer)
                                        >
                                            "Timer Settings"
                                        </button>
                                        <button
                                            class={move || format!(
                                                "py-2 px-4 font-medium transition-colors {}",
                                                if active_settings_tab.get() == SettingsTab::Camera {
                                                    "border-b-2 border-blue-500 text-blue-600 dark:text-blue-400"
                                                } else {
                                                    "text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200"
                                                }
                                            )}
                                            on:click=move |_| active_settings_tab.set(SettingsTab::Camera)
                                        >
                                            "Camera Settings"
                                        </button>
                                    </div>

                                    // Settings content
                                    {
                                        let timer_controller_settings = timer_controller_clone.clone();
                                        let camera_controller_settings = camera_controller_clone.clone();
                                        move || {
                                            match active_settings_tab.get() {
                                                SettingsTab::Timer => view! {
                                                    <TimerSettings controller=timer_controller_settings.clone() />
                                                }.into_any(),
                                                SettingsTab::Camera => view! {
                                                    <CameraSettings controller=camera_controller_settings.clone() />
                                                }.into_any(),
                                            }
                                        }
                                    }
                                </div>
                            }.into_any(),
                        }
                    }
                    }
                </div>
            </div>
        </main>
    }
}