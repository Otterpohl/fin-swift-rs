use crate::tags;
use regex::Regex;

#[derive(Debug)]
pub struct Basic<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    pub data: &'a str,
}

impl<'a> Basic<'a> {
    pub fn new() -> Self {
        Basic {
            id: 1,
            name: "Basic",
            description: "Fundamental reference for any particular message",
            data: "",
        }
    }
}

#[derive(Debug)]
pub struct Application<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    pub data: &'a str,
}

impl<'a> Application<'a> {
    pub fn new() -> Self {
        Application {
            id: 2,
            name: "Application",
            description: "Information about the message itself",
            data: "",
        }
    }
}

#[derive(Debug)]
pub struct User<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    pub data: &'a str,
}

impl<'a> User<'a> {
    pub fn new() -> Self {
        User {
            id: 3,
            name: "User",
            description: "Allows users to provide their own reference",
            data: "",
        }
    }
}

#[derive(Debug)]
pub struct Text<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    pub data: &'a str,
    pub tags: Vec<tags::Tags<'a>>,
}

impl<'a> Text<'a> {
    pub fn new() -> Self {
        Text {
            id: 4,
            name: "Text",
            description: "Contains the text of the message",
            data: "",
            tags: vec![],
        }
    }

    pub fn parse_tags(&mut self) {
        let tag_regex = Regex::new(r"(?m)(?:(\d\d|\d\d[A-Z]):.+)").unwrap();

        for tag in tag_regex.captures_iter(self.data) {
            let key = tag.get(1).unwrap().as_str();
            let data = tag.get(0).unwrap().as_str();
            let value = data[key.len()..data.len()].trim_matches(|c| c == ':' || c == '\r');

            match key {
                "20" => {
                    self.tags.push(tags::Tags::Tag20(tags::Tag20::new(value)));
                }
                "25" => {
                    self.tags.push(tags::Tags::Tag25(tags::Tag25::new(value)));
                }
                "28C" => {
                    self.tags.push(tags::Tags::Tag28C(tags::Tag28C::new(value)));
                }
                "60F" => {
                    self.tags.push(tags::Tags::Tag60F(tags::Tag60F::new(value)));
                }
                "62F" => {
                    self.tags.push(tags::Tags::Tag62F(tags::Tag62F::new(value)));
                }
                "61" => {
                    self.tags.push(tags::Tags::Tag61(tags::Tag61::new(value)));
                }
                "86" => {
                    self.tags.push(tags::Tags::Tag86(tags::Tag86::new(value)));
                }
                "64" => {
                    self.tags.push(tags::Tags::Tag64(tags::Tag64::new(value)));
                }
                _ => {
                    panic!("We really shouldn't have reached this, too bad!");
                }
            };
        }
    }
}

#[derive(Debug)]
pub struct Trailer<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    pub data: &'a str,
}

impl<'a> Trailer<'a> {
    pub fn new() -> Self {
        Trailer {
            id: 5,
            name: "Trailer",
            description: "Indicates special circumstances that relate to message handling or contains security information",
            data: "",
        }
    }
}
