use std::env;
use std::process::Command;

fn main() {

    let args: Vec<String> = env::args().collect();
    let program = Command::new(&args[1])
            .arg("rust is great")
            .spawn();

    match program {
        Ok(mut child) => {
            child.wait().expect("failed to wait the child process");
            println!("succefully created the spawned process");
        }
        Err(err) => {
            println!("error: {}", err);
        }
    }
}
