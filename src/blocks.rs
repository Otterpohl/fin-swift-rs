#[derive(Debug)]
pub struct Block1<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    data: &'a str,
}

impl<'a> Block1<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        Block1 {
            id: id,
            name: "Basic",
            description: "Fundamental reference for any particular message",
            data: data,
        }
    }
}

#[derive(Debug)]
pub struct Block2<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    data: &'a str,
}

impl<'a> Block2<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        Block2 {
            id: id,
            name: "Application",
            description: "Information about the message itself",
            data: data,
        }
    }
}

#[derive(Debug)]
pub struct Block3<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    data: &'a str,
}

impl<'a> Block3<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        Block3 {
            id: id,
            name: "User",
            description: "Allows users to provide their own reference",
            data: data,
        }
    }
}

#[derive(Debug)]
pub struct Block4<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    data: &'a str,
    //tags: Vec<Tags>,
}

impl<'a> Block4<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        Block4 {
            id: id,
            name: "Text",
            description: "Contains the text of the message",
            data: data,
            //tags: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Block5<'a> {
    id: i8,
    name: &'a str,
    description: &'a str,
    data: &'a str,
}

impl<'a> Block5<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        Block5 {
            id: id,
            name: "Trailers",
            description: "Indicates special circumstances that relate to message handling or contains security information",
            data: data,
        }
    }
}
