use leptos::prelude::*;
use crate::types::TimerState;
use crate::timer::TimerController;
use crate::components::{TimerDisplay, TimerControls, SessionSelector};

#[component]
pub fn App() -> impl IntoView {
    let controller = TimerController::new();

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
        <main class="min-h-screen bg-gray-100 dark:bg-gray-900 flex flex-col items-center justify-center p-4">
            <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8 w-full max-w-md">
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
                <div class="text-center text-gray-600 dark:text-gray-400">
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
        
        </main>
    }
}