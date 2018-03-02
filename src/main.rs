use std::env;
use std::fs::File;
// prelude for file-handling
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);

    println!("Search for: {}", query);
    println!("In file: {}", filename);

    let mut file = File::open(filename).expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With the text:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    // args[0] is the program itself
    (
        &args[1],
        &args[2],
    )
}
