use web_dom::*;

#[no_mangle]
pub fn sys_call_handler(op: i32, sub_op: i32, param_a: i32, param_b: i32, param_c: i32, param_d: i32) -> () {
    if op == OP_SYSTEM {
        if sub_op == SUBOP_INITIALIZATION {
            element::set_inner_html(param_a, "terminal goes here");
        } else {
            console::error(&format!("unknown system call: {} {} {} {} {} {}",op,sub_op,param_a,param_b,param_c,param_d));
        }
    }
}

#[no_mangle]
pub fn main() -> () {
    sys_call(OP_SYSTEM, SUBOP_INITIALIZATION, 0, 0, 0, 0);
}
