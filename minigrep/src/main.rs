use std::{env, process};

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Find term: {}", config.query);
    println!("In file: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Error running program: {}", e);
        process::exit(1);
    }
}
