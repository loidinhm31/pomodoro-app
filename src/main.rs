mod app;
mod types;
mod utils;
mod timer;
mod components;
mod task;
mod theme;
mod keyboard_shortcuts;
mod cleanup_scheduler;

use app::*;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}