use crate::console_log;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use web_sys::{window, Storage};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum ThemeType {
    Classic,
    Nordic,
}

impl ThemeType {
    pub fn to_string(&self) -> String {
        match self {
            ThemeType::Classic => "classic".to_string(),
            ThemeType::Nordic => "nordic".to_string(),
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ThemeType::Classic => "Classic",
            ThemeType::Nordic => "Nordic",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ThemeType::Classic => "Traditional Pomodoro colors",
            ThemeType::Nordic => "Cool Scandinavian-inspired palette",
        }
    }

    pub fn css_classes(&self) -> String {
        match self {
            ThemeType::Classic => "theme-classic".to_string(),
            ThemeType::Nordic => "theme-nordic dark".to_string(),
        }
    }

    pub fn work_color(&self) -> &'static str {
        match self {
            ThemeType::Classic => "#EF4444", // red
            ThemeType::Nordic => "#5E81AC",   // nordic blue
        }
    }

    pub fn short_break_color(&self) -> &'static str {
        match self {
            ThemeType::Classic => "#22C55E", // green
            ThemeType::Nordic => "#88C0D0",   // nordic light blue
        }
    }

    pub fn long_break_color(&self) -> &'static str {
        match self {
            ThemeType::Classic => "#3B82F6", // blue
            ThemeType::Nordic => "#81A1C1",   // nordic frost blue
        }
    }

    pub fn all_themes() -> Vec<ThemeType> {
        vec![
            ThemeType::Classic,
            ThemeType::Nordic,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeSettings {
    pub current_theme: ThemeType,
    pub auto_dark_mode: bool,
    pub system_theme_sync: bool,
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            current_theme: ThemeType::Classic,
            auto_dark_mode: false,
            system_theme_sync: false,
        }
    }
}

impl ThemeSettings {
    pub fn save_to_storage(&self) {
        if let Ok(settings_json) = serde_json::to_string(&self) {
            if let Some(storage) = get_local_storage() {
                let _ = storage.set_item("pomodoro_theme_settings", &settings_json);
                console_log!("Theme settings saved");
            }
        }
    }

    pub fn load_from_storage() -> Self {
        if let Some(storage) = get_local_storage() {
            if let Ok(Some(settings_json)) = storage.get_item("pomodoro_theme_settings") {
                if let Ok(settings) = serde_json::from_str::<ThemeSettings>(&settings_json) {
                    console_log!("Theme settings loaded");
                    return settings;
                }
            }
        }
        console_log!("Using default theme settings");
        Self::default()
    }
}

#[derive(Clone)]
pub struct ThemeController {
    pub theme_settings: RwSignal<ThemeSettings>,
}

impl ThemeController {
    pub fn new() -> Self {
        let settings = ThemeSettings::load_from_storage();

        let controller = Self {
            theme_settings: RwSignal::new(settings),
        };

        // Apply initial theme
        controller.apply_theme();

        controller
    }

    pub fn set_theme(&self, theme: ThemeType) {
        let mut settings = self.theme_settings.get();
        settings.current_theme = theme;
        self.theme_settings.set(settings.clone());
        settings.save_to_storage();
        self.apply_theme();
        console_log!("Theme changed to: {:?}", theme);
    }

    pub fn toggle_auto_dark_mode(&self) {
        let mut settings = self.theme_settings.get();
        settings.auto_dark_mode = !settings.auto_dark_mode;
        self.theme_settings.set(settings.clone());
        settings.save_to_storage();

        if settings.auto_dark_mode {
            self.check_and_apply_auto_dark_mode();
        } else {
            self.apply_theme();
        }

        console_log!("Auto dark mode: {}", settings.auto_dark_mode);
    }

    pub fn toggle_system_sync(&self) {
        let mut settings = self.theme_settings.get();
        settings.system_theme_sync = !settings.system_theme_sync;
        self.theme_settings.set(settings.clone());
        settings.save_to_storage();

        if settings.system_theme_sync {
            self.sync_with_system_theme();
        }

        console_log!("System theme sync: {}", settings.system_theme_sync);
    }

    fn apply_theme(&self) {
        if let Some(document) = window().and_then(|w| w.document()) {
            if let Some(body) = document.body() {
                let settings = self.theme_settings.get();
                let theme_classes = settings.current_theme.css_classes();

                // Remove all theme classes first
                for theme in ThemeType::all_themes() {
                    let classes = theme.css_classes();
                    for class in classes.split_whitespace() {
                        let _ = body.class_list().remove_1(class);
                    }
                }

                // Add new theme classes
                for class in theme_classes.split_whitespace() {
                    let _ = body.class_list().add_1(class);
                }

                // Update CSS custom properties for dynamic theming
                if let Some(html_element) = document.document_element() {
                    // Use setAttribute to set CSS custom properties
                    let theme = &settings.current_theme;
                    let style_string = format!(
                        "--work-color: {}; --short-break-color: {}; --long-break-color: {};",
                        theme.work_color(),
                        theme.short_break_color(),
                        theme.long_break_color()
                    );
                    let _ = html_element.set_attribute("style", &style_string);
                }
            }
        }
    }

    fn check_and_apply_auto_dark_mode(&self) {
        let settings = self.theme_settings.get();
        if !settings.auto_dark_mode {
            return;
        }

        // Check time of day for auto dark mode
        let date = js_sys::Date::new_0();
        let hour = date.get_hours();

        // Auto dark mode between 6 PM and 6 AM switches to Nordic theme
        let should_be_dark = hour >= 18 || hour < 6;

        if should_be_dark && settings.current_theme != ThemeType::Nordic {
            self.set_theme(ThemeType::Nordic);
        } else if !should_be_dark && settings.current_theme == ThemeType::Nordic {
            self.set_theme(ThemeType::Classic);
        }
    }

    fn sync_with_system_theme(&self) {
        // Check if system prefers dark mode and switch to Nordic theme
        if let Some(window) = window() {
            match window.match_media("(prefers-color-scheme: dark)") {
                Ok(media_query_result) => {
                    if let Some(media_query_list) = media_query_result {
                        if media_query_list.matches() {
                            self.set_theme(ThemeType::Nordic);
                            console_log!("System theme sync: switched to Nordic (dark mode detected)");
                        } else {
                            self.set_theme(ThemeType::Classic);
                            console_log!("System theme sync: switched to Classic (light mode detected)");
                        }
                        return;
                    }
                }
                Err(e) => {
                    console_log!("Media query not supported: {:?}", e);
                }
            }
        }

        // Fallback: switch to Nordic theme if media queries aren't supported
        console_log!("System theme sync - fallback to Nordic theme");
        self.set_theme(ThemeType::Nordic);
    }

    pub fn get_current_theme(&self) -> ThemeType {
        self.theme_settings.get().current_theme
    }
}

fn get_local_storage() -> Option<Storage> {
    window()?.local_storage().ok()?
}