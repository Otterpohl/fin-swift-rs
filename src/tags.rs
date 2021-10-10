#[derive(Debug)]
pub struct Tag20 {
    name: String,
    description: String,
    key: String,
    value: String,
}

impl Tag20 {
    pub fn new(value: String) -> Self {
        Tag20 {
            name: "Transaction Reference Number".to_string(),
            description: "Used by the Sender to unambiguously identify the message".to_string(),
            key: "20".to_string(),
            value: value,
        }
    }
}
