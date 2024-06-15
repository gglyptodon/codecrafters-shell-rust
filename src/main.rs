#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let known_commands: Vec<&str> = Vec::new();

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
}
fn check_exists<'a>(user_input: &'a str, known_commands: &Vec<&str>) -> Result<&'a str, String> {
    if !known_commands.contains(&user_input.trim_end()) {
        return Err(format!("{}: command not found", &user_input.trim_end()));
    }
    Ok(user_input)
}
