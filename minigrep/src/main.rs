use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

fn parse_args(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args);

    let contents = fs::read_to_string(config.filename).expect("file do not found");
    println!("contetns={contents}");
}
