use std::env;
use std::process;
use minigrep::Input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = Input::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);

        process::exit(1);
    });

    println!("Searching for: {}", input.query);
    println!("In file: {}", input.filename);
    if let Err(err) = minigrep::run(input) {
        println!("Application error: {}", err);

        process::exit(1);
    }
}
