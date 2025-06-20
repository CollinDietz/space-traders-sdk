use serde_derive::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requirements {
    pub power: Option<i32>,
    pub crew: Option<i32>,
    pub slots: Option<i32>,
}
