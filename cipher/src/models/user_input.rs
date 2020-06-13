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
        Err(&str) ==> if parsing failed, an error message is returned
        */
        if args.len() < 3 {
            let err_message = "more args needed. \
             1st would be path of message to encrypt. \
             2nd would be the encryption key that is an integer.\n\
             example: `cipher message.txt 3`\n\
             or run: `cipher --help` for more info\n\
             terminating program… kthxbye…";
            return Err(err_message);
        }
        let key = args[2].parse::<u32>().unwrap_or_else(|_err| {
            0
        });

        if key == 0 {
            let err_message = "key must be a positive integer.";
            return Err(err_message);
        }

        let config = UserInput {
            message_path: args[1].clone(),
            key,
        };
        Ok(config)
    }
}