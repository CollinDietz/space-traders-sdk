use serde_derive::Deserialize;

use crate::faction::Factions;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
    pub name: String,
    pub faction_symbol: Factions,
    pub role: ShipRole,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ShipRole {
    Fabricator,
    Harvester,
    Hauler,
    Interceptor,
    Excavator,
    Transport,
    Repair,
    Surveyor,
    Command,
    Carrier,
    Patrol,
    Satellite,
    Explorer,
    Refinery,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::string;

    pub fn some_registration() -> Registration {
        Registration {
            name: string!("BADGER-1"),
            faction_symbol: Factions::Cosmic,
            role: ShipRole::Command,
        }
    }

    #[test]
    fn some_registration_should_be_deserializable() {
        let json_str = r#"
        {
            "name": "BADGER-1",
            "factionSymbol": "COSMIC",
            "role": "COMMAND"
        }"#;

        let actual: Registration = serde_json::from_str(json_str).unwrap();
        let expected = some_registration();

        assert_eq!(expected, actual);
    }

    pub fn some_other_registration() -> Registration {
        Registration {
            name: string!("BADGER-2"),
            faction_symbol: Factions::Cosmic,
            role: ShipRole::Satellite,
        }
    }

    #[test]
    fn some_other_registration_should_be_deserializable() {
        let json_str = r#"
        {
            "name": "BADGER-2",
            "factionSymbol": "COSMIC",
            "role": "SATELLITE"
        }"#;

        let actual: Registration = serde_json::from_str(json_str).unwrap();
        let expected = some_other_registration();

        assert_eq!(expected, actual);
    }
}
