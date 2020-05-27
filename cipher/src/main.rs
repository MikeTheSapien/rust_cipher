use std::process;
use std::env;
use cipher::models::user_input::UserInput;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = UserInput::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = cipher::encrypt(config) {
        println!("App error: {}", e);
        process::exit(1);
    }
}
