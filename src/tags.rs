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

#[derive(Debug)]
pub struct Tag25 {
    name: String,
    description: String,
    key: String,
    value: String,
}

impl Tag25 {
    pub fn new(value: String) -> Self {
        Tag25 {
            name: "Account Identification".to_string(),
            description: "Identifies the account for which the statement is sent".to_string(),
            key: "25".to_string(),
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag28C {
    name: String,
    description: String,
    key: String,
    value: String,
}

impl Tag28C {
    pub fn new(value: String) -> Self {
        Tag28C {
            name: "Statement Number / Sequence Number".to_string(),
            description: "Sequential number of the statement, optionally followed by the sequence number of the message".to_string(),
            key: "28C".to_string(),
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag60F {
    name: String,
    description: String,
    key: String,
    value: String,
}

impl Tag60F {
    pub fn new(value: String) -> Self {
        Tag60F {
            name: "Opening Balance".to_string(),
            description: "Whether it is a debit or credit balance, the date, the currency and the amount of the balance".to_string(),
            key: "60F".to_string(),
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag62F {
    name: String,
    description: String,
    key: String,
    value: String,
}

impl Tag62F {
    pub fn new(value: String) -> Self {
        Tag62F {
            name: "Closing Balance (Booked Funds)".to_string(),
            description: "Whether it is a debit or credit balance, the date, the currency and the amount of the balance".to_string(),
            key: "62F".to_string(),
            value: value,
        }
    }
}

fn is_tag(data: &str) {
    let tag_regex = Regex::new(r"(?m)(:(\d\d|\d\d[A-Z]):.+)").unwrap();
    for tag in tag_regex.captures_iter(data) {
        println!("tag = {:?}", tag);
    }
}

#[derive(Debug)]
pub struct Tags {
    tag20: Option<Tag20>,
    tag25: Option<String>,
    tag28c: Option<String>,
    tag60f: Option<String>,
    tag62f: Option<String>,
}

impl Tags {
    pub fn new(id: i8, data: String) -> Self {
        Tag {
            tag20: None, // new Tag20
            tag25: None,
            tag28c: None,
            tag60f: None,
            tag62f: None,
        }
    }
}
