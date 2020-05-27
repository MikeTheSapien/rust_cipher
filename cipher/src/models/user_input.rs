pub struct UserInput {
    message_path: String,
    key: String,
}

impl UserInput{

    // @ Mike Dula
    // @ 2020-5-27 19:43
    // @ constructor for UserInput struct
    // ==params==
    // args: Vec<String> => the arguments passed by the user, where
    //                      1st would be for the path of the file
    //                      to be encrypted.
    //                      2nd would be the key to encrypt the file.
    pub fn new(args: Vec<String>) -> UserInput {
        UserInput {
            message_path: args[1].clone(),
            key: args[2].clone(),
        }
    }
}
