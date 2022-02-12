use std::error::Error;
use std::fs;
use std::env;

pub fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(input.filename)?;
    let results = if input.case_sensitive {
        search(&input.query, &contents)
    } else {
        search_case_insensitive(&input.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Input {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Input {
    pub fn new(args: &[String]) -> Result<Input, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err() || env::var("CASE_INSENSITIVE").unwrap() == "0";

        Ok(Input { query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "to";
        let contents = fs::read_to_string("test.txt").unwrap();
        assert_eq!(vec!["Are you nobody, too?", "How dreary to be somebody!"], search(query, &contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "i'm";
        let contents = fs::read_to_string("test.txt").unwrap();
        assert_eq!(vec!["I'm nobody! Who are you?"], search_case_insensitive(query, &contents));
    }

    #[test]
    fn several_results() {
        let query = "body";
        let contents = fs::read_to_string("test.txt").unwrap();
        assert_eq!(
            vec![
                "I'm nobody! Who are you?",
                "Are you nobody, too?",
                "How dreary to be somebody!"
            ],
            search(query, &contents)
        );
    }
}
