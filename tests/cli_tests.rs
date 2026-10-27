use serde_json::{Value, json};
use std::error::Error;
use std::io::{self, Write};
use std::process::{Command, Stdio};

// Run the real program so these tests include main.rs and its input/output loop.
fn run(input: &[u8]) -> Result<Vec<Value>, Box<dyn Error>> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_Saronic-Interview"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    {
        let mut stdin = match child.stdin.take() {
            Some(stdin) => stdin,
            None => return Err(io::Error::other("Test could not open stdin").into()),
        };
        stdin.write_all(input)?;
    } // Closing stdin tells the program there are no more commands.

    let output = child.wait_with_output()?;
    assert!(
        output.status.success(),
        "Program failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout)?;
    let mut responses = Vec::new();
    for line in stdout.lines() {
        // An extra debug line on stdout or a multiline response fails here.
        responses.push(serde_json::from_str(line)?);
    }
    Ok(responses)
}

#[test]
fn stdin_commands_produce_one_json_response_each_in_order() -> Result<(), Box<dyn Error>> {
    let input = concat!(
        "{\"command\":\"init\",\"parameters\":{\"travel_times\":[[0,1],[1,0]],\"dock_throughput\":[2,2],\"vessels\":[{\"id\":\"v1\",\"location\":0}]}}\n",
        "{\"command\":\"set_destination\",\"parameters\":{\"vessel_id\":\"v1\",\"destination\":1}}\n",
        "not json\n",
        "{\"command\":\"tick\",\"parameters\":{}}\n",
        "{\"command\":\"get_state\",\"parameters\":{}}"
    ); // The last command deliberately has no trailing newline.
    let responses = run(input.as_bytes())?;
    assert_eq!(responses.len(), 5);
    assert_eq!(
        responses[0],
        json!({"status":"ok","payload":{
            "tick":0,"vessels":{"v1":{"state":"idle","location":0,"cargo":[]}},"cargo":[]
        }})
    );
    assert_eq!(
        responses[1],
        json!({"status":"ok","payload":{
            "tick":0,"vessels":{"v1":{"state":"transit","from":0,"to":1,"arrives_at_tick":1,"cargo":[]}},"cargo":[]
        }})
    );
    assert_eq!(responses[2]["status"], "error");
    assert!(responses[2]["payload"]["message"].is_string());
    assert_eq!(
        responses[3],
        json!({"status":"ok","payload":{
            "tick":1,"vessels":{"v1":{"state":"idle","location":1,"cargo":[]}},"cargo":[]
        }})
    );
    assert_eq!(responses[4], responses[3]);
    Ok(())
}

#[test]
fn invalid_text_and_blank_lines_do_not_stop_later_commands() -> Result<(), Box<dyn Error>> {
    let mut input = b"\xff\n\n".to_vec();
    input.extend_from_slice(concat!(
        "{\"command\":\"init\",\"parameters\":{\"travel_times\":[[0]],\"dock_throughput\":[1],\"vessels\":[]}}\n",
        "{\"command\":\"get_state\",\"parameters\":{}}\n"
    ).as_bytes());
    let responses = run(&input)?;
    assert_eq!(responses.len(), 4);
    for response in &responses[..2] {
        assert_eq!(response["status"], "error");
        assert!(response["payload"]["message"].is_string());
    }
    assert_eq!(
        responses[2],
        json!({"status":"ok","payload":{
            "tick":0,"vessels":{},"cargo":[]
        }})
    );
    assert_eq!(responses[3], responses[2]);
    assert!(run(b"")?.is_empty());
    Ok(())
}
