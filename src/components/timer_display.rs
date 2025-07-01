use crate::timer::TimerController;
use crate::utils::{calculate_progress_percentage, format_time};
use leptos::prelude::*;

#[component]
pub fn TimerDisplay(controller: TimerController) -> impl IntoView {
    let format_time_display = move || format_time(controller.time_remaining.get());

    let progress_percentage = move || {
        let settings = controller.timer_settings.get();
        let total_duration = controller.session_type.get().duration_minutes(&settings) * 60;
        calculate_progress_percentage(controller.time_remaining.get(), total_duration)
    };

    view! {
        <div class="text-center mb-8">
            <div class="text-6xl font-mono font-bold text-gray-800 dark:text-white mb-4">
                {format_time_display}
            </div>

            // Progress Bar
            <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-3 mb-2">
                <div
                    class={move || format!("h-3 rounded-full transition-all duration-1000 {}", controller.session_type.get().color_class())}
                    style:width=move || format!("{}%", progress_percentage())
                ></div>
            </div>

            // Progress info
            <div class="text-sm text-gray-600 dark:text-gray-400">
                {move || {
                    let settings = controller.timer_settings.get();
                    let total_duration = controller.session_type.get().duration_minutes(&settings) * 60;
                    let elapsed = total_duration - controller.time_remaining.get();
                    let elapsed_minutes = elapsed / 60;
                    let total_minutes = total_duration / 60;
                    
                    format!("{} / {} minutes ({:.0}% complete)", 
                        elapsed_minutes, 
                        total_minutes, 
                        progress_percentage()
                    )
                }}
            </div>
        </div>
    }
}