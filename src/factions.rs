#[derive(Debug, PartialEq)]
pub enum Factions {
    Cosmic,
    Void,
    Galactic,
    Quantum,
    Dominion,
    Astro,
    Corsairs,
    Obsidian,
    Aegis,
    United,
    Solitary,
    Cobalt,
    Omega,
    Echo,
    Lords,
    Cult,
    Ancients,
    Shadow,
    Ethereal,
}

impl std::str::FromStr for Factions {
    type Err = String; // Define the error type

    fn from_str(s: &str) -> Result<Factions, String> {
        match s {
            "COSMIC" => Ok(Factions::Cosmic),
            "VOID" => Ok(Factions::Void),
            "GALACTIC" => Ok(Factions::Galactic),
            "QUANTUM" => Ok(Factions::Quantum),
            "DOMINION" => Ok(Factions::Dominion),
            "ASTRO" => Ok(Factions::Astro),
            "CORSAIRS" => Ok(Factions::Corsairs),
            "OBSIDIAN" => Ok(Factions::Obsidian),
            "AEGIS" => Ok(Factions::Aegis),
            "UNITED" => Ok(Factions::United),
            "SOLITARY" => Ok(Factions::Solitary),
            "COBALT" => Ok(Factions::Cobalt),
            "OMEGA" => Ok(Factions::Omega),
            "ECHO" => Ok(Factions::Echo),
            "LORDS" => Ok(Factions::Lords),
            "CULT" => Ok(Factions::Cult),
            "ANCIENTS" => Ok(Factions::Ancients),
            "SHADOW" => Ok(Factions::Shadow),
            "ETHEREAL" => Ok(Factions::Ethereal),
            _ => Err("oops".to_string())
        }
    }
}

impl ToString for Factions {
    fn to_string(&self) -> String {
        match self {
            Factions::Cosmic => "COSMIC".to_string(),
            Factions::Void => "VOID".to_string(),
            Factions::Galactic => "GALACTIC".to_string(),
            Factions::Quantum => "QUANTUM".to_string(),
            Factions::Dominion => "DOMINION".to_string(),
            Factions::Astro => "ASTRO".to_string(),
            Factions::Corsairs => "CORSAIRS".to_string(),
            Factions::Obsidian => "OBSIDIAN".to_string(),
            Factions::Aegis => "AEGIS".to_string(),
            Factions::United => "UNITED".to_string(),
            Factions::Solitary => "SOLITARY".to_string(),
            Factions::Cobalt => "COBALT".to_string(),
            Factions::Omega => "OMEGA".to_string(),
            Factions::Echo => "ECHO".to_string(),
            Factions::Lords => "LORDS".to_string(),
            Factions::Cult => "CULT".to_string(),
            Factions::Ancients => "ANCIENTS".to_string(),
            Factions::Shadow => "SHADOW".to_string(),
            Factions::Ethereal => "ETHEREAL".to_string(),
        }
    }
}
