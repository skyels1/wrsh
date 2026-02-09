use std::io; // obvious
use std::io::Write; // this allows flush on stdout
use std::process::Command; // used to take in external commands
use std::env; // changing environment variables
use std::process::Stdio; // not even sure what this is for

mod builtins;
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
                        builtins::pipe(cmds[0], cmds[1]);
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
                    if let Err(e) = builtins::ls(parts) {
                        eprintln!("ls: {}", e)
                    }
                    continue;
                }

                // cd command
                if command == "cd" {
                    if let Err(e) = builtins::cd(parts) {
                        eprintln!("cd: {}", e)
                    }
                    continue;
                }

                // pwd command
                if command == "pwd" {
                    if let Err(e) = builtins::pwd() {
                        eprintln!("pwd: {}", e)
                    }
                    continue;
                }

                // cat command
                if command == "cat" {
                    if let Err(e) = builtins::cat(parts) {
                        eprintln!("cat: {}", e)
                    }
                    continue;
                }

                // grep command
                if command == "grep" {
                    if let Err(e) = builtins::grep(parts) {
                        eprintln!("grep: {}", e)
                    }
                    continue;
                }

                // fetch command
                if command == "fetch" {
                    if let Err(e) = builtins::fetch() {
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