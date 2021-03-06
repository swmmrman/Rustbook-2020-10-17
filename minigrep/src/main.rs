use std::{env,fs,process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
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
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(format!("Usage: {} \"search term\" file_name", args[0]));
        }
        let _binary_name = args[0].clone();
        let query = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config {_binary_name, query, file_name})
    }
}
