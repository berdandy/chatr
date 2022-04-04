use std::str;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <chat-code>", args[0]);
        process::exit(1);
    }

    let chatcode = &args[1];
    println!("Deciphering {}", chatcode);

    let decoded = base64::decode(chatcode).unwrap();
    println!("{:?}", str::from_utf8(&decoded));
}
