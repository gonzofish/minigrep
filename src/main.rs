use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // args[0] is the program itself
    let query = &args[1];
    let filename = &args[2];

    println!("Search for: {}", query);
    println!("In file: {}", filename);
}
