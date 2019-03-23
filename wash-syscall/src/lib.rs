pub const WASH:i32 = 0;
pub const OP_SYSTEM: i32 = 0;
pub const SUBOP_INITIALIZATION: i32 = 0;
pub const SUBOP_SPAWN: i32 = 1;
pub const SUBOP_STDOUT_PUTC: i32 = 2;
pub const SUBOP_STDOUT_FLUSH: i32 = 3;
pub const SUBOP_STDIN_PUTC: i32 = 4;
pub const SUBOP_STDIN_FLUSH: i32 = 5;

extern "C" {
    fn global_sys_call(
        op: i32,
        sub_op: i32,
        param_a: i32,
        param_b: i32,
        param_c: i32,
        param_d: i32,
    ) -> i32;
}

pub fn sys_call(
    op: i32,
    sub_op: i32,
    param_a: i32,
    param_b: i32,
    param_c: i32,
    param_d: i32,
) -> i32 {
    unsafe { global_sys_call(op, sub_op, param_a, param_b, param_c, param_d) }
}

pub fn print(s:&str){
    let cs:Vec<char> = s.chars().collect();
    for i in 0..cs.len(){
        let c = cs[i] as i32;
        sys_call(OP_SYSTEM,SUBOP_STDOUT_PUTC,WASH,c,0,0);
    }
    sys_call(OP_SYSTEM,SUBOP_STDOUT_FLUSH,WASH,0,0,0);
}
