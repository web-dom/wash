use web_dom::*;
use wash_syscall::*;
use ref_thread_local::RefThreadLocal;
#[macro_use]
extern crate ref_thread_local;

#[derive(Default)]
struct Shell {
    component: DOMReference,
    stdout: Vec<u8>,
    canvas: DOMReference,
    ctx: DOMReference
}

ref_thread_local! {
    static managed SHELL: Shell = Shell::default();
}

impl Shell {
    fn handle_sys_call(&mut self,op: i32, sub_op: i32, param_a: i32, param_b: i32, param_c: i32, param_d: i32) -> i32 {
        if op == OP_SYSTEM {
            if sub_op == SUBOP_INITIALIZATION {
                self.component = param_a;
                self.canvas = document::create_element(window::get_document(window()),"canvas",UNDEFINED);
                element::set_attribute(self.canvas, "width", "600");
                element::set_attribute(self.canvas, "height", "400");
                node::append_child(self.component, self.canvas);
                self.ctx = htmlcanvas::get_context(self.canvas, "2d");
                drawing::set_fill_style(self.ctx, "black");
                drawing::fill_rect(self.ctx, 0.0, 0.0, 600.0, 400.0);
            } else if sub_op == SUBOP_STDOUT_PUTC {
                self.stdout.push(param_b as u8);
            } else if sub_op == SUBOP_STDOUT_FLUSH {
                for i in 0..self.stdout.len() {
                    let c = self.stdout[i] as char;
                    drawing::set_fill_style(self.ctx, "white");

                    drawing::fill_text(self.ctx,&c.to_string(), 50.0+10.0*i as f32, 50.0,1000.0);
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
