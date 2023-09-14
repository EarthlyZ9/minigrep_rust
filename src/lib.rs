use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 아래 함수가 반환하는 값이 Error 타입이면 자동으로 리턴될 것임
    let contents = fs::read_to_string(config.filename)?;

    for (index, line) in search(&config.query, &contents, config.case_sensitive).iter().enumerate() {
        println!("Hit {}: {}", index + 1, line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("Not enough arguments!".to_string());
        }

        // ownership 을 넘겨받지 않기 위해서 clone
        // not efficient but for now...
        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        let mut is_case_sensitive: bool = true;
        if args.len() == 4 {
            if !args[3].eq("--case-insensitive") {
                return Err(format!("No option of {}", args[4]));
            }
            is_case_sensitive = false;
        }

        Ok(Config { query, filename, case_sensitive: is_case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str, is_case_sensitive: bool) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if is_case_sensitive {
            if line.contains(query) {
                results.push(line);
            }
        } else {
            // case insensitive
            let query: String = query.to_lowercase();
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }
    }
    results
}

// TDD - test module
#[cfg(test)]
mod tests {
    // inherit all packages from parent

    use crate::search;

    #[test]
    fn case_sensitive() {
        let query: &str = "duct";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents, true));
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "rUsT";
        let contents: &str = "\
        Rust:\
        safe, fast, productive.\
        Pick three.\
        Trust me.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents, false));
    }
}

