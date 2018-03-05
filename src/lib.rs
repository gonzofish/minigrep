use std::error::Error;
use std::fs::File;
// prelude for file-handling
use std::io::prelude::*;

pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        } else {
            // args[0] is the program itself
            let filename = args[2].clone();
            let query = args[1].clone();

            Ok(Config {
                filename,
                query,
            })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    println!("With the text:\n{}", contents);

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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn one_result() {
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    let query = "duct";

    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents),
    );
  }
}
