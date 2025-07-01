use serde::{Deserialize, Serialize};
use web_sys::{window, Storage};
use crate::console_log;

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
                let just_completed_session = completed_work_sessions;

                // Check if it's time for a long break (takes priority)
                if just_completed_session % settings.sessions_before_long_break == 0 {
                    SessionType::LongBreak
                }
                // Check if it's time for a short break
                else if just_completed_session % settings.sessions_before_short_break == 0 {
                    SessionType::ShortBreak
                }
                // Continue with work if no break needed
                else {
                    SessionType::Work
                }
            }
            SessionType::ShortBreak | SessionType::LongBreak => {
                // After any break, always go back to work
                SessionType::Work
            }
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
    pub task_id: Option<String>,
    pub subtask_id: Option<String>,
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
    pub task_id: Option<String>,
    pub subtask_id: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub color: String, // hex color for UI theming
    pub created_at: String,
    pub completed: bool,
    pub estimated_pomodoros: Option<u32>,
    pub actual_pomodoros: u32,
    pub total_focus_time: u32, // in seconds
    pub order_index: u32, // for sorting
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubTask {
    pub id: String,
    pub task_id: String,
    pub name: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: String,
    pub estimated_pomodoros: Option<u32>,
    pub actual_pomodoros: u32,
    pub total_focus_time: u32, // in seconds
    pub order_index: u32, // for sorting
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStats {
    pub task: Task,
    pub subtasks: Vec<SubTask>,
    pub total_focus_time: u32,
    pub total_pomodoros: u32,
    pub completion_percentage: f64,
    pub estimated_vs_actual: Option<(u32, u32)>, // (estimated, actual) pomodoros
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTask {
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub estimated_pomodoros: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSubTask {
    pub task_id: String,
    pub name: String,
    pub description: Option<String>,
    pub estimated_pomodoros: Option<u32>,
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
        task_id: None,
        subtask_id: None,
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

// Task database operations
pub fn generate_task_id() -> String {
    format!("task_{}", js_sys::Date::now() as u64)
}

pub fn generate_subtask_id() -> String {
    format!("subtask_{}", js_sys::Date::now() as u64)
}

// Task Colors (predefined set)
pub const TASK_COLORS: &[&str] = &[
    "#EF4444", // red
    "#F97316", // orange
    "#EAB308", // yellow
    "#22C55E", // green
    "#06B6D4", // cyan
    "#3B82F6", // blue
    "#8B5CF6", // violet
    "#EC4899", // pink
    "#6B7280", // gray
    "#059669", // emerald
];

impl Task {
    pub fn get_random_color() -> String {
        let index = (js_sys::Math::random() * TASK_COLORS.len() as f64) as usize;
        TASK_COLORS[index].to_string()
    }

    pub fn calculate_completion_percentage(&self, subtasks: &[SubTask]) -> f64 {
        if subtasks.is_empty() {
            if self.completed { 100.0 } else { 0.0 }
        } else {
            let completed_subtasks = subtasks.iter().filter(|st| st.completed).count();
            (completed_subtasks as f64 / subtasks.len() as f64) * 100.0
        }
    }
}

// Database operations for tasks
pub async fn save_task_to_db(task: NewTask) -> Result<String, String> {
    let storage = get_local_storage().ok_or("Cannot access localStorage")?;
    let task_id = generate_task_id();
    let now = js_sys::Date::new_0().to_iso_string();

    let task_record = Task {
        id: task_id.clone(),
        name: task.name,
        description: task.description,
        color: task.color,
        estimated_pomodoros: task.estimated_pomodoros,
        created_at: now.into(),
        completed: false,
        actual_pomodoros: 0,
        total_focus_time: 0,
        order_index: get_next_task_order().await,
    };

    let mut tasks = get_all_tasks().await.unwrap_or_default();
    tasks.push(task_record);

    let all_tasks_json = serde_json::to_string(&tasks).map_err(|e| e.to_string())?;
    storage.set_item("pomodoro_tasks", &all_tasks_json)
        .map_err(|e| format!("{:?}", e))?;

    Ok(task_id)
}

pub async fn save_subtask_to_db(subtask: NewSubTask) -> Result<String, String> {
    let storage = get_local_storage().ok_or("Cannot access localStorage")?;
    let subtask_id = generate_subtask_id();
    let now = js_sys::Date::new_0().to_iso_string();

    let subtask_record = SubTask {
        id: subtask_id.clone(),
        task_id: subtask.task_id.clone(),
        name: subtask.name,
        description: subtask.description,
        estimated_pomodoros: subtask.estimated_pomodoros,
        created_at: now.into(),
        completed: false,
        actual_pomodoros: 0,
        total_focus_time: 0,
        order_index: get_next_subtask_order(&subtask.task_id).await,
    };

    let mut subtasks = get_all_subtasks().await.unwrap_or_default();
    subtasks.push(subtask_record);

    let all_subtasks_json = serde_json::to_string(&subtasks).map_err(|e| e.to_string())?;
    storage.set_item("pomodoro_subtasks", &all_subtasks_json)
        .map_err(|e| format!("{:?}", e))?;

    Ok(subtask_id)
}

pub async fn get_all_tasks() -> Result<Vec<Task>, String> {
    let storage = get_local_storage().ok_or("Cannot access localStorage")?;

    match storage.get_item("pomodoro_tasks").map_err(|e| format!("{:?}", e))? {
        Some(json) => {
            let mut tasks: Vec<Task> = serde_json::from_str(&json).map_err(|e| e.to_string())?;
            tasks.sort_by_key(|t| t.order_index);
            Ok(tasks)
        },
        None => Ok(Vec::new()),
    }
}

pub async fn get_all_subtasks() -> Result<Vec<SubTask>, String> {
    let storage = get_local_storage().ok_or("Cannot access localStorage")?;

    match storage.get_item("pomodoro_subtasks").map_err(|e| format!("{:?}", e))? {
        Some(json) => {
            let mut subtasks: Vec<SubTask> = serde_json::from_str(&json).map_err(|e| e.to_string())?;
            subtasks.sort_by_key(|st| st.order_index);
            Ok(subtasks)
        },
        None => Ok(Vec::new()),
    }
}

pub async fn get_subtasks_for_task(task_id: &str) -> Result<Vec<SubTask>, String> {
    let all_subtasks = get_all_subtasks().await?;
    Ok(all_subtasks.into_iter().filter(|st| st.task_id == task_id).collect())
}

pub async fn update_task_in_db(updated_task: Task) -> Result<(), String> {
    let storage = get_local_storage().ok_or("Cannot access localStorage")?;
    let mut tasks = get_all_tasks().await.unwrap_or_default();

    if let Some(index) = tasks.iter().position(|t| t.id == updated_task.id) {
        tasks[index] = updated_task;
        let all_tasks_json = serde_json::to_string(&tasks).map_err(|e| e.to_string())?;
        storage.set_item("pomodoro_tasks", &all_tasks_json)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    } else {
        Err("Task not found".to_string())
    }
}

pub async fn update_subtask_in_db(updated_subtask: SubTask) -> Result<(), String> {
    let storage = get_local_storage().ok_or("Cannot access localStorage")?;
    let mut subtasks = get_all_subtasks().await.unwrap_or_default();

    if let Some(index) = subtasks.iter().position(|st| st.id == updated_subtask.id) {
        subtasks[index] = updated_subtask;
        let all_subtasks_json = serde_json::to_string(&subtasks).map_err(|e| e.to_string())?;
        storage.set_item("pomodoro_subtasks", &all_subtasks_json)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    } else {
        Err("SubTask not found".to_string())
    }
}

pub async fn delete_task_from_db(task_id: String) -> Result<bool, String> {
    let storage = get_local_storage().ok_or("Cannot access localStorage")?;

    // Delete task
    let mut tasks = get_all_tasks().await.unwrap_or_default();
    let initial_len = tasks.len();
    tasks.retain(|t| t.id != task_id);

    // Delete associated subtasks
    let mut subtasks = get_all_subtasks().await.unwrap_or_default();
    subtasks.retain(|st| st.task_id != task_id);

    if tasks.len() < initial_len {
        let all_tasks_json = serde_json::to_string(&tasks).map_err(|e| e.to_string())?;
        storage.set_item("pomodoro_tasks", &all_tasks_json)
            .map_err(|e| format!("{:?}", e))?;

        let all_subtasks_json = serde_json::to_string(&subtasks).map_err(|e| e.to_string())?;
        storage.set_item("pomodoro_subtasks", &all_subtasks_json)
            .map_err(|e| format!("{:?}", e))?;

        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn delete_subtask_from_db(subtask_id: String) -> Result<bool, String> {
    let storage = get_local_storage().ok_or("Cannot access localStorage")?;
    let mut subtasks = get_all_subtasks().await.unwrap_or_default();
    let initial_len = subtasks.len();
    subtasks.retain(|st| st.id != subtask_id);

    if subtasks.len() < initial_len {
        let all_subtasks_json = serde_json::to_string(&subtasks).map_err(|e| e.to_string())?;
        storage.set_item("pomodoro_subtasks", &all_subtasks_json)
            .map_err(|e| format!("{:?}", e))?;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn get_task_stats() -> Result<Vec<TaskStats>, String> {
    let tasks = get_all_tasks().await?;
    let all_subtasks = get_all_subtasks().await?;

    let mut task_stats = Vec::new();

    for task in tasks {
        let subtasks: Vec<SubTask> = all_subtasks.iter()
            .filter(|st| st.task_id == task.id)
            .cloned()
            .collect();

        let total_focus_time = task.total_focus_time +
            subtasks.iter().map(|st| st.total_focus_time).sum::<u32>();

        let total_pomodoros = task.actual_pomodoros +
            subtasks.iter().map(|st| st.actual_pomodoros).sum::<u32>();

        let completion_percentage = task.calculate_completion_percentage(&subtasks);

        let estimated_vs_actual = if let Some(estimated) = task.estimated_pomodoros {
            Some((estimated, task.actual_pomodoros))
        } else {
            None
        };

        task_stats.push(TaskStats {
            task,
            subtasks,
            total_focus_time,
            total_pomodoros,
            completion_percentage,
            estimated_vs_actual,
        });
    }

    Ok(task_stats)
}

async fn get_next_task_order() -> u32 {
    let tasks = get_all_tasks().await.unwrap_or_default();
    tasks.iter().map(|t| t.order_index).max().unwrap_or(0) + 1
}

async fn get_next_subtask_order(task_id: &str) -> u32 {
    let subtasks = get_subtasks_for_task(task_id).await.unwrap_or_default();
    subtasks.iter().map(|st| st.order_index).max().unwrap_or(0) + 1
}

// Update work session completion to track task time
pub async fn complete_work_session_with_task(
    session: NewSession,
    focus_time_seconds: u32,
) -> Result<String, String> {
    // Save the session first
    let session_id = save_session_to_db(session.clone()).await?;

    // Update task/subtask time tracking if this was a work session
    if session.session_type == "Work" && session.completed && focus_time_seconds > 0 {
        if let Some(subtask_id) = session.subtask_id {
            // Update subtask with actual time
            let mut subtasks = get_all_subtasks().await?;
            if let Some(subtask) = subtasks.iter_mut().find(|st| st.id == subtask_id) {
                subtask.total_focus_time += focus_time_seconds;
                // Convert actual time to "pomodoro equivalents" (25 min each) for estimation comparison
                let pomodoro_equivalent = (focus_time_seconds + 1499) / 1500; // Round up to nearest 25-min block
                subtask.actual_pomodoros += pomodoro_equivalent;
                update_subtask_in_db(subtask.clone()).await?;

                console_log!("Updated subtask {} with {} seconds ({} pomodoro-equivalents)", 
                           subtask_id, focus_time_seconds, pomodoro_equivalent);
            }
        } else if let Some(task_id) = session.task_id {
            // Update task directly with actual time
            let mut tasks = get_all_tasks().await?;
            if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
                task.total_focus_time += focus_time_seconds;
                // Convert actual time to "pomodoro equivalents" (25 min each) for estimation comparison
                let pomodoro_equivalent = (focus_time_seconds + 1499) / 1500; // Round up to nearest 25-min block
                task.actual_pomodoros += pomodoro_equivalent;
                update_task_in_db(task.clone()).await?;

                console_log!("Updated task {} with {} seconds ({} pomodoro-equivalents)", 
                           task_id, focus_time_seconds, pomodoro_equivalent);
            }
        }
    }

    Ok(session_id)
}

// Helper function to get task name by ID
pub async fn get_task_name_by_id(task_id: &str) -> Result<Option<String>, String> {
    let tasks = get_all_tasks().await?;
    Ok(tasks.iter().find(|t| t.id == task_id).map(|t| t.name.clone()))
}

// Helper function to get subtask name by ID
pub async fn get_subtask_name_by_id(subtask_id: &str) -> Result<Option<String>, String> {
    let subtasks = get_all_subtasks().await?;
    Ok(subtasks.iter().find(|st| st.id == subtask_id).map(|st| st.name.clone()))
}

// Helper function to get full task path (Task → Subtask)
pub async fn get_task_path_by_ids(task_id: Option<&str>, subtask_id: Option<&str>) -> Result<Option<String>, String> {
    if let Some(st_id) = subtask_id {
        let subtasks = get_all_subtasks().await?;
        if let Some(subtask) = subtasks.iter().find(|st| st.id == st_id) {
            let tasks = get_all_tasks().await?;
            if let Some(task) = tasks.iter().find(|t| t.id == subtask.task_id) {
                return Ok(Some(format!("{} → {}", task.name, subtask.name)));
            } else {
                return Ok(Some(format!("Unknown Task → {}", subtask.name)));
            }
        }
    } else if let Some(t_id) = task_id {
        let tasks = get_all_tasks().await?;
        if let Some(task) = tasks.iter().find(|t| t.id == t_id) {
            return Ok(Some(task.name.clone()));
        }
    }
    Ok(None)
}