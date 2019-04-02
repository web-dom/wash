# wash

`wash` is a web assembly shell for a unix-like operating system.  Applications use [JOSS (JSON Operating System Schema)](https://github.com/web-dom/joss/) to communicate with the operating system. For example a hello world:

```rust
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
```

See a demo [here](https://web-dom.github.io/wash/examples/helloworld/)

`wash` is meant to be very easily embeddable with a simple html element that defines what commands are available:

```
<wasm-shell module="wash.wasm">
  <command module="helloworld.wasm" name="hello"></command>
</wasm-shell>
```
