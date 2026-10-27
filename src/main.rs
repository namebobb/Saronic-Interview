use std::io::{self, BufRead};

mod engine;
mod protocol;
mod state;

fn main() {
    let stdin = io::stdin();
    let mut engine = engine::Engine {
        committed: None,
        planned: None,
        history: Vec::new(),
        travel_times: Vec::new(),
        dock_throughput: Vec::new(),
        dock_operations: Vec::new(),
    };
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                println!(
                    "{}",
                    serde_json::json!({
                        "status": "error",
                        "payload": {"message": format!("Failed to read input: {}", error)}
                    })
                );
                if error.kind() == io::ErrorKind::InvalidData {
                    continue;
                }
                break;
            }
        };
        let response = protocol::process(&line, &mut engine);
        match serde_json::to_string(&response) {
            Ok(output) => println!("{}", output),
            Err(error) => {
                eprintln!("Failed to serialize response: {}", error);
                println!(
                    "{}",
                    serde_json::json!({
                        "status": "error",
                        "payload": {"message": "Failed to serialize response"}
                    })
                );
            }
        }
    }
}
