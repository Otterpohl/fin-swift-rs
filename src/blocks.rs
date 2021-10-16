use crate::tags;
use regex::Regex;

#[derive(Debug)]
pub struct Basic<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    data: &'a str,
}

impl<'a> Basic<'a> {
    pub fn new(data: &'a str) -> Self {
        Basic {
            id: 1,
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
    pub fn new(data: &'a str) -> Self {
        Application {
            id: 2,
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
    pub fn new(data: &'a str) -> Self {
        User {
            id: 3,
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
    pub tag20: Option<tags::Tag20<'a>>,
    pub tag25: Option<tags::Tag25<'a>>,
    pub tag28c: Option<tags::Tag28C<'a>>,
    pub tag60f: Option<tags::Tag60F<'a>>,
    pub tag62f: Option<tags::Tag62F<'a>>,
}

impl<'a> Text<'a> {
    pub fn new(data: &'a str) -> Self {
        Text {
            id: 4,
            name: "Text",
            description: "Contains the text of the message",
            data: data,
            tag20: None,
            tag25: None,
            tag28c: None,
            tag60f: None,
            tag62f: None,
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
                    self.tag20 = Some(tags::Tag20::new(value));
                }
                "25" => {
                    self.tag25 = Some(tags::Tag25::new(value));
                }
                "28C" => {
                    self.tag28c = Some(tags::Tag28C::new(value));
                }
                "60F" => {
                    self.tag60f = Some(tags::Tag60F::new(value));
                }
                "62F" => {
                    self.tag62f = Some(tags::Tag62F::new(value));
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
    data: &'a str,
}

impl<'a> Trailer<'a> {
    pub fn new(data: &'a str) -> Self {
        Trailer {
            id: 5,
            name: "Trailer",
            description: "Indicates special circumstances that relate to message handling or contains security information",
            data: data,
        }
    }
}
