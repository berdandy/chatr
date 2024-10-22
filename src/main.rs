use std::error::Error;
use std::env;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 || args.len() > 3 {  // optional -d
        println!("Usage: {} [-d | -D] <chat-code>", args[0]);
        process::exit(1);
    }

    let debug_code = args[1] == "-d";
    let depgen = args[1] == "-D";

    let input = &args[args.len()-1];
    if let Ok(code) = chatr::ChatCode::build(input) {

        if let Ok(skill) = chatr::Skill::try_from_chatcode(&code) {
            if debug_code {
                eprintln!("Decoded:\n{:?}\n", skill);
            }

            println!("Skill: {}", skill.id);
        }

        if let Ok(build) = chatr::BuildTemplate::try_from_chatcode(&code) {
            if debug_code {
                eprintln!("Decoded:\n{:?}\n", build);
                eprintln!("Skills:\n{:?} \n", build.get_skill_ids()?);
            }

            if depgen {
                println!("{{ 'specializations': {:?}, 'traits': {:?}, 'skills': {:?} }}",
                    build.get_specializations(),
                    build.get_traits(),
                    build.get_skill_ids()?
                );
            } else {
                println!("{}", chatr::markup::armory(build)?);
            }
        }
    }

    Ok(())
}
