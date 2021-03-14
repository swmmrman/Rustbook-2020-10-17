use std::error::Error;
use std::{fs,env};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(config.file_name)?;
    let results = if config.case_sensitive {
        search(&config.query, &text)
    }
    else {
        search_case_insensitive(&config.query, &text)
    };

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

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let mut results = Vec::new();
    let smallq = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&smallq) {
            results.push(line);
        }
    }
    results
}

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, String> { //Had to change te type
        let count = args.len();  //Grab the # of args here before using the iterator.
        let binary_name = args.next().unwrap(); //Store binary name for error.

        let query = match args.next() {
            Some(arg) => arg,
            None => "\"query\"".to_string(),
        };
        let file_name = match args.next() {
            Some(arg) => arg,
            None => "\"file name\"".to_string(),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        //Returning a completed error here, allows keeping all the logic here.
        if count < 3 {  //If the count is less then 3.  We have an error.
            return Err(format!("\nUsage: {} {} {}", //This is the usage errors I expect.
                &binary_name,
                &query,
                &file_name ,
            ));
        }
        Ok(Config{
            query,
            file_name,
            case_sensitive,
        })
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
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
