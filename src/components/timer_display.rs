use crate::timer::TimerController;
use crate::theme::ThemeController;
use crate::utils::{calculate_progress_percentage, format_time};
use leptos::prelude::*;

#[component]
pub fn TimerDisplay(
    controller: TimerController,
    theme_controller: ThemeController,
) -> impl IntoView {
    let format_time_display = {
        let controller = controller.clone();
        move || format_time(controller.time_remaining.get())
    };

    let progress_percentage = {
        let controller = controller.clone();
        move || {
            let settings = controller.timer_settings.get();
            let total_duration = controller.session_type.get().duration_minutes(&settings) * 60;
            calculate_progress_percentage(controller.time_remaining.get(), total_duration)
        }
    };

    // Get theme-aware colors
    let get_session_color = {
        let theme_controller = theme_controller.clone();
        let controller = controller.clone();
        move || {
            let theme = theme_controller.get_current_theme();
            let session_type = controller.session_type.get();
            match session_type {
                crate::types::SessionType::Work => theme.work_color().to_string(),
                crate::types::SessionType::ShortBreak => theme.short_break_color().to_string(),
                crate::types::SessionType::LongBreak => theme.long_break_color().to_string(),
            }
        }
    };

    view! {
        <div class="text-center mb-8">
            // Enhanced Timer Display with Pulse Animation
            <div class={
                let controller = controller.clone();
                move || format!(
                    "text-6xl font-mono font-bold text-gray-800 dark:text-white mb-4 {}",
                    if controller.timer_state.get() == crate::types::TimerState::Running {
                        "timer-pulse"
                    } else {
                        ""
                    }
                )
            }>
                {format_time_display}
            </div>

            // Enhanced Progress Bar with Theme Colors
            <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-4 mb-3 overflow-hidden shadow-inner">
                <div
                    class="h-4 rounded-full transition-all duration-1000 relative overflow-hidden"
                    style:width=move || format!("{}%", progress_percentage())
                    style:background-color={
                        let get_session_color = get_session_color.clone();
                        move || get_session_color()
                    }
                >
                    // Animated shine effect
                    <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white to-transparent opacity-20 transform -skew-x-12 animate-pulse"></div>
                </div>
            </div>

            // Enhanced Progress Info with Time Breakdown
            <div class="text-sm text-gray-600 dark:text-gray-400 space-y-1">
                <div>
                    {
                        let controller = controller.clone();
                        move || {
                            let settings = controller.timer_settings.get();
                            let total_duration = controller.session_type.get().duration_minutes(&settings) * 60;
                            let elapsed = total_duration - controller.time_remaining.get();
                            let elapsed_minutes = elapsed / 60;
                            let elapsed_seconds = elapsed % 60;
                            let total_minutes = total_duration / 60;
                            let remaining_minutes = controller.time_remaining.get() / 60;
                            let remaining_seconds = controller.time_remaining.get() % 60;
                            
                            format!("⏱️ Elapsed: {}:{:02} • Remaining: {}:{:02} • Total: {}m", 
                                elapsed_minutes, elapsed_seconds,
                                remaining_minutes, remaining_seconds,
                                total_minutes
                            )
                        }
                    }
                </div>
                
                <div class="flex justify-center items-center space-x-2">
                    <div class="w-2 h-2 rounded-full"
                         style:background-color={
                             let get_session_color = get_session_color.clone();
                             move || get_session_color()
                         }></div>
                    <span>{move || format!("{:.1}% complete", progress_percentage())}</span>
                </div>

                // Session State Indicator
                <div class="mt-2">
                    <span class={
                        let controller = controller.clone();
                        move || format!(
                            "inline-flex items-center px-3 py-1 rounded-full text-xs font-medium {}",
                            match controller.timer_state.get() {
                                crate::types::TimerState::Running => "bg-green-100 text-green-800 dark:bg-green-800 dark:text-green-200",
                                crate::types::TimerState::Paused => "bg-yellow-100 text-yellow-800 dark:bg-yellow-800 dark:text-yellow-200",
                                crate::types::TimerState::Stopped => "bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300",
                            }
                        )
                    }>
                        <div class={
                            let controller = controller.clone();
                            move || format!(
                                "w-2 h-2 rounded-full mr-2 {}",
                                match controller.timer_state.get() {
                                    crate::types::TimerState::Running => "bg-green-500 animate-pulse",
                                    crate::types::TimerState::Paused => "bg-yellow-500",
                                    crate::types::TimerState::Stopped => "bg-gray-500",
                                }
                            )
                        }></div>
                        {
                            let controller = controller.clone();
                            move || match controller.timer_state.get() {
                                crate::types::TimerState::Running => "Running",
                                crate::types::TimerState::Paused => "Paused",
                                crate::types::TimerState::Stopped => "Stopped",
                            }
                        }
                    </span>
                </div>
            </div>

            // Enhanced Visual Focus Indicator
            {
                let controller = controller.clone();
                let get_session_color = get_session_color.clone();
                move || {
                    if controller.timer_state.get() == crate::types::TimerState::Running {
                        let color = get_session_color();
                        view! {
                            <div class="mt-4 flex justify-center">
                                <div class="flex space-x-1">
                                    <div class="w-2 h-8 rounded-full sound-wave"
                                         style:background-color=color.clone()></div>
                                    <div class="w-2 h-6 rounded-full sound-wave"
                                         style:background-color=color.clone()></div>
                                    <div class="w-2 h-8 rounded-full sound-wave"
                                         style:background-color=color.clone()></div>
                                    <div class="w-2 h-4 rounded-full sound-wave"
                                         style:background-color=color.clone()></div>
                                    <div class="w-2 h-8 rounded-full sound-wave"
                                         style:background-color=color.clone()></div>
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