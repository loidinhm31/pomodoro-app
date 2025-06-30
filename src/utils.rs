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

pub fn get_current_iso_time() -> String {
    let date = js_sys::Date::new_0();
    date.to_iso_string().into()
}

pub fn format_duration_hours_minutes(total_seconds: u32) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;

    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

pub fn format_iso_date(iso_string: &str) -> String {
    // Simple date formatting - you might want to use a proper date library
    let date = js_sys::Date::new(&iso_string.into());
    let options = js_sys::Object::new();
    js_sys::Reflect::set(&options, &"year".into(), &"numeric".into()).unwrap();
    js_sys::Reflect::set(&options, &"month".into(), &"short".into()).unwrap();
    js_sys::Reflect::set(&options, &"day".into(), &"numeric".into()).unwrap();
    js_sys::Reflect::set(&options, &"hour".into(), &"2-digit".into()).unwrap();
    js_sys::Reflect::set(&options, &"minute".into(), &"2-digit".into()).unwrap();

    date.to_locale_string("en-US", &options).into()
}