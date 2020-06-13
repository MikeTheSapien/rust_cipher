pub struct UserInput {
    pub message_path: String,
    pub key: u32,
}

impl UserInput {
    pub fn new(args: Vec<String>) -> Result<UserInput, &'static str> {
    /*
    @ Mike Dula
    @ 2020-5-27 20:30
    @ constructor for UnserInput struct
    ===params===
    args: Vec<String> ==> the arguments passed by the user,
                            where 1st would be for the path of the file to be  
                            encrypted.
                            2nd would be the key to encrypt the file.

    ===returns===
    UserInput ==> if parsing of arguments is successful
    &str ==> if parsing failed, an error message is returned
    */
        if args.len() < 3 {
            let err_message = "more args needed. \
             1st would be path of message to encrypt. \
             2nd would be the encryption key.\n\
             example: `cipher message.txt secret_key`\n\
             or run: `cipher --help` for more info\n\
             terminating program… kthxbye…";
            Err(err_message)
        } else {
            let key = args[2].parse::<u32>().unwrap_or_else(|err| {
                println!("key must be a positive integer. {}", err);
                0
            });
            // let key = match args[2].parse::<u32>() {
            //     Ok(n) => n,
            //     Err(e) => e,
            // };

            let config = UserInput {
                message_path: args[1].clone(),
                key,
            };
            Ok(config)
        }
    }
}
