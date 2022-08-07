use std::env;
use std::fs;

fn parse_args(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_args(&args);

    let contents = fs::read_to_string(filename).expect("file do not found");
    println!("contetns={contents}");
}
