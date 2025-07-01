use serde::{Deserialize, Serialize};
use web_sys::{window, Storage};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimerState {
    Stopped,
    Running,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SessionType {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerSettings {
    pub work_duration_minutes: u32,
    pub short_break_duration_minutes: u32,
    pub long_break_duration_minutes: u32,
    pub sessions_before_short_break: u32,
    pub sessions_before_long_break: u32,
    pub auto_start_breaks: bool,
    pub auto_start_work: bool,
}

impl Default for TimerSettings {
    fn default() -> Self {
        Self {
            work_duration_minutes: 25,
            short_break_duration_minutes: 5,
            long_break_duration_minutes: 15,
            sessions_before_short_break: 1, // Short break after every work session
            sessions_before_long_break: 4,  // Long break after every 4 work sessions
            auto_start_breaks: false,
            auto_start_work: false,
        }
    }
}

impl TimerSettings {
    pub fn save_to_storage(&self) {
        if let Ok(settings_json) = serde_json::to_string(&self) {
            if let Some(storage) = get_local_storage() {
                let _ = storage.set_item("pomodoro_timer_settings", &settings_json);
            }
        }
    }

    pub fn load_from_storage() -> Self {
        if let Some(storage) = get_local_storage() {
            if let Ok(Some(settings_json)) = storage.get_item("pomodoro_timer_settings") {
                if let Ok(settings) = serde_json::from_str::<TimerSettings>(&settings_json) {
                    return settings;
                }
            }
        }
        Self::default()
    }
}

impl SessionType {
    pub fn duration_minutes(&self, settings: &TimerSettings) -> u32 {
        match self {
            SessionType::Work => settings.work_duration_minutes,
            SessionType::ShortBreak => settings.short_break_duration_minutes,
            SessionType::LongBreak => settings.long_break_duration_minutes,
        }
    }

    pub fn color_class(&self) -> &'static str {
        match self {
            SessionType::Work => "bg-red-500",
            SessionType::ShortBreak => "bg-green-500",
            SessionType::LongBreak => "bg-blue-500",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            SessionType::Work => "Work",
            SessionType::ShortBreak => "Short Break",
            SessionType::LongBreak => "Long Break",
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            SessionType::Work => "Work".to_string(),
            SessionType::ShortBreak => "ShortBreak".to_string(),
            SessionType::LongBreak => "LongBreak".to_string(),
        }
    }

    pub fn next_session(&self, completed_work_sessions: u32, settings: &TimerSettings) -> Self {
        match self {
            SessionType::Work => {
                // Check if it's time for a long break
                if (completed_work_sessions + 1) % settings.sessions_before_long_break == 0 {
                    SessionType::LongBreak
                } else if (completed_work_sessions + 1) % settings.sessions_before_short_break == 0 {
                    SessionType::ShortBreak
                } else {
                    // This shouldn't happen with normal settings, but fallback to short break
                    SessionType::ShortBreak
                }
            }
            SessionType::ShortBreak | SessionType::LongBreak => SessionType::Work,
        }
    }

    pub fn display_with_duration(&self, settings: &TimerSettings) -> String {
        format!("{} ({}m)", self.name(), self.duration_minutes(settings))
    }
}

#[derive(Serialize, Deserialize)]
pub struct GreetArgs<'a> {
    pub name: &'a str,
}

// Database-related types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub session_type: String,
    pub planned_duration: u32,
    pub actual_duration: u32,
    pub start_time: String,
    pub end_time: String,
    pub completed: bool,
    pub created_at: String,
    pub video_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSession {
    pub session_type: String,
    pub planned_duration: u32,
    pub actual_duration: u32,
    pub start_time: String,
    pub end_time: String,
    pub completed: bool,
    pub video_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CameraState {
    Inactive,
    Initializing,
    Recording,
    Stopped,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraSettings {
    pub enabled: bool,
    pub only_during_breaks: bool,
    pub video_quality: String, // "low", "medium", "high"
    pub max_duration_minutes: u32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            only_during_breaks: true,
            video_quality: "medium".to_string(),
            max_duration_minutes: 30,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStats {
    pub total_sessions: u32,
    pub completed_sessions: u32,
    pub total_focus_time: u32,
    pub total_break_time: u32,
    pub work_sessions: u32,
    pub short_break_sessions: u32,
    pub long_break_sessions: u32,
    pub average_session_duration: f64,
    pub completion_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionQuery {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub session_type: Option<String>,
    pub completed: Option<bool>,
}

fn get_local_storage() -> Option<Storage> {
    window()?.local_storage().ok()?
}

pub fn generate_session_id() -> String {
    format!("session_{}", js_sys::Date::now() as u64)
}

pub async fn save_session_to_db(session: NewSession) -> Result<String, String> {
    let storage = get_local_storage().ok_or("Cannot access localStorage")?;
    let session_id = generate_session_id();
    let now = js_sys::Date::new_0().to_iso_string();

    let session_record = Session {
        id: session_id.clone(),
        session_type: session.session_type,
        planned_duration: session.planned_duration,
        actual_duration: session.actual_duration,
        start_time: session.start_time,
        end_time: session.end_time,
        completed: session.completed,
        created_at: now.into(),
        video_path: session.video_path,
    };

    // Get existing sessions
    let mut sessions = get_all_sessions().await.unwrap_or_default();
    sessions.push(session_record);

    // Save updated sessions list
    let all_sessions_json = serde_json::to_string(&sessions).map_err(|e| e.to_string())?;
    storage
        .set_item("pomodoro_sessions", &all_sessions_json)
        .map_err(|e| format!("{:?}", e))?;

    Ok(session_id)
}

pub async fn get_sessions_from_db(
    limit: Option<u32>,
    session_type: Option<String>,
) -> Result<Vec<Session>, String> {
    let mut sessions = get_all_sessions().await.unwrap_or_default();

    // Filter by session type if provided
    if let Some(filter_type) = session_type {
        sessions.retain(|s| s.session_type == filter_type);
    }

    // Sort by created_at descending (newest first)
    sessions.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    // Apply limit
    if let Some(limit) = limit {
        sessions.truncate(limit as usize);
    }

    Ok(sessions)
}

async fn get_all_sessions() -> Result<Vec<Session>, String> {
    let storage = get_local_storage().ok_or("Cannot access localStorage")?;

    match storage
        .get_item("pomodoro_sessions")
        .map_err(|e| format!("{:?}", e))?
    {
        Some(json) => serde_json::from_str(&json).map_err(|e| e.to_string()),
        None => Ok(Vec::new()),
    }
}

pub async fn get_session_stats_from_db() -> Result<SessionStats, String> {
    let sessions = get_all_sessions().await.unwrap_or_default();

    let total_sessions = sessions.len() as u32;
    let completed_sessions = sessions.iter().filter(|s| s.completed).count() as u32;

    let work_sessions = sessions
        .iter()
        .filter(|s| s.session_type == "Work" && s.completed)
        .count() as u32;

    let short_break_sessions = sessions
        .iter()
        .filter(|s| s.session_type == "ShortBreak" && s.completed)
        .count() as u32;

    let long_break_sessions = sessions
        .iter()
        .filter(|s| s.session_type == "LongBreak" && s.completed)
        .count() as u32;

    let total_focus_time = sessions
        .iter()
        .filter(|s| s.session_type == "Work" && s.completed)
        .map(|s| s.actual_duration)
        .sum::<u32>();

    let total_break_time = sessions
        .iter()
        .filter(|s| {
            (s.session_type == "ShortBreak" || s.session_type == "LongBreak") && s.completed
        })
        .map(|s| s.actual_duration)
        .sum::<u32>();

    let average_session_duration = if completed_sessions > 0 {
        sessions
            .iter()
            .filter(|s| s.completed)
            .map(|s| s.actual_duration as f64)
            .sum::<f64>()
            / completed_sessions as f64
    } else {
        0.0
    };

    let completion_rate = if total_sessions > 0 {
        (completed_sessions as f64 / total_sessions as f64) * 100.0
    } else {
        0.0
    };

    Ok(SessionStats {
        total_sessions,
        completed_sessions,
        total_focus_time,
        total_break_time,
        work_sessions,
        short_break_sessions,
        long_break_sessions,
        average_session_duration,
        completion_rate,
    })
}

pub async fn delete_session_from_db(session_id: String) -> Result<bool, String> {
    let storage = get_local_storage().ok_or("Cannot access localStorage")?;
    let mut sessions = get_all_sessions().await.unwrap_or_default();

    let initial_len = sessions.len();
    sessions.retain(|s| s.id != session_id);

    if sessions.len() < initial_len {
        let all_sessions_json = serde_json::to_string(&sessions).map_err(|e| e.to_string())?;
        storage
            .set_item("pomodoro_sessions", &all_sessions_json)
            .map_err(|e| format!("{:?}", e))?;
        Ok(true)
    } else {
        Ok(false)
    }
}