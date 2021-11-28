#[derive(Debug)]
pub struct Tag20<'a> {
    pub data: &'a str,
}

impl<'a> Tag20<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag20 {
            data: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag25<'a> {
    pub data: &'a str,
}

impl<'a> Tag25<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag25 {
            data: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag28C<'a> {
    pub data: &'a str,
}

impl<'a> Tag28C<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag28C {
            data: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag60F<'a> {
    pub data: &'a str,
}

impl<'a> Tag60F<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag60F {
            data: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag62F<'a> {
    pub data: &'a str,
}

impl<'a> Tag62F<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag62F {
            data: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag61<'a> {
    pub data: &'a str,
}

impl<'a> Tag61<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag61 {
            data: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag86<'a> {
    pub data: &'a str,
}

impl<'a> Tag86<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag86 {
            data: value,
        }
    }
}

#[derive(Debug)]
pub struct Tag64<'a> {
    pub data: &'a str,
}

impl<'a> Tag64<'a> {
    pub fn new(value: &'a str) -> Self {
        Tag64 {
            data: value,
        }
    }
}
