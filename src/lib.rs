use core::str::from_utf8;
use ref_thread_local::RefThreadLocal;
use std::collections::HashMap;
use wash_syscall::*;
use web_dom::*;
#[macro_use]
extern crate ref_thread_local;

#[derive(Default)]
struct Shell {
    window: DOMReference,
    document: DOMReference,
    component: DOMReference,
    stdout: Vec<u8>,
    canvas: DOMReference,
    ctx: DOMReference,
    shadow: DOMReference,
    characters: Vec<u8>,
    width: usize,
    height: usize,
    pos: usize,
    key_down_listener: EventListener,
    command: Vec<u8>,
    known_commands: HashMap<String, String>,
}

ref_thread_local! {
    static managed SHELL: Shell = Shell::default();
}

impl Shell {
    fn handle_sys_call(
        &mut self,
        op: i32,
        sub_op: i32,
        param_a: i32,
        param_b: i32,
        param_c: i32,
        param_d: i32,
    ) -> i32 {
        if op == OP_SYSTEM {
            if sub_op == SUBOP_INITIALIZATION {
                self.width = 60;
                self.height = 40;
                self.characters = vec![32; self.width * self.height];
                self.component = param_a;
                self.window = window();
                self.document = window::get_document(self.window);
                self.canvas = document::create_element(self.document, "canvas");
                element::set_attribute(
                    self.canvas,
                    "style",
                    r#"image-rendering: -moz-crisp-edges;
  image-rendering: -webkit-crisp-edges;
  image-rendering: pixelated;
  image-rendering: crisp-edges;"#,
                );
                element::set_attribute(self.canvas, "width", "800");
                element::set_attribute(self.canvas, "height", "600");
                self.shadow = customelement::attach_shadow(self.component);
                node::append_child(self.shadow, self.canvas);
                self.ctx = htmlcanvas::get_context(self.canvas, "2d");
                canvas::set_fill_style(self.ctx, "black");
                canvas::fill_rect(self.ctx, 0.0, 0.0, 800.0, 600.0);
                self.key_down_listener = create_event_listener();
                eventtarget::add_event_listener(self.document, "keydown", self.key_down_listener);
                self.print("welcome to WASH, type \"help\" to see a list of commands\n");
                self.characters[self.pos] = 124;
                self.render();
                let child_count = element::get_child_element_count(self.component) as i32;
                if child_count > 0 {
                    let mut el = element::get_first_element_child(self.component);
                    for i in 0..child_count {
                        if i != 0 {
                            el = element::get_next_element_sibling(el);
                        }
                        self.known_commands.insert(
                            convert_to_string(element::get_attribute(el, "name")),
                            convert_to_string(element::get_attribute(el, "module")),
                        );
                    }
                }
            } else if sub_op == SUBOP_STDOUT_PUTC {
                self.stdout.push(param_b as u8);
            } else if sub_op == SUBOP_STDOUT_FLUSH {
                for i in 0..self.stdout.len() {
                    self.process_char(self.stdout[i]);
                }
                self.render();
                self.stdout = vec![];
            } else {
                console::error(&format!(
                    "unknown system call: {} {} {} {} {} {}",
                    op, sub_op, param_a, param_b, param_c, param_d
                ));
            }
        }
        0
    }

    fn print(&mut self, s: &str) {
        let cs: Vec<char> = s.chars().collect();
        for i in 0..cs.len() {
            let c = cs[i] as u8;
            self.characters[self.pos] = c;
            if c == 10 {
                self.pos = (self.pos / self.width + 1) * self.width;
            } else {
                self.pos += 1;
            }
        }
        self.render();
    }

    fn handle_event(&mut self, listener: EventListener, event: Event) {
        if listener == self.key_down_listener {
            let key = keyboardevent::get_key(event);
            let key_chars: Vec<char> = key.chars().collect();
            if key.len() == 1 {
                let key_code = key_chars[0] as u8;
                self.process_char(key_code);
                self.command.push(key_code);
            } else if key == "Backspace" {
                self.process_char(8);
            } else if key == "Enter" {
                self.process_char(13);
            } else {
                return;
            }
            self.render();
        }
    }

    fn execute(&mut self) {
        let s = from_utf8(&self.command).unwrap();
        if s == "help" {
            let keys: Vec<String> = self.known_commands.keys().map(|x| x.clone()).collect();
            if keys.len() == 0 {
                self.print("embarassing...there doesn't seem to be any commands\n");
                self.characters[self.pos] = 124;
                self.render();
            } else {
                self.print(&keys.join(" "));
                self.print("\n");
                self.characters[self.pos] = 124;
                self.render();
            }
        } else {
            let cmd = self.known_commands.get(s);
            if cmd.is_some() {
                sys_call(OP_SYSTEM, SUBOP_SPAWN, to_cstring(cmd.unwrap()), 0, 0, 0);
            } else {
                self.print("command not found\n");
                self.characters[self.pos] = 124;
                self.render();
            }
        }
        self.command = vec![];
    }

    fn process_char(&mut self, k: u8) {
        if k == 13 {
            self.characters[self.pos] = 32;
            self.pos = (self.pos / self.width + 1) * self.width;
            self.characters[self.pos] = 124;
            self.execute();
            return;
        }
        if k == 10 {
            self.characters[self.pos] = 32;
            self.pos = (self.pos / self.width + 1) * self.width;
            self.characters[self.pos] = 124;
            return;
        }
        if k == 8 {
            if self.command.len() == 0 {
                return;
            }
            self.command.pop();
            self.characters[self.pos] = 32;
            self.pos -= 1;
            self.characters[self.pos] = 124;
            return;
        }
        self.characters[self.pos] = k;
        self.pos += 1;
        self.characters[self.pos] = 124;
    }

    fn render(&self) {
        canvas::set_fill_style(self.ctx, "black");
        canvas::fill_rect(self.ctx, 0.0, 0.0, 800.0, 600.0);
        canvas::set_fill_style(self.ctx, "white");
        canvas::set_font(self.ctx, "18px monospace");
        for x in 0..self.width {
            for y in 0..self.height {
                canvas::fill_text(
                    self.ctx,
                    &(self.characters[y * self.width + x] as char).to_string(),
                    0.0 + (800.0 / self.width as f32) * x as f32,
                    15.0 + (600.0 / self.height as f32) * y as f32,
                );
            }
        }
    }
}

#[no_mangle]
pub fn callback(listener: EventListener, event: Event) -> () {
    SHELL.borrow_mut().handle_event(listener, event);
}

#[no_mangle]
pub fn sys_call_handler(
    op: i32,
    sub_op: i32,
    param_a: i32,
    param_b: i32,
    param_c: i32,
    param_d: i32,
) -> i32 {
    SHELL
        .borrow_mut()
        .handle_sys_call(op, sub_op, param_a, param_b, param_c, param_d)
}

#[no_mangle]
pub fn main() -> () {
    sys_call(OP_SYSTEM, SUBOP_INITIALIZATION, 0, 0, 0, 0);
}
