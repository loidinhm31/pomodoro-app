use crate::task::TaskController;
use leptos::prelude::*;

#[component]
pub fn TaskSelector(task_controller: TaskController) -> impl IntoView {
    let show_task_selection = RwSignal::new(false);

    view! {
        <div class="task-selector mb-6">
            <div class="flex items-center justify-between mb-3">
                <h4 class="text-sm font-medium text-gray-700 dark:text-gray-300">
                    "Active Task"
                </h4>
                <button
                    class="text-sm text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-200 transition-colors"
                    on:click=move |_| show_task_selection.set(!show_task_selection.get())
                >
                    {move || if show_task_selection.get() { "Hide Tasks" } else { "Select Task" }}
                </button>
            </div>

            // Current selection display
            <div class="p-3 bg-gray-50 dark:bg-gray-700 rounded-lg mb-3">
                {
                    let task_controller_display = task_controller.clone();
                    move || {
                        if let Some(task_info) = task_controller_display.get_active_task_info() {
                            view! {
                                <div class="flex items-center justify-between">
                                    <div class="flex items-center space-x-2">
                                        {
                                            let task_controller_color = task_controller_display.clone();
                                            move || {
                                                if let Some(task) = task_controller_color.selected_task.get() {
                                                    view! {
                                                        <div
                                                            class="w-3 h-3 rounded-full flex-shrink-0"
                                                            style:background-color=task.color
                                                        ></div>
                                                    }.into_any()
                                                } else {
                                                    view! { <div></div> }.into_any()
                                                }
                                            }
                                        }
                                        <span class="text-sm font-medium text-gray-800 dark:text-white">
                                            {task_info}
                                        </span>
                                    </div>
                                    <button
                                        class="text-xs text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 transition-colors"
                                        on:click={
                                            let task_controller_clear = task_controller_display.clone();
                                            move |_| {
                                                task_controller_clear.select_task(None);
                                                task_controller_clear.select_subtask(None);
                                            }
                                        }
                                    >
                                        "Clear"
                                    </button>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="text-sm text-gray-500 dark:text-gray-400 text-center">
                                    "No task selected"
                                </div>
                            }.into_any()
                        }
                    }
                }
            </div>

            // Task selection dropdown
            {
                let task_controller_dropdown = task_controller.clone();
                move || {
                    if show_task_selection.get() {
                        view! {
                            <div class="border rounded-lg bg-white dark:bg-gray-800 border-gray-200 dark:border-gray-600 max-h-64 overflow-y-auto">
                                {
                                    let task_controller_list = task_controller_dropdown.clone();
                                    move || {
                                        let filtered_tasks = task_controller_list.get_filtered_tasks();
                                        
                                        if filtered_tasks.is_empty() {
                                            view! {
                                                <div class="p-4 text-center text-gray-500 dark:text-gray-400 text-sm">
                                                    "No tasks available. Create a task first!"
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <div class="divide-y divide-gray-200 dark:divide-gray-600">
                                                    {filtered_tasks.into_iter().map(|task| {
                                                        let task_id = task.id.clone();
                                                        let task_clone = task.clone();
                                                        let task_controller_item = task_controller_list.clone();
                                                        
                                                        view! {
                                                            <div class="p-3">
                                                                // Task option
                                                                <button
                                                                    class="w-full text-left flex items-center space-x-3 p-2 rounded hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
                                                                    on:click={
                                                                        let task_for_select = task_clone.clone();
                                                                        let controller_for_task = task_controller_item.clone();
                                                                        move |_| {
                                                                            controller_for_task.select_task(Some(task_for_select.clone()));
                                                                            show_task_selection.set(false);
                                                                        }
                                                                    }
                                                                >
                                                                    <div
                                                                        class="w-3 h-3 rounded-full flex-shrink-0"
                                                                        style:background-color=task.color.clone()
                                                                    ></div>
                                                                    <div class="flex-grow min-w-0">
                                                                        <div class="text-sm font-medium text-gray-800 dark:text-white truncate">
                                                                            {task.name.clone()}
                                                                        </div>
                                                                        {task.description.as_ref().map(|desc| {
                                                                            view! {
                                                                                <div class="text-xs text-gray-500 dark:text-gray-400 truncate">
                                                                                    {desc.clone()}
                                                                                </div>
                                                                            }
                                                                        })}
                                                                    </div>
                                                                    <div class="text-xs text-blue-600 dark:text-blue-400 bg-blue-50 dark:bg-blue-900 px-2 py-1 rounded flex-shrink-0">
                                                                        {task.actual_pomodoros} " min"
                                                                    </div>
                                                                </button>

                                                                // Subtasks
                                                                {
                                                                    let subtasks = task_controller_item.get_subtasks_for_task(&task_id);
                                                                    if !subtasks.is_empty() {
                                                                        view! {
                                                                            <div class="ml-6 mt-2 space-y-1">
                                                                                {subtasks.into_iter().map(|subtask| {
                                                                                    let subtask_clone = subtask.clone();
                                                                                    let controller_for_subtask = task_controller_item.clone();
                                                                                    let task_clone_for_subtask = task_clone.clone();
                                                                                    
                                                                                    view! {
                                                                                        <button
                                                                                            class="w-full text-left flex items-center space-x-3 p-2 rounded text-sm hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
                                                                                            on:click=move |_| {
                                                                                                // Select both task and subtask
                                                                                                controller_for_subtask.select_task(Some(task_clone_for_subtask.clone()));
                                                                                                controller_for_subtask.select_subtask(Some(subtask_clone.clone()));
                                                                                                show_task_selection.set(false);
                                                                                            }
                                                                                        >
                                                                                            <div class="w-2 h-2 rounded-full bg-gray-400 dark:bg-gray-500 flex-shrink-0 ml-1"></div>
                                                                                            <div class="flex-grow min-w-0">
                                                                                                <span class="text-gray-700 dark:text-gray-300 truncate block">
                                                                                                    {subtask.name}
                                                                                                </span>
                                                                                            </div>
                                                                                            <div class="text-xs text-blue-600 dark:text-blue-400 bg-blue-50 dark:bg-blue-900 px-2 py-1 rounded flex-shrink-0">
                                                                                                {subtask.actual_pomodoros} " min"
                                                                                            </div>
                                                                                        </button>
                                                                                    }
                                                                                }).collect::<Vec<_>>()}
                                                                            </div>
                                                                        }.into_any()
                                                                    } else {
                                                                        view! { <div></div> }.into_any()
                                                                    }
                                                                }
                                                            </div>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            }.into_any()
                                        }
                                    }
                                }
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