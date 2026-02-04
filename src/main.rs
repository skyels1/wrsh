use std::io; // obvious
use std::io::Write; // this allows flush on stdout
use std::process::Command; // used to take in commands
use std::env;
use std::fs;
use std::path::Path;

fn builtin_cd<'a>(mut parts: impl Iterator<Item = &'a str>) -> io::Result<()> {
    let path = parts.next().unwrap_or(".");
    std::env::set_current_dir(path)?;
    Ok(())
}

fn builtin_ls<'a>(mut parts: impl Iterator<Item = &'a str>) -> io::Result<()> {
    let path = parts.next().unwrap_or(".");
    let path = Path::new(path);

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        print!("{}  ", entry.file_name().to_string_lossy());
    }

    println!();
    Ok(())
}

fn builtin_pwd() -> io::Result<()> {
    let dir = env::current_dir()?;
    println!("{}", dir.display());
    Ok(())
}

fn main() {
    loop {
        let cwd = env::current_dir().unwrap_or_else(|_| "?".into());
        print!("{}> ", cwd.display());
        io::stdout().flush().unwrap();
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {

            Ok(0) => break,
            Ok(_) => {

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

                // ls command work around for windows (bad work around but initial works)
                if command == "ls" {
                    if let Err(e) = builtin_ls(parts) {
                        eprintln!("ls: {}", e)
                    }
                    continue;
                }

                // cd command
                if command == "cd" {
                    if let Err(e) = builtin_cd(parts) {
                        eprintln!("ls: {}", e)
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

                match Command::new(command)
                    .args(parts)
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