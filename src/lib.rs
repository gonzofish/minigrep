use std::error::Error;
use std::env;
use std::fs::File;
// prelude for file-handling
use std::io::prelude::*;

pub struct Config {
    pub case_sensitive: bool,
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        args.next();

        let query = match args.next() {
          Some(arg) => arg,
          None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
          Some(arg) => arg,
          None => return Err("Didn't get a filename"),
        };

        Ok(Config {
            case_sensitive,
            filename,
            query,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
      search(&config.query, &contents)
    } else {
      search_case_insensitive(&config.query, &contents)
    };

    for line in results {
      println!("{}", line);
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let lower_query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&lower_query) {
      results.push(line);
    }
  }

  results
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn case_sensitive_search() {
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
    let query = "duct";

    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents),
    );
  }

  #[test]
  fn case_insensitive_search() {
    let contents = "\"
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    let query = "rUsT";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}
