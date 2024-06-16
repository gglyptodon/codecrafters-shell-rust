use std::collections::VecDeque;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let mut known_commands: Vec<&str> = Vec::new();
    known_commands.push("exit");
    known_commands.push("echo");

    let mut input = reset();
    loop {
        let commands = check_exists(&input, &known_commands);
        match commands {
            Err(e) => {
                eprintln!("{}", e);
                input = reset();
            }
            Ok(mut cmd) => match cmd.pop_front().unwrap() {
                "echo" => {
                    println!("{}", cmd.make_contiguous().join(" "));
                    input = reset();
                }

                "exit" => {
                    std::process::exit(0);
                }
                _ => {
                    input = reset();
                }
            },
        }
    }
}
fn check_exists<'a>(
    user_input: &'a str,
    known_commands: &Vec<&str>,
) -> Result<VecDeque<&'a str>, String> {
    let mut input = user_input.split_whitespace().collect::<VecDeque<&str>>();
    let cmd = input.pop_front().unwrap();
    let mut params = input.clone();
    if !known_commands.contains(&cmd) {
        return Err(format!("{}: command not found", cmd));
    }
    let mut cmd_params = VecDeque::new();
    cmd_params.push_front(cmd);
    cmd_params.append(&mut params);
    Ok(cmd_params)
}

fn reset() -> String {
    print!("$ ");
    io::stdout().flush().unwrap();
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    input
}
