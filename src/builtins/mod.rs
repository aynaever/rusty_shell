mod builtins;

pub fn is_builtin(command: &Vec<String>) -> bool {
    if command[0] == "exit" {
        builtins::exit();
    }
    else if command[0] == "cd" {
        builtins::cd(&command);
        return true;
    }
    else if command[0] == "pwd" {
        builtins::pwd();
        return true;
    }
    return false;
}
