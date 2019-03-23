use web_dom::*;
use wash_syscall::*;

#[no_mangle]
pub fn main() -> () {
    sys_call(OP_SYSTEM,SUBOP_STDOUT_PUTC,WASH,72,0,0);
    sys_call(OP_SYSTEM,SUBOP_STDOUT_PUTC,WASH,69,0,0);
    sys_call(OP_SYSTEM,SUBOP_STDOUT_PUTC,WASH,76,0,0);
    sys_call(OP_SYSTEM,SUBOP_STDOUT_PUTC,WASH,76,0,0);
    sys_call(OP_SYSTEM,SUBOP_STDOUT_PUTC,WASH,79,0,0);
    sys_call(OP_SYSTEM,SUBOP_STDOUT_PUTC,WASH,33,0,0);
    sys_call(OP_SYSTEM,SUBOP_STDOUT_FLUSH,WASH,0,0,0);
}
