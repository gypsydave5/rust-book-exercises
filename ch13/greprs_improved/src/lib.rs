use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        grep(&config.search, &contents)
    } else {
        grep_case_insensitive(&config.search, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub search: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let search = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a search string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let mut case_sensitive = true;

        for (name, _) in env::vars() {
            if name == "CASE_INSENSITIVE" {
                case_sensitive = false;
            }
        }

        Ok(Config {
            search: search,
            filename: filename,
            case_sensitive: case_sensitive,
        })
    }
}

fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(search) {
            results.push(line);
        }
    }

    results
}

fn grep_case_insensitive<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    let search = search.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&search) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use {grep, grep_case_insensitive};

    #[test]
    fn one_result_case_sensitive() {
        let search = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], grep(search, contents))
    }

    #[test]
    fn two_results_case_sensitive() {
        let search = "robot";
        let contents = "\
robots are attacking
how many?
many robots";
        assert_eq!(vec!["robots are attacking", "many robots"],
                   grep(search, contents))
    }

    #[test]
    fn case_insensitive() {
        let search = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."],
                   grep_case_insensitive(search, contents));
    }
}
