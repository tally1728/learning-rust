use std::fs::File;
// to get read_to_string() method enabled on File Struct
use std::env;
use std::io::Read;

#[derive(PartialEq, Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        // pub fn var<K: AsRef<OsStr>>(key: K) -> Result<String, VarError>
        // pub const fn is_err(&self) -> bool
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
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

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
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

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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
    use super::search_case_insensitive;
    use super::Config;

    #[test]
    fn config_new_with_1arg() {
        let query = "you";
        let args = vec![String::from("cmd"), String::from(query)];

        assert_eq!(Config::new(&args), Err("not enough arguments"));
    }

    #[test]
    fn config_new_with_2args_wo_option() {
        let query = "you";
        let filename = "poem.txt";
        let args = vec![
            String::from("cmd"),
            String::from(query),
            String::from(filename),
        ];

        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, String::from(query));
        assert_eq!(config.filename, String::from(filename));
    }

    #[test]
    fn run_with_an_existing_file() -> std::io::Result<()> {
        let query = "you";
        let filename = "poem.txt";
        let config = Config {
            query: String::from(query),
            filename: String::from(filename),
            case_sensitive: true,
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
            case_sensitive: true,
        };

        match run(config) {
            Ok(_) => panic!("run() returns Ok!"),
            Err(e) => {
                assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
            }
        }
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
