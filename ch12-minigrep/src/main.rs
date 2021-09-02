use std::env;
use std::process;

// from the library crate
use ch12_minigrep::run;
use ch12_minigrep::Config;

fn main() {
    // pub fn args() -> Args
    // Args: An iterator over the arguments of a process, yielding a String value for each
    // argument.
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    //println!("Searching for {}", config.query);
    //println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
