use serde_derive::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cargo {
    pub capacity: i32,
    pub units: i32,
    pub inventory: Vec<InventoryItem>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItem {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub units: i32,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn some_cargo() -> Cargo {
        Cargo {
            capacity: 40,
            units: 0,
            inventory: vec![],
        }
    }

    #[test]
    fn should_be_deserializable() {
        let json_str = r#"
          {
              "capacity": 40,
              "units": 0,
              "inventory": []
          }"#;

        let actual: Cargo = serde_json::from_str(json_str).unwrap();
        let expected = some_cargo();

        assert_eq!(expected, actual);
    }
}
