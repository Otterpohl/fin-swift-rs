#[derive(Debug)]
pub enum Tags<'a> {
    Tag20(Tag20<'a>),
    Tag25(Tag25<'a>),
    Tag28C(Tag28C<'a>),
    Tag60F(Tag60F<'a>),
    Tag61(Tag61<'a>),
    Tag62F(Tag62F<'a>),
    Tag64(Tag64<'a>),
    Tag86(Tag86<'a>),
}

#[derive(Debug)]
pub struct Tag20<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag20<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag20 {
            key: "20",
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag25<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag25<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag25 {
            key: "25",
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag28C<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag28C<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag28C {
            key: "28C",
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag60F<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag60F<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag60F {
            key: "60F",
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag62F<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag62F<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag62F {
            key: "62F",
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag61<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag61<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag61 {
            key: "61",
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag86<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag86<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag86 {
            key: "86",
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag64<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Tag64<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag64 {
            key: "64",
            value: value,
        }
    }
}
