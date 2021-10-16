use crate::blocks;

#[derive(Debug)]
pub struct Mt940<'a> {
    pub raw_data: &'a str,
    pub block_basic: blocks::Basic<'a>,
    pub block_application: blocks::Application<'a>,
    pub block_user: blocks::User<'a>,
    pub block_text: blocks::Text<'a>,
    pub block_trailer: blocks::Trailer<'a>,
}

impl<'a> Mt940<'a> {
    pub fn new(input: &'a str) -> Self {
        Mt940 {
            raw_data: input,
            block_basic: blocks::Basic::new(),
            block_application: blocks::Application::new(),
            block_user: blocks::User::new(),
            block_text: blocks::Text::new(),
            block_trailer: blocks::Trailer::new(),
        }
    }

    pub fn parse(&mut self) {
        self.parse_blocks();
    }

    fn parse_blocks(&mut self) {
        let block_start: Vec<usize> = self.raw_data.match_indices("{").map(|(i, _)| i).collect();
        let block_end: Vec<usize> = self.raw_data.match_indices("}").map(|(i, _)| i).collect();
        let block_segments = block_start.iter().zip(block_end.iter());

        for (i, (start, end)) in block_segments.enumerate() {
            let id = 1 + i as i8;
            let data = self.strip_block(id, start, end);

            match id {
                1 => {
                    self.block_basic.data = data;
                }
                2 => {
                    self.block_application.data = data;
                }
                3 => {
                    self.block_user.data = data;
                }
                4 => {
                    self.block_text.data = data;
                    self.block_text.parse_tags();
                }
                5 => {
                    self.block_trailer.data = data;
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

        self.raw_data[*start..=*end]
            .strip_prefix(&prefix)
            .unwrap()
            .strip_suffix(suffix)
            .unwrap()
    }
}
