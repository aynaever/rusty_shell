use std::process;

pub fn is_builtin(command: &String) -> bool {
    if command == "exit" {
        process::exit(0);
    }
     return false;
}
