// Fixed src/task.rs

use crate::console_log;
use crate::types::{
    delete_subtask_from_db, delete_task_from_db, get_all_subtasks, get_all_tasks,
    get_task_stats, save_subtask_to_db, save_task_to_db,
    update_subtask_in_db, update_task_in_db, NewSubTask, NewTask, SubTask, Task, TaskStats,
};
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone)]
pub struct TaskController {
    pub tasks: RwSignal<Vec<Task>>,
    pub subtasks: RwSignal<Vec<SubTask>>,
    pub task_stats: RwSignal<Vec<TaskStats>>,
    pub selected_task: RwSignal<Option<Task>>,
    pub selected_subtask: RwSignal<Option<SubTask>>,
    pub loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
    pub show_completed: RwSignal<bool>,
}

impl TaskController {
    pub fn new() -> Self {
        let controller = Self {
            tasks: RwSignal::new(Vec::new()),
            subtasks: RwSignal::new(Vec::new()),
            task_stats: RwSignal::new(Vec::new()),
            selected_task: RwSignal::new(None),
            selected_subtask: RwSignal::new(None),
            loading: RwSignal::new(false),
            error: RwSignal::new(None),
            show_completed: RwSignal::new(false),
        };

        // Load initial data
        controller.load_tasks();
        controller.load_task_stats();

        controller
    }

    pub fn load_tasks(&self) {
        let controller = self.clone();
        spawn_local(async move {
            controller.loading.set(true);
            controller.error.set(None);

            match get_all_tasks().await {
                Ok(tasks) => {
                    controller.tasks.set(tasks);
                    console_log!("Tasks loaded successfully");
                }
                Err(e) => {
                    console_log!("Error loading tasks: {}", e);
                    controller.error.set(Some(e));
                }
            }

            match get_all_subtasks().await {
                Ok(subtasks) => {
                    controller.subtasks.set(subtasks);
                    console_log!("Subtasks loaded successfully");
                }
                Err(e) => {
                    console_log!("Error loading subtasks: {}", e);
                    controller.error.set(Some(e));
                }
            }

            controller.loading.set(false);
        });
    }

    pub fn load_task_stats(&self) {
        let controller = self.clone();
        spawn_local(async move {
            match get_task_stats().await {
                Ok(stats) => {
                    controller.task_stats.set(stats);
                    console_log!("Task stats loaded successfully");
                }
                Err(e) => {
                    console_log!("Error loading task stats: {}", e);
                    controller.error.set(Some(e));
                }
            }
        });
    }

    pub fn create_task(&self, new_task: NewTask) {
        let controller = self.clone();
        spawn_local(async move {
            controller.loading.set(true);
            controller.error.set(None);

            match save_task_to_db(new_task).await {
                Ok(task_id) => {
                    console_log!("Task created with ID: {}", task_id);
                    controller.load_tasks();
                    controller.load_task_stats();
                }
                Err(e) => {
                    console_log!("Error creating task: {}", e);
                    controller.error.set(Some(e));
                }
            }

            controller.loading.set(false);
        });
    }

    pub fn create_subtask(&self, new_subtask: NewSubTask) {
        let controller = self.clone();
        spawn_local(async move {
            controller.loading.set(true);
            controller.error.set(None);

            match save_subtask_to_db(new_subtask).await {
                Ok(subtask_id) => {
                    console_log!("Subtask created with ID: {}", subtask_id);
                    controller.load_tasks();
                    controller.load_task_stats();
                }
                Err(e) => {
                    console_log!("Error creating subtask: {}", e);
                    controller.error.set(Some(e));
                }
            }

            controller.loading.set(false);
        });
    }

    pub fn update_task(&self, updated_task: Task) {
        let controller = self.clone();
        spawn_local(async move {
            match update_task_in_db(updated_task.clone()).await {
                Ok(_) => {
                    console_log!("Task updated: {}", updated_task.id);
                    controller.load_tasks();
                    controller.load_task_stats();
                }
                Err(e) => {
                    console_log!("Error updating task: {}", e);
                    controller.error.set(Some(e));
                }
            }
        });
    }

    pub fn update_subtask(&self, updated_subtask: SubTask) {
        let controller = self.clone();
        spawn_local(async move {
            match update_subtask_in_db(updated_subtask.clone()).await {
                Ok(_) => {
                    console_log!("Subtask updated: {}", updated_subtask.id);
                    controller.load_tasks();
                    controller.load_task_stats();
                }
                Err(e) => {
                    console_log!("Error updating subtask: {}", e);
                    controller.error.set(Some(e));
                }
            }
        });
    }

    pub fn toggle_task_completion(&self, task_id: String) {
        let controller = self.clone();
        spawn_local(async move {
            let mut tasks = controller.tasks.get();
            if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
                task.completed = !task.completed;
                controller.update_task(task.clone());
            }
        });
    }

    pub fn toggle_subtask_completion(&self, subtask_id: String) {
        let controller = self.clone();
        spawn_local(async move {
            let mut subtasks = controller.subtasks.get();
            if let Some(subtask) = subtasks.iter_mut().find(|st| st.id == subtask_id) {
                subtask.completed = !subtask.completed;
                controller.update_subtask(subtask.clone());
            }
        });
    }

    pub fn delete_task(&self, task_id: String) {
        let controller = self.clone();
        spawn_local(async move {
            controller.loading.set(true);

            match delete_task_from_db(task_id.clone()).await {
                Ok(deleted) => {
                    if deleted {
                        console_log!("Task deleted: {}", task_id);

                        // Clear selection if deleted task was selected
                        if let Some(selected) = controller.selected_task.get() {
                            if selected.id == task_id {
                                controller.selected_task.set(None);
                                controller.selected_subtask.set(None);
                            }
                        }

                        controller.load_tasks();
                        controller.load_task_stats();
                    } else {
                        console_log!("Task not found for deletion: {}", task_id);
                    }
                }
                Err(e) => {
                    console_log!("Error deleting task: {}", e);
                    controller.error.set(Some(e));
                }
            }

            controller.loading.set(false);
        });
    }

    pub fn delete_subtask(&self, subtask_id: String) {
        let controller = self.clone();
        spawn_local(async move {
            controller.loading.set(true);

            match delete_subtask_from_db(subtask_id.clone()).await {
                Ok(deleted) => {
                    if deleted {
                        console_log!("Subtask deleted: {}", subtask_id);

                        // Clear selection if deleted subtask was selected
                        if let Some(selected) = controller.selected_subtask.get() {
                            if selected.id == subtask_id {
                                controller.selected_subtask.set(None);
                            }
                        }

                        controller.load_tasks();
                        controller.load_task_stats();
                    } else {
                        console_log!("Subtask not found for deletion: {}", subtask_id);
                    }
                }
                Err(e) => {
                    console_log!("Error deleting subtask: {}", e);
                    controller.error.set(Some(e));
                }
            }

            controller.loading.set(false);
        });
    }

    pub fn select_task(&self, task: Option<Task>) {
        self.selected_task.set(task);
        // Clear subtask selection when task changes
        self.selected_subtask.set(None);
        console_log!("Task selection changed");
    }

    pub fn select_subtask(&self, subtask: Option<SubTask>) {
        self.selected_subtask.set(subtask);
        console_log!("Subtask selection changed");
    }

    pub fn get_filtered_tasks(&self) -> Vec<Task> {
        let tasks = self.tasks.get();
        let show_completed = self.show_completed.get();

        if show_completed {
            tasks
        } else {
            tasks.into_iter().filter(|t| !t.completed).collect()
        }
    }

    pub fn get_subtasks_for_task(&self, task_id: &str) -> Vec<SubTask> {
        let subtasks = self.subtasks.get();
        let show_completed = self.show_completed.get();

        let filtered: Vec<SubTask> = subtasks
            .into_iter()
            .filter(|st| st.task_id == task_id)
            .collect();

        if show_completed {
            filtered
        } else {
            filtered.into_iter().filter(|st| !st.completed).collect()
        }
    }

    pub fn get_active_task_info(&self) -> Option<String> {
        if let Some(subtask) = self.selected_subtask.get() {
            if let Some(task) = self.tasks.get().iter().find(|t| t.id == subtask.task_id) {
                Some(format!("{} → {}", task.name, subtask.name))
            } else {
                Some(subtask.name)
            }
        } else if let Some(task) = self.selected_task.get() {
            Some(task.name)
        } else {
            None
        }
    }

    pub fn get_current_selection(&self) -> (Option<String>, Option<String>) {
        let task_id = if let Some(subtask) = self.selected_subtask.get() {
            Some(subtask.task_id)
        } else {
            self.selected_task.get().map(|t| t.id)
        };

        let subtask_id = self.selected_subtask.get().map(|st| st.id);

        (task_id, subtask_id)
    }

    // Helper method to get task by ID
    pub fn get_task_by_id(&self, task_id: &str) -> Option<Task> {
        self.tasks.get().into_iter().find(|t| t.id == task_id)
    }

    // Helper method to get subtask by ID
    pub fn get_subtask_by_id(&self, subtask_id: &str) -> Option<SubTask> {
        self.subtasks.get().into_iter().find(|st| st.id == subtask_id)
    }

    // Get tasks with their completion status
    pub fn get_task_progress_summary(&self) -> Vec<(Task, f64, u32, u32)> {
        let tasks = self.tasks.get();
        let all_subtasks = self.subtasks.get();

        tasks
            .into_iter()
            .map(|task| {
                let subtasks: Vec<_> = all_subtasks
                    .iter()
                    .filter(|st| st.task_id == task.id)
                    .collect();

                let completion_percentage = task.calculate_completion_percentage(&subtasks.iter().map(|st| (*st).clone()).collect::<Vec<_>>());
                let total_subtasks = subtasks.len() as u32;
                let completed_subtasks = subtasks.iter().filter(|st| st.completed).count() as u32;

                (task, completion_percentage, completed_subtasks, total_subtasks)
            })
            .collect()
    }
}