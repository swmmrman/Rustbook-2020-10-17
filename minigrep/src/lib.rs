use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(config.file_name)?;
    let results = search(&config.query, &text);

    for result in results {
        println!("{}", result);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub struct Config {
    _binary_name: String,
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> { //Had to change te type
        if args.len() < 3 {
            //Returning an completed error here, allows keeping all the logic here.
            return Err(format!("Usage: {} \"search term\" file_name", args[0]));
        }
        let _binary_name = args[0].clone();
        let query = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config {_binary_name, query, file_name})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
