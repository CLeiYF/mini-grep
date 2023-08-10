use std::{env, process};
use minigrep::Config;

/// find search key in file
/// ignore case: IGNORE_CASE=1 cargo run -- query_key file_path
/// normal     : cargo run -- query_key file_path
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.querry_key);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}