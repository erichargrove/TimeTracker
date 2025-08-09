use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod storage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: String,
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub description: String,
}

impl TimeEntry {
    pub fn new(description: String) -> Self {
        TimeEntry {
            id: Uuid::new_v4().to_string(),
            start: Utc::now(),
            end: None,
            description,
        }
    }

    pub fn stop(&mut self) {
        self.end = Some(Utc::now());
    }

    pub fn is_running(&self) -> bool {
        self.end.is_none()
    }

    pub fn duration(&self) -> chrono::Duration {
        match self.end {
            Some(end) => end - self.start,
            None => Utc::now() - self.start,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeTracker {
    pub entries: Vec<TimeEntry>,
}

impl TimeTracker {
    pub fn new() -> Self {
        TimeTracker {
            entries: Vec::new(),
        }
    }

    pub fn start_timer(&mut self, description: String) {
        let entry = TimeEntry::new(description);
        self.entries.push(entry);
    }

    pub fn get_running_timer(&self) -> Option<&TimeEntry> {
        self.entries.iter().find(|entry| entry.is_running())
    }

    pub fn stop_current_timer(&mut self) -> bool {
        for entry in &mut self.entries {
            if entry.is_running() {
                entry.stop();
                return true;
            }
        }
        false
    }
}
