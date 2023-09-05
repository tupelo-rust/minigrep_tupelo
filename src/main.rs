#![feature(format_args_capture)]

use std::env;
use std::process;
use minigrep_tupelo::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 解析参数
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // 业务逻辑
    if let Err(e) = minigrep_tupelo::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}