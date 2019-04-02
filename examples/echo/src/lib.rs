#[macro_use]
extern crate serde_derive;
extern crate malloc;
use joss;
use serde_json::{from_str, json};

#[derive(Serialize, Deserialize)]
struct CommandLineArguments {
    arguments: Vec<String>,
}

#[no_mangle]
fn main() {
    // get command line args
    let request_json = json!({
        "operation": "get_command_line_arguments"
    });
    let response = joss::syscall(request_json.to_string());
    let response_json: CommandLineArguments = from_str(&response).unwrap();
    let output = response_json.arguments.clone().join(" ");

    // write to stdout
    let output_json = json!({
        "operation": "write_file",
        "file_descriptor": 1,
        "text":output
    });
    joss::syscall(output_json.to_string());
}
