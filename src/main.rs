mod chatbuildr;

use base64::engine::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use deku::DekuContainerRead;

use std::error::Error;
use std::env;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 || args.len() > 3 {	// optional -d
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

    let trait_ids_by_spec = chatbuildr::get_trait_ids([build.specialization1, build.specialization2, build.specialization3])?;
    // println!("{:?}", trait_ids_by_spec);

    let skill_ids = chatbuildr::get_skill_ids(&build);
    // println!("{:?}", skill_ids);

	let misc = chatbuildr::get_misc_data_string(&build);
    // println!("{:?}", misc);

	chatbuildr::print_armory_code(build, skill_ids?, trait_ids_by_spec, misc?);

	Ok(())
}
