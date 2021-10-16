#[derive(Debug, Clone, Copy)]
pub struct Tag20<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag20<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag20 {
            name: "Transaction Reference Number",
            description: "Used by the Sender to unambiguously identify the message",
            key: "20",
            value: value,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tag25<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag25<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag25 {
            name: "Account Identification",
            description: "Identifies the account for which the statement is sent",
            key: "25",
            value: value,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tag28C<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag28C<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag28C {
            name: "Statement Number / Sequence Number",
            description: "Sequential number of the statement, optionally followed by the sequence number of the message",
            key: "28C",
            value: value,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tag60F<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag60F<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag60F {
            name: "Opening Balance",
            description: "Whether it is a debit or credit balance, the date, the currency and the amount of the balance",
            key: "60F",
            value: value,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tag62F<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag62F<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag62F {
            name: "Closing Balance (Booked Funds)",
            description: "Whether it is a debit or credit balance, the date, the currency and the amount of the balance",
            key: "62F",
            value: value,
        }
    }
}
