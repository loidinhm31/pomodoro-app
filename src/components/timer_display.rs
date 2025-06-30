use leptos::prelude::*;
use crate::timer::TimerController;
use crate::utils::{format_time, calculate_progress_percentage};

#[component]
pub fn TimerDisplay(controller: TimerController) -> impl IntoView {
    let format_time_display = move || format_time(controller.time_remaining.get());

    let progress_percentage = move || {
        let total_duration = controller.session_type.get().duration_minutes() * 60;
        calculate_progress_percentage(controller.time_remaining.get(), total_duration)
    };

    view! {
        <div class="text-center mb-8">
            <div class="text-6xl font-mono font-bold text-gray-800 dark:text-white mb-4">
                {format_time_display}
            </div>
            
            // Progress Bar
            <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                <div 
                    class={move || format!("h-2 rounded-full transition-all duration-1000 {}", controller.session_type.get().color_class())}
                    style:width=move || format!("{}%", progress_percentage())
                ></div>
            </div>
        </div>
    }
}