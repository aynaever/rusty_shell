use std::process::Command;
use std::io;
use std::io::Write;

pub fn sh_command() {
    let mut program_buffer = String::new();
    let prompt: Vec<String>;

    print!("[user@user ~]$ ");
    io::stdout().flush()
        .expect("Error in flushing the prompt output");

    io::stdin().read_line(&mut program_buffer)
        .expect("Error reading from command line");

    prompt = program_buffer.split_whitespace().map(String::from).collect();
    let program = Command::new(&prompt[0])
            .args(prompt.clone().drain(1..))
            .spawn();


    match program {
        Ok(mut child) => {
            child.wait().expect("failed to wait the child process");
        }
        Err(err) => {
            println!("error: {}", err);
        }
    }
}
