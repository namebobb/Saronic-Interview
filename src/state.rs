use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "lowercase")]
pub enum VesselState {
    Idle {
        location: usize,
        cargo: Vec<Cargo>,
    },
    Transit {
        from: usize,
        to: usize,
        arrives_at_tick: u64,
        cargo: Vec<Cargo>,
    },
    Docked {
        location: usize,
        cargo: Vec<Cargo>,
    },
}

#[derive(Clone, Serialize)]
pub struct WorldState {
    pub tick: u64,
    pub vessels: std::collections::HashMap<String, VesselState>,
    pub cargo: Vec<DockCargo>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Cargo {
    pub destination: usize,
    pub quantity: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DockCargo {
    pub location: usize,
    pub destination: usize,
    pub quantity: u64,
}
