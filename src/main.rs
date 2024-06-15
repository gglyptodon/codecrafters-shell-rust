#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let known_commands: Vec<&str> = Vec::new();

    let _command = match check_exists(&input, &known_commands) {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("{}", e);
            ""
        }
    };
}
fn check_exists<'a>(user_input: &'a str, known_commands: &Vec<&str>) -> Result<&'a str, String> {
    if !known_commands.contains(&user_input.trim_end()) {
        return Err(format!("{}: command not found", &user_input.trim_end()));
    }
    Ok(user_input)
}
