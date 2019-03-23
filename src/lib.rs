use web_dom::*;
use wash_syscall::*;
use ref_thread_local::RefThreadLocal;
#[macro_use]
extern crate ref_thread_local;

#[derive(Default)]
struct Shell {
    component: DOMReference,
    stdout: Vec<u8>
}

ref_thread_local! {
    static managed SHELL: Shell = Shell::default();
}

impl Shell {
    fn handle_sys_call(&mut self,op: i32, sub_op: i32, param_a: i32, param_b: i32, param_c: i32, param_d: i32) -> i32 {
        if op == OP_SYSTEM {
            if sub_op == SUBOP_INITIALIZATION {
                //setup
            } else if sub_op == SUBOP_STDOUT_PUTC {
                self.stdout.push(param_b as u8);
            } else if sub_op == SUBOP_STDOUT_FLUSH {
                for i in 0..self.stdout.len() {
                    let c = self.stdout[i] as char;
                    element::set_inner_html(self.component, &format!("{}{}",element::get_inner_html(self.component),&c.to_string()));
                }
                self.stdout = vec![];
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
