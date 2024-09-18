use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} type whoami", args[0]);
        process::exit(1);
    }

    let input = &args[1];
    let whoami = "whoami";
    let superuser = "im01";
    if input.len() == whoami.len() {
        println!("You are {}", superuser);
    } else {
        println!("No command {} detected in this OS", input); 
    }
}
