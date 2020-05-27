use std::fs;
use std::error::Error;

pub mod models;

use models::user_input::UserInput;

pub fn run(ui: UserInput) -> Result<(), Box<dyn Error>> {
    let message = fs::read_to_string(ui.message_path)?;
    println!("{}", message);
    Ok(())
}