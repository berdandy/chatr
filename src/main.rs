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
    let build = chatr::BuildTemplate::from_string(input);

    if debug_code {
        eprintln!("Decoded:\n{:?}\n", build);
        eprintln!("Skills:\n{:?} \n", build.get_skill_ids()?);
    }

    println!("{}", chatr::markup::armory(build)?);

    Ok(())
}
