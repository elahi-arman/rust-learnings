extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    //env::args will have command line parameters as an iterator
    // gotta explicitly state the type of the collection we want
    let args: Vec<String> = env::args().collect();

    // unwrap_or_else custom non panic error handling, unwraps if Ok, unwrap_or_else
    // calse the method inside the closure.
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // don't use unwrap since we don't care about unwrapping a value
    if let Err(e) = minigrep::run(config){
        println!("Application error: {}", e);
        process::exit(1);
    }
}
