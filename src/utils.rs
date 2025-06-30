use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    pub fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    pub fn clearInterval(id: i32);
}

// Macro for easier console logging
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::utils::log(&format_args!($($t)*).to_string()))
}

pub fn format_time(total_seconds: u32) -> String {
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

pub fn calculate_progress_percentage(time_remaining: u32, total_duration: u32) -> f64 {
    let elapsed = total_duration - time_remaining;
    (elapsed as f64 / total_duration as f64) * 100.0
}