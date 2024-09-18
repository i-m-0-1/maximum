fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} type whoami", args[0]);
        process::exit(1);
    }

    let whoami = "whoami";




}
