use std::fs;
use std::error::Error;
use std::env;

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // move past executable name

        let query =  match args.next() {
            Some(arg) => arg,
            None => return Err("Expected a query string"),
        };

        let filename =  match args.next() {
            Some(arg) => arg,
            None => return Err("Expected a filename")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }

    pub fn get_description(&self) -> String {
        format!("Searching for {} in {}", self.query, self.filename)
    }
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    // println!("Cfg: {:?}", cfg);
    // println!("Description: {}", cfg.get_description());

    let contents = fs::read_to_string(cfg.filename)?;

    for line in search(&cfg.query, &contents, cfg.case_sensitive) {
        println!("Match found: {}", line);
    }
    
    // println!("Contents: {}", contents);
    Ok(())
}

// Below is an example of documentation for a function. # Examples section is used, but other
// common sections include # Panics, # Errors (if Result is returned), and # Safety (if there is unsafe code)

// HOLY SHIT the code here actually gets run as a test HOW COOL IS THIS

// I think this only works from a library? Not sure. Apparently you can do this in python as well.

/// Searches a multi-line string for a query and returns lines containing that string
///
/// # Examples
///
/// ```
/// let q = "my query";
/// let ml_str = "\
/// this string contains
/// my query which
/// is awesome.";
///
/// assert_eq!(vec!["my query which"], minigrep::search(q, ml_str, true));
/// ```
pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let stz_qry = if case_sensitive {
        query.to_string()
    } else {
        query.to_lowercase()
    };

    contents
        .lines()
        .filter(|line| if case_sensitive {
            line.contains(&stz_qry)
        } else {
            line.to_lowercase().contains(&stz_qry)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let qry = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(qry, contents, false));
    }

    #[test]
    fn case_sensitive() {
        let qry = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duck tape.";

        assert_eq!(vec!["safe, fast, productive."], search(qry, contents, false));
    }

    #[test]
    fn case_insensitive() {
        let qry = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(qry, contents, false));
    }
}