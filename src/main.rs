use std::io; // obvious
use std::io::Write; // this allows flush on stdout
use std::process::Command; // used to take in commands

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {

            Ok(0) => break,
            Ok(_) => {

                let mut parts = input.trim().split_whitespace(); // create parts and split to get the command
                let command = parts.next().unwrap_or("");

                if command == "q" { // shell closing handling 
                    print!("closing shell...");
                    break;
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