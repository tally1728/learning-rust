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
    // Arguments: `[-i/--ignore-case] <query> <filename>`
    // <query> & <filename> must be specified => Err("not enough argument")
    // unsupported option => Err("unrecognized option")
    // `-i --another-option <query> <filename>` => Err("at most 1 option is allowed")
    // unnecessary arguments after the 3rd will be ignored
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let query;
        let filename;

        // pub fn var<K: AsRef<OsStr>>(key: K) -> Result<String, VarError>
        // pub const fn is_err(&self) -> bool
        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        // to a Vector of Arg Enum
        let args: Vec<Arg> = args.iter().map(|s| Arg::parse(s)).collect();

        match &args[1] {
            Arg::IgnoreCaseOption => {
                if args.len() < 4 {
                    return Err("not enough arguments");
                }

                // the CLI Option takes priority!
                case_sensitive = false;

                if let Arg::String(s) = &args[2] {
                    query = s.clone();
                } else {
                    // --ignore-case --another-one <query> <filename>
                    return Err("at most 1 option is allowed");
                }

                if let Arg::String(s) = &args[3] {
                    filename = s.clone();
                } else {
                    // --ignore-case <query> --another-one <filename>
                    return Err("at most 1 option is allowed");
                }
            }
            Arg::WrongOption(_) => {
                // --wrong-option <query> <filename>
                return Err("unrecognized option");
            }
            Arg::String(_) => {
                if args.len() < 3 {
                    return Err("not enough arguments");
                }

                if let Arg::String(s) = &args[1] {
                    query = s.clone();
                } else {
                    // <query> -i <filename>
                    return Err("options must be specified before pattern and filename");
                }

                if let Arg::String(s) = &args[2] {
                    filename = s.clone();
                } else {
                    // <query> <filename> -i
                    return Err("options must be specified before pattern and filename");
                }
            }
        }

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[derive(PartialEq, Debug)]
enum Arg {
    IgnoreCaseOption,
    WrongOption(String),
    String(String),
}

impl Arg {
    fn parse(s: &str) -> Arg {
        match s {
            "-i" | "--ignore-case" => Arg::IgnoreCaseOption,
            s if s.starts_with("-") => Arg::WrongOption(String::from(s)),
            s => Arg::String(String::from(s)),
        }
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
    use super::Arg;
    use super::Config;

    #[test]
    fn config_new_length1() {
        let args = vec![String::from("cmd")];

        assert_eq!(Config::new(&args), Err("not enough arguments"));
    }

    #[test]
    fn config_new_length2_1query() {
        let query = "you";
        let args = vec![String::from("cmd"), String::from(query)];

        assert_eq!(Config::new(&args), Err("not enough arguments"));
    }

    #[test]
    fn config_new_length2_1_option() {
        let option = "--ignore-case";
        let args = vec![String::from("cmd"), String::from(option)];

        assert_eq!(Config::new(&args), Err("not enough arguments"));
    }

    #[test]
    fn config_new_length2_1wrong_option() {
        let option = "--wrong-one";
        let args = vec![String::from("cmd"), String::from(option)];

        assert_eq!(Config::new(&args), Err("unrecognized option"));
    }

    #[test]
    fn config_new_length3_1option_1query() {
        let option = "--ignore-case";
        let query = "you";
        let args = vec![
            String::from("cmd"),
            String::from(option),
            String::from(query),
        ];

        assert_eq!(Config::new(&args), Err("not enough arguments"));
    }

    #[test]
    fn config_new_length3_1wrong_option_1query() {
        let option = "--wrong-one";
        let query = "you";
        let args = vec![
            String::from("cmd"),
            String::from(option),
            String::from(query),
        ];

        assert_eq!(Config::new(&args), Err("unrecognized option"));
    }

    #[test]
    fn config_new_length3_1query_1filename() {
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
    fn config_new_length3_1query_1option() {
        let query = "you";
        let option = "-i";
        let args = vec![
            String::from("cmd"),
            String::from(query),
            String::from(option),
        ];

        assert_eq!(
            Config::new(&args),
            Err("options must be specified before pattern and filename")
        );
    }

    #[test]
    fn config_new_length4_1option_1query_1filename() {
        let option = "--ignore-case";
        let query = "you";
        let filename = "poem.txt";
        let args = vec![
            String::from("cmd"),
            String::from(option),
            String::from(query),
            String::from(filename),
        ];

        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, String::from(query));
        assert_eq!(config.filename, String::from(filename));
        assert_eq!(config.case_sensitive, false);
    }

    #[test]
    fn config_new_length4_2options_1query() {
        let option_1 = "--ignore-case";
        let option_2 = "--another-one";
        let query = "you";
        let args = vec![
            String::from("cmd"),
            String::from(option_1),
            String::from(option_2),
            String::from(query),
        ];

        assert_eq!(Config::new(&args), Err("at most 1 option is allowed"));
    }

    #[test]
    fn config_new_length4_1wrong_option_1query_1filename() {
        let option = "--wrong-one";
        let query = "you";
        let filename = "poem.txt";
        let args = vec![
            String::from("cmd"),
            String::from(option),
            String::from(query),
            String::from(filename),
        ];

        assert_eq!(Config::new(&args), Err("unrecognized option"));
    }

    #[test]
    fn config_new_length4_1query_1filename_something() {
        let query = "you";
        let filename = "poem.txt";
        let s = "something";
        let args = vec![
            String::from("cmd"),
            String::from(query),
            String::from(filename),
            String::from(s),
        ];

        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, String::from(query));
        assert_eq!(config.filename, String::from(filename));
    }

    #[test]
    fn config_new_length4_1query_1option_1filename() {
        let query = "you";
        let option = "-i";
        let filename = "poem.txt";
        let args = vec![
            String::from("cmd"),
            String::from(query),
            String::from(option),
            String::from(filename),
        ];

        assert_eq!(
            Config::new(&args),
            Err("options must be specified before pattern and filename")
        );
    }

    #[test]
    fn arg_parse_i_option() {
        let arg = "-i";
        assert_eq!(Arg::parse(arg), Arg::IgnoreCaseOption);
    }

    #[test]
    fn arg_parse_ignore_case_option() {
        let arg = "--ignore-case";
        assert_eq!(Arg::parse(arg), Arg::IgnoreCaseOption);
    }

    #[test]
    fn arg_parse_wrong_option() {
        let arg = "--wrong-one";
        assert_eq!(Arg::parse(arg), Arg::WrongOption(String::from(arg)));
    }

    #[test]
    fn arg_parse_string() {
        let arg = "hello";
        assert_eq!(Arg::parse(arg), Arg::String(String::from(arg)));
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
