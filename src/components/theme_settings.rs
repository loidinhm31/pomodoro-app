use crate::theme::{ThemeController, ThemeType};
use leptos::prelude::*;

#[component]
pub fn ThemeSettings(theme_controller: ThemeController) -> impl IntoView {
    view! {
        <div class="theme-settings space-y-6">
            <h4 class="text-lg font-medium text-gray-700 dark:text-gray-300 mb-4">
                "Theme Settings"
            </h4>

            // Current Theme Display
            <div class="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg border">
                <div class="flex items-center justify-between mb-3">
                    <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
                        "Current Theme"
                    </span>
                    <span class="text-lg font-semibold text-gray-800 dark:text-white">
                        {
                            let theme_controller_display = theme_controller.clone();
                            move || theme_controller_display.get_current_theme().display_name()
                        }
                    </span>
                </div>
                <p class="text-xs text-gray-600 dark:text-gray-400">
                    {
                        let theme_controller_desc = theme_controller.clone();
                        move || theme_controller_desc.get_current_theme().description()
                    }
                </p>
            </div>

            // Theme Selection - Optimized for 2 themes
            <div class="space-y-4">
                <h5 class="text-md font-medium text-gray-700 dark:text-gray-300">
                    "Choose Your Theme"
                </h5>
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    {ThemeType::all_themes().into_iter().map(|theme| {
                        let theme_controller_button = theme_controller.clone();
                        let theme_for_comparison = theme;
                        let theme_for_colors = theme;
                        let theme_for_text = theme;
                        let theme_for_click = theme;
                        
                        view! {
                            <button
                                class={
                                    let theme_controller_class = theme_controller_button.clone();
                                    move || format!(
                                        "relative p-6 border-2 rounded-xl transition-all duration-300 hover:shadow-lg theme-button {}",
                                        if theme_controller_class.get_current_theme() == theme_for_comparison {
                                            "border-blue-500 bg-blue-50 dark:bg-blue-900/20 selected"
                                        } else {
                                            "border-gray-200 dark:border-gray-600 bg-white dark:bg-gray-800 hover:border-gray-300 dark:hover:border-gray-500"
                                        }
                                    )
                                }
                                on:click={
                                    let theme_controller_click = theme_controller_button.clone();
                                    move |_| theme_controller_click.set_theme(theme_for_click)
                                }
                            >
                                // Theme Preview Colors - Larger for better visibility
                                <div class="flex space-x-3 mb-4 justify-center">
                                    <div 
                                        class="w-8 h-8 rounded-full border-2 border-gray-300 shadow-sm color-preview"
                                        style=format!("background-color: {}", theme_for_colors.work_color())
                                        title="Work session color"
                                    ></div>
                                    <div 
                                        class="w-8 h-8 rounded-full border-2 border-gray-300 shadow-sm color-preview"
                                        style=format!("background-color: {}", theme_for_colors.short_break_color())
                                        title="Short break color"
                                    ></div>
                                    <div 
                                        class="w-8 h-8 rounded-full border-2 border-gray-300 shadow-sm color-preview"
                                        style=format!("background-color: {}", theme_for_colors.long_break_color())
                                        title="Long break color"
                                    ></div>
                                </div>

                                <div class="text-center">
                                    <h6 class="font-semibold text-gray-800 dark:text-white text-base mb-2">
                                        {theme_for_text.display_name()}
                                    </h6>
                                    <p class="text-sm text-gray-600 dark:text-gray-400">
                                        {theme_for_text.description()}
                                    </p>
                                </div>

                                // Selected Indicator
                                {
                                    let theme_controller_indicator = theme_controller_button.clone();
                                    move || {
                                        if theme_controller_indicator.get_current_theme() == theme {
                                            view! {
                                                <div class="absolute top-3 right-3 w-6 h-6 bg-blue-500 text-white rounded-full flex items-center justify-center shadow-lg">
                                                    <span class="text-sm font-bold">"✓"</span>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }
                                    }
                                }
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            // Theme Automation Options
            <div class="space-y-4 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
                <h5 class="text-md font-medium text-gray-700 dark:text-gray-300">
                    "Automatic Theme Switching"
                </h5>

                // Auto Dark Mode (now switches to Nordic)
                <div class="flex items-center justify-between">
                    <div>
                        <span class="text-sm font-medium text-gray-600 dark:text-gray-400">
                            "Auto Evening Mode"
                        </span>
                        <p class="text-xs text-gray-500 dark:text-gray-400">
                            "Switch to Nordic theme from 6 PM to 6 AM"
                        </p>
                    </div>
                    <input
                        type="checkbox"
                        class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                        checked={
                            let theme_controller_auto = theme_controller.clone();
                            move || theme_controller_auto.theme_settings.get().auto_dark_mode
                        }
                        on:change={
                            let theme_controller_auto_change = theme_controller.clone();
                            move |_| theme_controller_auto_change.toggle_auto_dark_mode()
                        }
                    />
                </div>

                // System Theme Sync (now switches to Nordic for dark preference)
                <div class="flex items-center justify-between">
                    <div>
                        <span class="text-sm font-medium text-gray-600 dark:text-gray-400">
                            "Follow System Theme"
                        </span>
                        <p class="text-xs text-gray-500 dark:text-gray-400">
                            "Use Nordic theme when system prefers dark mode"
                        </p>
                    </div>
                    <input
                        type="checkbox"
                        class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                        checked={
                            let theme_controller_sync = theme_controller.clone();
                            move || theme_controller_sync.theme_settings.get().system_theme_sync
                        }
                        on:change={
                            let theme_controller_sync_change = theme_controller.clone();
                            move |_| theme_controller_sync_change.toggle_system_sync()
                        }
                    />
                </div>
            </div>

            // Theme Preview Section - Enhanced for 2-theme system
            <div class="p-4 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-600">
                <h5 class="text-md font-medium text-gray-700 dark:text-gray-300 mb-3">
                    "Live Preview"
                </h5>
                
                <div class="space-y-4">
                    // Session Type Previews
                    <div class="grid grid-cols-3 gap-2">
                        <div 
                            class="p-3 rounded-lg text-white text-center font-medium text-sm shadow-sm"
                            style={
                                let theme_controller_work = theme_controller.clone();
                                move || format!("background-color: {}", theme_controller_work.get_current_theme().work_color())
                            }
                        >
                            "Work"
                        </div>
                        <div 
                            class="p-3 rounded-lg text-white text-center font-medium text-sm shadow-sm"
                            style={
                                let theme_controller_short = theme_controller.clone();
                                move || format!("background-color: {}", theme_controller_short.get_current_theme().short_break_color())
                            }
                        >
                            "Short Break"
                        </div>
                        <div 
                            class="p-3 rounded-lg text-white text-center font-medium text-sm shadow-sm"
                            style={
                                let theme_controller_long = theme_controller.clone();
                                move || format!("background-color: {}", theme_controller_long.get_current_theme().long_break_color())
                            }
                        >
                            "Long Break"
                        </div>
                    </div>

                    // Sample Timer Display
                    <div class="text-center p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
                        <div class="text-3xl font-mono font-bold text-gray-800 dark:text-white mb-3">
                            "25:00"
                        </div>
                        <div class="w-full bg-gray-200 dark:bg-gray-600 rounded-full h-3">
                            <div 
                                class="h-3 rounded-full w-1/3 transition-all"
                                style={
                                    let theme_controller_progress = theme_controller.clone();
                                    move || format!("background-color: {}", theme_controller_progress.get_current_theme().work_color())
                                }
                            ></div>
                        </div>
                        <p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
                            "Sample timer appearance"
                        </p>
                    </div>
                </div>
            </div>

            // Theme Information - Updated for 2-theme system
            <div class="p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg border border-blue-200 dark:border-blue-800">
                <h5 class="text-md font-medium text-blue-800 dark:text-blue-200 mb-2">
                    "About Themes"
                </h5>
                <div class="text-sm text-blue-700 dark:text-blue-300 space-y-2">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                            <h6 class="font-semibold mb-1">"🎨 Classic Theme"</h6>
                            <p class="text-xs">
                                "Traditional Pomodoro colors with bright, energizing tones. Perfect for daytime focus sessions."
                            </p>
                        </div>
                        <div>
                            <h6 class="font-semibold mb-1">"❄️ Nordic Theme"</h6>
                            <p class="text-xs">
                                "Cool, calming Scandinavian palette ideal for evening work and extended sessions."
                            </p>
                        </div>
                    </div>
                    <div class="pt-2 border-t border-blue-200 dark:border-blue-700">
                        <p class="text-xs">
                            "💡 Tip: Use automatic switching to have Classic during the day and Nordic in the evening for optimal eye comfort."
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}