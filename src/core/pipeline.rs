use std::process::Command;

pub fn parse_prompt(prompt: &String) {
    let commands: Vec<&str> = prompt.split('|').collect();
    dbg!(commands);
}

pub fn pipe(mut command1: Command) {
    let output1 = command1.output().ok();

    dbg!("{}", output1);
}
