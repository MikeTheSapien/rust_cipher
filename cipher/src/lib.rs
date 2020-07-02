use std::fs;
use std::error::Error;

pub mod models;

use models::user_input::UserInput;

pub fn encrypt(ui: UserInput) -> Result<(), Box<dyn Error>> {
    println!("printing with key: {}", ui.key);
    let message = fs::read_to_string(ui.message)?;
    for c in message.chars() {
        println!("{}", c as u32 + ui.key);
    }
    Ok(())
}