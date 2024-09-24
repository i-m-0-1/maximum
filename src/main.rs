// TODO: change the user details to another file
// TODO: make it like terminal, always waiting for commands

use std::env;
use std::process;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} type whoami", args[0]);
        process::exit(1);
    }


    let input = &args[1];
   
    let filepath = Path::new("data.txt");
    let file = File::open(&filepath)?;
    let reader = io::BufReader::new(file);

    let mut user = String::new();
    let mut role = String::new();

    for (index, line ) in reader.lines().enumerate() {
        let line = line?;

        let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();

        match index {
            0 => { if parts[0] == "user" { user = parts[1].to_string(); } },
            1 => { if parts[0] == "role" { role = parts[1].to_string(); } },
            _ => break,
        }
    }

    match input.as_str() {
        "whoami" => println!("{}:{}", user, role),
        _ => println!("Command not found: '{}'", input),
    }

       Ok(())
}
