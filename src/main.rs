mod chatbuildr;

use base64::engine::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use deku::DekuContainerRead;

use std::error::Error;
use std::env;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 || args.len() > 3 {  // optional -d
        println!("Usage: {} [-d] <chat-code>", args[0]);
        process::exit(1);
    }

    let debug_code = args[1] == "-d";

    let input = &args[args.len()-1];
    let (chatcode, decorated) = chatbuildr::fix_chatcode_decoration(input);
    println!("`{}`\n\n---\n", decorated);

    let data = BASE64.decode(chatcode)
        .expect("invaid base64");

    let (_rest, build) = chatbuildr::BuildTemplate::from_bytes((data.as_ref(), 0))?;

    if debug_code {
        eprintln!("{:?}", build);
    }

    println!("{:?}", chatbuildr::armory_markup(build)?);

    Ok(())
}
