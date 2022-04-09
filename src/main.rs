use std::env;
use std::process;
use std::collections::HashMap;

use deku::prelude::*;

// see cpp reference doc
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct BuildTemplate {
    magic: u8,              // must be 0xD
    profession: u8,         // 0-based, IDs on API are 1-based

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

    // only valid if ranger (profession==4) or revenant (profession==9)
    terrestrial_pet1_active_legend: u8,
    terrestrial_pet2_inactive_legend: u8,
    aquatic_pet1_active_legend: u8,
    aquatic_pet2_inactive_legend: u8,
}

const PROFESSIONS: &'static [&str] = &[
    "Guardian",
    "Warrior",
    "Engineer",
    "Ranger",
    "Thief",
    "Elementalist",
    "Mesmer",
    "Necromancer",
    "Revenant"
  ];

fn get_skill_ids(build: &BuildTemplate) -> Result<[u16; 5], serde_json::Error> {
    // skills:
    //   build contains palette ids
    //   palette ids are mapped to ability ids via https://api.guildwars2.com/v2/professions/{PROFESSIONS[build.profession]}

    let profession_id = PROFESSIONS[(build.profession - 1) as usize];
    let request_url = format!("https://api.guildwars2.com/v2/professions/{}?v=2019-12-19T00:00:00Z", profession_id);
    let palette_data = reqwest::blocking::get(request_url).unwrap().text().unwrap();
    // println!("{:?}", palette_data);

    // Parse the string of data into serde_json::Value.
    let v: serde_json::Value = serde_json::from_str(&palette_data)?;
    let a = v["skills_by_palette"].as_array().unwrap();
    // println!("{:?}", a);
    let mut skill_palette_map = HashMap::new();
    for mapping in a {
        let skills_by_palette = mapping.as_array().unwrap();
        skill_palette_map.insert(
            skills_by_palette[0].as_u64().unwrap() as u16,
            skills_by_palette[1].as_u64().unwrap() as u16
        );
    }
    // println!("{:?}", skill_palette_map);

    let skills: [u16 ; 5] = [
        skill_palette_map[&build.terrestrial_healing_skill_palette],
        skill_palette_map[&build.terrestrial_utility_skill_palette_1],
        skill_palette_map[&build.terrestrial_utility_skill_palette_2],
        skill_palette_map[&build.terrestrial_utility_skill_palette_3],
        skill_palette_map[&build.terrestrial_elite_skill_palette],
    ];
    Ok(skills)
}

// ranger: https://api.guildwars2.com/v2/pets/{}
// - Ranger pets aren't supported by armory-embeds. We'll have to roll our own. /v2/pets API gives
//   a link to a png file for `icon`. We can render that. Also a caption for the pet `name` just to
//   be clear about things.
//
// revenant: https://api.guildwars2.com/v2/legends/Legend{}
// - Revenant skills DO NOT use the skill palette. /v2/legends API gives a structure with `swap`,
//   `heal`, `elite` and an array for `utilities`. We can use that. Or just not bother at all for
//   revenant. Players can't change them.

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

/*
fn print_armory_code(build: BuildTemplate, skill_ids_by_palette : HashMap<u8, [u16;9]>) {
<div
    data-armory-embed='skills'
    data-armory-ids='5857,5927,5912,5836,5868'
>
</div>
*/

fn print_armory_code(build: BuildTemplate, skills: [u16; 5], trait_ids_by_spec : HashMap<u8, [u16;9]>) {
    let trait_ids1 = trait_ids_by_spec[&build.specialization1];
    let trait_ids2 = trait_ids_by_spec[&build.specialization2];
    let trait_ids3 = trait_ids_by_spec[&build.specialization3];

    println!("\
<div
  data-armory-embed='skills'
  data-armory-ids='{healing},{utility1},{utility2},{utility3},{elite}'
>
</div>
<div
  data-armory-embed='specializations'
  data-armory-ids='{spec1},{spec2},{spec3}'
  data-armory-{spec1}-traits='{trait11},{trait12},{trait13}'
  data-armory-{spec2}-traits='{trait21},{trait22},{trait23}'
  data-armory-{spec3}-traits='{trait31},{trait32},{trait33}'
>
</div>
<script async src='https://unpkg.com/armory-embeds@^0.x.x/armory-embeds.js'></script>
",
    healing=skills[0],
    utility1=skills[1],
    utility2=skills[2],
    utility3=skills[3],
    elite=skills[4],
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

fn fix_chatcode_decoration(input: &String) -> (String, String) { // code, decorated
    if input.starts_with("[&") && input.ends_with("]") {
		let mut raw = input.chars();
		raw.next();
		raw.next();
		raw.next_back();

        return (String::from(raw.as_str()), input.to_string())
    } else {
		let decorated = format!("[&{}]", input);
		return (input.to_string(), decorated.to_string())
	}
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <chat-code>", args[0]);
        process::exit(1);
    }

    let input = &args[1];
    let (chatcode, decorated) = fix_chatcode_decoration(input);
    println!("`{}`\n\n---\n", decorated);

    let data = base64::decode(chatcode)
        .expect("invaid base64");

    let (_rest, build) = BuildTemplate::from_bytes((data.as_ref(), 0)).unwrap();

    // eprintln!("DEBUG DUMP: {:?}", build);

    let trait_ids_by_spec = get_trait_ids([build.specialization1, build.specialization2, build.specialization3]).unwrap();
    // println!("{:?}", trait_ids_by_spec);

    let skill_ids = get_skill_ids(&build);
    // println!("{:?}", skill_ids);

    print_armory_code(build, skill_ids.unwrap(), trait_ids_by_spec);
}
