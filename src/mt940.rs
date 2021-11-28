use crate::blocks;

#[derive(Debug)]
pub struct Mt940<'a> {
    pub data: &'a str,
    pub basic: blocks::Basic<'a>,
    pub application: blocks::Application<'a>,
    pub user: blocks::User<'a>,
    pub text: blocks::Text<'a>,
    pub trailer: blocks::Trailer<'a>,
}

impl<'a> Mt940<'a> {
    pub fn new(message_data: &'a str) -> Self {
        // TODO: do we really need the data field? can we not just parse before returning?
        Mt940 {
            data: message_data, 
            basic: blocks::Basic::new(message_data),
            application: blocks::Application::new(message_data),
            user: blocks::User::new(message_data),
            text: blocks::Text::new(message_data),
            trailer: blocks::Trailer::new(message_data),
        }
    }

    pub fn parse(&mut self) {
        let block_start: Vec<usize> = self
            .data
            .match_indices('\u{007B}') // {
            .map(|(i, _)| i)
            .collect();
        let block_end: Vec<usize> = self
            .data
            .match_indices('\u{007D}') // }
            .map(|(i, _)| i)
            .collect();
        let block_segments = block_start.iter().zip(block_end.iter());

        for (i, (start, end)) in block_segments.enumerate() {
            let id = 1 + i as i8;
            let block_data = self.strip_block(id, start, end);

            match id {
                1 => {
                    self.basic.data = block_data;
                }
                2 => {
                    self.application.data = block_data;
                }
                3 => {
                    self.user.data = block_data;
                }
                4 => {
                    self.text.data = block_data;
                    self.text.parse_tags();
                }
                5 => {
                    self.trailer.data = block_data;
                }
                _ => {
                    panic!("We really shouldn't have reached this, too bad!");
                }
            }
        }
    }

    // TODO: this does not need to be part of the message struct, trait this shit up
    fn strip_block(&self, block_id: i8, start: &usize, end: &usize) -> &'a str {
        let prefix = format!("{{{}:", block_id.to_string());
        let suffix = match block_id {
            4 => "-}",
            _ => "}",
        };

        self.data[*start..=*end]
            .strip_prefix(&prefix)
            .unwrap()
            .strip_suffix(suffix)
            .unwrap()
    }
}
