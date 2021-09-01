use std::env;
use std::fs::File;
// to get read_to_string() method enabled on File Struct
use std::io::Read;

fn main() {
    // pub fn args() -> Args
    // Args: An iterator over the arguments of a process, yielding a String value for each
    // argument.
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    // pub fn open<P: AsRef<Path>>(path: P) -> Result<File>
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    // fn read_to_string(&mut self, buf: &mut String) -> Result<usize>
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
