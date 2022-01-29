use std::error::Error;
use std::fs;
use std::env;
use std::env::Args;

pub struct Config {
    pub query : String,
    pub filename : String,
    pub case_sensitive : bool
}

impl Config {
    pub fn new(mut args : env::Args) -> Result<Config, &'static str> {
        args.next();

        let query  = match args.next() {
            Some(args) => args,
            None => return Err("Did not get query")
        };
        let filename  = match args.next() {
            Some(args) => args,
            None => return Err("Did not get filename")
        };

        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

        Ok(Config{ query, filename, case_sensitive })
    }
}

pub fn run(config : Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results= if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'z>(query : &str, content : &'z str) -> Vec<&'z str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
pub fn search_case_insensitive<'x>(query : &str, content : &'x str) -> Vec<&'x str> {
    let query = query.to_lowercase();
    content.lines().filter(|line| line.to_lowercase().contains(query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
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