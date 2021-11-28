use crate::tags;
use regex::Regex;

#[derive(Debug)]
pub struct Basic<'a> {
    description: &'a str,
    pub data: &'a str,
}

impl<'a> Basic<'a> {
    pub fn new(block_data: &'a str) -> Self {
        Basic {
            description: "Fundamental reference for any particular message",
            data: block_data,
        }
    }
}

#[derive(Debug)]
pub struct Application<'a> {
    description: &'a str,
    pub data: &'a str,
}

impl<'a> Application<'a> {
    pub fn new(block_data: &'a str) -> Self {
        Application {
            description: "Information about the message itself",
            data: block_data,
        }
    }
}

#[derive(Debug)]
pub struct User<'a> {
    description: &'a str,
    pub data: &'a str,
}

impl<'a> User<'a> {
    pub fn new(block_data: &'a str) -> Self {
        User {
            description: "Allows users to provide their own reference",
            data: block_data,
        }
    }
}

#[derive(Debug)]
pub struct Text<'a> {
    description: &'a str,
    pub data: &'a str,
    pub tag_20: tags::Tag20<'a>,
    pub tag_25: tags::Tag25<'a>,
    pub tag_28c: tags::Tag28C<'a>,
    pub tag_60f: tags::Tag60F<'a>,
    pub tag_62f: tags::Tag62F<'a>,
    pub tag_61: tags::Tag61<'a>,
    pub tag_86: tags::Tag86<'a>,
    pub tag_64: tags::Tag64<'a>,
}

impl<'a> Text<'a> {
    pub fn new(block_data: &'a str) -> Self {
        Text {
            description: "Contains the text of the message",
            data: block_data,
            tag_20: tags::Tag20::new(""),
            tag_25: tags::Tag25::new(""),
            tag_28c: tags::Tag28C::new(""),
            tag_60f: tags::Tag60F::new(""),
            tag_62f: tags::Tag62F::new(""),
            tag_61: tags::Tag61::new(""),
            tag_86: tags::Tag86::new(""),
            tag_64: tags::Tag64::new(""),
        }
    }

    pub fn parse_tags(&mut self) {
        let tag_regex = Regex::new(r"(?m)(?:(\d\d|\d\d[A-Z]):.+)").unwrap();

        for tag in tag_regex.captures_iter(self.data) {
            let key = tag.get(1).unwrap().as_str();
            let block_data = tag.get(0).unwrap().as_str();
            let value =
                block_data[key.len()..block_data.len()].trim_matches(|c| c == ':' || c == '\r');

            match key {
                "20" => {
                    self.tag_20.data = value;
                }
                "25" => {
                    self.tag_25.data = value;
                }
                "28C" => {
                    self.tag_28c.data = value;
                }
                "60F" => {
                    self.tag_60f.data = value;
                }
                "62F" => {
                    self.tag_62f.data = value;
                }
                "61" => {
                    self.tag_61.data = value;
                }
                "86" => {
                    self.tag_86.data = value;
                }
                "64" => {
                    self.tag_64.data = value;
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
    description: &'a str,
    pub data: &'a str,
}

impl<'a> Trailer<'a> {
    pub fn new(block_data: &'a str) -> Self {
        Trailer {
            description: "Indicates special circumstances that relate to message handling or contains security information",
            data: block_data,
        }
    }
}
