/// One who signs
#[derive(Deserialize, Debug, PartialEq)]
pub struct Signer {
    weight: u32,
    key: String,
    #[serde(rename = "type")]
    r#type: String,
}

impl Signer {
    /// Create a Signer Object
    pub fn new(weight: u32, key: String, r#type: String) -> Signer {
        Signer {
            weight,
            key,
            r#type,
        }
    }
}
