use serde::Serialize;

#[derive(Serialize)]
pub struct HealthStatus {
    pub healthy: bool,
}
