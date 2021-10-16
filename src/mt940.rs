use crate::blocks;
use crate::tags;

#[derive(Debug)]
pub struct Mt940<'a> {
    pub raw_data: &'a str,
    pub block_basic: Option<blocks::Basic<'a>>,
    pub block_application: Option<blocks::Application<'a>>,
    pub block_user: Option<blocks::User<'a>>,
    pub block_text: Option<blocks::Text<'a>>,
    pub block_trailer: Option<blocks::Trailer<'a>>,
}

impl<'a> Mt940<'a> {
    pub fn new(input: &'a str) -> Self {
        Mt940 {
            raw_data: input,
            block_basic: None,
            block_application: None,
            block_user: None,
            block_text: None,
            block_trailer: None,
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
                    self.block_basic = Some(blocks::Basic::new(data));
                }
                2 => {
                    self.block_application = Some(blocks::Application::new(data));
                }
                3 => {
                    self.block_user = Some(blocks::User::new(data));
                }
                4 => {
                    let mut block = blocks::Text::new(data);
                    block.parse_tags();
                    self.block_text = Some(block);
                }
                5 => {
                    self.block_trailer = Some(blocks::Trailer::new(data));
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
