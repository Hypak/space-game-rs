#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum HealthStatus {
    Alive,
    Dead,
    Invulnerable,
}

impl HealthStatus {
    pub fn kill_if_alive(status: Self) -> Self {
        if status == HealthStatus::Alive {
            HealthStatus::Dead
        } else {
            status
        }
    }
}
