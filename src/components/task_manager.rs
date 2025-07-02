use crate::task::TaskController;
use crate::types::{NewSubTask, NewTask, TASK_COLORS};
use crate::utils::format_duration_hours_minutes;
use leptos::prelude::*;

#[component]
pub fn TaskManager(task_controller: TaskController) -> impl IntoView {
    let show_new_task_form = RwSignal::new(false);
    let new_task_name = RwSignal::new(String::new());
    let new_task_description = RwSignal::new(String::new());
    let new_task_color = RwSignal::new(TASK_COLORS[0].to_string());
    let new_task_estimated_pomodoros = RwSignal::new(String::new());

    let show_new_subtask_form = RwSignal::new(None::<String>); // Task ID for which to show subtask form
    let new_subtask_name = RwSignal::new(String::new());
    let new_subtask_description = RwSignal::new(String::new());
    let new_subtask_estimated_pomodoros = RwSignal::new(String::new());

    view! {
        <div class="task-manager">
            <div class="flex justify-between items-center mb-4">
                <h3 class="text-lg font-semibold text-gray-800 dark:text-white">Task Management</h3>

                <div class="flex space-x-2">
                    <button
                        class="text-sm px-3 py-1 rounded border border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 transition-colors"
                        on:click={
                            let task_controller = task_controller.clone();
                            move |_| task_controller.show_completed.set(!task_controller.show_completed.get())
                        }
                    >
                        {
                            let task_controller = task_controller.clone();
                            move || if task_controller.show_completed.get() { "Hide Completed" } else { "Show Completed" }
                        }
                    </button>

                    <button
                        class="text-sm px-3 py-1 bg-blue-500 hover:bg-blue-600 text-white rounded transition-colors"
                        on:click=move |_| show_new_task_form.set(!show_new_task_form.get())
                    >
                        {move || if show_new_task_form.get() { "Cancel" } else { "+ New Task" }}
                    </button>
                </div>
            </div>

            // New Task Form
            {
                let task_controller_form = task_controller.clone();
                move || {
                    if show_new_task_form.get() {
                        view! {
                            <div class="mb-6 p-4 border rounded-lg bg-gray-50 dark:bg-gray-700 border-gray-200 dark:border-gray-600">
                                <h4 class="text-md font-medium text-gray-800 dark:text-white mb-3">Create New Task</h4>

                                <div class="space-y-3">
                                    <div>
                                        <input
                                            type="text"
                                            placeholder="Task name"
                                            class="w-full px-3 py-2 border rounded bg-white dark:bg-gray-800 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 placeholder-gray-500 dark:placeholder-gray-400"
                                            prop:value=move || new_task_name.get()
                                            on:input=move |ev| new_task_name.set(event_target_value(&ev))
                                        />
                                    </div>

                                    <div>
                                        <textarea
                                            placeholder="Description (optional)"
                                            class="w-full px-3 py-2 border rounded bg-white dark:bg-gray-800 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 placeholder-gray-500 dark:placeholder-gray-400"
                                            rows="2"
                                            prop:value=move || new_task_description.get()
                                            on:input=move |ev| new_task_description.set(event_target_value(&ev))
                                        ></textarea>
                                    </div>

                                    <div class="flex space-x-3">
                                        <div class="flex-1">
                                            <select
                                                class="w-full px-3 py-2 border rounded bg-white dark:bg-gray-800 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300"
                                                on:change=move |ev| new_task_color.set(event_target_value(&ev))
                                            >
                                                {TASK_COLORS.iter().enumerate().map(|(i, color)| {
                                                    view! {
                                                        <option value=color.to_string() selected=move || i == 0>
                                                            {format!("Color {}", i + 1)}
                                                        </option>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </select>
                                        </div>

                                        <div class="flex-1">
                                            <input
                                                type="number"
                                                min="1"
                                                placeholder="Est. sessions"
                                                class="w-full px-3 py-2 border rounded bg-white dark:bg-gray-800 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 placeholder-gray-500 dark:placeholder-gray-400"
                                                prop:value=move || new_task_estimated_pomodoros.get()
                                                on:input=move |ev| new_task_estimated_pomodoros.set(event_target_value(&ev))
                                            />
                                        </div>
                                    </div>

                                    <button
                                        class="w-full px-4 py-2 bg-green-500 hover:bg-green-600 text-white rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                                        on:click={
                                            let task_controller_create = task_controller_form.clone();
                                            move |_| {
                                                let name = new_task_name.get().trim().to_string();
                                                if name.is_empty() {
                                                    return;
                                                }

                                                let estimated = if new_task_estimated_pomodoros.get().trim().is_empty() {
                                                    None
                                                } else {
                                                    new_task_estimated_pomodoros.get().parse().ok()
                                                };

                                                let new_task = NewTask {
                                                    name,
                                                    description: if new_task_description.get().trim().is_empty() {
                                                        None
                                                    } else {
                                                        Some(new_task_description.get().trim().to_string())
                                                    },
                                                    color: new_task_color.get(),
                                                    estimated_pomodoros: estimated,
                                                };

                                                task_controller_create.create_task(new_task);

                                                // Reset form
                                                new_task_name.set(String::new());
                                                new_task_description.set(String::new());
                                                new_task_estimated_pomodoros.set(String::new());
                                                show_new_task_form.set(false);
                                            }
                                        }
                                        disabled=move || new_task_name.get().trim().is_empty()
                                    >
                                        "Create Task"
                                    </button>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }
            }

            // Loading State
            {
                let task_controller = task_controller.clone();
                move || {
                    if task_controller.loading.get() {
                        view! {
                            <div class="text-center py-4">
                                <div class="loading-spinner inline-block w-6 h-6 border-2 border-gray-300 border-t-blue-500 rounded-full"></div>
                                <p class="text-gray-600 dark:text-gray-400 mt-2 text-sm">"Loading tasks..."</p>
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }
            }

            // Error State
            {
                let task_controller = task_controller.clone();
                move || {
                    if let Some(error) = task_controller.error.get() {
                        view! {
                            <div class="bg-red-100 dark:bg-red-900 border border-red-300 dark:border-red-700 text-red-700 dark:text-red-300 px-4 py-3 rounded mb-4">
                                "Error: " {error}
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }
            }

            // Tasks List
            <div class="space-y-4">
                <TaskList
                    task_controller=task_controller
                    show_new_subtask_form=show_new_subtask_form
                    new_subtask_name=new_subtask_name
                    new_subtask_description=new_subtask_description
                    new_subtask_estimated_pomodoros=new_subtask_estimated_pomodoros
                />
            </div>
        </div>
    }
}

#[component]
pub fn TaskList(
    task_controller: TaskController,
    show_new_subtask_form: RwSignal<Option<String>>,
    new_subtask_name: RwSignal<String>,
    new_subtask_description: RwSignal<String>,
    new_subtask_estimated_pomodoros: RwSignal<String>,
) -> impl IntoView {
    move || {
        let progress_summary = task_controller.get_task_progress_summary();

        if progress_summary.is_empty() && !task_controller.loading.get() {
            view! {
                <div class="text-center py-8 text-gray-500 dark:text-gray-400">
                    "No tasks found. Create your first task to get started!"
                </div>
            }
            .into_any()
        } else {
            view! {
                <div class="space-y-4">
                    {progress_summary.into_iter().map(|(task, completion_percentage, completed_subtasks, total_subtasks)| {
                        view! {
                            <TaskItem
                                task=task
                                completion_percentage=completion_percentage
                                completed_subtasks=completed_subtasks
                                total_subtasks=total_subtasks
                                task_controller=task_controller.clone()
                                show_new_subtask_form=show_new_subtask_form
                                new_subtask_name=new_subtask_name
                                new_subtask_description=new_subtask_description
                                new_subtask_estimated_pomodoros=new_subtask_estimated_pomodoros
                            />
                        }
                    }).collect::<Vec<_>>()}
                </div>
            }.into_any()
        }
    }
}

#[component]
pub fn TaskItem(
    task: crate::types::Task,
    completion_percentage: f64,
    completed_subtasks: u32,
    total_subtasks: u32,
    task_controller: TaskController,
    show_new_subtask_form: RwSignal<Option<String>>,
    new_subtask_name: RwSignal<String>,
    new_subtask_description: RwSignal<String>,
    new_subtask_estimated_pomodoros: RwSignal<String>,
) -> impl IntoView {
    let task_id = task.id.clone();

    view! {
        <div class="border rounded-lg bg-white dark:bg-gray-800 border-gray-200 dark:border-gray-600 p-4">
            // Task Header
            <div class="flex items-start justify-between mb-3">
                <div class="flex items-start space-x-3 flex-grow">
                    <div
                        class="w-4 h-4 rounded-full flex-shrink-0 mt-0.5"
                        style:background-color=task.color.clone()
                    ></div>

                    <div class="flex-grow">
                        <div class="flex items-center space-x-2">
                            <h4 class=format!("font-medium text-gray-800 dark:text-white {}",
                                if task.completed { "line-through opacity-60" } else { "" })>
                                {task.name.clone()}
                            </h4>

                            {if task.completed {
                                view! {
                                    <span class="text-xs bg-green-100 dark:bg-green-800 text-green-800 dark:text-green-200 px-2 py-1 rounded">
                                        "Completed"
                                    </span>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }}
                        </div>

                        {task.description.as_ref().map(|desc| {
                            view! {
                                <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
                                    Description: {desc.clone()}
                                </p>
                            }
                        })}

                        // Progress and Stats - FIXED VERSION
                        <div class="mt-2 space-y-2">
                            <div class="flex items-center space-x-4 text-sm">
                                <span class="text-blue-600 dark:text-blue-400 font-medium">
                                    {task.actual_pomodoros} " minutes tracked 🍅"
                                </span>
                                <span class="text-gray-600 dark:text-gray-400">
                                    "Total: " {format_duration_hours_minutes(task.total_focus_time)}
                                </span>
                                {if total_subtasks > 0 {
                                    view! {
                                        <span class="text-gray-600 dark:text-gray-400">
                                            "Subtasks: " {completed_subtasks} "/" {total_subtasks}
                                        </span>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }}
                                {task.estimated_pomodoros.map(|est| {
                                    view! {
                                        <span class="text-xs text-gray-500 dark:text-gray-400 bg-gray-100 dark:bg-gray-700 px-2 py-1 rounded">
                                            "Est: " {est} " sessions 🍅"
                                        </span>
                                    }
                                })}
                            </div>

                            // Progress Bar
                            {if total_subtasks > 0 {
                                view! {
                                    <div class="flex items-center space-x-2">
                                        <div class="flex-grow bg-gray-200 dark:bg-gray-600 rounded-full h-2">
                                            <div
                                                class="h-2 rounded-full transition-all duration-300"
                                                style:background-color=task.color.clone()
                                                style:width=format!("{}%", completion_percentage)
                                            ></div>
                                        </div>
                                        <span class="text-xs text-gray-500 dark:text-gray-400 min-w-[3rem] text-right">
                                            {format!("{:.0}%", completion_percentage)}
                                        </span>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }}
                        </div>
                    </div>
                </div>

                // Task Actions
                <TaskActions
                    task=task.clone()
                    task_controller=task_controller.clone()
                    show_new_subtask_form=show_new_subtask_form
                />
            </div>

            // New Subtask Form
            <SubtaskForm
                task_id=task_id.clone()
                task_controller=task_controller.clone()
                show_new_subtask_form=show_new_subtask_form
                new_subtask_name=new_subtask_name
                new_subtask_description=new_subtask_description
                new_subtask_estimated_pomodoros=new_subtask_estimated_pomodoros
            />

            // Subtasks List
            <SubtaskList
                task_id=task_id
                task_controller=task_controller
            />
        </div>
    }
}

#[component]
pub fn TaskActions(
    task: crate::types::Task,
    task_controller: TaskController,
    show_new_subtask_form: RwSignal<Option<String>>,
) -> impl IntoView {
    let task_id = task.id.clone();

    view! {
        <div class="flex space-x-2">
            <button
                class="text-xs px-2 py-1 bg-blue-500 hover:bg-blue-600 text-white rounded transition-colors"
                on:click={
                    let task_id_subtask = task_id.clone();
                    move |_| show_new_subtask_form.set(Some(task_id_subtask.clone()))
                }
            >
                "+ Sub"
            </button>

            <button
                class={format!("text-xs px-2 py-1 rounded transition-colors {}",
                    if task.completed {
                        "bg-yellow-500 hover:bg-yellow-600 text-white"
                    } else {
                        "bg-green-500 hover:bg-green-600 text-white"
                    }
                )}
                on:click={
                    let task_id_toggle = task_id.clone();
                    let task_controller_toggle = task_controller.clone();
                    move |_| task_controller_toggle.toggle_task_completion(task_id_toggle.clone())
                }
            >
                {if task.completed { "Reopen" } else { "Done" }}
            </button>

            <button
                class="text-xs px-2 py-1 bg-red-500 hover:bg-red-600 text-white rounded transition-colors"
                on:click={
                    let task_id_delete = task_id.clone();
                    let task_controller_delete = task_controller.clone();
                    move |_| {
                        if web_sys::window()
                            .and_then(|w| w.confirm_with_message("Delete this task and all its subtasks?").ok())
                            .unwrap_or(false)
                        {
                            task_controller_delete.delete_task(task_id_delete.clone());
                        }
                    }
                }
            >
                "Delete"
            </button>
        </div>
    }
}

#[component]
pub fn SubtaskForm(
    task_id: String,
    task_controller: TaskController,
    show_new_subtask_form: RwSignal<Option<String>>,
    new_subtask_name: RwSignal<String>,
    new_subtask_description: RwSignal<String>,
    new_subtask_estimated_pomodoros: RwSignal<String>,
) -> impl IntoView {
    move || {
        if show_new_subtask_form.get() == Some(task_id.clone()) {
            view! {
                <div class="mt-3 p-3 bg-gray-50 dark:bg-gray-700 rounded border-t">
                    <div class="space-y-2">
                        <input
                            type="text"
                            placeholder="Subtask name"
                            class="w-full px-2 py-1 text-sm border rounded bg-white dark:bg-gray-800 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 placeholder-gray-500 dark:placeholder-gray-400"
                            prop:value=move || new_subtask_name.get()
                            on:input=move |ev| new_subtask_name.set(event_target_value(&ev))
                        />

                        <div class="flex space-x-2">
                            <input
                                type="text"
                                placeholder="Description (optional)"
                                class="flex-1 px-2 py-1 text-sm border rounded bg-white dark:bg-gray-800 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 placeholder-gray-500 dark:placeholder-gray-400"
                                prop:value=move || new_subtask_description.get()
                                on:input=move |ev| new_subtask_description.set(event_target_value(&ev))
                            />

                            <input
                                type="number"
                                min="1"
                                placeholder="Est. sessions"
                                class="w-24 px-2 py-1 text-sm border rounded bg-white dark:bg-gray-800 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 placeholder-gray-500 dark:placeholder-gray-400"
                                prop:value=move || new_subtask_estimated_pomodoros.get()
                                on:input=move |ev| new_subtask_estimated_pomodoros.set(event_target_value(&ev))
                            />
                        </div>

                        <div class="flex space-x-2">
                            <button
                                class="px-3 py-1 bg-green-500 hover:bg-green-600 text-white text-sm rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                                on:click={
                                    let task_id_create = task_id.clone();
                                    let task_controller_create = task_controller.clone();
                                    move |_| {
                                        let name = new_subtask_name.get().trim().to_string();
                                        if name.is_empty() {
                                            return;
                                        }

                                        let estimated = if new_subtask_estimated_pomodoros.get().trim().is_empty() {
                                            None
                                        } else {
                                            new_subtask_estimated_pomodoros.get().parse().ok()
                                        };

                                        let new_subtask = NewSubTask {
                                            task_id: task_id_create.clone(),
                                            name,
                                            description: if new_subtask_description.get().trim().is_empty() {
                                                None
                                            } else {
                                                Some(new_subtask_description.get().trim().to_string())
                                            },
                                            estimated_pomodoros: estimated,
                                        };

                                        task_controller_create.create_subtask(new_subtask);

                                        // Reset form
                                        new_subtask_name.set(String::new());
                                        new_subtask_description.set(String::new());
                                        new_subtask_estimated_pomodoros.set(String::new());
                                        show_new_subtask_form.set(None);
                                    }
                                }
                                disabled=move || new_subtask_name.get().trim().is_empty()
                            >
                                "Add"
                            </button>
                            <button
                                class="px-3 py-1 bg-gray-500 hover:bg-gray-600 text-white text-sm rounded transition-colors"
                                on:click=move |_| show_new_subtask_form.set(None)
                            >
                                "Cancel"
                            </button>
                        </div>
                    </div>
                </div>
            }.into_any()
        } else {
            view! { <div></div> }.into_any()
        }
    }
}

#[component]
pub fn SubtaskList(task_id: String, task_controller: TaskController) -> impl IntoView {
    move || {
        let subtasks = task_controller.get_subtasks_for_task(&task_id);
        if !subtasks.is_empty() {
            view! {
                <div class="mt-3 space-y-2">
                    {subtasks.into_iter().map(|subtask| {
                        let subtask_id = subtask.id.clone();

                        view! {
                            <div class="flex items-center justify-between p-2 bg-gray-50 dark:bg-gray-700 rounded">
                                <div class="flex items-center space-x-2 flex-grow">
                                    <div class="w-2 h-2 rounded-full bg-gray-400 dark:bg-gray-500 flex-shrink-0"></div>
                                    <span class=format!("text-sm text-gray-800 dark:text-gray-200 {}",
                                        if subtask.completed { "line-through opacity-60" } else { "" })>
                                        {subtask.name}
                                    </span>
                                    {if subtask.completed {
                                        view! {
                                            <span class="text-xs bg-green-100 dark:bg-green-800 text-green-800 dark:text-green-200 px-1 rounded">
                                                "✓"
                                            </span>
                                        }.into_any()
                                    } else {
                                        view! { <div></div> }.into_any()
                                    }}
                                </div>

                                <div class="flex items-center space-x-3 text-xs text-gray-600 dark:text-gray-400">
                                    <span class="bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 px-2 py-1 rounded">
                                        {subtask.actual_pomodoros} " min 🍅"
                                    </span>
                                    {subtask.estimated_pomodoros.map(|est| {
                                        view! {
                                            <span class="text-gray-500 dark:text-gray-400">
                                                "/ " {est} " sessions 🍅"
                                            </span>
                                        }
                                    })}

                                    <button
                                        class={format!("px-2 py-1 rounded text-xs transition-colors {}",
                                            if subtask.completed {
                                                "bg-yellow-500 hover:bg-yellow-600 text-white"
                                            } else {
                                                "bg-green-500 hover:bg-green-600 text-white"
                                            }
                                        )}
                                        on:click={
                                            let subtask_id_toggle = subtask_id.clone();
                                            let subtask_controller_toggle = task_controller.clone();
                                            move |_| subtask_controller_toggle.toggle_subtask_completion(subtask_id_toggle.clone())
                                        }
                                        title={if subtask.completed { "Mark as incomplete" } else { "Mark as complete" }}
                                    >
                                        {if subtask.completed { "↶" } else { "✓" }}
                                    </button>

                                    <button
                                        class="px-2 py-1 bg-red-500 hover:bg-red-600 text-white rounded text-xs transition-colors"
                                        on:click={
                                            let subtask_id_delete = subtask_id.clone();
                                            let subtask_controller_delete = task_controller.clone();
                                            move |_| {
                                                if web_sys::window()
                                                    .and_then(|w| w.confirm_with_message("Delete this subtask?").ok())
                                                    .unwrap_or(false)
                                                {
                                                    subtask_controller_delete.delete_subtask(subtask_id_delete.clone());
                                                }
                                            }
                                        }
                                        title="Delete subtask"
                                    >
                                        "×"
                                    </button>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            }.into_any()
        } else {
            view! { <div></div> }.into_any()
        }
    }
}
