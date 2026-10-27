use crate::state::{Cargo, DockCargo, VesselState, WorldState};

pub struct Engine {
    pub committed: Option<WorldState>,
    pub planned: Option<WorldState>,
    pub history: Vec<WorldState>,
    pub travel_times: Vec<Vec<u64>>,
    pub dock_throughput: Vec<u64>,
    pub dock_operations: Vec<u64>,
}

impl Engine {
    pub fn get_state(&self) -> Result<&WorldState, String> {
        match &self.planned {
            Some(state) => Ok(state),
            None => Err("Not initialized".into()),
        }
    }

    pub fn tick(&mut self, cargo_manifest: Vec<DockCargo>) -> Result<&WorldState, String> {
        let mut world = match &self.planned {
            Some(state) => state.clone(),
            None => return Err("Not initialized".into()),
        };
        let n = self.travel_times.len();
        for entry in &cargo_manifest {
            if entry.location >= n || entry.destination >= n || entry.quantity == 0 {
                return Err("Invalid manifest values".into());
            }
        }
        world.tick = match world.tick.checked_add(1) {
            Some(tick) => tick,
            None => return Err("Tick is too large".into()),
        };

        for vessel in world.vessels.values_mut() {
            match vessel {
                VesselState::Transit {
                    to,
                    arrives_at_tick,
                    cargo,
                    ..
                } => {
                    if *arrives_at_tick <= world.tick {
                        *vessel = VesselState::Idle {
                            location: *to,
                            cargo: cargo.clone(),
                        }
                    }
                }
                _ => {
                    continue;
                }
            }
        }
        for incoming in cargo_manifest {
            let existing = world.cargo.iter_mut().find(|entry| {
                entry.location == incoming.location && entry.destination == incoming.destination
            });

            if let Some(entry) = existing {
                entry.quantity = match entry.quantity.checked_add(incoming.quantity) {
                    Some(quantity) => quantity,
                    None => return Err("Cargo quantity is too large".into()),
                };
            } else {
                world.cargo.push(incoming);
            }
        }
        self.committed = Some(world.clone());
        self.history.push(world.clone());
        self.planned = Some(world);
        self.dock_operations.fill(0);
        self.get_state()
    }

    pub fn set_destination(
        &mut self,
        vessel_id: String,
        destination: usize,
    ) -> Result<&WorldState, String> {
        let world = match &self.committed {
            Some(world) => world,
            None => return Err("Not initialized".into()),
        };
        let from = match world.vessels.get(&vessel_id) {
            Some(VesselState::Idle { location, .. }) => *location,
            Some(_) => return Err(format!("vessel {} is not idle", vessel_id)),
            None => return Err(format!("Cannot find vessel {}", vessel_id)),
        };
        if destination >= self.travel_times.len() {
            return Err("invalid destination".into());
        };
        if destination == from {
            return Err("already at destination".into());
        };
        let row = match self.travel_times.get(from) {
            Some(row) => row,
            None => return Err("Invalid starting location".into()),
        };

        let duration = match row.get(destination) {
            Some(duration) => *duration,
            None => return Err("Invalid destination".into()),
        };
        let arrives_at_tick = match world.tick.checked_add(duration) {
            Some(tick) => tick,
            None => return Err("Arrival tick is too large".into()),
        };
        let planned = match self.planned.as_mut() {
            Some(planned) => planned,
            None => return Err("Not initialized".into()),
        };
        let cargo = match planned.vessels.get(&vessel_id) {
            Some(VesselState::Idle { cargo, .. }) => cargo.clone(),
            Some(VesselState::Transit { cargo, .. }) => cargo.clone(),
            Some(VesselState::Docked { cargo, .. }) => cargo.clone(),
            None => return Err(format!("Cannot find vessel {}", vessel_id)),
        };
        planned.vessels.insert(
            vessel_id,
            VesselState::Transit {
                from,
                to: destination,
                arrives_at_tick,
                cargo,
            },
        );
        self.get_state()
    }

    pub fn get_state_at(&self, tick: u64) -> Result<&WorldState, String> {
        let current_tick = match &self.committed {
            Some(state) => state.tick,
            None => return Err("Not initialized".into()),
        };
        if tick > current_tick {
            return Err(format!(
                "tick {} is in the future (current tick: {})",
                tick, current_tick
            ));
        }
        let history_index = match usize::try_from(tick) {
            Ok(index) => index,
            Err(_) => return Err(format!("Can't find world at tick {}", tick)),
        };
        match self.history.get(history_index) {
            Some(world) => Ok(world),
            None => Err(format!("Can't find world at tick {}", tick)),
        }
    }

    pub fn dock(&mut self, vessel_id: String) -> Result<&WorldState, String> {
        self.change_docking(vessel_id, true)
    }

    pub fn undock(&mut self, vessel_id: String) -> Result<&WorldState, String> {
        self.change_docking(vessel_id, false)
    }

    fn change_docking(&mut self, vessel_id: String, docking: bool) -> Result<&WorldState, String> {
        let committed = match &self.committed {
            Some(world) => world,
            None => return Err("Not initialized".into()),
        };
        let location = match committed.vessels.get(&vessel_id) {
            Some(VesselState::Idle { location, .. }) if docking => *location,
            Some(VesselState::Docked { location, .. }) if !docking => *location,
            Some(_) => {
                return Err(format!(
                    "vessel {} is not {}",
                    vessel_id,
                    if docking { "idle" } else { "docked" }
                ));
            }
            None => return Err(format!("Cannot find vessel {}", vessel_id)),
        };
        let planned = match &mut self.planned {
            Some(world) => world,
            None => return Err("Not initialized".into()),
        };
        let cargo = match planned.vessels.get(&vessel_id) {
            Some(vessel) => vessel_cargo(vessel).clone(),
            None => return Err(format!("Cannot find vessel {}", vessel_id)),
        };
        let vessel = if docking {
            VesselState::Docked { location, cargo }
        } else {
            VesselState::Idle { location, cargo }
        };
        planned.vessels.insert(vessel_id, vessel);
        self.get_state()
    }

    pub fn load(&mut self, vessel_id: String, destination: usize) -> Result<&WorldState, String> {
        self.transfer(vessel_id, Some(destination), None)
    }

    pub fn unload(&mut self, vessel_id: String, destination: usize) -> Result<&WorldState, String> {
        self.transfer(vessel_id, None, Some(destination))
    }

    pub fn swap(
        &mut self,
        vessel_id: String,
        load_destination: usize,
        unload_destination: usize,
    ) -> Result<&WorldState, String> {
        self.transfer(vessel_id, Some(load_destination), Some(unload_destination))
    }

    // Stage transfers on a copy so an error cannot apply half a swap.
    fn transfer(
        &mut self,
        vessel_id: String,
        load: Option<usize>,
        unload: Option<usize>,
    ) -> Result<&WorldState, String> {
        let committed = match &self.committed {
            Some(world) => world,
            None => return Err("Not initialized".into()),
        };
        let location = match committed.vessels.get(&vessel_id) {
            Some(VesselState::Docked { location, .. }) => *location,
            Some(_) => return Err(format!("vessel {} is not docked", vessel_id)),
            None => return Err(format!("Cannot find vessel {}", vessel_id)),
        };
        if let Some(destination) = load {
            if destination >= self.travel_times.len() {
                return Err("Invalid cargo destination".into());
            }
        }
        if let Some(destination) = unload {
            if destination >= self.travel_times.len() {
                return Err("Invalid cargo destination".into());
            }
        }
        let used = self.dock_operations[location];
        let limit = self.dock_throughput[location];
        if used >= limit {
            return Err(format!(
                "transfer of {} operations exceeds dock throughput of {} at location {}",
                u128::from(used) + 1,
                limit,
                location
            ));
        }
        let mut world = match &self.planned {
            Some(world) => world.clone(),
            None => return Err("Not initialized".into()),
        };
        let vessel = match world.vessels.get_mut(&vessel_id) {
            Some(vessel) => vessel,
            None => return Err(format!("Cannot find vessel {}", vessel_id)),
        };
        let cargo = vessel_cargo_mut(vessel);

        // Check both source inventories before changing either one.
        if let Some(destination) = load {
            if !world.cargo.iter().any(|entry| {
                entry.location == location && entry.destination == destination && entry.quantity > 0
            }) {
                return Err("Cargo is not available at dock".into());
            }
        }
        if let Some(destination) = unload {
            if !cargo
                .iter()
                .any(|entry| entry.destination == destination && entry.quantity > 0)
            {
                return Err("Cargo is not available on vessel".into());
            }
        }
        // Remove both sources first, then add destinations (including same-cargo swaps).
        if let Some(destination) = load {
            for entry in &mut world.cargo {
                if entry.location == location && entry.destination == destination {
                    entry.quantity -= 1;
                    break;
                }
            }
        }
        if let Some(destination) = unload {
            for entry in cargo.iter_mut() {
                if entry.destination == destination {
                    entry.quantity -= 1;
                    break;
                }
            }
        }
        if let Some(destination) = load {
            match cargo
                .iter_mut()
                .find(|entry| entry.destination == destination)
            {
                Some(entry) => {
                    entry.quantity = match entry.quantity.checked_add(1) {
                        Some(quantity) => quantity,
                        None => return Err("Cargo quantity is too large".into()),
                    }
                }
                None => cargo.push(Cargo {
                    destination,
                    quantity: 1,
                }),
            }
        }
        if let Some(destination) = unload {
            if destination != location {
                match world
                    .cargo
                    .iter_mut()
                    .find(|entry| entry.location == location && entry.destination == destination)
                {
                    Some(entry) => {
                        entry.quantity = match entry.quantity.checked_add(1) {
                            Some(quantity) => quantity,
                            None => return Err("Cargo quantity is too large".into()),
                        }
                    }
                    None => world.cargo.push(DockCargo {
                        location,
                        destination,
                        quantity: 1,
                    }),
                }
            }
        }
        cargo.retain(|entry| entry.quantity > 0);
        world.cargo.retain(|entry| entry.quantity > 0);
        self.planned = Some(world);
        self.dock_operations[location] += 1;
        self.get_state()
    }
}

fn vessel_cargo(vessel: &VesselState) -> &Vec<Cargo> {
    match vessel {
        VesselState::Idle { cargo, .. }
        | VesselState::Transit { cargo, .. }
        | VesselState::Docked { cargo, .. } => cargo,
    }
}

fn vessel_cargo_mut(vessel: &mut VesselState) -> &mut Vec<Cargo> {
    match vessel {
        VesselState::Idle { cargo, .. }
        | VesselState::Transit { cargo, .. }
        | VesselState::Docked { cargo, .. } => cargo,
    }
}
