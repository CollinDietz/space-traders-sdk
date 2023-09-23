use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Faction {
    pub symbol: Factions,
    pub name: String,
    pub description: String,
    pub headquarters: String,
    pub traits: Vec<Trait>,
    pub is_recruiting: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trait {
    pub symbol: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::string;

    pub fn some_faction() -> Faction {
        Faction {
            symbol: Factions::Cosmic,
            name: string!("Cosmic Engineers"),
            description: string!("The Cosmic Engineers are a group of highly advanced scientists and engineers who seek to terraform and colonize new worlds, pushing the boundaries of technology and exploration."),
            headquarters: string!("X1-GM20-33220C"),
            traits: vec![
               Trait {
                  name: string!("Innovative"),
                  symbol: string!("INNOVATIVE"),
                  description: string!("Willing to try new and untested ideas. Sometimes able to come up with creative and original solutions to problems, and may be able to think outside the box. Sometimes at the forefront of technological or social change, and may be willing to take risks in order to advance the boundaries of human knowledge and understanding.")
               },
               Trait {
                  name: string!("Bold"),
                  symbol: string!("BOLD"),
                  description: string!("Unafraid to take risks and challenge the status quo. Sometimes willing to do things that others would not dare, and may be able to overcome obstacles and challenges that would be insurmountable for others. Sometimes able to inspire and motivate others to take bold action as well.")
               },
               Trait {
                  name: string!("Visionary"),
                  symbol: string!("VISIONARY"),
                  description: string!("Possessing a clear and compelling vision for the future. Sometimes able to see beyond the present and anticipate the needs and challenges of tomorrow. Sometimes able to inspire and guide others towards a better and brighter future, and may be willing to take bold and decisive action to make their vision a reality.")
               },
               Trait {
                  name: string!("Curious"),
                  symbol: string!("CURIOUS"),
                  description: string!("Possessing a strong desire to learn and explore. Sometimes interested in a wide range of topics and may be willing to take risks in order to satisfy their curiosity. Sometimes able to think outside the box and come up with creative solutions to challenges.")
               },
            ],
            is_recruiting: true
        }
    }
    #[test]
    fn should_be_deserializable() {
        let json_str = r#"
        {
            "symbol": "COSMIC",
            "name": "Cosmic Engineers",
            "description": "The Cosmic Engineers are a group of highly advanced scientists and engineers who seek to terraform and colonize new worlds, pushing the boundaries of technology and exploration.",
            "headquarters": "X1-GM20-33220C",
            "traits": [
               {
                  "symbol": "INNOVATIVE",
                  "name": "Innovative",
                  "description": "Willing to try new and untested ideas. Sometimes able to come up with creative and original solutions to problems, and may be able to think outside the box. Sometimes at the forefront of technological or social change, and may be willing to take risks in order to advance the boundaries of human knowledge and understanding."
               },
               {
                  "symbol": "BOLD",
                  "name": "Bold",
                  "description": "Unafraid to take risks and challenge the status quo. Sometimes willing to do things that others would not dare, and may be able to overcome obstacles and challenges that would be insurmountable for others. Sometimes able to inspire and motivate others to take bold action as well."
               },
               {
                  "symbol": "VISIONARY",
                  "name": "Visionary",
                  "description": "Possessing a clear and compelling vision for the future. Sometimes able to see beyond the present and anticipate the needs and challenges of tomorrow. Sometimes able to inspire and guide others towards a better and brighter future, and may be willing to take bold and decisive action to make their vision a reality."
               },
               {
                  "symbol": "CURIOUS",
                  "name": "Curious",
                  "description": "Possessing a strong desire to learn and explore. Sometimes interested in a wide range of topics and may be willing to take risks in order to satisfy their curiosity. Sometimes able to think outside the box and come up with creative solutions to challenges."
               }
            ],
            "isRecruiting": true
         }"#;

        let actual: Faction = serde_json::from_str(json_str).unwrap();

        let expected = some_faction();

        assert_eq!(expected, actual);
    }
}
