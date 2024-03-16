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

/// data structure for build templates, as extracted from chat codes
#[derive(Default, Debug, PartialEq, DekuRead, DekuWrite)]
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
	pub fn from_chatcode(code: &ChatCode) -> BuildTemplate {
        let data = BASE64.decode(code.raw).expect("invaid base64");
        let (_rest, build) = BuildTemplate::from_bytes((data.as_ref(), 0)).unwrap();
		build
	}

	pub fn from_string(codestring: &str) -> BuildTemplate {
		let code = ChatCode::build(&codestring).unwrap();
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

	pub fn default() -> Self {
		Self {
			magic: 0xD, ..Default::default()
		}
	}
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

/// Extract skill ids from BuildTemplate
///
/// skills:
///   build contains palette ids
///   palette ids are mapped to ability ids via https://api.guildwars2.com/v2/professions/{PROFESSIONS[build.profession]}
pub fn get_skill_ids(build: &BuildTemplate) -> Result<[u16; 5], Box<dyn Error>> {

    let profession_id = PROFESSIONS[(build.profession - 1) as usize];
    let request_url = format!("https://api.guildwars2.com/v2/professions/{profession_id}?v=latest");
    let palette_data = reqwest::blocking::get(request_url)?.text()?;

    // Parse the string of data into serde_json::Value.
    let v: serde_json::Value = serde_json::from_str(&palette_data)?;
    let a = v["skills_by_palette"].as_array().ok_or("skills_by_palette issue")?;
    let mut skill_palette_map = HashMap::new();
    for mapping in a {
        let skills_by_palette = mapping.as_array().ok_or("invalid mapping in skills_by_palette")?;
        skill_palette_map.insert(
            skills_by_palette[0].as_u64().expect("integer") as u16,
            skills_by_palette[1].as_u64().expect("integer") as u16
        );
    }

    let skills: [u16 ; 5] = [
        skill_palette_map[&build.healing.terrestrial],
        skill_palette_map[&build.utility[0].terrestrial],
        skill_palette_map[&build.utility[1].terrestrial],
        skill_palette_map[&build.utility[2].terrestrial],
        skill_palette_map[&build.elite.terrestrial],
    ];
    Ok(skills)
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
	pub fn build(code: &'a str) -> Result<ChatCode, &str> {

		let head = code.strip_prefix("[&");
		let tail = head.and_then(|c| c.strip_suffix("]"));

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
	fn build_template_direct_from_chatcode() {
        let input = "[&DQYpGyU+OD90AAAAywAAAI8AAACRAAAAJgAAAAAAAAAAAAAAAAAAAAAAAAA=]";
		let code = ChatCode::build(&input).unwrap();
        let build = BuildTemplate::from_chatcode(&code);

        assert_eq!(markup::armory(build).unwrap(), String::from("<div data-armory-embed='skills' data-armory-ids='5503,5542,5570,5571,5666'></div><div data-armory-embed='specializations' data-armory-ids='41,37,56' data-armory-41-traits='232,214,226' data-armory-37-traits='266,257,1511' data-armory-56-traits='2115,2170,2138'></div>"));
	}

	#[test]
	fn build_template_direct_from_string() {
        let build = BuildTemplate::from_string("[&DQYpGyU+OD90AAAAywAAAI8AAACRAAAAJgAAAAAAAAAAAAAAAAAAAAAAAAA=]");
        assert_eq!(markup::armory(build).unwrap(), String::from("<div data-armory-embed='skills' data-armory-ids='5503,5542,5570,5571,5666'></div><div data-armory-embed='specializations' data-armory-ids='41,37,56' data-armory-41-traits='232,214,226' data-armory-37-traits='266,257,1511' data-armory-56-traits='2115,2170,2138'></div>"));
	}
}