use std::collections::HashMap;
use std::error::Error;

use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;

// see docs/build_template_reference.cpp

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct PalettePair {
	terrestrial: u16,
	aquatic: u16,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
struct InactiveLegendUtilitiesTriple {
	utilities: [u16; 3]
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct InactiveLegendUtilities {
	terrestrial: InactiveLegendUtilitiesTriple,
	aquatic: InactiveLegendUtilitiesTriple,
}

// new with SotO
#[derive(Debug, PartialEq, Default, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian", ctx_default = "deku::ctx::Endian::Little")]
pub struct WeaponMastery {
    #[deku(update = "self.weapon_palette_ids.len()")]
    weapon_palette_count: u8,
    #[deku(count = "weapon_palette_count")]
    weapon_palette_ids: Vec<u16>,

    #[deku(update = "self.weapon_variant_skill_ids.len()")]
    weapon_variant_skill_count: u8,
    #[deku(count = "weapon_variant_skill_count")]
    weapon_variant_skill_ids: Vec<u32>,
}

// if there's trailing SotO weapon data in the chat code, this will handle it
impl WeaponMastery {
    fn optional_read(
        rest: &BitSlice<u8, Msb0>,
    ) -> Result<(&BitSlice<u8, Msb0>, WeaponMastery), DekuError> {
		match rest.is_empty() {
			true => {
				Ok((rest, Default::default()))
			}
			false => {
				Ok(WeaponMastery::read(rest, ())?)
			}
		}
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct BuildTemplate {
    magic: u8,              // must be 0xD
    pub profession: u8,         // 0-based, IDs on API are 1-based

    pub specialization1: u8,
    #[deku(bits = "2", pad_bits_before = "2")]
    pub trait_grandmaster_1: u8,
    #[deku(bits = "2")]
    pub trait_master_1: u8,
    #[deku(bits = "2")]
    pub trait_adept_1: u8,

    pub specialization2: u8,
    #[deku(bits = "2", pad_bits_before = "2")]
    pub trait_grandmaster_2: u8,
    #[deku(bits = "2")]
    pub trait_master_2: u8,
    #[deku(bits = "2")]
    pub trait_adept_2: u8,

    pub specialization3: u8,
    #[deku(bits = "2", pad_bits_before = "2")]
    pub trait_grandmaster_3: u8,
    #[deku(bits = "2")]
    pub trait_master_3: u8,
    #[deku(bits = "2")]
    pub trait_adept_3: u8,

	pub healing: PalettePair,      	// 4B
	pub utility: [PalettePair; 3], 	// 12B
	pub elite: PalettePair,			// 4B

    // only valid contents if ranger (profession==4) or revenant (profession==9), otherwise forcibly zero out
    #[deku(cond = "*profession == 4 || *profession == 9", default = "0")]
    pub terrestrial_pet1_active_legend: u8,
    #[deku(cond = "*profession == 4 || *profession == 9", default = "0")]
    pub terrestrial_pet2_inactive_legend: u8,
    #[deku(cond = "*profession == 4 || *profession == 9", default = "0")]
    pub aquatic_pet1_active_legend: u8,
    #[deku(cond = "*profession == 4 || *profession == 9", default = "0")]
    pub aquatic_pet2_inactive_legend: u8,

	// on a revenant, these 12 bytes preserves skill order for inactive legend utilities; ignored otherwise but always present
	pub inactive_legend_utilities: InactiveLegendUtilities,

	// post-SotO, chat codes may have optional additional data on read
    #[deku(reader = "WeaponMastery::optional_read(deku::rest)")]
	pub weapons: WeaponMastery,
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

pub fn get_skill_ids(build: &BuildTemplate) -> Result<[u16; 5], Box<dyn Error>> {
    // skills:
    //   build contains palette ids
    //   palette ids are mapped to ability ids via https://api.guildwars2.com/v2/professions/{PROFESSIONS[build.profession]}

    let profession_id = PROFESSIONS[(build.profession - 1) as usize];
    let request_url = format!("https://api.guildwars2.com/v2/professions/{}?v=latest", profession_id);
    let palette_data = reqwest::blocking::get(request_url)?.text()?;
    // println!("{:?}", palette_data);

    // Parse the string of data into serde_json::Value.
    let v: serde_json::Value = serde_json::from_str(&palette_data)?;
    let a = v["skills_by_palette"].as_array().ok_or("skills_by_palette issue")?;
    // println!("{:?}", a);
    let mut skill_palette_map = HashMap::new();
    for mapping in a {
        let skills_by_palette = mapping.as_array().ok_or("invalid mapping in skills_by_palette")?;
        skill_palette_map.insert(
            skills_by_palette[0].as_u64().expect("integer") as u16,
            skills_by_palette[1].as_u64().expect("integer") as u16
        );
    }
    // println!("{:?}", skill_palette_map);

    let skills: [u16 ; 5] = [
        skill_palette_map[&build.healing.terrestrial],
        skill_palette_map[&build.utility[0].terrestrial],
        skill_palette_map[&build.utility[1].terrestrial],
        skill_palette_map[&build.utility[2].terrestrial],
        skill_palette_map[&build.elite.terrestrial],
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

pub fn get_ranger_string(pet1: u8, pet2: u8) -> Result<String, Box<dyn Error>> {
	let mut pet_names = "Pets: ".to_string();

	/*
	println!("Checking pets:\n{:?}\n{:?}", 
		format!("https://api.guildwars2.com/v2/pets/{}", pet1),
		format!("https://api.guildwars2.com/v2/pets/{}", pet2)
	);
	*/

	let request_url = format!("https://api.guildwars2.com/v2/pets/{}?v=latest", pet1);
	let pet_data  = reqwest::blocking::get(request_url)?.text()?;
	// println!("{:?}", pet_data);

	let v: serde_json::Value = serde_json::from_str(&pet_data)?;
	// println!(">> {:?}", v);

	if v.as_object().expect("Invalid JSON Object").contains_key("name") {
		pet_names += &String::from(v["name"].as_str().expect("invalid pet1 name"));
	}

	let request_url = format!("https://api.guildwars2.com/v2/pets/{}?v=latest", pet2);
	let pet2_data  = reqwest::blocking::get(request_url)?.text()?;
	// println!("{:?}", pet2_data);

	let v2: serde_json::Value = serde_json::from_str(&pet2_data)?;
	// println!(">> {:?}", v2);

	if v2.as_object().expect("Invalid JSON Object").contains_key("name") {
		// ie, both pets
		if v.as_object().expect("Invalid JSON Object").contains_key("name") {
			pet_names += " / ";
		}
		pet_names += &String::from(v2["name"].as_str().expect("invalid pet2 name"));
	}

	Ok(pet_names)
}

pub fn get_revenant_string(legend1: u8, legend2: u8) -> Result<String, Box<dyn Error>> {
	// get list of legends
	let request_url = format!("https://api.guildwars2.com/v2/legends?v=latest");//, legend1);
	let legend_name_data  = reqwest::blocking::get(request_url)?.text()?;
	// println!("{:?}", legend_name_data);
	let legend_name_v: serde_json::Value = serde_json::from_str(&legend_name_data)?;
	// println!("{:?}", legend_name_v);
	let legend_names = legend_name_v.as_array().ok_or("invalid array of legend names")?;
	// println!("{:?}", legend_names);

	let mut skill_output = "".to_string();

	let mut first = true;
	let mut output = "<div
  data-armory-embed='skills'
  data-armory-nokey=true
  data-armory-ids='".to_string();
	for legend in legend_names {
		let request_url = format!("https://api.guildwars2.com/v2/legends/{}?v=latest", legend.as_str().ok_or("invalid legend")?);
		// println!("{:?}", request_url);
		let legend_data  = reqwest::blocking::get(request_url)?.text()?;
		let v: serde_json::Value = serde_json::from_str(&legend_data)?;

		// println!("{:?}", v);

		let code = v["code"].as_u64().expect("integer") as u8;
		if code == legend1 || code == legend2 {
			let swap = v["swap"].as_u64().expect("integer"); 
			if first {
				output += &format!("{}", swap).to_string();
				first = false;
			} else {
				output += &format!(",{}", swap).to_string();
			}

			skill_output += "<div data-armory-embed='skills' data-armory-ids='";
				skill_output += &format!("{},", v["heal"].as_u64().expect("integer"));
				let utils = v["utilities"].as_array().ok_or("invalid utilities")?;
				for util in utils {
					skill_output += &format!("{},", util.as_u64().expect("integer"));
				}
				skill_output += &format!("{}", v["elite"].as_u64().expect("integer"));
			
			skill_output += "'></div>";
		}
	}
	output += &"'
>
</div>".to_string();


	// println!("{:?}", output);

	Ok(output + &skill_output)
}

// only non-empty if ranger (profession==4) or revenant (profession==9)
pub fn get_misc_data_string(build: &BuildTemplate) -> Result<String, Box<dyn Error>> {
	match build.profession {
		4 => get_ranger_string(build.terrestrial_pet1_active_legend, build.terrestrial_pet2_inactive_legend),
		9 => get_revenant_string(build.terrestrial_pet1_active_legend, build.terrestrial_pet2_inactive_legend),
		_ => Ok(String::new())
	}
}

pub fn get_trait_ids(specs: [u8; 3]) -> Result<HashMap<u8, [u16; 9]>, Box<dyn Error>> {
    let mut trait_map = HashMap::new();

    for spec_id in specs {
        let request_url = format!("https://api.guildwars2.com/v2/specializations/{}?v=latest", spec_id);
        let spec_data = reqwest::blocking::get(request_url)?.text()?;
        // println!("{:?}", spec_data);

        // Parse the string of data into serde_json::Value.
        let v: serde_json::Value = serde_json::from_str(&spec_data)?;
        let trait_ids: [u16; 9] = [
            v["major_traits"][0].as_u64().expect("integer") as u16,
            v["major_traits"][1].as_u64().expect("integer") as u16,
            v["major_traits"][2].as_u64().expect("integer") as u16,
            v["major_traits"][3].as_u64().expect("integer") as u16,
            v["major_traits"][4].as_u64().expect("integer") as u16,
            v["major_traits"][5].as_u64().expect("integer") as u16,
            v["major_traits"][6].as_u64().expect("integer") as u16,
            v["major_traits"][7].as_u64().expect("integer") as u16,
            v["major_traits"][8].as_u64().expect("integer") as u16
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

pub fn print_armory_code(build: BuildTemplate, skills: [u16; 5], trait_ids_by_spec : HashMap<u8, [u16;9]>, misc_text : String) {
    let trait_ids1 = trait_ids_by_spec[&build.specialization1];
    let trait_ids2 = trait_ids_by_spec[&build.specialization2];
    let trait_ids3 = trait_ids_by_spec[&build.specialization3];

	if build.profession != 9 {
    println!("\
{misc}
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
",
	misc=misc_text,
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
	} else {
    println!("\
{misc}
<div
  data-armory-embed='specializations'
  data-armory-ids='{spec1},{spec2},{spec3}'
  data-armory-{spec1}-traits='{trait11},{trait12},{trait13}'
  data-armory-{spec2}-traits='{trait21},{trait22},{trait23}'
  data-armory-{spec3}-traits='{trait31},{trait32},{trait33}'
>
</div>
",
	misc=misc_text,
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
}

pub fn fix_chatcode_decoration(input: &String) -> (String, String) { // code, decorated
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

#[cfg(test)]
mod tests {
	use base64::engine::Engine as _;
	use base64::engine::general_purpose::STANDARD as BASE64;

    use super::*;

    #[test]
    fn trim_chatcode_decoration() {
        let data = "[&123456]".to_string();
        assert_eq!(fix_chatcode_decoration(&data), ("123456".to_string(), "[&123456]".to_string()));
    }

    #[test]
    fn non_chatcode_no_trim() {
        let data = "123456".to_string();
        assert_eq!(fix_chatcode_decoration(&data), ("123456".to_string(), "[&123456]".to_string()));
    }

	#[test]
	fn ranger_code_to_pet_string() {
		let input  = "[&DQQePQgaSDd5AHgAARuWAbUAmgCsAbgADxvtAC87KhUAAAAAAAAAAAAAAAA=]".to_string();
		let (chatcode, _decorated) = fix_chatcode_decoration(&input);

		let data = BASE64.decode(chatcode)
        	.expect("invaid base64");

		let (_rest, build) = BuildTemplate::from_bytes((data.as_ref(), 0)).unwrap();

		assert_eq!(get_misc_data_string(&build).unwrap(), "Pets: Juvenile Tiger / Juvenile Rock Gazelle".to_string());
	}

	#[test]
	fn revenant_code_to_legend_string() {
		let input  = "[&DQkOHQMmPzrcEdwRBhIGEisSKxLUEdQRyhHKEQUEAwLUESsSBhIGEisS1BE=]".to_string();
		let (chatcode, _decorated) = fix_chatcode_decoration(&input);

		let data = BASE64.decode(chatcode)
        	.expect("invaid base64");

		let (_rest, build) = BuildTemplate::from_bytes((data.as_ref(), 0)).unwrap();

		assert_eq!(get_misc_data_string(&build).unwrap(), "<div\n  data-armory-embed='skills'\n  data-armory-nokey=true\n  data-armory-ids='28494,41858'\n>\n</div><div data-armory-embed='skills' data-armory-ids='28219,27322,27505,27917,28287'></div><div data-armory-embed='skills' data-armory-ids='45686,42949,40485,41220,45773'></div>".to_string());
	}

	#[test]
	fn ranger_code_to_invalid_pet_string() {
		let input  = "[&DQQILxk+BRsJEwAAvQAAALkAAADmEgAAtBIAADxAAAAAAAAAAAAAAAAAAAA=]".to_string();
		let (chatcode, _decorated) = fix_chatcode_decoration(&input);

		let data = BASE64.decode(chatcode)
        	.expect("invaid base64");

		let (_rest, build) = BuildTemplate::from_bytes((data.as_ref(), 0)).unwrap();

		assert_eq!(get_misc_data_string(&build).unwrap(), "Pets: Juvenile Wallow".to_string());
	}

	#[test]
	fn revenant_code_to_invalid_legend_string() {
		let input  = "[&DQkDOg8qRQDcEQAABhIAACsSAADUEQAAyhEAAAIAAAAAAAAAAAAAAAAAAAA=]".to_string();
		let (chatcode, _decorated) = fix_chatcode_decoration(&input);

		let data = BASE64.decode(chatcode)
        	.expect("invaid base64");

		let (_rest, build) = BuildTemplate::from_bytes((data.as_ref(), 0)).unwrap();

		// this should omit the broken legend in the output
		assert_eq!(get_misc_data_string(&build).unwrap(), "<div\n  data-armory-embed='skills'\n  data-armory-nokey=true\n  data-armory-ids='28134'\n>\n</div><div data-armory-embed='skills' data-armory-ids='26937,29209,28231,27107,28406'></div>".to_string());
	}

	#[test]
	fn long_soto_chatcode() {
		// this is a chat code with ranger hammer variants (soto undocumented feature)
			// 2,				// count
			// 51, 0, 35, 0,	// 2 weapon palettes (u16)
			// 
			// 4,			    // count
			// 103, 247, 0, 0, 	// 4 weapon variants (u32)
			// 221, 246, 0, 0,
			// 155, 246, 0, 0,
			// 232, 246, 0, 0
		let input = "[&DQQZGggqHiYlD3kAvQAAALkAAAC8AAAAlwEAABYAAAAAAAAAAAAAAAAAAAACMwAjAARn9wAA3fYAAJv2AADo9gAA]".to_string();
		let (chatcode, _decorated) = fix_chatcode_decoration(&input);
		
		let data = BASE64.decode(chatcode)
        	.expect("invaid base64");

		let (_rest, build) = BuildTemplate::from_bytes((data.as_ref(), 0)).unwrap();

		assert_eq!(build.profession, 4);
		assert_eq!(build.weapons.weapon_palette_count, 2);
		assert_eq!(build.weapons.weapon_palette_ids, vec!(51_u16, 35_u16));
		assert_eq!(build.weapons.weapon_variant_skill_count, 4);
		assert_eq!(build.weapons.weapon_variant_skill_ids, vec!(63335_u32, 63197_u32, 63131_u32, 63208_u32));
	}
}
