use std::fs;
use std::error::Error;

pub mod models;

use models::user_input::UserInput;

pub fn encrypt(ui: UserInput) -> Result<(), Box<dyn Error>> {
    println!("printing with key: {}", ui.key);
    let message = fs::read_to_string(ui.message_path)?;
    // for c in message.bytes() {
    for c in message.chars() {
        // const RADIX: u32 = 10;
        // println!("{:?}",c.to_digit(RADIX));
        // let new = u32::from_be_bytes(c);
        println!("{}", c as u32);
    }
    // println!("{:#?}",message.chars());
    Ok(())
}