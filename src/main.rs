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
    if input == whoami {
        println!("{}", superuser);
    } else {
        println!("command not found: '{}'", input); 
    }
}
