use crate::components::CameraController;
use crate::timer::TimerController;
use crate::types::TimerState;
use leptos::prelude::*;

#[component]
pub fn TimerControls(
    timer_controller: TimerController,
    camera_controller: CameraController,
) -> impl IntoView {
    view! {
        <div class="flex justify-center space-x-4 mb-6">
            {move || {
                match timer_controller.timer_state.get() {
                    TimerState::Stopped => {
                        let timer_start = timer_controller.clone();
                        let camera_start = camera_controller.clone();
                        view! {
                            <button
                                on:click=move |_| timer_start.start_timer_with_camera(Some(&camera_start))
                                class="bg-green-500 hover:bg-green-600 text-white font-bold py-3 px-6 rounded-lg transition-colors"
                            >
                                "Start"
                            </button>
                        }.into_any()
                    },
                    TimerState::Running => {
                        let timer_pause = timer_controller.clone();
                        let camera_pause = camera_controller.clone();
                        let timer_stop = timer_controller.clone();
                        let camera_stop = camera_controller.clone();
                        view! {
                            <button
                                on:click=move |_| timer_pause.pause_timer_with_camera(Some(&camera_pause))
                                class="bg-yellow-500 hover:bg-yellow-600 text-white font-bold py-3 px-6 rounded-lg transition-colors"
                            >
                                "Pause"
                            </button>
                            <button
                                on:click=move |_| timer_stop.stop_timer_with_camera(Some(&camera_stop))
                                class="bg-red-500 hover:bg-red-600 text-white font-bold py-3 px-6 rounded-lg transition-colors"
                            >
                                "Stop"
                            </button>
                        }.into_any()
                    },
                    TimerState::Paused => {
                        let timer_resume = timer_controller.clone();
                        let camera_resume = camera_controller.clone();
                        let timer_stop = timer_controller.clone();
                        let camera_stop = camera_controller.clone();
                        view! {
                            <button
                                on:click=move |_| timer_resume.start_timer_with_camera(Some(&camera_resume))
                                class="bg-green-500 hover:bg-green-600 text-white font-bold py-3 px-6 rounded-lg transition-colors"
                            >
                                "Resume"
                            </button>
                            <button
                                on:click=move |_| timer_stop.stop_timer_with_camera(Some(&camera_stop))
                                class="bg-red-500 hover:bg-red-600 text-white font-bold py-3 px-6 rounded-lg transition-colors"
                            >
                                "Stop"
                            </button>
                        }.into_any()
                    },
                }
            }}
        </div>
    }
}
