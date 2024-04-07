use std::env;
use std::process;

use filegrep::Config;

fn main() {

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(| err | {
        println!("Problem parsing args: {}", err);
        process::exit(1);
    });

    println!("This is query: {}", config.query);
    println!("This is filename with path: {}", config.filename);

    if let Err(e) = filegrep::run(config) {
        println!("Application Error: {}", e);
        process::exit(1)
    }
}

