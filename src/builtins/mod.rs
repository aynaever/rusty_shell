use std::process;
use std::path::Path;
use std::env;

pub fn is_builtin(command: &Vec<String>) -> bool {
    if command[0] == "exit" {
        exit();
    }
    else if command[0] == "cd" {
        cd(&command[1]);
        return true;
    }
     return false;
}

fn exit() {
    process::exit(0);
}

fn cd(path: &String) {
    let new_path = Path::new(path);
    env::set_current_dir(new_path)
        .expect("Error in change the current working directory");
}
