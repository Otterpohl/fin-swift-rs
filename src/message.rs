use crate::mt940;

#[derive(Debug)]
pub struct Message<'a> {
    pub data: mt940::MT940<'a>,
}

impl<'a> Message<'a> {
    pub fn new(message_data: &'a str) -> Message {
        let message_type_start = message_data
            .match_indices('\u{007B}') // {
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()[1]
            + 4;
        let message_type_end = message_type_start + 3;
        let message_type = &message_data[message_type_start..message_type_end];

        match message_type {
            "940" => Self {
                data: mt940::MT940::new(message_data),
            },
            _ => {
                panic!("Unknown swift type {}", message_type);
            }
        }
    }
}
