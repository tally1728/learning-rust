use std::fs::File;
// to get read_to_string() method enabled on File Struct
use std::io::Read;

#[derive(PartialEq, Debug)]
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

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

////////////////////////////////////////////////////////////
// Tests
//
#[cfg(test)]
mod test {
    use super::run;
    use super::search;
    use super::Config;

    #[test]
    fn config_new_with_1arg() {
        let query = "you";
        let args = vec![String::from("cmd"), String::from(query)];

        assert_eq!(Config::new(&args), Err("not enough arguments"));
    }

    #[test]
    fn config_new_with_2args() {
        let query = "you";
        let filename = "poem.txt";
        let args = vec![
            String::from("cmd"),
            String::from(query),
            String::from(filename),
        ];

        assert_eq!(
            Config::new(&args),
            Ok(Config {
                query: String::from(query),
                filename: String::from(filename)
            })
        );
    }

    #[test]
    fn run_with_an_existing_file() -> std::io::Result<()> {
        let query = "you";
        let filename = "poem.txt";
        let config = Config {
            query: String::from(query),
            filename: String::from(filename),
        };

        run(config)
        //assert_eq!(run(config), Ok(()));
    }

    #[test]
    fn run_with_no_existing_file() {
        let query = "you";
        let filename = "hoge.txt";
        let config = Config {
            query: String::from(query),
            filename: String::from(filename),
        };

        match run(config) {
            Ok(_) => panic!("run() returns Ok!"),
            Err(e) => {
                assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
            }
        }
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
