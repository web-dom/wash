use joss;

#[no_mangle]
pub fn main() -> () {
    // write to stdout
    joss::syscall(r#"{
        "operation": "write_file",
        "file_descriptor": 1,
        "text":"Hello World!"
    }"#.to_owned());
}
