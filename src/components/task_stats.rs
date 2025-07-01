use crate::task::TaskController;
use crate::utils::format_duration_hours_minutes;
use leptos::prelude::*;

#[component]
pub fn TaskStats(task_controller: TaskController) -> impl IntoView {
    view! {
        <div class="mt-6">
            <h3 class="text-lg font-semibold text-gray-800 dark:text-white mb-4">Task Statistics</h3>
            
            {
                let task_controller_stats = task_controller.clone();
                move || {
                    let task_stats = task_controller_stats.task_stats.get();
                    
                    if task_stats.is_empty() && !task_controller_stats.loading.get() {
                        view! {
                            <div class="text-center py-8 text-gray-500 dark:text-gray-400">
                                "No task statistics available. Create and work on tasks to see insights!"
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="space-y-6">
                                // Overall Summary
                                {
                                    let total_tasks = task_stats.len() as u32;
                                    let completed_tasks = task_stats.iter().filter(|ts| ts.task.completed).count() as u32;
                                    let total_focus_time: u32 = task_stats.iter().map(|ts| ts.total_focus_time).sum();
                                    let total_minutes: u32 = task_stats.iter().map(|ts| ts.total_pomodoros).sum();
                                    
                                    view! {
                                        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                                            <div class="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-4">
                                                <div class="text-2xl font-bold text-blue-600 dark:text-blue-400">
                                                    {total_tasks}
                                                </div>
                                                <div class="text-sm text-gray-600 dark:text-gray-400">
                                                    "Total Tasks"
                                                </div>
                                            </div>

                                            <div class="bg-green-50 dark:bg-green-900/20 rounded-lg p-4">
                                                <div class="text-2xl font-bold text-green-600 dark:text-green-400">
                                                    {completed_tasks}
                                                </div>
                                                <div class="text-sm text-gray-600 dark:text-gray-400">
                                                    "Completed"
                                                </div>
                                            </div>

                                            <div class="bg-red-50 dark:bg-red-900/20 rounded-lg p-4">
                                                <div class="text-2xl font-bold text-red-600 dark:text-red-400">
                                                    {total_minutes}
                                                </div>
                                                <div class="text-sm text-gray-600 dark:text-gray-400">
                                                    "Total Minutes"
                                                </div>
                                            </div>

                                            <div class="bg-purple-50 dark:bg-purple-900/20 rounded-lg p-4">
                                                <div class="text-2xl font-bold text-purple-600 dark:text-purple-400">
                                                    {format_duration_hours_minutes(total_focus_time)}
                                                </div>
                                                <div class="text-sm text-gray-600 dark:text-gray-400">
                                                    "Focus Time"
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }

                                // Individual Task Stats
                                <div class="space-y-4">
                                    <h4 class="text-md font-semibold text-gray-700 dark:text-gray-300">Individual Tasks</h4>
                                    
                                    {task_stats.into_iter().map(|task_stat| {
                                        let task = task_stat.task;
                                        let subtasks = task_stat.subtasks;
                                        
                                        view! {
                                            <div class="border rounded-lg p-4 bg-white dark:bg-gray-800 border-gray-200 dark:border-gray-600">
                                                // Task Header
                                                <div class="flex items-start justify-between mb-3">
                                                    <div class="flex items-center space-x-3">
                                                        <div
                                                            class="w-4 h-4 rounded-full flex-shrink-0"
                                                            style:background-color=task.color.clone()
                                                        ></div>
                                                        <div>
                                                            <h5 class=format!("font-medium text-gray-800 dark:text-white {}", 
                                                                if task.completed { "line-through opacity-60" } else { "" })>
                                                                {task.name.clone()}
                                                            </h5>
                                                            {task.description.as_ref().map(|desc| {
                                                                view! {
                                                                    <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
                                                                        {desc.clone()}
                                                                    </p>
                                                                }
                                                            })}
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="text-right">
                                                        <div class="text-lg font-bold text-gray-800 dark:text-white">
                                                            {format!("{:.0}%", task_stat.completion_percentage)}
                                                        </div>
                                                        <div class="text-xs text-gray-500 dark:text-gray-400">
                                                            "Complete"
                                                        </div>
                                                    </div>
                                                </div>

                                                // Progress Bar
                                                <div class="mb-4">
                                                    <div class="flex items-center space-x-2">
                                                        <div class="flex-grow bg-gray-200 dark:bg-gray-600 rounded-full h-3">
                                                            <div
                                                                class="h-3 rounded-full transition-all duration-500"
                                                                style:background-color=task.color.clone()
                                                                style:width=format!("{}%", task_stat.completion_percentage)
                                                            ></div>
                                                        </div>
                                                    </div>
                                                </div>

                                                // Stats Grid
                                                <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-4">
                                                    <div class="text-center">
                                                        <div class="text-lg font-bold text-gray-800 dark:text-white">
                                                            {task_stat.total_pomodoros}
                                                        </div>
                                                        <div class="text-xs text-gray-600 dark:text-gray-400">
                                                            "Minutes Tracked"
                                                        </div>
                                                    </div>

                                                    <div class="text-center">
                                                        <div class="text-lg font-bold text-gray-800 dark:text-white">
                                                            {format_duration_hours_minutes(task_stat.total_focus_time)}
                                                        </div>
                                                        <div class="text-xs text-gray-600 dark:text-gray-400">
                                                            "Focus Time"
                                                        </div>
                                                    </div>

                                                    <div class="text-center">
                                                        <div class="text-lg font-bold text-gray-800 dark:text-white">
                                                            {subtasks.len()}
                                                        </div>
                                                        <div class="text-xs text-gray-600 dark:text-gray-400">
                                                            "Subtasks"
                                                        </div>
                                                    </div>

                                                    <div class="text-center">
                                                        <div class="text-lg font-bold text-gray-800 dark:text-white">
                                                            {subtasks.iter().filter(|st| st.completed).count()}
                                                        </div>
                                                        <div class="text-xs text-gray-600 dark:text-gray-400">
                                                            "Completed"
                                                        </div>
                                                    </div>
                                                </div>

                                                // Estimation Accuracy
                                                {task_stat.estimated_vs_actual.map(|(estimated, actual)| {
                                                    let avg_minutes_per_session = if estimated > 0 {
                                                        actual as f64 / estimated as f64
                                                    } else {
                                                        0.0
                                                    };
                                                    
                                                    let accuracy_color = if avg_minutes_per_session <= 30.0 { // Within reasonable time per session
                                                        "text-green-600 dark:text-green-400"
                                                    } else {
                                                        "text-red-600 dark:text-red-400"
                                                    };
                                                    
                                                    view! {
                                                        <div class="bg-gray-50 dark:bg-gray-700 rounded p-3">
                                                            <div class="flex justify-between items-center">
                                                                <span class="text-sm text-gray-600 dark:text-gray-400">
                                                                    "Time vs Estimation:"
                                                                </span>
                                                                <div class="text-right">
                                                                    <span class=format!("text-sm font-medium {}", accuracy_color)>
                                                                        {actual} " min / " {estimated} " sessions"
                                                                    </span>
                                                                    <div class=format!("text-xs {}", accuracy_color)>
                                                                        {format!("{:.1} min per session", avg_minutes_per_session)}
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    }
                                                })}

                                                // Subtask Breakdown
                                                {if !subtasks.is_empty() {
                                                    view! {
                                                        <div class="mt-4">
                                                            <h6 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                                                                "Subtask Breakdown"
                                                            </h6>
                                                            <div class="space-y-2">
                                                                {subtasks.into_iter().map(|subtask| {
                                                                    view! {
                                                                        <div class="flex items-center justify-between p-2 bg-gray-50 dark:bg-gray-700 rounded text-sm">
                                                                            <div class="flex items-center space-x-2 flex-grow min-w-0">
                                                                                <div class="w-2 h-2 rounded-full bg-gray-400 dark:bg-gray-500 flex-shrink-0"></div>
                                                                                <span class=format!("truncate {}",
                                                                                    if subtask.completed { "line-through opacity-60" } else { "" })>
                                                                                    {subtask.name}
                                                                                </span>
                                                                                {if subtask.completed {
                                                                                    view! {
                                                                                        <span class="text-xs bg-green-100 dark:bg-green-800 text-green-800 dark:text-green-200 px-1 rounded flex-shrink-0">
                                                                                            "✓"
                                                                                        </span>
                                                                                    }.into_any()
                                                                                } else {
                                                                                    view! { <div></div> }.into_any()
                                                                                }}
                                                                            </div>
                                                                            
                                                                            <div class="flex items-center space-x-2 text-xs text-gray-500 dark:text-gray-400 flex-shrink-0">
                                                                                <span class="bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 px-2 py-1 rounded">
                                                                                    {subtask.actual_pomodoros} " min 🍅"
                                                                                </span>
                                                                                <span>{format_duration_hours_minutes(subtask.total_focus_time)}</span>
                                                                                {subtask.estimated_pomodoros.map(|est| {
                                                                                    let avg_minutes_per_session = if est > 0 {
                                                                                        format!(" ({:.1}m/session)", subtask.actual_pomodoros as f64 / est as f64)
                                                                                    } else {
                                                                                        String::new()
                                                                                    };
                                                                                    
                                                                                    view! {
                                                                                        <span class="text-gray-500 dark:text-gray-400">
                                                                                            "Est: " {est} " sessions 🍅" {avg_minutes_per_session}
                                                                                        </span>
                                                                                    }
                                                                                })}
                                                                            </div>
                                                                        </div>
                                                                    }
                                                                }).collect::<Vec<_>>()}
                                                            </div>
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! { <div></div> }.into_any()
                                                }}
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        }.into_any()
                    }
                }
            }
        </div>
    }
}