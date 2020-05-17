use std::{env,process,fs};
use std::error::Error;

pub mod models;

pub fn initialize(){
    let result = get_args();
    if let Err(err) = result {
        println!("{}",err);
        process::exit(0);
    }
}

//fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
//}

// @ Mike Dula
// @ 2020-5-17 18:19
// @ function to get args passed by the user.
// @ should return a Err(&str) if an error occurs,
// @ and a Ok(Vec<String>) for the args if successful
fn get_args() -> Result<Vec<String>, &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        let err_message = 
            "more args needed.\
            1st would be path of message to encrypt. \
            2nd would be the encryption key.\n\
            example: `cipher message.txt secret_key`\n\
            or run: `cipher --help` for more info\n\
            terminating program… kthxbye…";
        Err(err_message)
    } else {
        Ok(args)
    }
}
