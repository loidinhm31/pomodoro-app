use crate::timer::TimerController;
use crate::utils::format_duration_hours_minutes;
use leptos::prelude::*;

#[component]
pub fn SessionStats(controller: TimerController) -> impl IntoView {
    view! {
        <div class="mt-6">
            <h3 class="text-lg font-semibold text-gray-800 dark:text-white mb-4">Statistics</h3>
            
            {move || {
                if let Some(stats) = controller.session_stats.get() {
                    view! {
                        <div class="grid grid-cols-2 gap-4">
                            // Total Sessions
                            <div class="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
                                <div class="text-2xl font-bold text-gray-800 dark:text-white">
                                    {stats.completed_sessions}
                                </div>
                                <div class="text-sm text-gray-600 dark:text-gray-400">
                                    "Total Sessions"
                                </div>
                            </div>

                            // Total Focus Time
                            <div class="bg-red-50 dark:bg-red-900/20 rounded-lg p-4">
                                <div class="text-2xl font-bold text-red-600 dark:text-red-400">
                                    {format_duration_hours_minutes(stats.total_focus_time)}
                                </div>
                                <div class="text-sm text-gray-600 dark:text-gray-400">
                                    "Focus Time"
                                </div>
                            </div>

                            // Work Sessions
                            <div class="bg-green-50 dark:bg-green-900/20 rounded-lg p-4">
                                <div class="text-2xl font-bold text-green-600 dark:text-green-400">
                                    {stats.work_sessions}
                                </div>
                                <div class="text-sm text-gray-600 dark:text-gray-400">
                                    "Work Sessions"
                                </div>
                            </div>

                            // Completion Rate
                            <div class="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-4">
                                <div class="text-2xl font-bold text-blue-600 dark:text-blue-400">
                                    {format!("{:.1}%", stats.completion_rate)}
                                </div>
                                <div class="text-sm text-gray-600 dark:text-gray-400">
                                    "Completion Rate"
                                </div>
                            </div>

                            // Break Sessions
                            <div class="bg-yellow-50 dark:bg-yellow-900/20 rounded-lg p-4 col-span-2">
                                <div class="flex justify-between items-center">
                                    <div>
                                        <div class="text-lg font-bold text-yellow-600 dark:text-yellow-400">
                                            {stats.short_break_sessions + stats.long_break_sessions}
                                        </div>
                                        <div class="text-sm text-gray-600 dark:text-gray-400">
                                            "Break Sessions"
                                        </div>
                                    </div>
                                    <div class="text-right">
                                        <div class="text-sm text-gray-600 dark:text-gray-400">
                                            "Short: " <span class="font-medium">{stats.short_break_sessions}</span>
                                        </div>
                                        <div class="text-sm text-gray-600 dark:text-gray-400">
                                            "Long: " <span class="font-medium">{stats.long_break_sessions}</span>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            // Average Duration (if available)
                            {if stats.average_session_duration > 0.0 {
                                view! {
                                    <div class="bg-purple-50 dark:bg-purple-900/20 rounded-lg p-4 col-span-2">
                                        <div class="text-xl font-bold text-purple-600 dark:text-purple-400">
                                            {format_duration_hours_minutes(stats.average_session_duration as u32)}
                                        </div>
                                        <div class="text-sm text-gray-600 dark:text-gray-400">
                                            "Average Session Duration"
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }}
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="text-center py-8">
                            {if controller.loading.get() {
                                view! {
                                    <div>
                                        <div class="loading-spinner inline-block w-6 h-6 border-2 border-gray-300 border-t-blue-500 rounded-full mb-2"></div>
                                        <p class="text-gray-600 dark:text-gray-400">"Loading statistics..."</p>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="text-gray-500 dark:text-gray-400">
                                        "No statistics available yet. Complete your first session!"
                                    </div>
                                }.into_any()
                            }}
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}