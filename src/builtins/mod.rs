use std::process;

pub fn is_builtin(command: &String) -> bool {
    if command == "exit" {
        exit();
    }
     return false;
}

fn exit() {
    process::exit(0);
}
