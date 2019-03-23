use web_dom::*;
use wash_syscall::*;
use ref_thread_local::RefThreadLocal;
#[macro_use]
extern crate ref_thread_local;

#[derive(Default)]
struct Shell {
    component: DOMReference
}

ref_thread_local! {
    static managed SHELL: Shell = Shell::default();
}

impl Shell {
    fn handle_sys_call(&mut self,op: i32, sub_op: i32, param_a: i32, param_b: i32, param_c: i32, param_d: i32) -> i32 {
        if op == OP_SYSTEM {
            if sub_op == SUBOP_INITIALIZATION {
                element::set_inner_html(param_a, "terminal goes here");
            } else if sub_op == SUBOP_STDOUT_PUTC {
                let c = (param_b as u8) as char;
                element::set_inner_html(self.component, &c.to_string());
            } else {
                console::error(&format!("unknown system call: {} {} {} {} {} {}",op,sub_op,param_a,param_b,param_c,param_d));
            }
        }
        0
    }
}

#[no_mangle]
pub fn sys_call_handler(op: i32, sub_op: i32, param_a: i32, param_b: i32, param_c: i32, param_d: i32) -> i32 {
    SHELL.borrow_mut().handle_sys_call(op,sub_op,param_a,param_b,param_c,param_d)
}

#[no_mangle]
pub fn main() -> () {
    sys_call(OP_SYSTEM, SUBOP_INITIALIZATION, 0, 0, 0, 0);
}
