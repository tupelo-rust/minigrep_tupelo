#![feature(format_args_capture)]

use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    
    println!("With text:\n{contents}");

    Ok(())
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // 参数检查
        if args.len() < 3 {
            // panic!("not enough arguments");
            return Err("not enough arguments");
        }

        let query=args[1].clone();
        let file_path=args[2].clone();

        // Config {query, file_path}
        Ok(Config {query, file_path})
    }
}