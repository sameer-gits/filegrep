use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.filename)?;
    let match_lines = search(&config.query, &contents);

    if match_lines.is_empty() {
        println!("No Match Found");
    }
    else {
        for line in search(&config.query, &contents) {
            println!("{}", line);
        }
    }

    Ok(())
}

pub struct Config {
    pub  query: String,
    pub  filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query: String = args[1].clone();
        let filename: String = args[2].clone();
        Ok(Config {query, filename}) 
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            results.push(line);
        }
    }

    results
}
