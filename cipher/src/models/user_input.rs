pub struct UserInput {
    message_path: String,
    key: String,
}

impl UserInput{
    pub fn new(args: &[String]) -> UserInput {
        UserInput {
            message_path: args[1].clone(),
            key: args[2].clone(),
        }
    }
}
