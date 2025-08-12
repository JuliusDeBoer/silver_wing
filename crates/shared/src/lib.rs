use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct HealthStatus {
    pub healthy: bool,
    pub last_push: Option<DateTime<Local>>,
    pub last_clean: Option<DateTime<Local>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub body: String,
    pub date: DateTime<Local>,
}
