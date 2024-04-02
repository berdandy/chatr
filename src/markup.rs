use std::error::Error;

use crate::BuildTemplate;

/// Get Ranger Pet markup
///
/// ranger: https://api.guildwars2.com/v2/pets/{}
///
/// - Ranger pets aren't supported by armory-embeds. We'll have to roll our own. /v2/pets API gives
///   a link to a png file for `icon`. We can render that in addition to the caption for the pet `name`
/// - Markup for Pets is output with css classes as follows:
///   peticon: div with added background, containing name as text
///
pub fn armory_pet(pet1: u8, pet2: u8) -> Result<String, Box<dyn Error>> {

    let all_pets_str = include_str!("pets.json");
    let all_pets: Vec<serde_json::Value> = serde_json::from_str(&all_pets_str)?;

    let pets: Vec<&serde_json::Value> = all_pets
        .iter()
        .filter(|&p| p["id"] == pet1 || p["id"] == pet2)
        .collect();

	// let request_url = format!("https://api.guildwars2.com/v2/pets?v=2024-03-25T00:00:00Z&ids={pet1},{pet2}");
	// let pet_data  = reqwest::blocking::get(request_url)?.text()?;
	// let v: serde_json::Value = serde_json::from_str(&pet_data)?;
	// let pets = v.as_array().expect("Invalid JSON Array");

	let mut markup = String::new();
	for pet in pets {
		let pet_markup:String = match (pet["name"].as_str(), pet["icon"].as_str()) {
			(Some(name), Some(icon)) => format!("<div class=\"peticon\" style=\"background: url('{icon}') 50% 50% no-repeat;\">{name}</div>"),
			(_, Some(icon)) => format!("<div class=\"peticon\" style=\"background: url('{icon}') 50% 50% no-repeat;\"></div>"),
			(Some(name), _) => format!("<div class=\"peticon\">{name}</div>"),
			(_,_) => String::new(),
		};

		markup += &pet_markup;
	}

    Ok(markup)
}

/// Get Revenant Legend markup
///
/// revenant: https://api.guildwars2.com/v2/legends/Legend{}
///
/// - Revenant skills DO NOT use the skill palette. /v2/legends API gives a structure with `swap`,
///   `heal`, `elite` and an array for `utilities`.
/// Note: Legend7 (Alliance) is missing. Probably because the skills are 'doubled'
/// 
/// TODO: use a single request with all legend ids
pub fn armory_legend(legend1: u8, legend2: u8) -> Result<String, Box<dyn Error>> {

    let all_legends_str = include_str!("legends.json");
    let all_legends: Vec<serde_json::Value> = serde_json::from_str(&all_legends_str)?;

    let legends: Vec<&serde_json::Value> = all_legends
        .iter()
        .filter(|&p| p["code"] == legend1 || p["code"] == legend2)
        .collect();

    // let request_url = format!("https://api.guildwars2.com/v2/legends?v=2024-03-25T00:00:00Z");//, legend1);
    // let legend_name_data  = reqwest::blocking::get(request_url)?.text()?;
    // let legend_name_v: serde_json::Value = serde_json::from_str(&legend_name_data)?;
    // let legend_names = legend_name_v.as_array().ok_or("invalid array of legend names")?;

    let mut skill_output = String::new();

    let mut first = true;
    let mut output = String::from("<div data-armory-embed='skills' data-armory-nokey=true data-armory-ids='");
    for legend in legends {
        // let request_url = format!("https://api.guildwars2.com/v2/legends/{}?v=2024-03-25T00:00:00Z", legend.as_str().ok_or("invalid legend")?);
        // let legend_data  = reqwest::blocking::get(request_url)?.text()?;
        // let v: serde_json::Value = serde_json::from_str(&legend_data)?;

        let code = legend["code"].as_u64().expect("integer") as u8;
        if code == legend1 || code == legend2 {
            let swap = legend["swap"].as_u64().expect("integer"); 
            if first {
                output += &String::from(format!("{swap}"));
                first = false;
            } else {
                output += &String::from(format!(",{swap}"));
            }

            skill_output += "<div data-armory-embed='skills' data-armory-ids='";
                skill_output += &format!("{},", legend["heal"].as_u64().expect("integer"));
                let utils = legend["utilities"].as_array().ok_or("invalid utilities")?;
                for util in utils {
                    skill_output += &format!("{},", util.as_u64().expect("integer"));
                }
                skill_output += &format!("{}", legend["elite"].as_u64().expect("integer"));
            
            skill_output += "'></div>";
        }

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

pub fn armory(build: BuildTemplate) -> Result<String, Box<dyn Error>> {

    let skills = build.get_skill_ids()?;

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

	let traits = build.get_traits();

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
        trait11=traits[0],
        trait12=traits[1],
        trait13=traits[2],
        trait21=traits[3],
        trait22=traits[4],
        trait23=traits[5],
        trait31=traits[6],
        trait32=traits[7],
        trait33=traits[8],
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

        assert_eq!(armory_misc(&build).unwrap(), String::from("<div class=\"peticon\" style=\"background: url('https://render.guildwars2.com/file/A41953A8682309700FA17601FE05DB040C6CD07B/1128514.png') 50% 50% no-repeat;\">Juvenile Tiger</div><div class=\"peticon\" style=\"background: url('https://render.guildwars2.com/file/F3EED35B2DD6B52FFCBFD7FCCA184EB7EBCF01A4/1769875.png') 50% 50% no-repeat;\">Juvenile Rock Gazelle</div>"));
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

        assert_eq!(armory_misc(&build).unwrap(), String::from("<div class=\"peticon\" style=\"background: url('https://render.guildwars2.com/file/58AFDFDFDDB8325F5E076C2400C63D450C15EA09/2604592.png') 50% 50% no-repeat;\">Juvenile Wallow</div>"));
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
