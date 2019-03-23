use web_dom::*;
use wash_syscall::*;

#[no_mangle]
pub fn main() -> () {
    sys_call(OP_SYSTEM,SUBOP_STDOUT_PUTC,WASH,72,0,0);
}
