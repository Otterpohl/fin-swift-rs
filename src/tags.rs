#[derive(Debug)]
pub struct Tag20<'a> {
    pub name: &'a str,
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag20<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag20 {
            name: "Transaction Reference Number",
            key: "20",
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag25<'a> {
    pub name: &'a str,
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag25<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag25 {
            name: "Account Identification",
            key: "25",
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag28C<'a> {
    pub name: &'a str,
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag28C<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag28C {
            name: "Statement Number / Sequence Number",
            key: "28C",
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag60F<'a> {
    pub name: &'a str,
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag60F<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag60F {
            name: "Opening Balance",
            key: "60F",
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag62F<'a> {
    pub name: &'a str,
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag62F<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag62F {
            name: "Closing Balance (Booked Funds)",
            key: "62F",
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag61<'a> {
    pub name: &'a str,
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag61<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag61 {
            name: "Statement Line",
            key: "61",
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag86<'a> {
    pub name: &'a str,
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag86<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag86 {
            name: "Information to Account Owner",
            key: "86",
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag64<'a> {
    pub name: &'a str,
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag64<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag64 {
            name: "Closing Available Balance (Available Funds)",
            key: "64",
            value: value,
        }
    }
}
