use std::collections::HashMap;
use std::error::Error;

use crate::BuildTemplate;
use crate::get_skill_ids;

/// Get Ranger Pet markup
///
/// ranger: https://api.guildwars2.com/v2/pets/{}
///
/// - Ranger pets aren't supported by armory-embeds. We'll have to roll our own. /v2/pets API gives
///   a link to a png file for `icon`. We can render that in addition to the caption for the pet `name`
/// 
/// TODO: use a single request with multiple ids
pub fn armory_pet(pet1: u8, pet2: u8) -> Result<String, Box<dyn Error>> {
    let mut pet_names = String::from("Pets: ");

    let request_url = format!("https://api.guildwars2.com/v2/pets/{pet1}?v=latest");
    let pet_data  = reqwest::blocking::get(request_url)?.text()?;

    let v: serde_json::Value = serde_json::from_str(&pet_data)?;
    if v.as_object().expect("Invalid JSON Object").contains_key("name") {
        pet_names += &String::from(v["name"].as_str().expect("invalid pet1 name"));
    }

    let request_url = format!("https://api.guildwars2.com/v2/pets/{pet2}?v=latest");
    let pet2_data  = reqwest::blocking::get(request_url)?.text()?;

    let v2: serde_json::Value = serde_json::from_str(&pet2_data)?;
    if v2.as_object().expect("Invalid JSON Object").contains_key("name") {
        // ie, both pets
        if v.as_object().expect("Invalid JSON Object").contains_key("name") {
            pet_names += " / ";
        }
        pet_names += &String::from(v2["name"].as_str().expect("invalid pet2 name"));
    }

    Ok(pet_names)
}

/// Get Revenant Legend markup
///
/// revenant: https://api.guildwars2.com/v2/legends/Legend{}
///
/// - Revenant skills DO NOT use the skill palette. /v2/legends API gives a structure with `swap`,
///   `heal`, `elite` and an array for `utilities`.
/// Note: Legend7 (Alliance) is missing. Probably because the skills are 'doubled'
/// TODO: use a single request with all legend ids
pub fn armory_legend(legend1: u8, legend2: u8) -> Result<String, Box<dyn Error>> {
    // get list of legends
    let request_url = format!("https://api.guildwars2.com/v2/legends?v=latest");//, legend1);
    let legend_name_data  = reqwest::blocking::get(request_url)?.text()?;
    let legend_name_v: serde_json::Value = serde_json::from_str(&legend_name_data)?;
    let legend_names = legend_name_v.as_array().ok_or("invalid array of legend names")?;

    let mut skill_output = String::new();

    let mut first = true;
    let mut output = String::from("<div data-armory-embed='skills' data-armory-nokey=true data-armory-ids='");
    for legend in legend_names {
        let request_url = format!("https://api.guildwars2.com/v2/legends/{}?v=latest", legend.as_str().ok_or("invalid legend")?);
        let legend_data  = reqwest::blocking::get(request_url)?.text()?;
        let v: serde_json::Value = serde_json::from_str(&legend_data)?;

        let code = v["code"].as_u64().expect("integer") as u8;
        if code == legend1 || code == legend2 {
            let swap = v["swap"].as_u64().expect("integer"); 
            if first {
                output += &String::from(format!("{swap}"));
                first = false;
            } else {
                output += &String::from(format!(",{swap}"));
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

	if 7 == legend1 || 7 == legend2 {
		// garbage special case because Anet didn't put Alliance legend in the API
		output += match first {
			true => "62891",
			false => ",62891"
		};
		skill_output += "<div data-armory-embed='skills' data-armory-ids='62719,62832,62962,62878,62942'></div>";
	}

    output += &String::from("'></div>");

    Ok(output + &skill_output)
}

/// additional markup for rangers & revenants
/// 
/// only non-empty if ranger (profession==4) or revenant (profession==9)
///  ranger: pet names
///  revenant: legend armory markup
pub fn armory_misc(build: &BuildTemplate) -> Result<String, Box<dyn Error>> {
    match build.profession {
        4 => armory_pet(build.terrestrial_pet1_active_legend, build.terrestrial_pet2_inactive_legend),
        9 => armory_legend(build.terrestrial_pet1_active_legend, build.terrestrial_pet2_inactive_legend),
        _ => Ok(String::new())
    }
}

pub fn get_trait_ids(specs: [u8; 3]) -> Result<HashMap<u8, [u16; 9]>, Box<dyn Error>> {
    let mut trait_map = HashMap::new();

    for spec_id in specs {
        let request_url = format!("https://api.guildwars2.com/v2/specializations/{spec_id}?v=latest");
        let spec_data = reqwest::blocking::get(request_url)?.text()?;

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

pub fn armory(build: BuildTemplate) -> Result<String, Box<dyn Error>> {

    let skills = get_skill_ids(&build)?;

    let trait_ids_by_spec = get_trait_ids([build.specialization1, build.specialization2, build.specialization3])?;
    let trait_ids1 = trait_ids_by_spec[&build.specialization1];
    let trait_ids2 = trait_ids_by_spec[&build.specialization2];
    let trait_ids3 = trait_ids_by_spec[&build.specialization3];

    let misc = armory_misc(&build)?;

    // revenant has legend skill overriding skills, so we only  use legend markup in misc
    let preamble = match build.profession {
        9 => format!("{misc}", misc=misc),
        _ => format!(concat!("{misc}",
                "<div ",
                  "data-armory-embed='skills' ",
                  "data-armory-ids='{healing},{utility1},{utility2},{utility3},{elite}'",
                ">",
                "</div>"),
			misc=misc,
            healing=skills[0],
            utility1=skills[1],
            utility2=skills[2],
            utility3=skills[3],
            elite=skills[4]
        )
    };

    let markup = format!(concat!("{preamble}",
        "<div ",
          "data-armory-embed='specializations' ",
          "data-armory-ids='{spec1},{spec2},{spec3}' ",
          "data-armory-{spec1}-traits='{trait11},{trait12},{trait13}' ",
          "data-armory-{spec2}-traits='{trait21},{trait22},{trait23}' ",
          "data-armory-{spec3}-traits='{trait31},{trait32},{trait33}'",
        ">",
        "</div>"),
		preamble=preamble,
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

    Ok(markup)
}

#[cfg(test)]
mod tests {
	use base64::engine::Engine as _;
	use base64::engine::general_purpose::STANDARD as BASE64;

	use crate::*;
	use super::*;

    #[test]
    fn chatcode_to_armory_markup() {
        let input = String::from("[&DQYpGyU+OD90AAAAywAAAI8AAACRAAAAJgAAAAAAAAAAAAAAAAAAAAAAAAA=]");
		let code = ChatCode::build(&input).unwrap();
        let data = BASE64.decode(code.raw)
            .expect("invaid base64");

        let (_rest, build) = BuildTemplate::from_bytes((data.as_ref(), 0)).unwrap();

        assert_eq!(armory(build).unwrap(), String::from("<div data-armory-embed='skills' data-armory-ids='5503,5542,5570,5571,5666'></div><div data-armory-embed='specializations' data-armory-ids='41,37,56' data-armory-41-traits='232,214,226' data-armory-37-traits='266,257,1511' data-armory-56-traits='2115,2170,2138'></div>"));
    }

    #[test]
    fn ranger_code_to_pet_string() {
        let input  = String::from("[&DQQePQgaSDd5AHgAARuWAbUAmgCsAbgADxvtAC87KhUAAAAAAAAAAAAAAAA=]");
		let code = ChatCode::build(&input).unwrap();
        let data = BASE64.decode(code.raw)
            .expect("invaid base64");

        let (_rest, build) = BuildTemplate::from_bytes((data.as_ref(), 0)).unwrap();

        assert_eq!(armory_misc(&build).unwrap(), String::from("Pets: Juvenile Tiger / Juvenile Rock Gazelle"));
    }

    #[test]
    fn revenant_code_to_legend_string() {
        let input  = String::from("[&DQkOHQMmPzrcEdwRBhIGEisSKxLUEdQRyhHKEQUEAwLUESsSBhIGEisS1BE=]");
		let code = ChatCode::build(&input).unwrap();
        let data = BASE64.decode(code.raw)
            .expect("invaid base64");

        let (_rest, build) = BuildTemplate::from_bytes((data.as_ref(), 0)).unwrap();

        assert_eq!(armory_misc(&build).unwrap(), String::from("<div data-armory-embed='skills' data-armory-nokey=true data-armory-ids='28494,41858'></div><div data-armory-embed='skills' data-armory-ids='28219,27322,27505,27917,28287'></div><div data-armory-embed='skills' data-armory-ids='45686,42949,40485,41220,45773'></div>"));
    }

    #[test]
    fn ranger_code_to_invalid_pet_string() {
        let input  = String::from("[&DQQILxk+BRsJEwAAvQAAALkAAADmEgAAtBIAADxAAAAAAAAAAAAAAAAAAAA=]");
		let code = ChatCode::build(&input).unwrap();
        let data = BASE64.decode(code.raw)
            .expect("invaid base64");

        let (_rest, build) = BuildTemplate::from_bytes((data.as_ref(), 0)).unwrap();

        assert_eq!(armory_misc(&build).unwrap(), String::from("Pets: Juvenile Wallow"));
    }

    #[test]
    fn revenant_code_to_invalid_legend_string() {
        let input  = String::from("[&DQkDOg8qRQDcEQAABhIAACsSAADUEQAAyhEAAAIAAAAAAAAAAAAAAAAAAAA=]");
		let code = ChatCode::build(&input).unwrap();
        let data = BASE64.decode(code.raw)
            .expect("invaid base64");

        let (_rest, build) = BuildTemplate::from_bytes((data.as_ref(), 0)).unwrap();

        // this should omit the broken legend in the output
        assert_eq!(armory_misc(&build).unwrap(), String::from("<div data-armory-embed='skills' data-armory-nokey=true data-armory-ids='28134'></div><div data-armory-embed='skills' data-armory-ids='26937,29209,28231,27107,28406'></div>"));
    }

    #[test]
    fn revenant_handle_alliance_api_missing() {
        let chatcode  = String::from("DQkDPg86RRncEdwRBhIrEisSBhLUEdQRyhHKEQcDAgMrEgYS1BEGEisS1BECWgAyAAA=");
        let data = BASE64.decode(chatcode).expect("invaid base64");

        let (_rest, build) = BuildTemplate::from_bytes((data.as_ref(), 0)).unwrap();
        let result = armory_misc(&build);

        assert!(result.is_ok());
        assert_eq!(armory_misc(&build).unwrap(), String::from("<div data-armory-embed='skills' data-armory-nokey=true data-armory-ids='28419,62891'></div><div data-armory-embed='skills' data-armory-ids='27372,28516,26679,26557,27975'></div><div data-armory-embed='skills' data-armory-ids='62719,62832,62962,62878,62942'></div>"));
    }
} 
