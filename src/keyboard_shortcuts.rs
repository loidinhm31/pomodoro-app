use crate::console_log;
use crate::timer::TimerController;
use crate::types::{SessionType, TimerState};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{window, KeyboardEvent};

#[derive(Clone)]
pub struct KeyboardShortcuts {
    timer_controller: TimerController,
}

impl KeyboardShortcuts {
    pub fn new(
        timer_controller: TimerController,
    ) -> Self {
        let shortcuts = Self {
            timer_controller,
        };

        shortcuts.setup_global_listeners();
        shortcuts
    }

    fn setup_global_listeners(&self) {
        if let Some(win) = window() {
            let timer_controller = self.timer_controller.clone();

            let keydown_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                // Don't interfere with typing in inputs
                if let Some(target) = event.target() {
                    if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                        let tag_name = element.tag_name().to_lowercase();
                        if tag_name == "input" || tag_name == "textarea" || tag_name == "select" {
                            return;
                        }

                        // Check if element is contenteditable
                        if element.is_content_editable() {
                            return;
                        }
                    }
                }

                let key = event.key();
                let ctrl_key = event.ctrl_key();
                let alt_key = event.alt_key();
                let shift_key = event.shift_key();

                match key.as_str() {
                    // Space: Start/Pause timer
                    " " => {
                        event.prevent_default();
                        match timer_controller.timer_state.get() {
                            TimerState::Stopped => {
                                timer_controller.start_timer();
                                console_log!("⌨️ Started timer via keyboard shortcut");
                            }
                            TimerState::Running => {
                                timer_controller.pause_timer();
                                console_log!("⌨️ Paused timer via keyboard shortcut");
                            }
                            TimerState::Paused => {
                                timer_controller.start_timer();
                                console_log!("⌨️ Resumed timer via keyboard shortcut");
                            }
                        }
                    }

                    // Escape: Stop timer
                    "Escape" => {
                        event.prevent_default();
                        if timer_controller.timer_state.get() != TimerState::Stopped {
                            timer_controller.stop_timer();
                            console_log!("⌨️ Stopped timer via keyboard shortcut");
                        }
                    }

                    // Tab: Switch session type (when stopped)
                    "Tab" => {
                        if timer_controller.timer_state.get() == TimerState::Stopped {
                            event.prevent_default();
                            let current = timer_controller.session_type.get();
                            let next = if shift_key {
                                // Shift+Tab: go backwards
                                match current {
                                    SessionType::Work => SessionType::LongBreak,
                                    SessionType::ShortBreak => SessionType::Work,
                                    SessionType::LongBreak => SessionType::ShortBreak,
                                }
                            } else {
                                // Tab: go forwards
                                match current {
                                    SessionType::Work => SessionType::ShortBreak,
                                    SessionType::ShortBreak => SessionType::LongBreak,
                                    SessionType::LongBreak => SessionType::Work,
                                }
                            };
                            timer_controller.set_session_type(next);
                            console_log!("⌨️ Switched to {:?} session via keyboard shortcut", next);
                        }
                    }

                    // Numbers 1-3: Quick session type selection
                    "1" => {
                        if timer_controller.timer_state.get() == TimerState::Stopped {
                            timer_controller.set_session_type(SessionType::Work);
                            console_log!("⌨️ Selected Work session via keyboard shortcut");
                        }
                    }
                    "2" => {
                        if timer_controller.timer_state.get() == TimerState::Stopped {
                            timer_controller.set_session_type(SessionType::ShortBreak);
                            console_log!("⌨️ Selected Short Break session via keyboard shortcut");
                        }
                    }
                    "3" => {
                        if timer_controller.timer_state.get() == TimerState::Stopped {
                            timer_controller.set_session_type(SessionType::LongBreak);
                            console_log!("⌨️ Selected Long Break session via keyboard shortcut");
                        }
                    }

                    // R: Reset work sessions (when stopped)
                    "r" | "R" => {
                        if timer_controller.timer_state.get() == TimerState::Stopped && ctrl_key {
                            event.prevent_default();
                            timer_controller.reset_work_sessions();
                            console_log!("⌨️ Reset work sessions via keyboard shortcut");
                        }
                    }

                    // F: Fullscreen mode toggle
                    "f" | "F" => {
                        if alt_key {
                            event.prevent_default();
                            if let Some(window_obj) = window() {
                                if let Some(document) = window_obj.document() {
                                    if let Some(element) = document.document_element() {
                                        if document.fullscreen_element().is_none() {
                                            let _ = element.request_fullscreen();
                                            console_log!("⌨️ Entered fullscreen mode");
                                        } else {
                                            let _ = document.exit_fullscreen();
                                            console_log!("⌨️ Exited fullscreen mode");
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Help shortcut: Show/hide keyboard shortcuts
                    "?" => {
                        if shift_key {
                            event.prevent_default();
                            console_log!("⌨️ Keyboard shortcuts:");
                            console_log!("  Space: Start/Pause timer");
                            console_log!("  Escape: Stop timer");
                            console_log!("  Tab: Switch session type");
                            console_log!("  1/2/3: Quick session selection");
                            console_log!("  M: Toggle ambient sounds");
                            console_log!("  Ctrl+R: Reset work sessions");
                            console_log!("  Ctrl++/-: Adjust volume");
                            console_log!("  Alt+F: Toggle fullscreen");
                            console_log!("  Shift+?: Show this help");
                        }
                    }

                    _ => {
                        // Ignore other keys
                    }
                }
            }) as Box<dyn FnMut(KeyboardEvent)>);

            let _ = win.add_event_listener_with_callback(
                "keydown",
                keydown_closure.as_ref().unchecked_ref(),
            );

            // Prevent the closure from being dropped
            keydown_closure.forget();

            console_log!("⌨️ Global keyboard shortcuts activated");
            console_log!("   Press Shift+? to see available shortcuts");
        }
    }
}

// Keyboard shortcuts help component
#[component]
pub fn KeyboardShortcutsHelp() -> impl IntoView {
    let show_help = RwSignal::new(false);

    view! {
        <div class="relative">
            // Help toggle button
            <button
                class="fixed bottom-4 right-4 w-12 h-12 bg-blue-500 hover:bg-blue-600 text-white rounded-full shadow-lg transition-all duration-200 hover:scale-110 z-50"
                on:click=move |_| show_help.set(!show_help.get())
                title="Keyboard shortcuts help"
            >
                <span class="text-lg">"⌨️"</span>
            </button>

            // Help modal
            {move || {
                if show_help.get() {
                    view! {
                        <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
                            <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full max-h-96 overflow-y-auto">
                                <div class="p-6">
                                    <div class="flex justify-between items-center mb-4">
                                        <h3 class="text-lg font-semibold text-gray-800 dark:text-white">
                                            "⌨️ Keyboard Shortcuts"
                                        </h3>
                                        <button
                                            class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
                                            on:click=move |_| show_help.set(false)
                                        >
                                            "✕"
                                        </button>
                                    </div>

                                    <div class="space-y-3 text-sm">
                                        <div class="grid grid-cols-1 gap-3">
                                            <div class="flex justify-between items-center">
                                                <span class="text-gray-600 dark:text-gray-400">"Start/Pause"</span>
                                                <kbd class="px-2 py-1 bg-gray-100 dark:bg-gray-700 rounded text-xs">"Space"</kbd>
                                            </div>
                                            <div class="flex justify-between items-center">
                                                <span class="text-gray-600 dark:text-gray-400">"Stop"</span>
                                                <kbd class="px-2 py-1 bg-gray-100 dark:bg-gray-700 rounded text-xs">"Esc"</kbd>
                                            </div>
                                            <div class="flex justify-between items-center">
                                                <span class="text-gray-600 dark:text-gray-400">"Switch Session"</span>
                                                <kbd class="px-2 py-1 bg-gray-100 dark:bg-gray-700 rounded text-xs">"Tab"</kbd>
                                            </div>
                                            <div class="flex justify-between items-center">
                                                <span class="text-gray-600 dark:text-gray-400">"Work/Break/Long"</span>
                                                <div class="space-x-1">
                                                    <kbd class="px-2 py-1 bg-gray-100 dark:bg-gray-700 rounded text-xs">"1"</kbd>
                                                    <kbd class="px-2 py-1 bg-gray-100 dark:bg-gray-700 rounded text-xs">"2"</kbd>
                                                    <kbd class="px-2 py-1 bg-gray-100 dark:bg-gray-700 rounded text-xs">"3"</kbd>
                                                </div>
                                            </div>
                                            <div class="flex justify-between items-center">
                                                <span class="text-gray-600 dark:text-gray-400">"Toggle Sounds"</span>
                                                <kbd class="px-2 py-1 bg-gray-100 dark:bg-gray-700 rounded text-xs">"M"</kbd>
                                            </div>
                                            <div class="flex justify-between items-center">
                                                <span class="text-gray-600 dark:text-gray-400">"Reset Cycle"</span>
                                                <kbd class="px-2 py-1 bg-gray-100 dark:bg-gray-700 rounded text-xs">"Ctrl+R"</kbd>
                                            </div>
                                            <div class="flex justify-between items-center">
                                                <span class="text-gray-600 dark:text-gray-400">"Volume"</span>
                                                <div class="space-x-1">
                                                    <kbd class="px-2 py-1 bg-gray-100 dark:bg-gray-700 rounded text-xs">"Ctrl++"</kbd>
                                                    <kbd class="px-2 py-1 bg-gray-100 dark:bg-gray-700 rounded text-xs">"Ctrl+-"</kbd>
                                                </div>
                                            </div>
                                            <div class="flex justify-between items-center">
                                                <span class="text-gray-600 dark:text-gray-400">"Fullscreen"</span>
                                                <kbd class="px-2 py-1 bg-gray-100 dark:bg-gray-700 rounded text-xs">"Alt+F"</kbd>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-600">
                                        <p class="text-xs text-gray-500 dark:text-gray-400">
                                            "💡 Shortcuts work globally when the app is focused"
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}