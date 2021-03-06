use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    println!("Hunting for \"{}\" in {}", config.query, config.file_name);

    let text = fs::read_to_string(config.file_name)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", text);

}

struct Config {
    query: String,
    file_name: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_name = args[2].clone();

        Config {query, file_name}
    }
}
