use std::env;

fn main() {
    // pub fn args() -> Args
    // Args: An iterator over the arguments of a process, yielding a String value for each
    // argument.
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
