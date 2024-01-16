use std::process;
use std::path::Path;
use std::env;

pub fn exit() {
    process::exit(0);
}

pub fn cd(command: &Vec<String>) {
    let new_path: &Path;
    let old_path: String = env::var("OLDPWD").unwrap();
    let home_dir = env::home_dir().unwrap();

    if command.len() >= 2 {
        if &command[1] == "-" {
            new_path = Path::new(&old_path);
            println!("{}", &new_path.display());
        }
        else {
            new_path = Path::new(&command[1]);
        }
    }
    else {
        new_path = Path::new(&home_dir);
    }

    env::set_var("OLDPWD", env::current_dir().unwrap());
    env::set_current_dir(new_path)
        .expect("Error in change the current working directory");
}

pub fn pwd() {
    let curr_path = env::current_dir();

    println!("{}", &curr_path.unwrap().display());
}
