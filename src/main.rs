mod core;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    core::sh_command(&args);
}

