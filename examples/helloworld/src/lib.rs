extern crate malloc;
use joss;
use serde_json::{from_str, json};

#[no_mangle]
pub fn main() -> () {
    // write to stdout
    let output_json = json!({
        "operation": "write_file",
        "file_descriptor": 1,
        "text":"Hello World!"
    });
    joss::syscall(output_json.to_string());
}
