use web_dom::*;

const OP_SYSTEM: i32 = 0;
const SUBOP_INITIALIZATION: i32 = 0;
const SUBOP_SPAWN: i32 = 1;

#[no_mangle]
pub fn sys_call_handler(op: i32, sub_op: i32, param_a: i32, param_b: i32, param_c: i32, param_d: i32) -> () {
    if op == OP_SYSTEM {
        if sub_op == SUBOP_INITIALIZATION {
            element::set_inner_html(param_a, "terminal goes here");
        }
    }
}

#[no_mangle]
pub fn main() -> () {
    sys_call(OP_SYSTEM, SUBOP_INITIALIZATION, 0, 0, 0, 0);
}
