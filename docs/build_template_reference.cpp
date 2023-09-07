// C++ format provided by ArenaNet: https://en-forum.guildwars2.com/topic/65255-api-updates-december-18-2019/
struct BuildChatLink
{
    std::uint8_t magic = 0xd;
    // Profession codes can be found in the `code` field at
    // https://api.guildwars2.com/professions
    std::uint8_t profession;
    // Specializations can be found at
    // https://api.guildwars2.com/specializations/:id
    // To find a trait ID given a specialization, first find the `major_traits`
    // array at https://api.guildwars2.com/v2/specializations/:id
    // Then, reference this table to find the trait ID:
    //        field        | value |    trait ID
    // ====================+=======+=================
    // traitAdeptN         |     0 | None
    //                     |     1 | major_traits[0]
    //                     |     2 | major_traits[1]
    //                     |     3 | major_traits[2]
    // traitMasterN        |     0 | None
    //                     |     1 | major_traits[3]
    //                     |     2 | major_traits[4]
    //                     |     3 | major_traits[5]
    // traitGrandmasterN   |     0 | None
    //                     |     1 | major_traits[6]
    //                     |     2 | major_traits[7]
    //                     |     3 | major_traits[8]
    // Traits themselves can be found at https://api.guildwars2.com/traits/:id
    std::uint8_t specialization1;
    std::uint8_t traitAdept1 : 2;
    std::uint8_t traitMaster1 : 2;
    std::uint8_t traitGrandmaster1 : 2;
    std::uint8_t traitPadding1 : 2;
    std::uint8_t specialization2;
    std::uint8_t traitAdept2 : 2;
    std::uint8_t traitMaster2 : 2;
    std::uint8_t traitGrandmaster2 : 2;
    std::uint8_t traitPadding2 : 2;
    std::uint8_t specialization3;
    std::uint8_t traitAdept3 : 2;
    std::uint8_t traitMaster3 : 2;
    std::uint8_t traitGrandmaster3 : 2;
    std::uint8_t traitPadding3 : 2;
    // To find a skill ID given a palette ID, first find the `skills_by_palette` array
    // at https://api.guildwars2.com/v2/professions/:id for this build's profession.
    // Then, find the pair with the palette ID and use the mapped value.
    // For example, in javascript you might look it up like so:
    //     new Map(skills_by_palette).get(paletteID)
    // Skills themselves can be found at https://api.guildwars2.com/v2/skills/:id
    std::uint16_t terrestrialHealingSkillPalette;
    std::uint16_t aquaticHealingSkillPalette;
    std::uint16_t terrestrialUtilitySkillPalette1;
    std::uint16_t aquaticUtilitySkillPalette1;
    std::uint16_t terrestrialUtilitySkillPalette2;
    std::uint16_t aquaticUtilitySkillPalette2;
    std::uint16_t terrestrialUtilitySkillPalette3;
    std::uint16_t aquaticUtilitySkillPalette3;
    std::uint16_t terrestrialEliteSkillPalette;
    std::uint16_t aquaticEliteSkillPalette;
    union
    {
        struct
        {
            // Pets can be found at
            // https://api.guildwars2.com/v2/pets/:id
            std::uint8_t terrestrialPet1;
            std::uint8_t terrestrialPet2;
            std::uint8_t aquaticPet1;
            std::uint8_t aquaticPet2;
        } ranger;
        struct
        {
            // Legend codes can be found in the `code` field at
            // https://api.guildwars2.com/v2/legends/:id
            std::uint8_t activeTerriestralLegend;
            std::uint8_t inactiveTerrestrialLegend;
            std::uint8_t activeAquaticLegend;
            std::uint8_t inactiveAquaticLegend;
            std::uint16_t inactiveTerrestrialLegendUtilitySkillPalette1;
            std::uint16_t inactiveTerrestrialLegendUtilitySkillPalette2;
            std::uint16_t inactiveTerrestrialLegendUtilitySkillPalette3;
            std::uint16_t inactiveAquaticLegendUtilitySkillPalette1;
            std::uint16_t inactiveAquaticLegendUtilitySkillPalette2;
            std::uint16_t inactiveAquaticLegendUtilitySkillPalette3;
        } revenant;
    };
	
	// additional data from SotO goes here
	// u8 count of weapon palettes
	// u16 little endian weapon palette id (no api endpoint) [multiple; count]
	// u8 count of weapon variants
	// u32 little endian skill id [multiple; count]
};
