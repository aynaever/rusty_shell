use std::process;
use std::path::Path;
use std::env;

pub fn exit() {
    process::exit(0);
}

pub fn cd(command: &Vec<String>) {
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

pub fn pwd() {
    let curr_path = env::current_dir();

    println!("{}", &curr_path.unwrap().display());
}
