use minigrep::Input;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = Input::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);

        process::exit(1);
    });

    if let Err(err) = minigrep::run(input) {
        println!("Application error: {}", err);

        process::exit(1);
    }
}
