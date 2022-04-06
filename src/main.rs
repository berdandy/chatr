use std::env;
use std::process;
use std::collections::HashMap;

use deku::prelude::*;

// see cpp reference doc
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct BuildTemplate {
    magic: u8,              // must be 0xD
    profession: u8,         // 0-based, IDs on API are 1-based (but not used by armory-embeds, so w/e)

    specialization1: u8,
    #[deku(bits = "2")]
    trait_padding_1: u8,
    #[deku(bits = "2")]
    trait_grandmaster_1: u8,
    #[deku(bits = "2")]
    trait_master_1: u8,
    #[deku(bits = "2")]
    trait_adept_1: u8,

    specialization2: u8,
    #[deku(bits = "2")]
    trait_padding_2: u8,
    #[deku(bits = "2")]
    trait_grandmaster_2: u8,
    #[deku(bits = "2")]
    trait_master_2: u8,
    #[deku(bits = "2")]
    trait_adept_2: u8,

    specialization3: u8,
    #[deku(bits = "2")]
    trait_padding_3: u8,
    #[deku(bits = "2")]
    trait_grandmaster_3: u8,
    #[deku(bits = "2")]
    trait_master_3: u8,
    #[deku(bits = "2")]
    trait_adept_3: u8,

    terrestrial_healing_skill_palette: u16,
    aquatic_healing_skill_palette: u16,
    terrestrial_utility_skill_palette_1: u16,
    aquatic_utility_skill_palette_1: u16,
    terrestrial_utility_skill_palette_2: u16,
    aquatic_utility_skill_palette_2: u16,
    terrestrial_utility_skill_palette_3: u16,
    aquatic_utility_skill_palette_3: u16,
    terrestrial_elite_skill_palette: u16,
    aquatic_elite_skill_palette: u16,
}

fn get_trait_ids(specs: [u8; 3]) -> Result<HashMap<u8, [u16; 9]>, serde_json::Error> {
    let mut trait_map = HashMap::new();

    for spec_id in specs {
        let request_url = format!("https://api.guildwars2.com/v2/specializations/{}", spec_id);
        let spec_data = reqwest::blocking::get(request_url).unwrap().text().unwrap();
        // println!("{:?}", spec_data);

        // Parse the string of data into serde_json::Value.
        let v: serde_json::Value = serde_json::from_str(&spec_data)?;
        let trait_ids: [u16; 9] = [
            v["major_traits"][0].as_u64().unwrap() as u16,
            v["major_traits"][1].as_u64().unwrap() as u16,
            v["major_traits"][2].as_u64().unwrap() as u16,
            v["major_traits"][3].as_u64().unwrap() as u16,
            v["major_traits"][4].as_u64().unwrap() as u16,
            v["major_traits"][5].as_u64().unwrap() as u16,
            v["major_traits"][6].as_u64().unwrap() as u16,
            v["major_traits"][7].as_u64().unwrap() as u16,
            v["major_traits"][8].as_u64().unwrap() as u16
        ];
        trait_map.insert(spec_id, trait_ids);
    }

    Ok(trait_map)
}

fn print_armory_code(build: BuildTemplate, trait_ids_by_spec : HashMap<u8, [u16;9]>) {
    let trait_ids1 = trait_ids_by_spec[&build.specialization1];
    let trait_ids2 = trait_ids_by_spec[&build.specialization2];
    let trait_ids3 = trait_ids_by_spec[&build.specialization3];

    println!("\
<div
  data-armory-embed='specializations'
  data-armory-ids='{spec1},{spec2},{spec3}'
  data-armory-{spec1}-traits='{trait11},{trait12},{trait13}'
  data-armory-{spec2}-traits='{trait21},{trait22},{trait23}'
  data-armory-{spec3}-traits='{trait31},{trait32},{trait33}'
>
</div>",
	spec1=&build.specialization1,
	spec2=&build.specialization2,
	spec3=&build.specialization3,
    trait11=trait_ids1[(build.trait_adept_1 - 1) as usize],
	trait12=trait_ids1[(build.trait_master_1 + 3 - 1) as usize],
	trait13=trait_ids1[(build.trait_grandmaster_1 + 6 - 1) as usize],
	trait21=trait_ids2[(build.trait_adept_2 - 1) as usize],
	trait22=trait_ids2[(build.trait_master_2 + 3 - 1) as usize],
	trait23=trait_ids2[(build.trait_grandmaster_2 + 6 - 1) as usize],
	trait31=trait_ids3[(build.trait_adept_3 - 1) as usize],
	trait32=trait_ids3[(build.trait_master_3 + 3 - 1) as usize],
	trait33=trait_ids3[(build.trait_grandmaster_3 + 6 - 1) as usize]
);
}

fn remove_chatcode_decoration(code: &str) -> &str {
    if ! code.starts_with("[&") || ! code.ends_with("]") {
        return code
    }
    eprintln!("Input: {}, trimming chat characters", code);
    let mut chars = code.chars();
    chars.next();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <chat-code>", args[0]);
        process::exit(1);
    }

    let input = &args[1];
    let chatcode = remove_chatcode_decoration(input);
    eprintln!("Deciphering {}", chatcode);

    let data = base64::decode(chatcode)
        .expect("invaid base64");

    let (_rest, build) = BuildTemplate::from_bytes((data.as_ref(), 0)).unwrap();

    // println!("{:?}", build);

    let trait_ids_by_spec = get_trait_ids([build.specialization1, build.specialization2, build.specialization3]).unwrap();
    // println!("{:?}", trait_ids_by_spec);

    print_armory_code(build, trait_ids_by_spec);
}
