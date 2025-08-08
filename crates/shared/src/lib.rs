use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HealthStatus {
    pub healthy: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Entry {
    pub id: String,
    pub body: String,
}
