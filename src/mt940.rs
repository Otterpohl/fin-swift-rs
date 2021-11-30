use crate::blocks::{self};

#[derive(Debug)]
pub struct MT940<'a> {
    pub basic: blocks::Basic<'a>,
    pub application: blocks::Application<'a>,
    pub user: blocks::User<'a>,
    pub text: blocks::Text<'a>,
    pub trailer: blocks::Trailer<'a>,
}

impl<'a> MT940<'a> {
    pub fn new(message_data: &'a str) -> Self {
        
        let mut basic = None;
        let mut application = None;
        let mut user = None;
        let mut text = None;
        let mut trailer = None;

        let block_start: Vec<usize> = message_data
            .match_indices('\u{007B}') // {
            .map(|(i, _)| i)
            .collect();
        let block_end: Vec<usize> = message_data
            .match_indices('\u{007D}') // }
            .map(|(i, _)| i)
            .collect();
        let block_segments = block_start.iter().zip(block_end.iter());
        
        for (i, (start, end)) in block_segments.enumerate() {
            let block_id = 1 + i as i8;

            let prefix = format!("{{{}:", block_id.to_string());
            let suffix = match block_id {
                4 => "-}",
                _ => "}",
            };

            let block_data = message_data[*start..=*end]
                .strip_prefix(&prefix)
                .unwrap()
                .strip_suffix(suffix)
                .unwrap();

            match block_id {
                1 => {
                    basic = Some(blocks::Basic::new(block_data));
                }
                2 => {
                    application = Some(blocks::Application::new(block_data));
                }
                3 => {
                    user = Some(blocks::User::new(block_data));
                }
                4 => {
                    text = Some(blocks::Text::new(block_data));
                    //self.text.parse_tags();
                }
                5 => {
                    trailer = Some(blocks::Trailer::new(block_data));
                }
                _ => {
                    panic!("We really shouldn't have reached this, too bad!");
                }
            }
        }

        MT940 {
            basic: basic.unwrap(),
            application: application.unwrap(),
            user: user.unwrap(),
            text: text.unwrap(),
            trailer: trailer.unwrap(),
        }
    }
}
