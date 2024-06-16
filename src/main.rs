use std::collections::VecDeque;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let mut known_commands: Vec<&str> = Vec::new();
    known_commands.push("exit");

    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    while let Err(e) = check_exists(&input, &known_commands) {
        eprintln!("{}", e);
        print!("$ ");
        io::stdout().flush().unwrap();
        input = String::new();
        stdin.read_line(&mut input).unwrap();
    }
    if let Ok(cmd) = check_exists(&input, &known_commands) {
        println!("{}", cmd.join(" "));
    }
}
fn check_exists<'a>(
    user_input: &'a str,
    known_commands: &Vec<&str>,
) -> Result<Vec<&'a str>, String> {
    let mut input = user_input.trim().split_whitespace().collect::<VecDeque<&str>>();
    let cmd = input.pop_front().unwrap();
    let mut params = Vec::from(input.clone());
    if !known_commands.contains(&cmd) {
        return Err(format!("{}: command not found", cmd));
    }
    let mut cmd_params = Vec::new();
    cmd_params.push(cmd);
    cmd_params.append(&mut params);
    Ok(cmd_params)
}
