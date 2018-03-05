use std::env;
use std::error::Error;
use std::fs::File;
// prelude for file-handling
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Search for: {}", config.query);
    println!("In file: {}", config.filename);

    if let Err(err) = run(config) {
        println!("Application error: {}", err);
        process::exit(1);
    }
}

struct Config {
    filename: String,
    query: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
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

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    println!("With the text:\n{}", contents);

    Ok(())
}
