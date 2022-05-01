// use std::env;
// use std::fs;

use bmoon::Run;



fn main() {

    // let args: Vec<String> = env::args().collect();
    // let config = parse_args(&args);


    Run();
}

/*
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

fn parse_args(args: &[String]) -> Config{
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
*/