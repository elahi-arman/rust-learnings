use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

// () - the unit type, Box -- function returns a type with the Error triat
pub fn run(config: Config) -> Result<(), Box<Error>>{

    // ? returns the error value from current function for caller to handle
    let mut f = File::open(config.filename)?;

    // the function borrows contents
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
       println!("{}", line);
   }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // successful will return a Config, unsuccessful returns an error string
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            // use a Result type so that we can exectively process this
            return Err("Not enough arguments need two arguments: pattern & filename");
        }
        // make sure that the returned Config structure owns its values
        // has a serious runtime cost, so probably will have it change
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {query, filename})
    }
}

// the returned vector should contain string slices that
// reference slices of the contents argument, not query
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query){
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }


    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
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
