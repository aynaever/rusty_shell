use std::process;
use std::path::Path;
use std::env;

pub fn is_builtin(command: &Vec<String>) -> bool {
    if command[0] == "exit" {
        exit();
    }
    else if command[0] == "cd" {
        cd(&command);
        return true;
    }
    else if command[0] == "pwd" {
        pwd();
        return true;
    }
    return false;
}

fn exit() {
    process::exit(0);
}

fn cd(command: &Vec<String>) {
    let new_path: &Path;
    let home_dir = env::home_dir().unwrap();
    if command.len() > 2 {
        new_path = Path::new(&command[1]);
    }
    else {
        new_path = Path::new(&home_dir);
    }
    env::set_current_dir(new_path)
        .expect("Error in change the current working directory");
}

fn pwd() {
    let curr_path = env::current_dir();

    println!("{}", &curr_path.unwrap().display());
}
