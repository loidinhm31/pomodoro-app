use crate::timer::TimerController;
use crate::types::{SessionType, TimerState};
use leptos::prelude::*;

#[component]
pub fn SessionSelector(controller: TimerController) -> impl IntoView {
    view! {
        <div class="mt-6 flex justify-center space-x-2">
            <button
                on:click={
                    let controller = controller.clone();
                    move |_| controller.set_session_type(SessionType::Work)
                }
                class={
                    let controller = controller.clone();
                    move || format!(
                        "px-3 py-1 rounded text-sm font-medium transition-colors {}",
                        if controller.session_type.get() == SessionType::Work {
                            "bg-red-500 text-white"
                        } else {
                            "bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600"
                        }
                    )
                }
                disabled={
                    let controller = controller.clone();
                    move || controller.timer_state.get() != TimerState::Stopped
                }
            >
                "Work (25m)"
            </button>
            <button
                on:click={
                    let controller = controller.clone();
                    move |_| controller.set_session_type(SessionType::ShortBreak)
                }
                class={
                    let controller = controller.clone();
                    move || format!(
                        "px-3 py-1 rounded text-sm font-medium transition-colors {}",
                        if controller.session_type.get() == SessionType::ShortBreak {
                            "bg-green-500 text-white"
                        } else {
                            "bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600"
                        }
                    )
                }
                disabled={
                    let controller = controller.clone();
                    move || controller.timer_state.get() != TimerState::Stopped
                }
            >
                "Short Break (5m)"
            </button>
            <button
                on:click={
                    let controller = controller.clone();
                    move |_| controller.set_session_type(SessionType::LongBreak)
                }
                class={
                    let controller = controller.clone();
                    move || format!(
                        "px-3 py-1 rounded text-sm font-medium transition-colors {}",
                        if controller.session_type.get() == SessionType::LongBreak {
                            "bg-blue-500 text-white"
                        } else {
                            "bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600"
                        }
                    )
                }
                disabled={
                    let controller = controller.clone();
                    move || controller.timer_state.get() != TimerState::Stopped
                }
            >
                "Long Break (15m)"
            </button>
        </div>
    }
}