use std::fs::File;
// to get read_to_string() method enabled on File Struct
use std::io::Read;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

//fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
//fn run<T>(config: Config) -> Result<(), T>
//where
//    T: std::error::Error + From<std::io::Error>,
//{
pub fn run(config: Config) -> std::io::Result<()> {
    // pub fn open<P: AsRef<Path>>(path: P) -> Result<File>
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    // fn read_to_string(&mut self, buf: &mut String) -> Result<usize>
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
