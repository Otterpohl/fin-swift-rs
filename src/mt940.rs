use crate::blocks;

#[derive(Debug)]
pub struct Mt940<'a> {
    pub message_data: &'a str,
    pub basic: blocks::Basic<'a>,
    pub application: blocks::Application<'a>,
    pub user: blocks::User<'a>,
    pub text: blocks::Text<'a>,
    pub trailer: blocks::Trailer<'a>,
}

impl<'a> Mt940<'a> {
    pub fn new(message_data: &'a str) -> Self {
        Mt940 {
            message_data: message_data, // do we really need this?
            basic: blocks::Basic::new(message_data),
            application: blocks::Application::new(message_data),
            user: blocks::User::new(message_data),
            text: blocks::Text::new(message_data),
            trailer: blocks::Trailer::new(message_data),
        }
    }

    pub fn parse(&mut self) {
        let block_start: Vec<usize> = self
            .message_data
            .match_indices("{")
            .map(|(i, _)| i)
            .collect();
        let block_end: Vec<usize> = self
            .message_data
            .match_indices("}")
            .map(|(i, _)| i)
            .collect();
        let block_segments = block_start.iter().zip(block_end.iter());

        for (i, (start, end)) in block_segments.enumerate() {
            let id = 1 + i as i8;
            let data = self.strip_block(id, start, end);

            match id {
                1 => {
                    self.basic.block_data = data;
                }
                2 => {
                    self.application.block_data = data;
                }
                3 => {
                    self.user.block_data = data;
                }
                4 => {
                    self.text.block_data = data;
                    self.text.parse_tags();
                }
                5 => {
                    self.trailer.block_data = data;
                }
                _ => {
                    panic!("We really shouldn't have reached this, too bad!");
                }
            }
        }
    }

    fn strip_block(&self, block_id: i8, start: &usize, end: &usize) -> &'a str {
        let prefix = format!("{{{}:", block_id.to_string());
        let suffix = match block_id {
            4 => "-}",
            _ => "}",
        };

        self.message_data[*start..=*end]
            .strip_prefix(&prefix)
            .unwrap()
            .strip_suffix(suffix)
            .unwrap()
    }
}
