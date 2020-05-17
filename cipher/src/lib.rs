use std::{env,process};

pub fn initialize(){
    get_args();
}

fn get_args() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!(
            "more args needed.\n\
            1st would be path of message to encrypt\n\
            2nd would be the encryption key\n\
            example: `cipher message.txt secret_key`\n\
            or run: `cipher --help` for more info\n\
            terminating program… kthxbye…"
                    );
        process::exit(0);
    }
}
