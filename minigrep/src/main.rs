use std::{env,process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err); //Now i can just print the error here.
        process::exit(1);
    });
    println!("Hunting for \"{}\" in {}", config.query, config.file_name);
    if let Err(e) = minigrep::run(config){
        eprint!("File error: {}", e);

        process::exit(1);
    }
}
