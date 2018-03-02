use std::env;
use std::fs::File;
// prelude for file-handling
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Search for: {}", config.query);
    println!("In file: {}", config.filename);

    let mut file = File::open(config.filename).expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With the text:\n{}", contents);
}

struct Config {
    filename: String,
    query: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        // args[0] is the program itself
        let filename = args[2].clone();
        let query = args[1].clone();

        Config {
            filename,
            query,
        }
    }
}
