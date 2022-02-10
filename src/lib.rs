use std::error::Error;
use std::fs;

pub fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(input.filename)?;
    for line in search(&input.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub struct Input {
    pub query: String,
    pub filename: String,
}

impl Input {
    pub fn new(args: &[String]) -> Result<Input, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Input { query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "reary";
        let contents = fs::read_to_string("test.txt").unwrap();
        assert_eq!(vec!["How dreary to be somebody!"], search(query, &contents));
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
