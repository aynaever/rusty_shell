use std::process::Command;
use std::io;
use std::io::Write;

use crate::builtins::is_builtin;

fn sh_prompt() {
    let mut program_buffer = String::new();
    let prompt: Vec<String>;

    print!("[user@user ~]$ ");
    io::stdout().flush()
        .expect("Error in flushing the prompt output");

    io::stdin().read_line(&mut program_buffer)
        .expect("Error reading from command line");

    prompt = program_buffer.split_whitespace().map(String::from).collect();
    if !is_builtin(&prompt) {
        let program = Command::new(&prompt[0])
                .args(prompt.clone().drain(1..))
                .spawn();
        match program {
            Ok(mut child) => {
                child.wait().expect("Error in waiting the child process");
            }
            Err(_) => {
                eprintln!("Error in creating child program");
            }
        }
    }

}

pub fn shell() {
    loop {
        sh_prompt();
    }
}
