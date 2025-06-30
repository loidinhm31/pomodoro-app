use leptos::prelude::*;
use crate::types::TimerState;
use crate::timer::TimerController;
use crate::components::{TimerDisplay, TimerControls, SessionSelector, SessionHistory, SessionStats};

#[derive(Debug, Clone, Copy, PartialEq)]
enum AppTab {
    Timer,
    History,
    Statistics,
}

#[component]
pub fn App() -> impl IntoView {
    let controller = TimerController::new();
    let active_tab = RwSignal::new(AppTab::Timer);

    // Timer completion effect
    Effect::new({
        let controller = controller.clone();
        move |_| {
            if controller.time_remaining.get() == 0 && controller.timer_state.get() == TimerState::Running {
                controller.complete_session();
            }
        }
    });

    view! {
        <main class="min-h-screen bg-gray-100 dark:bg-gray-900 flex flex-col items-center justify-start p-4">
            <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg w-full max-w-lg">
                
                // Tab Navigation
                <div class="flex border-b border-gray-200 dark:border-gray-700">
                    <button
                        class={move || format!(
                            "flex-1 py-3 px-4 text-center font-medium transition-colors {}",
                            if active_tab.get() == AppTab::Timer {
                                "border-b-2 border-blue-500 text-blue-600 dark:text-blue-400"
                            } else {
                                "text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200"
                            }
                        )}
                        on:click=move |_| active_tab.set(AppTab::Timer)
                    >
                        "Timer"
                    </button>
                    <button
                        class={move || format!(
                            "flex-1 py-3 px-4 text-center font-medium transition-colors {}",
                            if active_tab.get() == AppTab::History {
                                "border-b-2 border-blue-500 text-blue-600 dark:text-blue-400"
                            } else {
                                "text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200"
                            }
                        )}
                        on:click=move |_| active_tab.set(AppTab::History)
                    >
                        "History"
                    </button>
                    <button
                        class={move || format!(
                            "flex-1 py-3 px-4 text-center font-medium transition-colors {}",
                            if active_tab.get() == AppTab::Statistics {
                                "border-b-2 border-blue-500 text-blue-600 dark:text-blue-400"
                            } else {
                                "text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200"
                            }
                        )}
                        on:click=move |_| active_tab.set(AppTab::Statistics)
                    >
                        "Stats"
                    </button>
                </div>

                // Tab Content
                <div class="p-6">
                    {move || {
                        match active_tab.get() {
                            AppTab::Timer => view! {
                                <div>
                                    // Session Type Header
                                    <div class="text-center mb-6">
                                        <h1 class="text-3xl font-bold text-gray-800 dark:text-white mb-2">
                                            "Pomodoro Timer"
                                        </h1>
                                        <div class={
                                            let controller = controller.clone();
                                            move || format!("inline-block px-4 py-2 rounded-full text-white font-semibold {}", controller.session_type.get().color_class())
                                        }>
                                            {
                                                let controller = controller.clone();
                                                move || controller.session_type.get().name()
                                            }
                                        </div>
                                    </div>

                                    // Timer Display
                                    <TimerDisplay controller=controller.clone() />

                                    // Timer Controls
                                    <TimerControls controller=controller.clone() />

                                    // Session Info
                                    <div class="text-center text-gray-600 dark:text-gray-400 mb-6">
                                        <p class="text-lg">
                                            "Sessions completed: " <span class="font-bold text-gray-800 dark:text-white">{
                                                let controller = controller.clone();
                                                move || controller.completed_sessions.get()
                                            }</span>
                                        </p>
                                    </div>

                                    // Session Type Selector
                                    <SessionSelector controller=controller.clone() />
                                </div>
                            }.into_any(),
                            
                            AppTab::History => view! {
                                <div>
                                    <SessionHistory controller=controller.clone() />
                                </div>
                            }.into_any(),
                            
                            AppTab::Statistics => view! {
                                <div>
                                    <SessionStats controller=controller.clone() />
                                </div>
                            }.into_any(),
                        }
                    }}
                </div>
            </div>
        </main>
    }
}