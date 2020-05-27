use cipher;
use std::process;
use std::env;
use cipher::models::user_input::UserInput;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = UserInput::new(args);
    // if let Err(e) = cipher::run(config) {
    //     println!("App error: {}", e);
    //     process::exit(1);
    // }
}
