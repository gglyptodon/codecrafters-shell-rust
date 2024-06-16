use std::env;
use std::ffi::OsStr;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path;
use std::{collections::VecDeque, path::PathBuf};

fn main() {
    let path = env::var_os("PATH").unwrap();

    let mut known_commands: Vec<&str> = Vec::new();
    known_commands.push("exit");
    known_commands.push("echo");
    known_commands.push("type");

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
                "type" => {
                    let querycmd = cmd.pop_front().unwrap();
                    let query = search_paths(&path, querycmd);
                    //let query = check_exists(querycmd, &known_commands);
                    match query {
                        Ok(p) => println!("{} is {}", querycmd, p.to_string_lossy()), //is a shell builtin", querycmd),
                        Err(_) => eprintln!("{}: not found", querycmd),
                    }
                    input = reset();
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
fn search_paths(path: &OsStr, cmd: &str) -> Result<PathBuf, ()> {
    let paths = env::split_paths(&path);
    let mut found = paths.filter_map(|path: path::PathBuf| {
        let to_check = path.join(cmd);
        if to_check.is_file() {
            Some(to_check)
        } else {
            None
        }
    });
    if let Some(p) = found.next() {
        return Ok(p.to_path_buf());
    }
    Err(())
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
