#[derive(Debug)]
pub struct BlockBasic<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    data: &'a str,
}

impl<'a> BlockBasic<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        BlockBasic {
            id: id,
            name: "Basic",
            description: "Fundamental reference for any particular message",
            data: data,
        }
    }
}

#[derive(Debug)]
pub struct BlockApplication<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    data: &'a str,
}

impl<'a> BlockApplication<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        BlockApplication {
            id: id,
            name: "Application",
            description: "Information about the message itself",
            data: data,
        }
    }
}

#[derive(Debug)]
pub struct BlockUser<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    data: &'a str,
}

impl<'a> BlockUser<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        BlockUser {
            id: id,
            name: "User",
            description: "Allows users to provide their own reference",
            data: data,
        }
    }
}

#[derive(Debug)]
pub struct BlockText<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    data: &'a str,
    //tags: Vec<Tags>,
}

impl<'a> BlockText<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        BlockText {
            id: id,
            name: "Text",
            description: "Contains the text of the message",
            data: data,
            //tags: vec![],
        }
    }
}

#[derive(Debug)]
pub struct BlockTrailer<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    data: &'a str,
}

impl<'a> BlockTrailer<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        BlockTrailer {
            id: id,
            name: "Trailer",
            description: "Indicates special circumstances that relate to message handling or contains security information",
            data: data,
        }
    }
}
