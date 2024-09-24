// TODO:mkdir, ls, create file, etc. 

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let filepath = Path::new("data.txt");
    let file = File::open(&filepath)?;
    let reader = io::BufReader::new(file);

    let mut user = String::new();
    let mut role = String::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();

        match index {
            0 => { if parts[0] == "user" { user = parts[1].to_string(); } },
            1 => { if parts[0] == "role" { role = parts[1].to_string(); } },
            _ => break,
        }
    }

    loop {
        print!(":");

        io::stdout().flush()?;

        let mut input = String::new();

        io::stdin().read_line(&mut input)?;

        let command = input.trim();

        match command {

            "whoami" => println!("{}", user),

            "role" => println!("{}", role),

            "exit" => {
                println!("**Terminal closed**");
                break;
            }

            _ => {
                eprintln!("Command '{}' not found.", command);
                eprintln!("Usage: 'whoami', 'role', 'exit'");
            }
        }
    }

    Ok(())

}
