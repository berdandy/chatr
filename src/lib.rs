//! Chatr - Build Chat Codes for GW2
//!
//! # Examples
//! ```
//! use chatr::BuildTemplate;
//! use chatr::ChatCode;
//!
//! let code = ChatCode::build("[&DQYpGyU+OD90AAAAywAAAI8AAACRAAAAJgAAAAAAAAAAAAAAAAAAAAAAAAA=]").unwrap();
//! let build = BuildTemplate::from_chatcode(&code);
//!
//! assert_eq!(build.profession, 6);
//! assert_eq!(build.healing.terrestrial, 116);
//!
//! let chatcode = build.to_decorated_chatcode();
//! assert_eq!(chatcode, code.decorate());
//! ```
//!

use std::collections::HashMap;
use std::error::Error;

use base64::engine::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use deku::DekuContainerRead;
use deku::bitvec::{BitSlice, BitVec, Msb0};
use deku::prelude::*;
use lazy_static::lazy_static;

pub mod markup;

// see docs/build_template_reference.cpp

/// 16-bit skill palette pairs
#[derive(Default, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct PalettePair {
    pub terrestrial: u16,
    pub aquatic: u16,
}

#[derive(Default, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct InactiveLegendUtilitiesTriple {
    pub utilities: [u16; 3]
}

#[derive(Default, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct InactiveLegendUtilities {
    pub terrestrial: InactiveLegendUtilitiesTriple,
    pub aquatic: InactiveLegendUtilitiesTriple,
}

/// weapon mastery variant data. Currently only used in-game with non-untamed ranger builds wielding hammer
// new with SotO
#[derive(Default, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian", ctx_default = "deku::ctx::Endian::Little")]
pub struct WeaponMastery {
    #[deku(update = "self.weapon_palette_ids.len()")]
    pub weapon_palette_count: u8,
    #[deku(count = "weapon_palette_count")]
    pub weapon_palette_ids: Vec<u16>,

    #[deku(update = "self.weapon_variant_skill_ids.len()")]
    pub weapon_variant_skill_count: u8,
    #[deku(count = "weapon_variant_skill_count")]
    pub weapon_variant_skill_ids: Vec<u32>,
}

/// optionally handle trailing weapon variant data
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

    fn optional_write(output: &mut BitVec<u8, Msb0>, weapons: &WeaponMastery) -> Result<(), DekuError> {
		match (weapons.weapon_palette_count, weapons.weapon_variant_skill_count) {
			(0,0) => Ok(()),
			_ => weapons.write(output, ()),
		}
    }
}

/// data structure for skills, as extracted from chat codes
#[derive(Default, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little", magic = b"\x06")]
pub struct Skill {
    #[deku(bits = "24")]
    pub id: u32
}

impl Skill {
	pub fn try_from_chatcode(code: &ChatCode) -> Result<Skill, Box<dyn Error>> {
        let data = BASE64.decode(code.raw)?;
        let (_rest, skill) = Skill::from_bytes((data.as_ref(), 0))?;
        Ok(skill)
    }

	pub fn from_chatcode(code: &ChatCode) -> Skill {
        let data = BASE64.decode(code.raw).expect("invalid base64");
        let (_rest, skill) = Skill::from_bytes((data.as_ref(), 0)).expect(&format!("invalid template from {}", code.raw)[..]);
		skill
    }

	pub fn from_string(codestring: &str) -> Skill {
		let code = ChatCode::build(codestring).expect(&format!("can't build chatcode from {}", codestring)[..]);
        Skill::from_chatcode(&code)
	}
}

/// data structure for build templates, as extracted from chat codes
#[derive(Default, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little", magic = b"\x0d")]
pub struct BuildTemplate {
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

    pub healing: PalettePair,       // 4B
    pub utility: [PalettePair; 3],  // 12B
    pub elite: PalettePair,         // 4B

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
    #[deku(
		reader = "WeaponMastery::optional_read(deku::rest)",
		writer = "WeaponMastery::optional_write(deku::output, &self.weapons)"
	)]
    pub weapons: WeaponMastery,
}

impl BuildTemplate {
	pub fn try_from_chatcode(code: &ChatCode) -> Result<BuildTemplate, Box<dyn Error>> {
        let data = BASE64.decode(code.raw)?;
        let (_rest, build) = BuildTemplate::from_bytes((data.as_ref(), 0))?;
        Ok(build)
    }

	pub fn from_chatcode(code: &ChatCode) -> BuildTemplate {
        let data = BASE64.decode(code.raw).expect("invalid base64");
        let (_rest, build) = BuildTemplate::from_bytes((data.as_ref(), 0)).expect(&format!("invalid template from {}", code.raw)[..]);
		build
	}

	pub fn from_string(codestring: &str) -> BuildTemplate {
		let code = ChatCode::build(codestring).expect(&format!("can't build chatcode from {}", codestring)[..]);
        BuildTemplate::from_chatcode(&code)
	}

	pub fn to_chatcode(&self) -> String {
		let bytes = BuildTemplate::to_bytes(self).expect("Couldn't serialize build");
		BASE64.encode(bytes)
	}

	pub fn to_decorated_chatcode(&self) -> String {
		let bytes = BuildTemplate::to_bytes(self).expect("Couldn't serialize build");
		format!("[&{}]", BASE64.encode(bytes))
	}

	pub fn build() -> Self {
		Self {
			..Default::default()
		}
	}

	// ------------------------------------------------------------
	
	/// Extract skill ids from BuildTemplate
	///
	/// skills:
	///   build contains palette ids
	///   palette ids are mapped to ability ids via https://api.guildwars2.com/v2/professions/{PROFESSIONS[build.profession]}
	pub fn get_skill_ids(&self) -> Result<[u16; 5], Box<dyn Error>> {
		let idx = (self.profession - 1) as usize;
		let palette = PALETTE_SKILLS_BY_PROFESSION[idx].as_ref().expect("invalid profession palette skill map");

		let skills: [u16 ; 5] = [
			palette[&self.healing.terrestrial],
			palette[&self.utility[0].terrestrial],
			palette[&self.utility[1].terrestrial],
			palette[&self.utility[2].terrestrial],
			palette[&self.elite.terrestrial],
		];
		Ok(skills)
	}

	pub fn set_palette_ids_from_skill_ids(&mut self, skill_ids: [u16; 5]) {
		let idx = (self.profession - 1) as usize;
		let palette = SKILLS_PALETTE_BY_PROFESSION[idx].as_ref().expect("invalid profession palette skill map");
		self.healing.terrestrial = palette[&skill_ids[0]];
		self.utility[0].terrestrial = palette[&skill_ids[1]];
		self.utility[1].terrestrial = palette[&skill_ids[2]];
		self.utility[2].terrestrial = palette[&skill_ids[3]];
		self.elite.terrestrial = palette[&skill_ids[4]];
	}

	pub fn get_trait_id_map_by_spec(&self) -> Result<HashMap<u8, [u16; 9]>, Box<dyn Error>> {
		let specs = [self.specialization1, self.specialization2, self.specialization3];

		let mut trait_map = HashMap::new();

		let all_specs_str = include_str!("specializations.json");
		let all_specs: Vec<serde_json::Value> = serde_json::from_str(all_specs_str)?;

		for spec_id in specs {
			// let request_url = format!("https://api.guildwars2.com/v2/specializations/{spec_id}?v=2024-03-25T00:00:00Z");
			// let spec_data = reqwest::blocking::get(request_url)?.text()?;

			let spec = all_specs
				.iter()
				.find(|&s| s["id"] == spec_id).unwrap();

			// Parse the string of data into serde_json::Value.
			let trait_ids: [u16; 9] = [
				spec["major_traits"][0].as_u64().expect("integer") as u16,
				spec["major_traits"][1].as_u64().expect("integer") as u16,
				spec["major_traits"][2].as_u64().expect("integer") as u16,
				spec["major_traits"][3].as_u64().expect("integer") as u16,
				spec["major_traits"][4].as_u64().expect("integer") as u16,
				spec["major_traits"][5].as_u64().expect("integer") as u16,
				spec["major_traits"][6].as_u64().expect("integer") as u16,
				spec["major_traits"][7].as_u64().expect("integer") as u16,
				spec["major_traits"][8].as_u64().expect("integer") as u16
			];
			trait_map.insert(spec_id, trait_ids);
		}

		Ok(trait_map)
	}

	pub fn get_specializations(&self) -> [u8; 3] {
		[self.specialization1, self.specialization2, self.specialization3]
	}

	pub fn get_traits(&self) -> [u16; 9] {
		let trait_ids_by_spec = self.get_trait_id_map_by_spec().unwrap();
		let trait_ids1 = trait_ids_by_spec[&self.specialization1];
		let trait_ids2 = trait_ids_by_spec[&self.specialization2];
		let trait_ids3 = trait_ids_by_spec[&self.specialization3];

		[
			trait_ids1[(self.trait_adept_1 - 1) as usize],
        	trait_ids1[(self.trait_master_1 + 3 - 1) as usize],
        	trait_ids1[(self.trait_grandmaster_1 + 6 - 1) as usize],
        	trait_ids2[(self.trait_adept_2 - 1) as usize],
        	trait_ids2[(self.trait_master_2 + 3 - 1) as usize],
        	trait_ids2[(self.trait_grandmaster_2 + 6 - 1) as usize],
        	trait_ids3[(self.trait_adept_3 - 1) as usize],
        	trait_ids3[(self.trait_master_3 + 3 - 1) as usize],
        	trait_ids3[(self.trait_grandmaster_3 + 6 - 1) as usize],
		]
	}

	pub fn set_spec_and_trait_indexes_from_ids(&mut self, specs: [u8; 3], traits: [u16; 9]) {
		self.specialization1 = specs[0];
		self.specialization2 = specs[1];
		self.specialization3 = specs[2];

		let trait_ids_by_spec = self.get_trait_id_map_by_spec().unwrap();

		let trait_ids1 = trait_ids_by_spec[&self.specialization1];
		self.trait_adept_1 = trait_ids1.iter().position(|t| t == &traits[0]).unwrap() as u8 + 1;
		self.trait_master_1 = trait_ids1.iter().position(|t| t == &traits[1]).unwrap() as u8 - 3 + 1;
		self.trait_grandmaster_1 = trait_ids1.iter().position(|t| t == &traits[2]).unwrap() as u8 - 6 + 1;

		let trait_ids2 = trait_ids_by_spec[&self.specialization2];
		self.trait_adept_2 = trait_ids2.iter().position(|t| t == &traits[3]).unwrap() as u8 + 1;
		self.trait_master_2 = trait_ids2.iter().position(|t| t == &traits[4]).unwrap() as u8 - 3 + 1;
		self.trait_grandmaster_2 = trait_ids2.iter().position(|t| t == &traits[5]).unwrap() as u8 - 6 + 1;
		
		let trait_ids3 = trait_ids_by_spec[&self.specialization3];
		self.trait_adept_3 = trait_ids3.iter().position(|t| t == &traits[6]).unwrap() as u8 + 1;
		self.trait_master_3 = trait_ids3.iter().position(|t| t == &traits[7]).unwrap() as u8 - 3 + 1;
		self.trait_grandmaster_3 = trait_ids3.iter().position(|t| t == &traits[8]).unwrap() as u8 - 6 + 1;
	}
}

const PROFESSIONS: &[&str] = &[
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

fn palette_builder(profession_id: &str) -> HashMap<u16, u16> {

	let professions_str = include_str!("professions.json");
	let professions: Vec<serde_json::Value> = serde_json::from_str(professions_str).unwrap();

	let prof = professions
		.iter()
		.find(|&s| s["id"] == profession_id).unwrap();

	// let request_url = format!("https://api.guildwars2.com/v2/professions/{profession_id}?v=2024-03-25T00:00:00Z");
	// let palette_data = reqwest::blocking::get(request_url).unwrap().text().unwrap();

	// Parse the string of data into serde_json::Value.
	// let v: serde_json::Value = serde_json::from_str(&palette_data).unwrap();
	let a = prof["skills_by_palette"].as_array().expect("skills_by_palette issue");
	let mut skill_palette_map = HashMap::new();
	for mapping in a {
		let skills_by_palette = mapping.as_array().expect("invalid mapping in skills_by_palette");
		skill_palette_map.insert(
			skills_by_palette[0].as_u64().expect("integer") as u16,
			skills_by_palette[1].as_u64().expect("integer") as u16
		);
	}

	skill_palette_map
}

lazy_static! {
	static ref PALETTE_SKILLS_BY_PROFESSION: [Option<HashMap<u16, u16>> ; 9] = {
		let mut p2s: [Option<HashMap<u16, u16>> ; 9] = Default::default();
		for (i, profession_id) in PROFESSIONS.iter().enumerate() {
			p2s[i] = Some(palette_builder(profession_id));
		}

		p2s
	};

	// inverted
	static ref SKILLS_PALETTE_BY_PROFESSION: [Option<HashMap<u16, u16>> ; 9] = {
		let mut s2p: [Option<HashMap<u16, u16>> ; 9] = Default::default();
		for (i, palette_map) in PALETTE_SKILLS_BY_PROFESSION.iter().enumerate() {
			s2p[i] = palette_map
				.as_ref()
				.map(|palette_to_skill| palette_to_skill
					.iter()
					.map(|(k, v)| (*v, *k)).collect());
		}
		s2p
	};
}

pub struct ChatCode<'a> {
	pub raw: &'a str
}

impl<'a> ChatCode<'a> {
	/// Builds a decorateable ChatCode
	///
	/// Returns Ok(chatcode) if "[&codestring]" or "codestring", but Err otherwise
	///
	/// * Note: codestring is not validated for base64 correctness
	pub fn build(code: &str) -> Result<ChatCode, &str> {

		let head = code.strip_prefix("[&");
		let tail = head.and_then(|c| c.strip_suffix(']'));

		match (head, tail) {
			(Some(_), Some(stripped)) => Ok(ChatCode { raw: stripped }),
			(None, None) => Ok(ChatCode { raw: code }),
			(None, Some(_)) => Err("Missing starting '[&' in chat code"),
			(Some(_), None) => Err("Missing ending ']' in chat code"),
		}
	}

	pub fn decorate(&self) -> String {
		format!("[&{}]", self.raw)
	}
}

#[cfg(test)]
mod tests {
    use base64::engine::Engine as _;
    use base64::engine::general_purpose::STANDARD as BASE64;

    use super::*;

	#[test]
	fn error_on_mismatched_chatcode_decoration() {
		let code = ChatCode::build("[&123456");
		assert!(code.is_err());
	}

    #[test]
    fn trim_decorated_chatcode() {
        let data = String::from("[&123456]");
		let code = ChatCode::build(&data).unwrap();
		assert_eq!(code.raw, "123456");
		assert_eq!(code.decorate(), "[&123456]");
    }

    #[test]
    fn non_chatcode_no_trim() {
        let data = String::from("123456");
		let code = ChatCode::build(&data).unwrap();
		assert_eq!(code.raw, "123456");
		assert_eq!(code.decorate(), "[&123456]");
    }

    #[test]
    fn long_soto_chatcode() {
        // this is a chat code with ranger hammer variants (soto undocumented feature)
            // 2,               // count
            // 51, 0, 35, 0,    // 2 weapon palettes (u16)
            // 
            // 4,               // count
            // 103, 247, 0, 0,  // 4 weapon variants (u32)
            // 221, 246, 0, 0,
            // 155, 246, 0, 0,
            // 232, 246, 0, 0
        let input = String::from("[&DQQZGggqHiYlD3kAvQAAALkAAAC8AAAAlwEAABYAAAAAAAAAAAAAAAAAAAACMwAjAARn9wAA3fYAAJv2AADo9gAA]");
		let code = ChatCode::build(&input).unwrap();

        let data = BASE64
			.decode(code.raw)
            .expect("invalid base64");

        let (_rest, build) = BuildTemplate::from_bytes((data.as_ref(), 0)).unwrap();

        assert_eq!(build.profession, 4);
        assert_eq!(build.weapons.weapon_palette_count, 2);
        assert_eq!(build.weapons.weapon_palette_ids, vec!(51_u16, 35_u16));
        assert_eq!(build.weapons.weapon_variant_skill_count, 4);
        assert_eq!(build.weapons.weapon_variant_skill_ids, vec!(63335_u32, 63197_u32, 63131_u32, 63208_u32));
    }

	#[test]
	fn bidirectional_skill_palette() {
        let input = "[&DQYpGyU+OD90AAAAywAAAI8AAACRAAAAJgAAAAAAAAAAAAAAAAAAAAAAAAA=]";
		let code = ChatCode::build(input).unwrap();
        let build = BuildTemplate::from_chatcode(&code);

		let skill_ids = build.get_skill_ids().unwrap();
		
		let mut build2 = BuildTemplate::build();
		build2.profession = build.profession;
		build2.set_palette_ids_from_skill_ids(skill_ids);
		let skill_ids2 = build2.get_skill_ids().unwrap();

		assert_eq!(skill_ids, skill_ids2);
	}

	#[test]
	fn bidirectional_traits() {
        let input = "[&DQYpGyU+OD90AAAAywAAAI8AAACRAAAAJgAAAAAAAAAAAAAAAAAAAAAAAAA=]";
		let code = ChatCode::build(input).unwrap();
        let build = BuildTemplate::from_chatcode(&code);

		let specs = build.get_specializations();
		let traits = build.get_traits();

		let mut build2 = BuildTemplate::build();
		build2.profession = build.profession;
		build2.set_spec_and_trait_indexes_from_ids(specs, traits);
		let traits2 = build2.get_traits();

		assert_eq!(traits, traits2);
	}

	#[test]
	fn build_template_direct_from_chatcode() {
        let input = "[&DQYpGyU+OD90AAAAywAAAI8AAACRAAAAJgAAAAAAAAAAAAAAAAAAAAAAAAA=]";
		let code = ChatCode::build(input).unwrap();
        let build = BuildTemplate::from_chatcode(&code);

        assert_eq!(markup::armory(build).unwrap(), String::from("<div data-armory-embed='skills' data-armory-ids='5503,5542,5570,5571,5666'></div><div data-armory-embed='specializations' data-armory-ids='41,37,56' data-armory-41-traits='232,214,226' data-armory-37-traits='266,257,1511' data-armory-56-traits='2115,2170,2138'></div>"));
	}

	#[test]
	fn build_template_direct_from_string() {
        let build = BuildTemplate::from_string("[&DQYpGyU+OD90AAAAywAAAI8AAACRAAAAJgAAAAAAAAAAAAAAAAAAAAAAAAA=]");
        assert_eq!(markup::armory(build).unwrap(), String::from("<div data-armory-embed='skills' data-armory-ids='5503,5542,5570,5571,5666'></div><div data-armory-embed='specializations' data-armory-ids='41,37,56' data-armory-41-traits='232,214,226' data-armory-37-traits='266,257,1511' data-armory-56-traits='2115,2170,2138'></div>"));
	}

    #[test]
    fn skill_template_from_string() {
        let skill = Skill::from_string("[&BucCAAA=]");
        assert_eq!(skill.id, 743);
    }

    #[test]
    fn try_skill_by_magic() {
		let code = ChatCode::build("[&BucCAAA=]").unwrap(); // aegis skill
        let skill = Skill::try_from_chatcode(&code).unwrap();
        let expected = Skill {
            id: 743
        };
        assert_eq!(skill, expected);

    }
    #[test]
    #[should_panic]
    fn try_build_template_by_magic_and_fail() {
		let code = ChatCode::build("[&BucCAAA=]").unwrap(); // same as above test (ie, not a build template)
        BuildTemplate::try_from_chatcode(&code).unwrap();
    }
}
