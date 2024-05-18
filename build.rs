use std::fs;
use std::path::Path;
use std::process::Command;

fn api_dump(search_flag: &str, json_filename: &str) -> std::io::Result<()> {
	let flag_arg = format!("-j{}", search_flag);
	let output = Command::new("gw2search")
		.arg(flag_arg)
		.output()
		.expect("error with gw2search");

    let dest_path = Path::new(json_filename);
	fs::write(&dest_path, output.stdout)
}

fn main() {
	api_dump("p", "src/professions.json").unwrap();
	api_dump("l", "src/legends.json").unwrap();
	api_dump("S", "src/specializations.json").unwrap();
	api_dump("P", "src/pets.json").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
