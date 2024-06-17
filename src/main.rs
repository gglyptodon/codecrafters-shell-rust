use std::env;
use std::ffi::OsStr;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::{self, Path};
use std::process::Command;
use std::{collections::VecDeque, path::PathBuf};

fn main() {
    let path = env::var_os("PATH").unwrap();

    let mut known_commands: Vec<&str> = Vec::new();
    known_commands.push("exit");
    known_commands.push("echo");
    known_commands.push("type");
    known_commands.push("pwd");
    known_commands.push("cd");

    let mut input = reset();
    loop {
        let commands = check_exists(&input, &known_commands);
        match commands {
            Err((mut cmd_with_params, e)) => {
                // was not a builtin, try finding in path

                if let Ok(fullpath) = search_paths(&path, cmd_with_params.pop_front().unwrap()) {
                    execute_simple(&fullpath, &cmd_with_params);
                } else {
                    eprintln!("{}", e);
                }
            }
            Ok(mut cmd) => match cmd.pop_front() {
                Some("echo") => {
                    println!("{}", cmd.make_contiguous().join(" "));
                }
                Some("exit") => {
                    std::process::exit(0);
                }
                Some("pwd") => {
                    if let Ok(workdir) = pwd_builtin() {
                        println!("{}", workdir.to_string_lossy());
                    }
                }
                Some("type") => {
                    let querycmd = cmd.pop_front().unwrap();
                    if let Ok(_) = check_exists(querycmd, &known_commands) {
                        println!("{} is a shell builtin", querycmd);
                    } else {
                        let query = search_paths(&path, querycmd);
                        match query {
                            Ok(p) => {
                                println!("{} is {}", querycmd, p.to_string_lossy());
                            }
                            Err(_) => eprintln!("{}: not found", querycmd),
                        }
                    }
                }
                Some("cd") => match cmd.pop_front() {
                    Some(new_path) => {
                        if let Err(_) = cd_builtin(Path::new(new_path)) {
                            println!("cd: {}: No such file or directory", new_path)
                        }
                    }
                    None => {}
                },
                _ => {}
            },
        }
        input = reset();
    }
}
fn check_exists<'a>(
    user_input: &'a str,
    known_commands: &Vec<&str>,
) -> Result<VecDeque<&'a str>, (VecDeque<&'a str>, String)> {
    let mut input = user_input.split_whitespace().collect::<VecDeque<&str>>();
    let cmd = input.pop_front();
    match cmd {
        Some(cmd) => {
            let mut params = input.clone();
            let mut cmd_params = VecDeque::new();
            cmd_params.push_front(cmd);
            cmd_params.append(&mut params);
            if !known_commands.contains(&cmd) {
                return Err((cmd_params, format!("{}: command not found", cmd)));
            }

            return Ok(cmd_params);
        }
        None => Ok(VecDeque::new()),
    }
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

fn execute_simple(fullpath: &Path, params: &VecDeque<&str>) {
    let _child = Command::new(fullpath)
        .args(params)
        .status()
        .expect("failed to execute child");
}

fn pwd_builtin() -> io::Result<PathBuf> {
    env::current_dir()
}
fn cd_builtin(path: &Path) -> io::Result<()> {
    env::set_current_dir(path)
}
