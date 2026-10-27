use crate::{
    engine::Engine,
    state::{DockCargo, VesselState, WorldState},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Command {
    command: String,
    parameters: Value,
}

#[derive(Deserialize)]
pub struct Parameters {
    travel_times: Vec<Vec<u64>>,
    dock_throughput: Vec<u64>,
    vessels: Vec<VesselInit>,

    #[serde(default)]
    cargo_manifest: Vec<DockCargo>,
}

#[derive(Deserialize)]
pub struct VesselInit {
    id: String,
    location: usize,
}

#[derive(Deserialize)]
pub struct DestinationParameters {
    vessel_id: String,
    destination: usize,
}

#[derive(Deserialize)]
pub struct HistoryParameter {
    tick: u64,
}

#[derive(Deserialize)]
pub struct TickParameters {
    #[serde(default)]
    cargo_manifest: Vec<DockCargo>,
}

#[derive(Deserialize)]
pub struct VesselParameters {
    vessel_id: String,
}

#[derive(Deserialize)]
pub struct SwapParameters {
    vessel_id: String,
    load_destination: usize,
    unload_destination: usize,
}

#[derive(Serialize)]
pub struct Response {
    status: String,
    payload: Value,
}

pub fn process(line: &str, engine: &mut Engine) -> Response {
    let cmd: Command = match serde_json::from_str(line) {
        Ok(cmd) => cmd,
        Err(e) => {
            return Response {
                status: "error".into(),
                payload: json!({
                    "message": e.to_string()
                }),
            };
        }
    };

    match cmd.command.as_str() {
        "init" => {
            if engine.committed.is_some() {
                return Response {
                    status: "error".into(),
                    payload: json!({
                        "message": "Already initialized"
                    }),
                };
            }
            let params: Parameters = match serde_json::from_value(cmd.parameters) {
                Ok(params) => params,
                Err(e) => {
                    return Response {
                        status: "error".into(),
                        payload: json!({
                            "message": e.to_string()
                        }),
                    };
                }
            };
            let n = params.travel_times.len();
            for row in &params.travel_times {
                if row.len() != n {
                    return Response {
                        status: "error".into(),
                        payload: json!({
                            "message": "Travel times must be a square matrix"
                        }),
                    };
                }
            }
            for (from, row) in params.travel_times.iter().enumerate() {
                for (to, duration) in row.iter().enumerate() {
                    if from == to && *duration != 0 {
                        return Response {
                            status: "error".into(),
                            payload: json!({
                                "message": "Diagonal travel times must be zero"
                            }),
                        };
                    }

                    if from != to && *duration == 0 {
                        return Response {
                            status: "error".into(),
                            payload: json!({
                                "message": "Travel between different locations must take positive time"
                            }),
                        };
                    }
                }
            }
            for vessel in &params.vessels {
                if vessel.location >= n {
                    return Response {
                        status: "error".into(),
                        payload: json!({
                            "message": format!(
                                "Invalid starting location for vessel {}",
                                vessel.id
                            )
                        }),
                    };
                }
            }
            if params.dock_throughput.len() != n {
                return Response {
                    status: "error".into(),
                    payload: json!({
                        "message": "Dock Throughput size invalid"
                    }),
                };
            }
            for throughput in &params.dock_throughput {
                if *throughput == 0 {
                    return Response {
                        status: "error".into(),
                        payload: json!({
                            "message": "Dock Throughput values have to be > 0"
                        }),
                    };
                }
            }
            for manifest in &params.cargo_manifest {
                if manifest.destination >= n || manifest.location >= n || manifest.quantity == 0 {
                    return Response {
                        status: "error".into(),
                        payload: json!({
                            "message": "Invalid manifest values"
                        }),
                    };
                };
            }
            let mut vessels = HashMap::new();
            for vessel in params.vessels {
                if vessels.contains_key(&vessel.id) {
                    return Response {
                        status: "error".into(),
                        payload: json!({"message": format!("Duplicate vessel ID: {}", vessel.id)}),
                    };
                }
                vessels.insert(
                    vessel.id,
                    VesselState::Idle {
                        location: vessel.location,
                        cargo: Vec::new(),
                    },
                );
            }
            let mut cargo: Vec<DockCargo> = Vec::new();
            for manifest in params.cargo_manifest {
                let mut existing = None;
                for entry in &mut cargo {
                    if entry.location == manifest.location
                        && entry.destination == manifest.destination
                    {
                        existing = Some(entry);
                        break;
                    }
                }
                if let Some(entry) = existing {
                    let total = entry.quantity.checked_add(manifest.quantity);
                    match total {
                        Some(quantity) => entry.quantity = quantity,
                        None => {
                            return Response {
                                status: "error".into(),
                                payload: json!({
                                    "message": "Cargo quantity is too large"
                                }),
                            };
                        }
                    }
                } else {
                    cargo.push(manifest);
                }
            }
            let world = WorldState {
                tick: 0,
                vessels,
                cargo,
            };
            engine.committed = Some(world.clone());
            engine.planned = Some(world.clone());
            engine.history = vec![world];
            engine.travel_times = params.travel_times;
            engine.dock_throughput = params.dock_throughput;
            engine.dock_operations = vec![0; n];

            Response {
                status: "ok".into(),
                payload: json!(engine.committed),
            }
        }
        "tick" => {
            let params: TickParameters = match serde_json::from_value(cmd.parameters) {
                Ok(params) => params,
                Err(e) => {
                    return Response {
                        status: "error".into(),
                        payload: json!({
                            "message": e.to_string()
                        }),
                    };
                }
            };
            match engine.tick(params.cargo_manifest) {
                Ok(world) => Response {
                    status: "ok".into(),
                    payload: json!(world),
                },
                Err(e) => Response {
                    status: "error".into(),
                    payload: json!({
                        "message": e
                    }),
                },
            }
        }
        "set_destination" => {
            let params: DestinationParameters = match serde_json::from_value(cmd.parameters) {
                Ok(params) => params,
                Err(e) => {
                    return Response {
                        status: "error".into(),
                        payload: json!({
                            "message": e.to_string()
                        }),
                    };
                }
            };
            match engine.set_destination(params.vessel_id, params.destination) {
                Ok(world) => Response {
                    status: "ok".into(),
                    payload: json!(world),
                },
                Err(message) => Response {
                    status: "error".into(),
                    payload: json!({"message": message}),
                },
            }
        }
        "dock" => {
            let params: VesselParameters = match serde_json::from_value(cmd.parameters) {
                Ok(params) => params,
                Err(e) => {
                    return Response {
                        status: "error".into(),
                        payload: json!({"message": e.to_string()}),
                    };
                }
            };
            match engine.dock(params.vessel_id) {
                Ok(world) => Response {
                    status: "ok".into(),
                    payload: json!(world),
                },
                Err(message) => Response {
                    status: "error".into(),
                    payload: json!({"message": message}),
                },
            }
        }
        "undock" => {
            let params: VesselParameters = match serde_json::from_value(cmd.parameters) {
                Ok(params) => params,
                Err(e) => {
                    return Response {
                        status: "error".into(),
                        payload: json!({"message": e.to_string()}),
                    };
                }
            };
            match engine.undock(params.vessel_id) {
                Ok(world) => Response {
                    status: "ok".into(),
                    payload: json!(world),
                },
                Err(message) => Response {
                    status: "error".into(),
                    payload: json!({"message": message}),
                },
            }
        }
        "load" => {
            let params: DestinationParameters = match serde_json::from_value(cmd.parameters) {
                Ok(params) => params,
                Err(e) => {
                    return Response {
                        status: "error".into(),
                        payload: json!({"message": e.to_string()}),
                    };
                }
            };
            match engine.load(params.vessel_id, params.destination) {
                Ok(world) => Response {
                    status: "ok".into(),
                    payload: json!(world),
                },
                Err(message) => Response {
                    status: "error".into(),
                    payload: json!({"message": message}),
                },
            }
        }
        "unload" => {
            let params: DestinationParameters = match serde_json::from_value(cmd.parameters) {
                Ok(params) => params,
                Err(e) => {
                    return Response {
                        status: "error".into(),
                        payload: json!({"message": e.to_string()}),
                    };
                }
            };
            match engine.unload(params.vessel_id, params.destination) {
                Ok(world) => Response {
                    status: "ok".into(),
                    payload: json!(world),
                },
                Err(message) => Response {
                    status: "error".into(),
                    payload: json!({"message": message}),
                },
            }
        }
        "swap" => {
            let params: SwapParameters = match serde_json::from_value(cmd.parameters) {
                Ok(params) => params,
                Err(e) => {
                    return Response {
                        status: "error".into(),
                        payload: json!({"message": e.to_string()}),
                    };
                }
            };
            match engine.swap(
                params.vessel_id,
                params.load_destination,
                params.unload_destination,
            ) {
                Ok(world) => Response {
                    status: "ok".into(),
                    payload: json!(world),
                },
                Err(message) => Response {
                    status: "error".into(),
                    payload: json!({"message": message}),
                },
            }
        }
        "get_state" => match engine.get_state() {
            Ok(world) => Response {
                status: "ok".into(),
                payload: json!(world),
            },
            Err(e) => Response {
                status: "error".into(),
                payload: json!({
                    "message": e
                }),
            },
        },
        "get_state_at" => {
            let param: HistoryParameter = match serde_json::from_value(cmd.parameters) {
                Ok(param) => param,
                Err(e) => {
                    return Response {
                        status: "error".into(),
                        payload: json!({
                            "message": e.to_string()
                        }),
                    };
                }
            };
            match engine.get_state_at(param.tick) {
                Ok(world) => Response {
                    status: "ok".into(),
                    payload: json!(world),
                },
                Err(message) => Response {
                    status: "error".into(),
                    payload: json!({"message": message}),
                },
            }
        }
        _ => Response {
            status: "error".into(),
            payload: json!({
                "message": "unknown command"
            }),
        },
    }
}
