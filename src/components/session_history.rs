// Updated src/components/session_history.rs

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::console_log;
use crate::timer::TimerController;
use crate::task::TaskController;
use crate::types::{delete_session_from_db, get_sessions_from_db, Session};
use crate::utils::{format_duration_hours_minutes, format_iso_date};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn SessionHistory(controller: TimerController) -> impl IntoView {
    let sessions = RwSignal::new(Vec::<Session>::new());
    let loading = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let filter_type = RwSignal::new(None::<String>);

    // Create task controller to resolve task names
    let task_controller = TaskController::new();

    // Function to open video file
    let open_video_file = move |video_path: String| {
        spawn_local(async move {
            console_log!("Attempting to open video file: {}", video_path);

            let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                "path": video_path.clone()
            }))
                .unwrap_or(JsValue::NULL);

            let result = invoke("open_video_file", args).await;

            // Try to deserialize the result as a Result<String, String>
            match serde_wasm_bindgen::from_value::<Result<String, String>>(result) {
                Ok(Ok(success_msg)) => {
                    console_log!("Video file opened: {}", success_msg);
                }
                Ok(Err(error_msg)) => {
                    console_log!("Failed to open video file: {}", error_msg);
                    // Try to reveal in file explorer as fallback
                    let reveal_args = serde_wasm_bindgen::to_value(&serde_json::json!({
                        "path": video_path
                    }))
                        .unwrap_or(JsValue::NULL);

                    let reveal_result = invoke("reveal_in_explorer", reveal_args).await;
                    match serde_wasm_bindgen::from_value::<Result<String, String>>(reveal_result) {
                        Ok(Ok(reveal_success)) => {
                            console_log!("File location revealed: {}", reveal_success);
                        }
                        Ok(Err(reveal_error)) => {
                            console_log!("Failed to reveal in explorer: {}", reveal_error);
                        }
                        Err(_) => {
                            console_log!("Unexpected response from reveal command");
                        }
                    }
                }
                Err(_) => {
                    console_log!("Unexpected response format from open_video_file command");
                }
            }
        });
    };

    // Load sessions on component mount
    let load_sessions = {
        let sessions = sessions.clone();
        let loading = loading.clone();
        let error = error.clone();
        let filter_type = filter_type.clone();

        move || {
            let sessions = sessions.clone();
            let loading = loading.clone();
            let error = error.clone();
            let filter_type = filter_type.clone();

            spawn_local(async move {
                loading.set(true);
                error.set(None);

                let query_limit = Some(50); // Increased to show more sessions
                let query_session_type = filter_type.get();

                match get_sessions_from_db(query_limit, query_session_type).await {
                    Ok(loaded_sessions) => {
                        sessions.set(loaded_sessions);
                    }
                    Err(e) => {
                        console_log!("Error loading sessions: {}", e);
                        error.set(Some(e));
                    }
                }

                loading.set(false);
            });
        }
    };

    // Load sessions initially
    Effect::new({
        let load_sessions = load_sessions.clone();
        move |_| {
            load_sessions();
        }
    });

    // Reload when filter changes
    Effect::new({
        let load_sessions = load_sessions.clone();
        move |_| {
            let _ = filter_type.get(); // Track changes
            load_sessions();
        }
    });

    let delete_session = {
        let sessions = sessions.clone();
        let controller = controller.clone();
        let task_controller = task_controller.clone();
        move |session_id: String| {
            let sessions = sessions.clone();
            let controller = controller.clone();
            let task_controller = task_controller.clone();
            spawn_local(async move {
                match delete_session_from_db(session_id.clone()).await {
                    Ok(_) => {
                        // Remove from local list
                        let current_sessions = sessions.get();
                        let updated_sessions: Vec<Session> = current_sessions
                            .into_iter()
                            .filter(|s| s.id != session_id)
                            .collect();
                        sessions.set(updated_sessions);

                        // Reload stats
                        controller.load_session_stats();
                        task_controller.load_task_stats();
                        console_log!("Session deleted successfully!");
                    }
                    Err(e) => {
                        console_log!("Error deleting session: {}", e);
                    }
                }
            });
        }
    };

    // Helper function to resolve task/subtask names
    let get_task_info = move |session: &Session| -> Option<String> {
        let tasks = task_controller.tasks.get();
        let subtasks = task_controller.subtasks.get();

        if let Some(subtask_id) = &session.subtask_id {
            if let Some(subtask) = subtasks.iter().find(|st| st.id == *subtask_id) {
                if let Some(task) = tasks.iter().find(|t| t.id == subtask.task_id) {
                    return Some(format!("{} → {}", task.name, subtask.name));
                } else {
                    return Some(format!("Unknown Task → {}", subtask.name));
                }
            }
        } else if let Some(task_id) = &session.task_id {
            if let Some(task) = tasks.iter().find(|t| t.id == *task_id) {
                return Some(task.name.clone());
            } else {
                return Some("Unknown Task".to_string());
            }
        }
        None
    };

    view! {
        <div class="mt-6">
            <div class="flex justify-between items-center mb-4">
                <h3 class="text-lg font-semibold text-gray-800 dark:text-white">Session History</h3>

                // Filter dropdown
                <select
                    class="px-3 py-1 border rounded text-sm bg-white dark:bg-gray-700 dark:border-gray-600 text-gray-700 dark:text-gray-300"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        if value.is_empty() || value == "all" {
                            filter_type.set(None);
                        } else {
                            filter_type.set(Some(value));
                        }
                    }
                >
                    <option value="all">"All Sessions"</option>
                    <option value="Work">"Work Sessions"</option>
                    <option value="ShortBreak">"Short Breaks"</option>
                    <option value="LongBreak">"Long Breaks"</option>
                </select>
            </div>

            // Loading state
            {move || {
                if loading.get() {
                    view! {
                        <div class="text-center py-4">
                            <div class="loading-spinner inline-block w-6 h-6 border-2 border-gray-300 border-t-blue-500 rounded-full"></div>
                            <p class="text-gray-600 dark:text-gray-400 mt-2">"Loading sessions..."</p>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}

            // Error state
            {move || {
                if let Some(err) = error.get() {
                    view! {
                        <div class="bg-red-100 dark:bg-red-900 border border-red-300 dark:border-red-700 text-red-700 dark:text-red-300 px-4 py-3 rounded mb-4">
                            "Error loading sessions: " {err}
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}

            // Sessions list
            <div class="max-h-96 overflow-y-auto">
                {move || {
                    let session_list = sessions.get();
                    if session_list.is_empty() && !loading.get() {
                        view! {
                            <div class="text-center py-8 text-gray-500 dark:text-gray-400">
                                "No sessions found. Complete your first session to see it here!"
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="space-y-3">
                                {session_list.into_iter().map(|session| {
                                    let session_id = session.id.clone();
                                    let delete_session = delete_session.clone();
                                    let open_video = open_video_file.clone();
                                    let has_video = session.video_path.is_some();
                                    let video_path = session.video_path.clone();
                                    let is_break_session = session.session_type == "ShortBreak" || session.session_type == "LongBreak";
                                    let is_work_session = session.session_type == "Work";
                                    let task_info = get_task_info(&session);

                                    let session_color = match session.session_type.as_str() {
                                        "Work" => "border-l-red-500 bg-red-50 dark:bg-red-900/20",
                                        "ShortBreak" => "border-l-green-500 bg-green-50 dark:bg-green-900/20",
                                        "LongBreak" => "border-l-blue-500 bg-blue-50 dark:bg-blue-900/20",
                                        _ => "border-l-gray-500 bg-gray-50 dark:bg-gray-800",
                                    };

                                    view! {
                                        <div class=format!("border-l-4 p-4 rounded-r-lg {}", session_color)>
                                            <div class="flex justify-between items-start">
                                                <div class="flex-grow">
                                                    <div class="flex items-center space-x-2 mb-2">
                                                        <span class="font-medium text-gray-800 dark:text-white">
                                                            {session.session_type.clone()}
                                                        </span>
                                                        <span class="text-sm text-gray-500 dark:text-gray-400">
                                                            {format_duration_hours_minutes(session.actual_duration)}
                                                        </span>

                                                        // Task indicator for work sessions
                                                        {if is_work_session {
                                                            if let Some(task_name) = task_info.clone() {
                                                                view! {
                                                                    <span class="text-xs bg-blue-100 dark:bg-blue-800 text-blue-800 dark:text-blue-200 px-2 py-1 rounded" title="Task tracked">
                                                                        "🎯 " {task_name}
                                                                    </span>
                                                                }.into_any()
                                                            } else {
                                                                view! {
                                                                    <span class="text-xs bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-400 px-2 py-1 rounded" title="No task tracked">
                                                                        "⚪ No task"
                                                                    </span>
                                                                }.into_any()
                                                            }
                                                        } else {
                                                            view! { <div></div> }.into_any()
                                                        }}

                                                        // Video indicator for break sessions
                                                        {if is_break_session {
                                                            if has_video {
                                                                view! {
                                                                    <span class="text-xs bg-green-100 dark:bg-green-800 text-green-800 dark:text-green-200 px-2 py-1 rounded" title="Video recorded">
                                                                        "📹 Video"
                                                                    </span>
                                                                }.into_any()
                                                            } else {
                                                                view! {
                                                                    <span class="text-xs bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-400 px-2 py-1 rounded" title="No video recording">
                                                                        "📹 No Video"
                                                                    </span>
                                                                }.into_any()
                                                            }
                                                        } else {
                                                            view! { <div></div> }.into_any()
                                                        }}
                                                    </div>

                                                    <div class="text-xs text-gray-500 dark:text-gray-400">
                                                        {format_iso_date(&session.created_at)}
                                                    </div>

                                                    // Task details for work sessions
                                                    {if is_work_session && task_info.is_some() {
                                                        view! {
                                                            <div class="mt-2 p-2 bg-blue-50 dark:bg-blue-900/20 rounded border border-blue-200 dark:border-blue-800">
                                                                <div class="text-xs font-medium text-blue-800 dark:text-blue-200 mb-1">
                                                                    "Task Progress:"
                                                                </div>
                                                                <div class="text-xs text-blue-700 dark:text-blue-300">
                                                                    {task_info.unwrap_or_default()}
                                                                </div>
                                                                <div class="text-xs text-blue-600 dark:text-blue-400 mt-1">
                                                                    "Focus time: " {format_duration_hours_minutes(session.actual_duration)}
                                                                </div>
                                                            </div>
                                                        }.into_any()
                                                    } else if is_work_session {
                                                        view! {
                                                            <div class="mt-2 text-xs text-gray-400 dark:text-gray-500 italic">
                                                                "No task was selected for this work session"
                                                            </div>
                                                        }.into_any()
                                                    } else {
                                                        view! { <div></div> }.into_any()
                                                    }}

                                                    // Video file info and controls
                                                    {if let Some(video_path_display) = video_path.clone() {
                                                        let filename = video_path_display.split('/').last().unwrap_or("unknown").to_string();
                                                        let filename_display = filename.clone();
                                                        let filename_title = filename.clone();
                                                        let video_path_for_open = video_path_display.clone();

                                                        view! {
                                                            <div class="mt-2 p-3 bg-white dark:bg-gray-700 rounded border border-gray-200 dark:border-gray-600">
                                                                <div class="flex flex-col lg:flex-row lg:items-center lg:justify-between gap-2">
                                                                    <div class="flex-grow min-w-0">
                                                                        <div class="text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">
                                                                            "Video Recording:"
                                                                        </div>
                                                                        <div class="text-xs text-gray-500 dark:text-gray-400 break-all" title={filename_title}>
                                                                            {filename_display}
                                                                        </div>
                                                                    </div>
                                                                    <div class="flex-shrink-0">
                                                                        <button
                                                                            class="px-3 py-1.5 text-xs bg-blue-500 hover:bg-blue-600 text-white rounded transition-colors whitespace-nowrap"
                                                                            on:click=move |_| open_video(video_path_for_open.clone())
                                                                            title="Open video file"
                                                                        >
                                                                            "📹 Open Video"
                                                                        </button>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        }.into_any()
                                                    } else if is_break_session {
                                                        view! {
                                                            <div class="mt-2 text-xs text-gray-400 dark:text-gray-500 italic">
                                                                "No video recording for this break session"
                                                            </div>
                                                        }.into_any()
                                                    } else {
                                                        view! { <div></div> }.into_any()
                                                    }}
                                                </div>

                                                <button
                                                    class="text-red-500 hover:text-red-700 text-sm ml-2 flex-shrink-0"
                                                    on:click=move |_| delete_session(session_id.clone())
                                                    title="Delete session"
                                                >
                                                    "×"
                                                </button>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    }
                }}
            </div>

            // Enhanced Legend for indicators
            <div class="mt-6 p-4 bg-gray-50 dark:bg-gray-800 rounded text-sm">
                <h4 class="font-medium text-gray-700 dark:text-gray-300 mb-3">"Legend:"</h4>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    // Work Session Indicators
                    <div>
                        <h5 class="text-sm font-medium text-gray-600 dark:text-gray-400 mb-2">"Work Sessions:"</h5>
                        <div class="space-y-2 text-gray-600 dark:text-gray-400">
                            <div class="flex items-center space-x-3">
                                <span class="bg-blue-100 dark:bg-blue-800 text-blue-800 dark:text-blue-200 px-2 py-1 rounded text-xs">"🎯 Task Name"</span>
                                <span>"Work session with task tracking"</span>
                            </div>
                            <div class="flex items-center space-x-3">
                                <span class="bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-400 px-2 py-1 rounded text-xs">"⚪ No Task"</span>
                                <span>"Work session without task tracking"</span>
                            </div>
                        </div>
                    </div>
                    
                    // Break Session Indicators
                    <div>
                        <h5 class="text-sm font-medium text-gray-600 dark:text-gray-400 mb-2">"Break Sessions:"</h5>
                        <div class="space-y-2 text-gray-600 dark:text-gray-400">
                            <div class="flex items-center space-x-3">
                                <span class="bg-green-100 dark:bg-green-800 text-green-800 dark:text-green-200 px-2 py-1 rounded text-xs">"📹 Video"</span>
                                <span>"Break session with video recording"</span>
                            </div>
                            <div class="flex items-center space-x-3">
                                <span class="bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-400 px-2 py-1 rounded text-xs">"📹 No Video"</span>
                                <span>"Break session without video recording"</span>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-400 mt-4 pl-2 border-l-2 border-gray-300 dark:border-gray-600">
                    <strong>"Note:"</strong> " Task tracking is only available for work sessions. Video recording is available for break sessions based on your camera settings."
                </div>
            </div>
        </div>
    }
}