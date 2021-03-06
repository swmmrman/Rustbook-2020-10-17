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
    _binary_name: String,
    query: String,
    file_name: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Usage: {} \"search term\" file_name", args[0])
        }
        let _binary_name = args[0].clone();
        let query = args[1].clone();
        let file_name = args[2].clone();

        Config {_binary_name, query, file_name}
    }
}
