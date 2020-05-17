use std::env;

pub fn initialize(){
    let result = get_args();
    println!("yey");
}

// @ Mike Dula
// @ 2020-5-17 18:19
// @ function to get args passed by the user.
// @ should return a String if an error occurs,
// @ and a Vec<String> for the args if successful
fn get_args() -> Result<Vec<String>, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        let err_message = 
            "more args needed.\n\
            1st would be path of message to encrypt\n\
            2nd would be the encryption key\n\
            example: `cipher message.txt secret_key`\n\
            or run: `cipher --help` for more info\n\
            terminating program… kthxbye…";
        Err(err_message.to_string())
    } else {
        Ok(args)
    }
}
