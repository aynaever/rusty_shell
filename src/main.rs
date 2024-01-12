use std::env;

fn main() {

    let args = env::args();

    println!("Number of arguments: {}", args.len());
    for arg in args {
        println!("{}", arg);
    }
}
