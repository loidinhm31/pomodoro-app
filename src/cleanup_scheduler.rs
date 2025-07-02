use crate::console_log;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, Storage};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupScheduleSettings {
    pub auto_cleanup_enabled: bool,
    pub days_to_keep: u32,
    pub last_cleanup_date: Option<String>, // Store date as YYYY-MM-DD
    pub cleanup_hour: u32, // Hour of day to run cleanup (0-23)
}

impl Default for CleanupScheduleSettings {
    fn default() -> Self {
        Self {
            auto_cleanup_enabled: true,
            days_to_keep: 3,
            last_cleanup_date: None,
            cleanup_hour: 2, // 2 AM by default
        }
    }
}

impl CleanupScheduleSettings {
    pub fn save_to_storage(&self) {
        if let Ok(settings_json) = serde_json::to_string(&self) {
            if let Some(storage) = get_local_storage() {
                let _ = storage.set_item("pomodoro_cleanup_schedule", &settings_json);
            }
        }
    }

    pub fn load_from_storage() -> Self {
        if let Some(storage) = get_local_storage() {
            if let Ok(Some(settings_json)) = storage.get_item("pomodoro_cleanup_schedule") {
                if let Ok(settings) = serde_json::from_str::<CleanupScheduleSettings>(&settings_json) {
                    return settings;
                }
            }
        }
        Self::default()
    }
}

#[derive(Clone)]
pub struct CleanupScheduler {
    pub settings: RwSignal<CleanupScheduleSettings>,
    pub is_running: RwSignal<bool>,
    pub last_check: RwSignal<Option<String>>,
}

impl CleanupScheduler {
    pub fn new() -> Self {
        let settings = CleanupScheduleSettings::load_from_storage();

        let scheduler = Self {
            settings: RwSignal::new(settings),
            is_running: RwSignal::new(false),
            last_check: RwSignal::new(None),
        };

        // Start the scheduler
        scheduler.start();

        scheduler
    }

    pub fn start(&self) {
        if self.is_running.get() {
            return; // Already running
        }

        self.is_running.set(true);
        console_log!("🕐 Cleanup scheduler started");

        let scheduler = self.clone();
        spawn_local(async move {
            scheduler.run_scheduler_loop().await;
        });
    }

    pub fn stop(&self) {
        self.is_running.set(false);
        console_log!("🕐 Cleanup scheduler stopped");
    }

    async fn run_scheduler_loop(&self) {
        while self.is_running.get() {
            // Check if cleanup should run
            if self.should_run_cleanup() {
                console_log!("🧹 Scheduled cleanup triggered");
                self.run_scheduled_cleanup().await;
            }

            // Update last check time
            let now = js_sys::Date::new_0();
            self.last_check.set(Some(now.to_iso_string().into()));

            // Wait 10 minutes before next check (600,000 ms)
            gloo_timers::future::sleep(std::time::Duration::from_millis(600_000)).await;
        }
    }

    fn should_run_cleanup(&self) -> bool {
        let settings = self.settings.get();

        if !settings.auto_cleanup_enabled {
            return false;
        }

        let now = js_sys::Date::new_0();
        let current_date = Self::get_date_string(&now);
        let current_hour = now.get_hours() as u32;

        // Check if we've already run today
        if let Some(last_cleanup_date) = &settings.last_cleanup_date {
            if last_cleanup_date == &current_date {
                return false; // Already ran today
            }
        }

        // Check if it's the right time of day (within 1 hour window)
        let target_hour = settings.cleanup_hour;
        current_hour >= target_hour && current_hour < target_hour + 1
    }

    async fn run_scheduled_cleanup(&self) {
        let settings = self.settings.get();
        console_log!("🧹 Running scheduled cleanup (keeping {} days)", settings.days_to_keep);

        match self.run_cleanup_command(settings.days_to_keep).await {
            Ok(result) => {
                console_log!("✅ Scheduled cleanup completed: {}", result);

                // Update last cleanup date
                let now = js_sys::Date::new_0();
                let current_date = Self::get_date_string(&now);

                let mut updated_settings = settings;
                updated_settings.last_cleanup_date = Some(current_date);
                self.settings.set(updated_settings.clone());
                updated_settings.save_to_storage();
            }
            Err(e) => {
                console_log!("❌ Scheduled cleanup failed: {}", e);
            }
        }
    }

    async fn run_cleanup_command(&self, days_to_keep: u32) -> Result<String, String> {
        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
            "daysOld": days_to_keep
        }))
            .map_err(|e| format!("Failed to serialize args: {}", e))?;

        let result = invoke("cleanup_old_videos", args).await;
        serde_wasm_bindgen::from_value(result)
            .map_err(|e| format!("Failed to run cleanup: {}", e))
    }

    fn get_date_string(date: &js_sys::Date) -> String {
        format!(
            "{:04}-{:02}-{:02}",
            date.get_full_year(),
            date.get_month() + 1, // JavaScript months are 0-based
            date.get_date()
        )
    }

    pub fn update_settings(&self, new_settings: CleanupScheduleSettings) {
        new_settings.save_to_storage();
        self.settings.set(new_settings);
        console_log!("🕐 Cleanup scheduler settings updated");
    }

    pub fn force_cleanup_now(&self) {
        let scheduler = self.clone();
        spawn_local(async move {
            console_log!("🧹 Manual cleanup triggered");
            scheduler.run_scheduled_cleanup().await;
        });
    }

    pub fn get_next_cleanup_time(&self) -> Option<String> {
        let settings = self.settings.get();

        if !settings.auto_cleanup_enabled {
            return None;
        }

        let now = js_sys::Date::new_0();
        let current_date = Self::get_date_string(&now);
        let current_hour = now.get_hours() as u32;
        let target_hour = settings.cleanup_hour;

        // If we've already run today, next cleanup is tomorrow
        let next_date = if let Some(last_cleanup) = &settings.last_cleanup_date {
            if last_cleanup == &current_date {
                // Already ran today, next is tomorrow
                let tomorrow = js_sys::Date::new_0();
                tomorrow.set_date(tomorrow.get_date() + 1);
                Self::get_date_string(&tomorrow)
            } else if current_hour >= target_hour {
                // Haven't run today but past the target hour, next is tomorrow
                let tomorrow = js_sys::Date::new_0();
                tomorrow.set_date(tomorrow.get_date() + 1);
                Self::get_date_string(&tomorrow)
            } else {
                // Haven't run today and before target hour, next is today
                current_date
            }
        } else {
            // Never run before
            if current_hour >= target_hour {
                let tomorrow = js_sys::Date::new_0();
                tomorrow.set_date(tomorrow.get_date() + 1);
                Self::get_date_string(&tomorrow)
            } else {
                current_date
            }
        };

        Some(format!("{} at {:02}:00", next_date, target_hour))
    }
}

fn get_local_storage() -> Option<Storage> {
    window()?.local_storage().ok()?
}