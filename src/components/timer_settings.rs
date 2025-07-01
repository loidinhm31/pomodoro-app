use crate::timer::TimerController;
use crate::types::{TimerSettings, TimerState};
use leptos::prelude::*;

#[component]
pub fn TimerSettings(controller: TimerController) -> impl IntoView {
    let settings = controller.timer_settings;

    // Validation states
    let work_duration_error = RwSignal::new(None::<String>);
    let short_break_error = RwSignal::new(None::<String>);
    let long_break_error = RwSignal::new(None::<String>);
    let sessions_error = RwSignal::new(None::<String>);

    // Function to save settings with validation
    let save_settings = {
        let controller = controller.clone();
        move |new_settings: TimerSettings| {
            // Reset errors
            work_duration_error.set(None);
            short_break_error.set(None);
            long_break_error.set(None);
            sessions_error.set(None);

            // Validate settings
            let mut is_valid = true;

            if new_settings.work_duration_minutes < 1 || new_settings.work_duration_minutes > 120 {
                work_duration_error.set(Some("Work duration must be between 1-120 minutes".to_string()));
                is_valid = false;
            }

            if new_settings.short_break_duration_minutes < 1 || new_settings.short_break_duration_minutes > 60 {
                short_break_error.set(Some("Short break must be between 1-60 minutes".to_string()));
                is_valid = false;
            }

            if new_settings.long_break_duration_minutes < 1 || new_settings.long_break_duration_minutes > 120 {
                long_break_error.set(Some("Long break must be between 1-120 minutes".to_string()));
                is_valid = false;
            }

            if new_settings.sessions_before_short_break < 1 || new_settings.sessions_before_short_break > 10 {
                sessions_error.set(Some("Sessions before short break must be between 1-10".to_string()));
                is_valid = false;
            }

            if new_settings.sessions_before_long_break < 1 || new_settings.sessions_before_long_break > 20 {
                sessions_error.set(Some("Sessions before long break must be between 1-20".to_string()));
                is_valid = false;
            }

            if new_settings.sessions_before_long_break <= new_settings.sessions_before_short_break {
                sessions_error.set(Some("Long break interval must be greater than short break interval".to_string()));
                is_valid = false;
            }

            if is_valid {
                controller.update_timer_settings(new_settings);
            }
        }
    };

    // Function to reset to defaults
    let reset_to_defaults = {
        let save_settings = save_settings.clone();
        move |_| {
            save_settings(TimerSettings::default());
        }
    };

    view! {
        <div class="timer-settings space-y-6">
            <h4 class="text-lg font-medium text-gray-700 dark:text-gray-300 mb-4">
                "Timer Settings"
            </h4>

            // Session Durations
            <div class="space-y-4 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
                <h5 class="text-md font-medium text-gray-700 dark:text-gray-300">
                    "Session Durations"
                </h5>
                
                // Work Duration
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-600 dark:text-gray-400 mb-1">
                            "Work Duration (minutes)"
                        </label>
                        <input
                            type="number"
                            min="1"
                            max="120"
                            class="w-full px-3 py-2 border rounded-md bg-white dark:bg-gray-800 dark:border-gray-600 text-gray-700 dark:text-gray-300"
                            class:border-red-500=move || work_duration_error.get().is_some()
                            value=move || settings.get().work_duration_minutes
                            disabled=move || controller.timer_state.get() != TimerState::Stopped
                            on:input={
                                let save_settings = save_settings.clone();
                                move |ev| {
                                    if let Ok(value) = event_target_value(&ev).parse::<u32>() {
                                        let mut current_settings = settings.get();
                                        current_settings.work_duration_minutes = value;
                                        save_settings(current_settings);
                                    }
                                }
                            }
                        />
                        {move || {
                            if let Some(error) = work_duration_error.get() {
                                view! {
                                    <p class="text-red-500 text-xs mt-1">{error}</p>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }
                        }}
                    </div>

                    // Short Break Duration
                    <div>
                        <label class="block text-sm font-medium text-gray-600 dark:text-gray-400 mb-1">
                            "Short Break (minutes)"
                        </label>
                        <input
                            type="number"
                            min="1"
                            max="60"
                            class="w-full px-3 py-2 border rounded-md bg-white dark:bg-gray-800 dark:border-gray-600 text-gray-700 dark:text-gray-300"
                            class:border-red-500=move || short_break_error.get().is_some()
                            value=move || settings.get().short_break_duration_minutes
                            disabled=move || controller.timer_state.get() != TimerState::Stopped
                            on:input={
                                let save_settings = save_settings.clone();
                                move |ev| {
                                    if let Ok(value) = event_target_value(&ev).parse::<u32>() {
                                        let mut current_settings = settings.get();
                                        current_settings.short_break_duration_minutes = value;
                                        save_settings(current_settings);
                                    }
                                }
                            }
                        />
                        {move || {
                            if let Some(error) = short_break_error.get() {
                                view! {
                                    <p class="text-red-500 text-xs mt-1">{error}</p>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }
                        }}
                    </div>

                    // Long Break Duration
                    <div>
                        <label class="block text-sm font-medium text-gray-600 dark:text-gray-400 mb-1">
                            "Long Break (minutes)"
                        </label>
                        <input
                            type="number"
                            min="1"
                            max="120"
                            class="w-full px-3 py-2 border rounded-md bg-white dark:bg-gray-800 dark:border-gray-600 text-gray-700 dark:text-gray-300"
                            class:border-red-500=move || long_break_error.get().is_some()
                            value=move || settings.get().long_break_duration_minutes
                            disabled=move || controller.timer_state.get() != TimerState::Stopped
                            on:input={
                                let save_settings = save_settings.clone();
                                move |ev| {
                                    if let Ok(value) = event_target_value(&ev).parse::<u32>() {
                                        let mut current_settings = settings.get();
                                        current_settings.long_break_duration_minutes = value;
                                        save_settings(current_settings);
                                    }
                                }
                            }
                        />
                        {move || {
                            if let Some(error) = long_break_error.get() {
                                view! {
                                    <p class="text-red-500 text-xs mt-1">{error}</p>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }
                        }}
                    </div>
                </div>
            </div>

            // Break Intervals
            <div class="space-y-4 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
                <h5 class="text-md font-medium text-gray-700 dark:text-gray-300">
                    "Break Intervals"
                </h5>
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    // Sessions before short break
                    <div>
                        <label class="block text-sm font-medium text-gray-600 dark:text-gray-400 mb-1">
                            "Work sessions before short break"
                        </label>
                        <input
                            type="number"
                            min="1"
                            max="10"
                            class="w-full px-3 py-2 border rounded-md bg-white dark:bg-gray-800 dark:border-gray-600 text-gray-700 dark:text-gray-300"
                            class:border-red-500=move || sessions_error.get().is_some()
                            value=move || settings.get().sessions_before_short_break
                            disabled=move || controller.timer_state.get() != TimerState::Stopped
                            on:input={
                                let save_settings = save_settings.clone();
                                move |ev| {
                                    if let Ok(value) = event_target_value(&ev).parse::<u32>() {
                                        let mut current_settings = settings.get();
                                        current_settings.sessions_before_short_break = value;
                                        save_settings(current_settings);
                                    }
                                }
                            }
                        />
                        <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                            "Take a short break after this many work sessions"
                        </p>
                    </div>

                    // Sessions before long break
                    <div>
                        <label class="block text-sm font-medium text-gray-600 dark:text-gray-400 mb-1">
                            "Work sessions before long break"
                        </label>
                        <input
                            type="number"
                            min="1"
                            max="20"
                            class="w-full px-3 py-2 border rounded-md bg-white dark:bg-gray-800 dark:border-gray-600 text-gray-700 dark:text-gray-300"
                            class:border-red-500=move || sessions_error.get().is_some()
                            value=move || settings.get().sessions_before_long_break
                            disabled=move || controller.timer_state.get() != TimerState::Stopped
                            on:input={
                                let save_settings = save_settings.clone();
                                move |ev| {
                                    if let Ok(value) = event_target_value(&ev).parse::<u32>() {
                                        let mut current_settings = settings.get();
                                        current_settings.sessions_before_long_break = value;
                                        save_settings(current_settings);
                                    }
                                }
                            }
                        />
                        <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                            "Take a long break after this many work sessions"
                        </p>
                    </div>
                </div>

                {move || {
                    if let Some(error) = sessions_error.get() {
                        view! {
                            <p class="text-red-500 text-sm">{error}</p>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }}
            </div>

            // Auto-start Options
            <div class="space-y-4 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
                <h5 class="text-md font-medium text-gray-700 dark:text-gray-300">
                    "Auto-start Behavior"
                </h5>
                
                <div class="space-y-3">
                    // Auto-start breaks
                    <div class="flex items-center justify-between">
                        <div>
                            <span class="text-sm font-medium text-gray-600 dark:text-gray-400">
                                "Auto-start breaks"
                            </span>
                            <p class="text-xs text-gray-500 dark:text-gray-400">
                                "Automatically start break sessions after work completion"
                            </p>
                        </div>
                        <input
                            type="checkbox"
                            class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                            checked=move || settings.get().auto_start_breaks
                            on:change={
                                let save_settings = save_settings.clone();
                                move |ev| {
                                    let mut current_settings = settings.get();
                                    current_settings.auto_start_breaks = event_target_checked(&ev);
                                    save_settings(current_settings);
                                }
                            }
                        />
                    </div>

                    // Auto-start work
                    <div class="flex items-center justify-between">
                        <div>
                            <span class="text-sm font-medium text-gray-600 dark:text-gray-400">
                                "Auto-start work sessions"
                            </span>
                            <p class="text-xs text-gray-500 dark:text-gray-400">
                                "Automatically start work sessions after break completion"
                            </p>
                        </div>
                        <input
                            type="checkbox"
                            class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                            checked=move || settings.get().auto_start_work
                            on:change={
                                let save_settings = save_settings.clone();
                                move |ev| {
                                    let mut current_settings = settings.get();
                                    current_settings.auto_start_work = event_target_checked(&ev);
                                    save_settings(current_settings);
                                }
                            }
                        />
                    </div>
                </div>
            </div>

            // Current Settings Preview
            <div class="p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg border border-blue-200 dark:border-blue-800">
                <h5 class="text-md font-medium text-blue-800 dark:text-blue-200 mb-3">
                    "Current Schedule Preview"
                </h5>
                
                {move || {
                    let current_settings = settings.get();
                    let short_break_interval = current_settings.sessions_before_short_break;
                    let long_break_interval = current_settings.sessions_before_long_break;
                    
                    view! {
                        <div class="text-sm text-blue-700 dark:text-blue-300 space-y-2">
                            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                                <div>
                                    <span class="font-medium">"Work:"</span>
                                    <span class="ml-1">{current_settings.work_duration_minutes}" min"</span>
                                </div>
                                <div>
                                    <span class="font-medium">"Short Break:"</span>
                                    <span class="ml-1">{current_settings.short_break_duration_minutes}" min"</span>
                                </div>
                                <div>
                                    <span class="font-medium">"Long Break:"</span>
                                    <span class="ml-1">{current_settings.long_break_duration_minutes}" min"</span>
                                </div>
                            </div>
                            
                            <div class="pt-2 border-t border-blue-200 dark:border-blue-700">
                                <p>
                                    <span class="font-medium">"Pattern:"</span>
                                    " Short break every " {short_break_interval} 
                                    {if short_break_interval == 1 { " session" } else { " sessions" }}
                                    ", long break every " {long_break_interval}
                                    {if long_break_interval == 1 { " session" } else { " sessions" }}
                                </p>
                                
                                // Show a sample schedule
                                <div class="mt-2">
                                    <span class="font-medium">"Example sequence:"</span>
                                    <div class="mt-1 flex flex-wrap gap-1">
                                        {(1..=8).map(|i| {
                                            let session_type = if i % long_break_interval == 0 {
                                                "Long Break"
                                            } else if i % short_break_interval == 0 {
                                                "Short Break" 
                                            } else {
                                                "Work"
                                            };
                                            
                                            let color = match session_type {
                                                "Work" => "bg-red-100 text-red-800 dark:bg-red-800 dark:text-red-200",
                                                "Short Break" => "bg-green-100 text-green-800 dark:bg-green-800 dark:text-green-200",
                                                "Long Break" => "bg-blue-100 text-blue-800 dark:bg-blue-800 dark:text-blue-200",
                                                _ => "bg-gray-100 text-gray-800"
                                            };
                                            
                                            view! {
                                                <span class=format!("px-2 py-1 rounded text-xs {}", color)>
                                                    {i}". " {session_type}
                                                </span>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                }}
            </div>

            // Actions
            <div class="flex justify-between items-center pt-4 border-t border-gray-200 dark:border-gray-600">
                <button
                    class="px-4 py-2 bg-gray-500 hover:bg-gray-600 text-white rounded transition-colors disabled:opacity-50"
                    disabled=move || controller.timer_state.get() != TimerState::Stopped
                    on:click=reset_to_defaults
                >
                    "Reset to Defaults"
                </button>

                <div class="text-sm text-gray-500 dark:text-gray-400">
                    {move || {
                        if controller.timer_state.get() != TimerState::Stopped {
                            "Settings can only be changed when timer is stopped"
                        } else {
                            "Settings are saved automatically"
                        }
                    }}
                </div>
            </div>

            // Work Session Counter Reset
            <div class="p-4 bg-yellow-50 dark:bg-yellow-900/20 rounded-lg border border-yellow-200 dark:border-yellow-800">
                <div class="flex justify-between items-center">
                    <div>
                        <h5 class="text-md font-medium text-yellow-800 dark:text-yellow-200">
                            "Work Session Counter"
                        </h5>
                        <p class="text-sm text-yellow-700 dark:text-yellow-300">
                            "Current cycle: " {move || controller.current_cycle_work_sessions.get()} " work sessions"
                        </p>
                        <p class="text-sm text-yellow-700 dark:text-yellow-300">
                            "Total completed: " {move || controller.completed_work_sessions.get()} " work sessions"
                        </p>
                        <p class="text-xs text-yellow-600 dark:text-yellow-400 mt-1">
                            "Reset the current cycle counter to restart the break pattern. This only affects break timing, not your historical statistics."
                        </p>
                    </div>
                    <button
                        class="px-3 py-2 bg-yellow-500 hover:bg-yellow-600 text-white text-sm rounded transition-colors disabled:opacity-50"
                        disabled=move || controller.timer_state.get() != TimerState::Stopped
                        on:click=move |_| controller.reset_work_sessions()
                    >
                        "Reset Cycle"
                    </button>
                </div>
            </div>
        </div>
    }
}