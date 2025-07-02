use crate::timer::TimerController;
use crate::theme::ThemeController;
use crate::types::{SessionType, TimerState};
use leptos::prelude::*;

#[component]
pub fn SessionSelector(
    controller: TimerController,
    theme_controller: ThemeController,
) -> impl IntoView {
    // Get theme-aware colors for buttons
    let get_button_style = {
        let theme_controller = theme_controller.clone();
        move |session_type: SessionType, is_selected: bool| {
            let theme = theme_controller.get_current_theme();
            let color = match session_type {
                SessionType::Work => theme.work_color(),
                SessionType::ShortBreak => theme.short_break_color(),
                SessionType::LongBreak => theme.long_break_color(),
            };

            if is_selected {
                format!("color: white; background-color: {}; border-color: {};", color, color)
            } else {
                format!("color: {}; border-color: {}; background-color: transparent;", color, color)
            }
        }
    };

    view! {
        <div class="mt-6 flex flex-col space-y-4">
            // Session type buttons
            <div class="flex justify-center space-x-3 flex-wrap gap-3">
                <button
                    on:click={
                        let controller = controller.clone();
                        move |_| controller.set_session_type(SessionType::Work)
                    }
                    class="px-4 py-3 rounded-lg text-sm font-medium transition-all duration-200 border-2 hover:shadow-md transform hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none"
                    style={
                        let controller_style = controller.clone();
                        let get_button_style = get_button_style.clone();
                        move || get_button_style(SessionType::Work, controller_style.session_type.get() == SessionType::Work)
                    }
                    disabled={
                        let controller = controller.clone();
                        move || controller.timer_state.get() != TimerState::Stopped
                    }
                >
                    <div class="flex items-center space-x-2">
                        <div class="w-3 h-3 rounded-full bg-current opacity-75"></div>
                        <span>
                            {
                                let controller = controller.clone();
                                move || {
                                    let settings = controller.timer_settings.get();
                                    SessionType::Work.display_with_duration(&settings)
                                }
                            }
                        </span>
                    </div>
                </button>

                <button
                    on:click={
                        let controller = controller.clone();
                        move |_| controller.set_session_type(SessionType::ShortBreak)
                    }
                    class="px-4 py-3 rounded-lg text-sm font-medium transition-all duration-200 border-2 hover:shadow-md transform hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none"
                    style={
                        let controller_style = controller.clone();
                        let get_button_style = get_button_style.clone();
                        move || get_button_style(SessionType::ShortBreak, controller_style.session_type.get() == SessionType::ShortBreak)
                    }
                    disabled={
                        let controller = controller.clone();
                        move || controller.timer_state.get() != TimerState::Stopped
                    }
                >
                    <div class="flex items-center space-x-2">
                        <div class="w-3 h-3 rounded-full bg-current opacity-75"></div>
                        <span>
                            {
                                let controller = controller.clone();
                                move || {
                                    let settings = controller.timer_settings.get();
                                    SessionType::ShortBreak.display_with_duration(&settings)
                                }
                            }
                        </span>
                    </div>
                </button>

                <button
                    on:click={
                        let controller = controller.clone();
                        move |_| controller.set_session_type(SessionType::LongBreak)
                    }
                    class="px-4 py-3 rounded-lg text-sm font-medium transition-all duration-200 border-2 hover:shadow-md transform hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none"
                    style={
                        let controller_style = controller.clone();
                        let get_button_style = get_button_style.clone();
                        move || get_button_style(SessionType::LongBreak, controller_style.session_type.get() == SessionType::LongBreak)
                    }
                    disabled={
                        let controller = controller.clone();
                        move || controller.timer_state.get() != TimerState::Stopped
                    }
                >
                    <div class="flex items-center space-x-2">
                        <div class="w-3 h-3 rounded-full bg-current opacity-75"></div>
                        <span>
                            {
                                let controller = controller.clone();
                                move || {
                                    let settings = controller.timer_settings.get();
                                    SessionType::LongBreak.display_with_duration(&settings)
                                }
                            }
                        </span>
                    </div>
                </button>
            </div>

            // Next session info
            {
                let controller_clone = controller.clone();
                let theme_controller_clone = theme_controller.clone();
                move || {
                    if controller_clone.timer_state.get() == TimerState::Stopped {
                        let (next_session, description) = controller_clone.get_next_session_info();
                        let settings = controller_clone.timer_settings.get();
                        
                        view! {
                            <div class="text-center bg-gray-50 dark:bg-gray-700 rounded-lg p-4 border border-gray-200 dark:border-gray-600">
                                <div class="flex items-center justify-center space-x-2 mb-2">
                                    <span class="text-sm text-gray-600 dark:text-gray-400">"Next session:"</span>
                                    <div class="flex items-center space-x-2">
                                        <div class="w-3 h-3 rounded-full"
                                             style={
                                                 let theme_controller_style = theme_controller_clone.clone();
                                                 move || {
                                                     let theme = theme_controller_style.get_current_theme();
                                                     let color = match next_session {
                                                         SessionType::Work => theme.work_color(),
                                                         SessionType::ShortBreak => theme.short_break_color(),
                                                         SessionType::LongBreak => theme.long_break_color(),
                                                     };
                                                     format!("background-color: {}", color)
                                                 }
                                             }></div>
                                        <span class="font-medium text-gray-800 dark:text-white">
                                            {next_session.display_with_duration(&settings)}
                                        </span>
                                    </div>
                                </div>
                                <p class="text-xs text-gray-500 dark:text-gray-400">{description}</p>
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
                let theme_controller_clone2 = theme_controller.clone();
                move || {
                    let work_sessions = controller_clone2.current_cycle_work_sessions.get();
                    let settings = controller_clone2.timer_settings.get();
                    let sessions_to_short = settings.sessions_before_short_break;
                    let sessions_to_long = settings.sessions_before_long_break;

                    if work_sessions > 0 {
                        let progress_to_short = work_sessions % sessions_to_short;
                        let progress_to_long = work_sessions % sessions_to_long;
    
                        view! {
                            <div class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-600">
                                <h4 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-3 text-center">
                                    "Cycle Progress"
                                </h4>
                                
                                <div class="space-y-4">
                                    // Short break progress
                                    <div>
                                        <div class="flex justify-between items-center mb-2">
                                            <span class="text-xs text-gray-600 dark:text-gray-400">
                                                "Next short break"
                                            </span>
                                            <span class="text-xs font-medium text-gray-800 dark:text-white">
                                                {progress_to_short} "/" {sessions_to_short}
                                            </span>
                                        </div>
                                        <div class="w-full bg-gray-200 dark:bg-gray-600 rounded-full h-2">
                                            <div
                                                class="h-2 rounded-full transition-all duration-500"
                                                style={
                                                    let theme_controller_short = theme_controller_clone2.clone();
                                                    move || {
                                                        let width = (progress_to_short as f32 / sessions_to_short as f32) * 100.0;
                                                        let color = theme_controller_short.get_current_theme().short_break_color();
                                                        format!("width: {}%; background-color: {}", width, color)
                                                    }
                                                }
                                            ></div>
                                        </div>
                                    </div>

                                    // Long break progress  
                                    <div>
                                        <div class="flex justify-between items-center mb-2">
                                            <span class="text-xs text-gray-600 dark:text-gray-400">
                                                "Next long break"
                                            </span>
                                            <span class="text-xs font-medium text-gray-800 dark:text-white">
                                                {progress_to_long} "/" {sessions_to_long}
                                            </span>
                                        </div>
                                        <div class="w-full bg-gray-200 dark:bg-gray-600 rounded-full h-2">
                                            <div
                                                class="h-2 rounded-full transition-all duration-500"
                                                style={
                                                    let theme_controller_long = theme_controller_clone2.clone();
                                                    move || {
                                                        let width = (progress_to_long as f32 / sessions_to_long as f32) * 100.0;
                                                        let color = theme_controller_long.get_current_theme().long_break_color();
                                                        format!("width: {}%; background-color: {}", width, color)
                                                    }
                                                }
                                            ></div>
                                        </div>
                                    </div>
                                </div>

                                // Cycle completion celebration
                                {move || {
                                    if progress_to_long == 0 && work_sessions > 0 {
                                        view! {
                                            <div class="mt-3 text-center">
                                                <span class="text-xs bg-blue-100 dark:bg-blue-800 text-blue-800 dark:text-blue-200 px-2 py-1 rounded-full">
                                                    "🎉 Cycle Complete! Well done!"
                                                </span>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <div></div> }.into_any()
                                    }
                                }}
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