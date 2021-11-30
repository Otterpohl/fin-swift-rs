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
        
        let mut tag_20 = None;
        let mut tag_25 = None;
        let mut tag_28c = None;
        let mut tag_60f = None;
        let mut tag_62f = None;
        let mut tag_61 = None;
        let mut tag_86 = None;
        let mut tag_64 = None;
        
        let tag_regex = Regex::new(r"(?m)(?:(\d\d|\d\d[A-Z]):.+)").unwrap();

        for tag in tag_regex.captures_iter(block_data) {
            let key = tag.get(1).unwrap().as_str();
            let block_data = tag.get(0).unwrap().as_str();
            let value =
                block_data[key.len()..block_data.len()].trim_matches(|c| c == ':' || c == '\r');

            match key {
                "20" => {
                    tag_20 = Some(tags::Tag20::new(value));
                }
                "25" => {
                    tag_25 = Some(tags::Tag25::new(value));
                }
                "28C" => {
                    tag_28c = Some(tags::Tag28C::new(value));
                }
                "60F" => {
                    tag_60f = Some(tags::Tag60F::new(value));
                }
                "62F" => {
                    tag_62f = Some(tags::Tag62F::new(value));
                }
                "61" => {
                    tag_61 = Some(tags::Tag61::new(value));
                }
                "86" => {
                    tag_86 = Some(tags::Tag86::new(value));
                }
                "64" => {
                    tag_64 = Some(tags::Tag64::new(value));
                }
                _ => {
                    panic!("We really shouldn't have reached this, too bad!");
                }
            };
        }

        Text {
            description: "Contains the text of the message",
            tag_20: tag_20.unwrap(),
            tag_25: tag_25.unwrap(),
            tag_28c: tag_28c.unwrap(),
            tag_60f: tag_60f.unwrap(),
            tag_62f: tag_62f.unwrap(),
            tag_61: tag_61.unwrap(),
            tag_86: tag_86.unwrap(),
            tag_64: tag_64.unwrap(),
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
