#[derive(Debug)]
pub struct Block1<'a> {
    id: i8,
    name: String,
    description: String,
    data: &'a str,
}

impl<'a> Block1<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        Block1 {
            id: id,
            name: "Basic".to_string(),
            description: "Fundamental reference for any particular message".to_string(),
            data: data,
        }
    }
}

#[derive(Debug)]
pub struct Block2<'a> {
    id: i8,
    name: String,
    description: String,
    data: &'a str,
}

impl<'a> Block2<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        Block2 {
            id: id,
            name: "Application".to_string(),
            description: "Information about the message itself".to_string(),
            data: data,
        }
    }
}

#[derive(Debug)]
pub struct Block3<'a> {
    id: i8,
    name: String,
    description: String,
    data: &'a str,
}

impl<'a> Block3<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        Block3 {
            id: id,
            name: "User".to_string(),
            description: "Allows users to provide their own referenceaaaasssssssssssss".to_string(),
            data: data,
        }
    }
}

#[derive(Debug)]
pub struct Block4<'a> {
    id: i8,
    name: String,
    description: String,
    data: &'a str,
    tags: String, //Vec<Tag>,
}

impl<'a> Block4<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        Block4 {
            id: id,
            name: "Text".to_string(),
            description: "Contains the text of the message.".to_string(),
            data: data,
            tags: "".to_string(), //vec![],
        }
    }
}

#[derive(Debug)]
pub struct Block5<'a> {
    id: i8,
    name: String,
    description: String,
    data: &'a str,
}

impl<'a> Block5<'a> {
    pub fn new(id: i8, data: &'a str) -> Self {
        Block5 {
            id: id,
            name: "Trailers".to_string(),
            description: "Indicates special circumstances that relate to message handling or contains security information".to_string(),
            data: data,
        }
    }
}

//fn parse_tags(&mut self) {
//    let tag_start_index: Vec<usize> = self
//        .data
//        .match_indices(|i, d| is_tag(d))
//        .map(|(i, _)| i)
//        .collect();
//    println!("tag_start_index = {:?}", tag_start_index);
//}
