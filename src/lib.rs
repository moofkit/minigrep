use std::fs;
use std::error::Error;

pub fn run(input: Input) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(input.filename)?;
  println!("With text:\n{}", contents);

  Ok(())
}

pub struct Input {
  pub query: String,
  pub filename: String
}

impl Input {
  pub fn new(args: &[String]) -> Result<Input, &str> {
      if args.len() < 3 {
          return Err("not enough arguments")
      }
      let query = args[1].clone();
      let filename = args[2].clone();

      Ok(Input { query, filename })
  }
}
