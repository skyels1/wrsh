use std::io; // obvious
use std::io::Write; // this allows flush on stdout
use std::process::Command; // used to take in external commands
use std::env; // changing environment variables
use std::fs; // file system
use std::path::Path; // used to get ls to list files
use std::fs::File;
use std::io::BufReader; // for better cat with big files
use std::io::prelude::*;
use std::process::Stdio; // not even sure what this is for
use sysinfo::System; // system info for fetch

fn builtin_pipe(left_cmd: &str, right_cmd: &str) {
    let mut left_parts = left_cmd.split_whitespace();
    let mut right_parts = right_cmd.split_whitespace();

    let mut left = Command::new(left_parts.next().unwrap())
        .args(left_parts)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn left command");

    let mut right = Command::new(right_parts.next().unwrap())
        .args(right_parts)
        .stdin(left.stdout.take().unwrap())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn right command");

    right.wait().unwrap();
    left.wait().unwrap();
}

fn builtin_fetch() -> io::Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // for let cpu check on windows needed if empty
    let cpu = {
    let brand = sys.global_cpu_info().brand();
    if brand.is_empty() {
        sys.cpus().first().map(|c| c.brand()).unwrap_or("Unknown CPU") // fix for empty cpu on windows
    } else {
        brand
    }
    };
    let total = sys.total_memory() / 1024 / 1024;
    let used = sys.used_memory() / 1024 / 1024;

    println!("OS: {}", std::env::consts::OS);
    println!("Shell: wrsh");
    println!("CPU: {}", cpu);
    println!("Ram: {} MiB / {} MiB", used, total);

    Ok(())
}

fn builtin_grep<'a>(mut parts: impl Iterator<Item = &'a str>) -> io::Result<()> {
    let pattern = parts.next().unwrap_or(".");
    let path = parts.next().unwrap_or(".");
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line?;
        if line.to_lowercase().contains(&pattern.to_lowercase()) {
            println!("{}", line)
        }
    }

    Ok(())
}

fn builtin_cat<'a>(mut parts: impl Iterator<Item = &'a str>) -> io::Result<()> {
    let path = parts.next().unwrap_or(".");
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    println!("{}", contents);
    Ok(())
}

// function for cd to change to the desired dir
fn builtin_cd<'a>(mut parts: impl Iterator<Item = &'a str>) -> io::Result<()> {
    let path = parts.next().unwrap_or(".");
    std::env::set_current_dir(path)?;
    Ok(())
}

// function for ls to list the files
fn builtin_ls<'a>(mut parts: impl Iterator<Item = &'a str>) -> io::Result<()> {
    let path = parts.next().unwrap_or(".");
    let path = Path::new(path);

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        println!("{}  ", entry.file_name().to_string_lossy());
    }

    println!();
    Ok(())
}

// print working dir
fn builtin_pwd() -> io::Result<()> {
    let dir = env::current_dir()?;
    println!("{}", dir.display());
    Ok(())
}

fn main() {
    loop {
        let cwd = env::current_dir().unwrap_or_else(|_| "?".into());
        print!("wrsh {}> ", cwd.display());
        io::stdout().flush().unwrap();
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {

            Ok(0) => break,
            Ok(_) => {

                let builtins = ["ls", "cd", "pwd", "cat", "grep"];
                let cmds: Vec<&str> = input.trim().split('|').map(|s| s.trim()).collect();

                if cmds.len() == 2 {
                    if builtins.contains(&cmds[0]) || builtins.contains(&cmds[1]) {
                        println!("Piping builtin commands not supported yet...")
                    } else {
                        builtin_pipe(cmds[0], cmds[1]);
                        continue;
                    }
                }
                // create parts and split to get the command
                let mut parts = input.trim().split_whitespace();
                let command = parts.next().unwrap_or("");

                // shell closing handling
                if command == "q" { 
                    println!("closing shell...");
                    break;
                }

                // for empty lines
                if command == "" {
                    continue;
                }

                // ls command
                if command == "ls" {
                    if let Err(e) = builtin_ls(parts) {
                        eprintln!("ls: {}", e)
                    }
                    continue;
                }

                // cd command
                if command == "cd" {
                    if let Err(e) = builtin_cd(parts) {
                        eprintln!("cd: {}", e)
                    }
                    continue;
                }

                // pwd command
                if command == "pwd" {
                    if let Err(e) = builtin_pwd() {
                        eprintln!("pwd: {}", e)
                    }
                    continue;
                }

                // cat command
                if command == "cat" {
                    if let Err(e) = builtin_cat(parts) {
                        eprintln!("cat: {}", e)
                    }
                    continue;
                }

                // grep command
                if command == "grep" {
                    if let Err(e) = builtin_grep(parts) {
                        eprintln!("grep: {}", e)
                    }
                    continue;
                }

                // fetch command
                if command == "fetch" {
                    if let Err(e) = builtin_fetch() {
                        eprintln!("fetch: {}", e)
                    }
                    continue;
                }

                match Command::new(command)
                    .args(parts)
                    .stdin(Stdio::inherit()) // this block for using nano and editers
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .status() // supposedly better here than output to handle crashing?
                {
                    Ok(status) => {
                        if !status.success() {
                            println!("Proccess ended: {}", status);
                        }
                    }
                    Err(e) => eprintln!("{}: {}", command, e) // error for reading commands
                }

            },
            Err(e) => eprintln!("Error reading input: {}", e) // error message handler
        }
    }
}