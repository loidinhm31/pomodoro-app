use crate::timer::TimerController;
use crate::types::{SessionType, TimerState};
use leptos::prelude::*;

#[component]
pub fn SessionSelector(controller: TimerController) -> impl IntoView {
    view! {
        <div class="mt-6 flex flex-col space-y-4">
            // Session type buttons
            <div class="flex justify-center space-x-2 flex-wrap gap-2">
                <button
                    on:click={
                        let controller = controller.clone();
                        move |_| controller.set_session_type(SessionType::Work)
                    }
                    class={
                        let controller = controller.clone();
                        move || format!(
                            "px-3 py-2 rounded text-sm font-medium transition-colors {}",
                            if controller.session_type.get() == SessionType::Work {
                                "bg-red-500 text-white"
                            } else {
                                "bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600"
                            }
                        )
                    }
                    disabled={
                        let controller = controller.clone();
                        move || controller.timer_state.get() != TimerState::Stopped
                    }
                >
                    {
                        let controller = controller.clone();
                        move || {
                            let settings = controller.timer_settings.get();
                            SessionType::Work.display_with_duration(&settings)
                        }
                    }
                </button>
                <button
                    on:click={
                        let controller = controller.clone();
                        move |_| controller.set_session_type(SessionType::ShortBreak)
                    }
                    class={
                        let controller = controller.clone();
                        move || format!(
                            "px-3 py-2 rounded text-sm font-medium transition-colors {}",
                            if controller.session_type.get() == SessionType::ShortBreak {
                                "bg-green-500 text-white"
                            } else {
                                "bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600"
                            }
                        )
                    }
                    disabled={
                        let controller = controller.clone();
                        move || controller.timer_state.get() != TimerState::Stopped
                    }
                >
                    {
                        let controller = controller.clone();
                        move || {
                            let settings = controller.timer_settings.get();
                            SessionType::ShortBreak.display_with_duration(&settings)
                        }
                    }
                </button>
                <button
                    on:click={
                        let controller = controller.clone();
                        move |_| controller.set_session_type(SessionType::LongBreak)
                    }
                    class={
                        let controller = controller.clone();
                        move || format!(
                            "px-3 py-2 rounded text-sm font-medium transition-colors {}",
                            if controller.session_type.get() == SessionType::LongBreak {
                                "bg-blue-500 text-white"
                            } else {
                                "bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600"
                            }
                        )
                    }
                    disabled={
                        let controller = controller.clone();
                        move || controller.timer_state.get() != TimerState::Stopped
                    }
                >
                    {
                        let controller = controller.clone();
                        move || {
                            let settings = controller.timer_settings.get();
                            SessionType::LongBreak.display_with_duration(&settings)
                        }
                    }
                </button>
            </div>

            // Next session info
            {
                let controller_clone = controller.clone();
                move || {
                    if controller_clone.timer_state.get() == TimerState::Stopped {
                        let (next_session, description) = controller_clone.get_next_session_info();
                        let settings = controller_clone.timer_settings.get();
                        
                        view! {
                            <div class="text-center text-sm text-gray-600 dark:text-gray-400">
                                <p>"Next up: " 
                                    <span class="font-medium text-gray-800 dark:text-white">
                                        {next_session.display_with_duration(&settings)}
                                    </span>
                                </p>
                                <p class="text-xs mt-1">{description}</p>
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }
            }

            // Current cycle progress
            {
                let controller_clone2 = controller.clone();
                move || {
                    let work_sessions = controller_clone2.current_cycle_work_sessions.get();
                    let settings = controller_clone2.timer_settings.get();
                let sessions_to_short = settings.sessions_before_short_break;
                let sessions_to_long = settings.sessions_before_long_break;

                    if work_sessions > 0 {
                        let progress_to_short = work_sessions % sessions_to_short;
                        let progress_to_long = work_sessions % sessions_to_long;
    
                        view! {
                            <div class="text-center text-xs text-gray-500 dark:text-gray-400 space-y-1">
                                <div class="flex justify-center space-x-4">
                                    <div>
                                        "Short break: " {progress_to_short} "/" {sessions_to_short}
                                    </div>
                                    <div>
                                        "Long break: " {progress_to_long} "/" {sessions_to_long}
                                    </div>
                                </div>
    
                                // Visual progress bars
                                <div class="flex justify-center space-x-4 mt-2">
                                    // Short break progress
                                    <div class="flex items-center space-x-1">
                                        <span class="text-xs">"Short:"</span>
                                        <div class="w-16 h-2 bg-gray-200 dark:bg-gray-700 rounded">
                                            <div
                                                class="h-full bg-green-500 rounded transition-all duration-300"
                                                style:width=move || format!("{}%", (progress_to_short as f32 / sessions_to_short as f32) * 100.0)
                                            ></div>
                                        </div>
                                    </div>
    
                                    // Long break progress  
                                    <div class="flex items-center space-x-1">
                                        <span class="text-xs">"Long:"</span>
                                        <div class="w-16 h-2 bg-gray-200 dark:bg-gray-700 rounded">
                                            <div
                                                class="h-full bg-blue-500 rounded transition-all duration-300"
                                                style:width=move || format!("{}%", (progress_to_long as f32 / sessions_to_long as f32) * 100.0)
                                            ></div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }
            }
        </div>
    }
}