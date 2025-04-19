use minigrep::Config;
use std::env;
use std::process;
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    match minigrep::run(config) {
        Ok(_) => println!("Success"),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
