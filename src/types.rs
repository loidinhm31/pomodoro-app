use serde::{Deserialize, Serialize};

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

impl SessionType {
    pub fn duration_minutes(&self) -> u32 {
        match self {
            SessionType::Work => 25,
            SessionType::ShortBreak => 5,
            SessionType::LongBreak => 15,
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

    pub fn next_session(&self, completed_sessions: u32) -> Self {
        match self {
            SessionType::Work => {
                if (completed_sessions + 1) % 4 == 0 {
                    SessionType::LongBreak
                } else {
                    SessionType::ShortBreak
                }
            },
            SessionType::ShortBreak | SessionType::LongBreak => SessionType::Work,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GreetArgs<'a> {
    pub name: &'a str,
}