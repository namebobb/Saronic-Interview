#[path = "../src/engine.rs"]
mod engine;
#[path = "../src/protocol.rs"]
mod protocol;
#[path = "../src/state.rs"]
mod state;

use engine::Engine;
use serde_json::{Value, json};

fn new_engine() -> Engine {
    Engine {
        committed: None,
        planned: None,
        history: Vec::new(),
        travel_times: Vec::new(),
        dock_throughput: Vec::new(),
        dock_operations: Vec::new(),
    }
}

fn send(engine: &mut Engine, command: &str, parameters: Value) -> Value {
    let request = json!({"command": command, "parameters": parameters});
    let response = protocol::process(&request.to_string(), engine);
    json!(response)
}

fn ok(engine: &mut Engine, command: &str, parameters: Value) -> Value {
    let response = send(engine, command, parameters);
    assert_eq!(response["status"], "ok", "{response}");
    response["payload"].clone()
}

fn unchanged_error(engine: &mut Engine, command: &str, parameters: Value) {
    let before = json!({
        "planned": engine.planned, "committed": engine.committed,
        "history": engine.history, "operations": engine.dock_operations,
        "throughput": engine.dock_throughput, "travel_times": engine.travel_times,
    });
    let response = send(engine, command, parameters);
    assert_eq!(response["status"], "error", "{response}");
    assert!(response["payload"]["message"].is_string());
    let after = json!({
        "planned": engine.planned, "committed": engine.committed,
        "history": engine.history, "operations": engine.dock_operations,
        "throughput": engine.dock_throughput, "travel_times": engine.travel_times,
    });
    assert_eq!(before, after, "Rejected {command} changed the engine");
}

fn parameters() -> Value {
    json!({
        "travel_times": [[0,2,3],[2,0,1],[3,1,0]],
        "dock_throughput": [2,2,2],
        "vessels": [{"id":"v1","location":0},{"id":"v2","location":0}],
        "cargo_manifest": [
            {"location":0,"destination":2,"quantity":3},
            {"location":0,"destination":1,"quantity":1}
        ]
    })
}

fn initialized() -> Engine {
    let mut engine = new_engine();
    ok(&mut engine, "init", parameters());
    engine
}

fn docked() -> Engine {
    let mut engine = initialized();
    ok(&mut engine, "dock", json!({"vessel_id":"v1"}));
    ok(&mut engine, "dock", json!({"vessel_id":"v2"}));
    ok(&mut engine, "tick", json!({}));
    engine
}

#[test]
fn commands_before_init_and_malformed_requests_return_errors() {
    let mut engine = new_engine();
    for command in [
        "tick",
        "get_state",
        "get_state_at",
        "set_destination",
        "dock",
        "undock",
        "load",
        "unload",
        "swap",
    ] {
        unchanged_error(
            &mut engine,
            command,
            json!({
                "vessel_id":"v1", "destination":1, "tick":0,
                "load_destination":1, "unload_destination":2
            }),
        );
    }
    for line in [
        "",
        "{",
        "null",
        "[]",
        "{}",
        "{\"command\":12,\"parameters\":{}}",
    ] {
        let response = json!(protocol::process(line, &mut engine));
        assert_eq!(response["status"], "error");
    }
    ok(&mut engine, "init", parameters());
    unchanged_error(&mut engine, "init", parameters());
    unchanged_error(&mut engine, "unknown", json!({}));
}

#[test]
fn invalid_init_is_rejected_without_partial_initialization() {
    for (field, value) in [
        ("travel_times", json!([[0, 2, 3], [2, 0], [3, 1, 0]])),
        ("travel_times", json!([[1, 2, 3], [2, 0, 1], [3, 1, 0]])),
        ("travel_times", json!([[0, 0, 3], [2, 0, 1], [3, 1, 0]])),
        ("travel_times", json!([[0, -1, 3], [2, 0, 1], [3, 1, 0]])),
        ("travel_times", json!([[0, 1.5, 3], [2, 0, 1], [3, 1, 0]])),
        ("dock_throughput", json!([1])),
        ("dock_throughput", json!([1, 0, 1])),
        ("vessels", json!([{"id":"v1","location":3}])),
        (
            "vessels",
            json!([{"id":"v1","location":0},{"id":"v1","location":1}]),
        ),
        (
            "cargo_manifest",
            json!([{"location":3,"destination":1,"quantity":1}]),
        ),
        (
            "cargo_manifest",
            json!([{"location":0,"destination":3,"quantity":1}]),
        ),
        (
            "cargo_manifest",
            json!([{"location":0,"destination":1,"quantity":0}]),
        ),
    ] {
        let mut engine = new_engine();
        let mut params = parameters();
        params[field] = value;
        unchanged_error(&mut engine, "init", params);
        ok(&mut engine, "init", parameters());
    }
}

#[test]
fn manifests_merge_and_invalid_ticks_preserve_all_state() {
    let mut engine = new_engine();
    let mut params = parameters();
    params["cargo_manifest"] = json!([
        {"location":0,"destination":2,"quantity":2},
        {"location":0,"destination":2,"quantity":3}
    ]);
    let world = ok(&mut engine, "init", params);
    assert_eq!(
        world["cargo"],
        json!([{"location":0,"destination":2,"quantity":5}])
    );
    let world = ok(
        &mut engine,
        "tick",
        json!({"cargo_manifest":[{"location":0,"destination":2,"quantity":1}]}),
    );
    assert_eq!(world["cargo"][0]["quantity"], 6);
    for entry in [
        json!({"location":3,"destination":2,"quantity":1}),
        json!({"location":0,"destination":3,"quantity":1}),
        json!({"location":0,"destination":2,"quantity":0}),
        json!({"location":0,"destination":2,"quantity":u64::MAX}),
    ] {
        unchanged_error(
            &mut engine,
            "tick",
            json!({"cargo_manifest":[
                {"location":1,"destination":2,"quantity":1}, entry
            ]}),
        );
    }
    let old = ok(&mut engine, "get_state_at", json!({"tick":0}));
    assert_eq!(old["cargo"][0]["quantity"], 5);
}

#[test]
fn absent_manifests_default_to_empty() {
    let mut engine = new_engine();
    let mut params = parameters();
    params
        .as_object_mut()
        .map(|object| object.remove("cargo_manifest"));
    let world = ok(&mut engine, "init", params);
    assert_eq!(world["cargo"], json!([]));
    let world = ok(&mut engine, "tick", json!({}));
    assert_eq!(world["tick"], 1);
    assert_eq!(world["cargo"], json!([]));
}

#[test]
fn destination_overrides_arrivals_and_history_follow_tick_boundaries() {
    let mut engine = initialized();
    unchanged_error(
        &mut engine,
        "set_destination",
        json!({"vessel_id":"missing","destination":1}),
    );
    unchanged_error(
        &mut engine,
        "set_destination",
        json!({"vessel_id":"v1","destination":3}),
    );
    unchanged_error(
        &mut engine,
        "set_destination",
        json!({"vessel_id":"v1","destination":0}),
    );
    ok(
        &mut engine,
        "set_destination",
        json!({"vessel_id":"v1","destination":2}),
    );
    let world = ok(
        &mut engine,
        "set_destination",
        json!({"vessel_id":"v1","destination":1}),
    );
    assert_eq!(world["vessels"]["v1"]["arrives_at_tick"], 2);
    let old = ok(&mut engine, "get_state_at", json!({"tick":0}));
    assert_eq!(old["vessels"]["v1"]["state"], "idle");
    ok(&mut engine, "tick", json!({}));
    unchanged_error(
        &mut engine,
        "set_destination",
        json!({"vessel_id":"v1","destination":2}),
    );
    let world = ok(&mut engine, "tick", json!({}));
    assert_eq!(world["vessels"]["v1"]["state"], "idle");
    assert_eq!(world["vessels"]["v1"]["location"], 1);
    ok(
        &mut engine,
        "set_destination",
        json!({"vessel_id":"v1","destination":2}),
    );
    unchanged_error(&mut engine, "get_state_at", json!({"tick":3}));
    unchanged_error(&mut engine, "get_state_at", json!({"tick":-1}));
    unchanged_error(&mut engine, "get_state_at", json!({"tick":1.5}));
    let old = ok(&mut engine, "get_state_at", json!({"tick":2}));
    assert_eq!(old["vessels"]["v1"]["state"], "idle");
    assert_eq!(engine.history.len(), 3);
}

#[test]
fn dock_and_undock_use_committed_state_and_preserve_planned_cargo() {
    let mut engine = initialized();
    ok(
        &mut engine,
        "set_destination",
        json!({"vessel_id":"v1","destination":2}),
    );
    ok(&mut engine, "dock", json!({"vessel_id":"v1"}));
    ok(&mut engine, "dock", json!({"vessel_id":"v1"}));
    ok(
        &mut engine,
        "set_destination",
        json!({"vessel_id":"v1","destination":1}),
    );
    ok(&mut engine, "dock", json!({"vessel_id":"v1"}));
    unchanged_error(&mut engine, "undock", json!({"vessel_id":"v1"}));
    unchanged_error(
        &mut engine,
        "load",
        json!({"vessel_id":"v1","destination":2}),
    );
    ok(&mut engine, "tick", json!({}));
    unchanged_error(&mut engine, "dock", json!({"vessel_id":"v1"}));
    ok(&mut engine, "undock", json!({"vessel_id":"v1"}));
    ok(
        &mut engine,
        "load",
        json!({"vessel_id":"v1","destination":2}),
    );
    let world = ok(&mut engine, "undock", json!({"vessel_id":"v1"}));
    assert_eq!(world["vessels"]["v1"]["state"], "idle");
    assert_eq!(
        world["vessels"]["v1"]["cargo"],
        json!([{"destination":2,"quantity":1}])
    );
    unchanged_error(&mut engine, "dock", json!({"vessel_id":"v1"}));
    ok(
        &mut engine,
        "unload",
        json!({"vessel_id":"v1","destination":2}),
    );
}

#[test]
fn throughput_is_shared_and_only_successful_tick_resets_it() {
    let mut engine = docked();
    ok(
        &mut engine,
        "load",
        json!({"vessel_id":"v1","destination":2}),
    );
    ok(
        &mut engine,
        "load",
        json!({"vessel_id":"v2","destination":2}),
    );
    assert_eq!(engine.dock_operations, vec![2, 0, 0]);
    unchanged_error(
        &mut engine,
        "load",
        json!({"vessel_id":"v1","destination":2}),
    );
    unchanged_error(
        &mut engine,
        "tick",
        json!({"cargo_manifest":[{"location":9,"destination":1,"quantity":1}]}),
    );
    unchanged_error(
        &mut engine,
        "unload",
        json!({"vessel_id":"v1","destination":2}),
    );
    ok(&mut engine, "tick", json!({}));
    assert_eq!(engine.dock_operations, vec![0, 0, 0]);
    ok(
        &mut engine,
        "unload",
        json!({"vessel_id":"v1","destination":2}),
    );
}

#[test]
fn empty_inventory_and_invalid_transfers_do_not_use_throughput() {
    let mut engine = docked();
    for command in ["load", "unload"] {
        unchanged_error(
            &mut engine,
            command,
            json!({"vessel_id":"missing","destination":1}),
        );
        unchanged_error(
            &mut engine,
            command,
            json!({"vessel_id":"v1","destination":3}),
        );
        unchanged_error(
            &mut engine,
            command,
            json!({"vessel_id":"v1","destination":0}),
        );
    }
    ok(
        &mut engine,
        "load",
        json!({"vessel_id":"v1","destination":1}),
    );
    unchanged_error(
        &mut engine,
        "load",
        json!({"vessel_id":"v2","destination":1}),
    );
    assert_eq!(engine.dock_operations[0], 1);
}

#[test]
fn swap_is_atomic_and_counts_as_one_operation() {
    let mut engine = docked();
    ok(
        &mut engine,
        "load",
        json!({"vessel_id":"v1","destination":2}),
    );
    ok(&mut engine, "tick", json!({}));
    unchanged_error(
        &mut engine,
        "swap",
        json!({"vessel_id":"v1","load_destination":1,"unload_destination":0}),
    );
    unchanged_error(
        &mut engine,
        "swap",
        json!({"vessel_id":"v1","load_destination":0,"unload_destination":2}),
    );
    let world = ok(
        &mut engine,
        "swap",
        json!({"vessel_id":"v1","load_destination":1,"unload_destination":2}),
    );
    assert_eq!(engine.dock_operations[0], 1);
    assert_eq!(
        world["vessels"]["v1"]["cargo"],
        json!([{"destination":1,"quantity":1}])
    );
    assert_eq!(
        world["cargo"],
        json!([{"location":0,"destination":2,"quantity":3}])
    );
}

#[test]
fn same_destination_swap_preserves_counts_but_uses_an_operation() {
    let mut engine = docked();
    ok(
        &mut engine,
        "load",
        json!({"vessel_id":"v1","destination":2}),
    );
    let before = ok(&mut engine, "get_state", json!({}));
    let after = ok(
        &mut engine,
        "swap",
        json!({"vessel_id":"v1","load_destination":2,"unload_destination":2}),
    );
    assert_eq!(before, after);
    assert_eq!(engine.dock_operations[0], 2);
}

#[test]
fn cargo_survives_travel_and_unload_at_destination_delivers_it() {
    let mut engine = docked();
    ok(
        &mut engine,
        "load",
        json!({"vessel_id":"v1","destination":1}),
    );
    ok(&mut engine, "undock", json!({"vessel_id":"v1"}));
    ok(&mut engine, "tick", json!({}));
    ok(
        &mut engine,
        "set_destination",
        json!({"vessel_id":"v1","destination":1}),
    );
    ok(&mut engine, "tick", json!({}));
    let world = ok(&mut engine, "tick", json!({}));
    assert_eq!(
        world["vessels"]["v1"],
        json!({"state":"idle","location":1,"cargo":[{"destination":1,"quantity":1}]})
    );
    ok(&mut engine, "dock", json!({"vessel_id":"v1"}));
    ok(&mut engine, "tick", json!({}));
    let world = ok(
        &mut engine,
        "unload",
        json!({"vessel_id":"v1","destination":1}),
    );
    assert_eq!(world["vessels"]["v1"]["cargo"], json!([]));
    assert_eq!(
        world["cargo"],
        json!([{"location":0,"destination":2,"quantity":3}])
    );
    let old = ok(&mut engine, "get_state_at", json!({"tick":5}));
    assert_eq!(
        old["vessels"]["v1"]["cargo"],
        json!([{"destination":1,"quantity":1}])
    );
}

#[test]
fn unload_overflow_does_not_remove_cargo_or_consume_capacity() {
    let mut engine = docked();
    ok(
        &mut engine,
        "load",
        json!({"vessel_id":"v1","destination":2}),
    );
    ok(
        &mut engine,
        "tick",
        json!({"cargo_manifest":[{"location":0,"destination":2,"quantity":u64::MAX-2}]}),
    );
    unchanged_error(
        &mut engine,
        "unload",
        json!({"vessel_id":"v1","destination":2}),
    );
    unchanged_error(
        &mut engine,
        "swap",
        json!({"vessel_id":"v1","load_destination":1,"unload_destination":2}),
    );
    assert_eq!(engine.dock_operations[0], 0);
}

#[test]
fn tick_and_arrival_overflow_return_errors_without_changes() {
    let mut engine = initialized();
    if let Some(world) = &mut engine.planned {
        world.tick = u64::MAX;
    }
    if let Some(world) = &mut engine.committed {
        world.tick = u64::MAX;
    }
    unchanged_error(&mut engine, "tick", json!({}));
    unchanged_error(
        &mut engine,
        "set_destination",
        json!({"vessel_id":"v1","destination":1}),
    );
}

#[test]
fn unload_at_an_intermediate_dock_moves_one_unit_and_keeps_its_destination() {
    let mut engine = docked();
    // Carry two units bound for location 2, then stop at location 1.
    ok(
        &mut engine,
        "load",
        json!({"vessel_id":"v1","destination":2}),
    );
    ok(
        &mut engine,
        "load",
        json!({"vessel_id":"v1","destination":2}),
    );
    ok(&mut engine, "undock", json!({"vessel_id":"v1"}));
    ok(&mut engine, "tick", json!({}));
    ok(
        &mut engine,
        "set_destination",
        json!({"vessel_id":"v1","destination":1}),
    );
    ok(&mut engine, "tick", json!({}));
    ok(&mut engine, "tick", json!({}));
    ok(&mut engine, "dock", json!({"vessel_id":"v1"}));
    let before = ok(&mut engine, "tick", json!({}));
    assert_eq!(
        before["vessels"]["v1"]["cargo"],
        json!([{"destination":2,"quantity":2}])
    );

    let after = ok(
        &mut engine,
        "unload",
        json!({"vessel_id":"v1","destination":2}),
    );
    assert_eq!(
        after["vessels"]["v1"],
        json!({
            "state":"docked", "location":1,
            "cargo":[{"destination":2,"quantity":1}]
        })
    );
    assert_eq!(after["vessels"]["v2"], before["vessels"]["v2"]);
    assert_eq!(
        after["cargo"],
        json!([
            {"location":0,"destination":2,"quantity":1},
            {"location":0,"destination":1,"quantity":1},
            {"location":1,"destination":2,"quantity":1}
        ])
    );
    assert_eq!(engine.dock_operations, vec![0, 1, 0]);
    assert_eq!(ok(&mut engine, "get_state", json!({})), after);
    assert_eq!(ok(&mut engine, "get_state_at", json!({"tick":5})), before);

    // A second unload merges into that dock entry and removes the empty vessel entry.
    let after = ok(
        &mut engine,
        "unload",
        json!({"vessel_id":"v1","destination":2}),
    );
    assert_eq!(after["vessels"]["v1"]["cargo"], json!([]));
    assert_eq!(
        after["cargo"],
        json!([
            {"location":0,"destination":2,"quantity":1},
            {"location":0,"destination":1,"quantity":1},
            {"location":1,"destination":2,"quantity":2}
        ])
    );
    assert_eq!(engine.dock_operations, vec![0, 2, 0]);
}
