use leptos::prelude::*;
use crate::types::TimerState;
use crate::timer::TimerController;

#[component]
pub fn TimerControls(controller: TimerController) -> impl IntoView {
    view! {
        <div class="flex justify-center space-x-4 mb-6">
            {move || {
                let controller = controller.clone();
                match controller.timer_state.get() {
                    TimerState::Stopped => view! {
                        <button
                            on:click={
                                let controller = controller.clone();
                                move |_| controller.start_timer()
                            }
                            class="bg-green-500 hover:bg-green-600 text-white font-bold py-3 px-6 rounded-lg transition-colors"
                        >
                            "Start"
                        </button>
                    }.into_any(),
                    TimerState::Running => view! {
                        <button
                            on:click={
                                let controller = controller.clone();
                                move |_| controller.pause_timer()
                            }
                            class="bg-yellow-500 hover:bg-yellow-600 text-white font-bold py-3 px-6 rounded-lg transition-colors"
                        >
                            "Pause"
                        </button>
                        <button
                            on:click={
                                let controller = controller.clone();
                                move |_| controller.stop_timer()
                            }
                            class="bg-red-500 hover:bg-red-600 text-white font-bold py-3 px-6 rounded-lg transition-colors"
                        >
                            "Stop"
                        </button>
                    }.into_any(),
                    TimerState::Paused => view! {
                        <button
                            on:click={
                                let controller = controller.clone();
                                move |_| controller.start_timer()
                            }
                            class="bg-green-500 hover:bg-green-600 text-white font-bold py-3 px-6 rounded-lg transition-colors"
                        >
                            "Resume"
                        </button>
                        <button
                            on:click={
                                let controller = controller.clone();
                                move |_| controller.stop_timer()
                            }
                            class="bg-red-500 hover:bg-red-600 text-white font-bold py-3 px-6 rounded-lg transition-colors"
                        >
                            "Stop"
                        </button>
                    }.into_any(),
                }
            }}
        </div>
    }
}