#[derive(Debug)]
pub struct Basic<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    data: &'a str,
}

impl<'a> Basic<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        Basic {
            id: id,
            name: "Basic",
            description: "Fundamental reference for any particular message",
            data: data,
        }
    }
}

#[derive(Debug)]
pub struct Application<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    data: &'a str,
}

impl<'a> Application<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        Application {
            id: id,
            name: "Application",
            description: "Information about the message itself",
            data: data,
        }
    }
}

#[derive(Debug)]
pub struct User<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    data: &'a str,
}

impl<'a> User<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        User {
            id: id,
            name: "User",
            description: "Allows users to provide their own reference",
            data: data,
        }
    }
}

#[derive(Debug)]
pub struct Text<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    data: &'a str,
    //tags: Vec<Tags>,
}

impl<'a> Text<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        Text {
            id: id,
            name: "Text",
            description: "Contains the text of the message",
            data: data,
            //tags: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Trailer<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    data: &'a str,
}

impl<'a> Trailer<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        Trailer {
            id: id,
            name: "Trailer",
            description: "Indicates special circumstances that relate to message handling or contains security information",
            data: data,
        }
    }
}
