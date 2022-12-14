use std::env;
use std::process;

use cppnew::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = cppnew::run(config) {
        println!("Application error: {err}");
        process::exit(1);
    }
}
