use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::console_log;
use crate::timer::TimerController;
use crate::types::{delete_session_from_db, get_sessions_from_db, Session};
use crate::utils::{format_duration_hours_minutes, format_iso_date};

#[component]
pub fn SessionHistory(controller: TimerController) -> impl IntoView {
    let sessions = RwSignal::new(Vec::<Session>::new());
    let loading = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let filter_type = RwSignal::new(None::<String>);

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

                let query_limit = Some(20);
                let query_session_type = filter_type.get();

                match get_sessions_from_db(query_limit, query_session_type).await {
                    Ok(loaded_sessions) => {
                        sessions.set(loaded_sessions);
                    },
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
        move |session_id: String| {
            let sessions = sessions.clone();
            let controller = controller.clone();
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
                        console_log!("Session deleted successfully!");
                    },
                    Err(e) => {
                        console_log!("Error deleting session: {}", e);
                    }
                }
            });
        }
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
            <div class="max-h-64 overflow-y-auto">
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
                            <div class="space-y-2">
                                {session_list.into_iter().map(|session| {
                                    let session_id = session.id.clone();
                                    let delete_session = delete_session.clone();
                                    
                                    let session_color = match session.session_type.as_str() {
                                        "Work" => "border-l-red-500 bg-red-50 dark:bg-red-900/20",
                                        "ShortBreak" => "border-l-green-500 bg-green-50 dark:bg-green-900/20",
                                        "LongBreak" => "border-l-blue-500 bg-blue-50 dark:bg-blue-900/20",
                                        _ => "border-l-gray-500 bg-gray-50 dark:bg-gray-800",
                                    };

                                    view! {
                                        <div class=format!("border-l-4 p-3 rounded-r-lg {}", session_color)>
                                            <div class="flex justify-between items-start">
                                                <div class="flex-grow">
                                                    <div class="flex items-center space-x-2">
                                                        <span class="font-medium text-gray-800 dark:text-white">
                                                            {session.session_type.clone()}
                                                        </span>
                                                        <span class="text-sm text-gray-500 dark:text-gray-400">
                                                            {format_duration_hours_minutes(session.actual_duration)}
                                                        </span>
                                                    </div>
                                                    <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                                                        {format_iso_date(&session.created_at)}
                                                    </div>
                                                </div>
                                                <button
                                                    class="text-red-500 hover:text-red-700 text-sm ml-2"
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
        </div>
    }
}