#![no_std]
#![no_main]

use alloc::{
    format,
    string::{String, ToString},
};
use ansi_rgb::{blue, blue_magenta, green, red, white, yellow, Background, Foreground};
use ulib::{console::getchar, exec, fork, waitpid};

extern crate alloc;

#[macro_use]
extern crate ulib;

const LF: u8 = 0x0a;
const CR: u8 = 0x0d;
const DL: u8 = 0x7f;
const BS: u8 = 0x08;

#[inline]
fn prompt(last_exit_code: i32) {
    let last_code_prompt = if last_exit_code == 0 {
        "0".to_string().fg(green())
    } else {
        format!("{}", last_exit_code).fg(red())
    };
    print!("{} {}", last_code_prompt, "wShell $ ".fg(blue_magenta()));
}

#[inline]
fn backspace() {
    print!("{0} {0}", BS as char);
}

#[no_mangle]
pub fn main() -> i32 {
    let mut last_exit_code = 0i32;
    println!(
        "{} {} {}",
        "wCore OS shell(wShell)".bg(blue()).fg(white()),
        "version".bg(white()),
        "0.1.0".bg(yellow())
    );
    let mut line = String::new();
    prompt(last_exit_code);
    loop {
        let c = getchar();
        match c {
            LF | CR => {
                if !line.is_empty() {
                    println!(""); // make terminal clean
                    line.push('\0');
                    let pid = fork();
                    if pid == 0 {
                        // child process
                        if exec(line.as_str()) == -1 {
                            println!("Error when executing! ");
                            return -4;
                        }
                        unreachable!("shell exec unreachable");
                    } else {
                        let mut exit_code = 0i32;
                        let exit_pid = waitpid(pid, &mut exit_code);
                        assert_eq!(exit_pid, pid);
                        last_exit_code = exit_code;
                    }
                    line.clear();
                }
                prompt(last_exit_code);
            }
            BS | DL => {
                if !line.is_empty() {
                    backspace();
                    line.pop();
                }
            }
            _ => {
                print!("{}", c as char);
                line.push(c as char);
            }
        }
    }
}
