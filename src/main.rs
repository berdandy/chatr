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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <chat-code>", args[0]);
        process::exit(1);
    }

    let chatcode = &args[1];
    println!("Deciphering {}", chatcode);

    let data = base64::decode(chatcode)
        .expect("invaid base64");

    let (_rest, build) = BuildTemplate::from_bytes((data.as_ref(), 0)).unwrap();

    println!("{:?}", build);

    // hardcoded for testing, but this should hit the GW2 API at https://api.guildwars2.com/v2/specializations/<id>
    let trait_ids_by_specialization = HashMap::from([
        (1, [701, 705, 700, 1889, 1960, 708, 692, 1950, 704]),      // mesmer - dueling
        (6, [514, 525, 1882, 482, 1892, 1944, 1541, 505, 1947]),    // engineer - explosives
    ]);

    if trait_ids_by_specialization.contains_key(&build.specialization1)
    {
        let trait_ids = trait_ids_by_specialization[&build.specialization1];
        let trait1 = trait_ids[(build.trait_adept_1 - 1) as usize];
        let trait2 = trait_ids[(build.trait_master_1 + 3 - 1) as usize];
        let trait3 = trait_ids[(build.trait_grandmaster_1 + 6 - 1) as usize];
        println!("Traits: {} {} {}", trait1, trait2, trait3);
    }
}
