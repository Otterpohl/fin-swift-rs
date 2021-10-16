use crate::blocks;

#[derive(Debug)]
pub struct Mt940<'a> {
    pub data: &'a str,
    pub block_basic: Option<blocks::Basic<'a>>,
    pub block_application: Option<blocks::Application<'a>>,
    pub block_user: Option<blocks::User<'a>>,
    pub block_text: Option<blocks::Text<'a>>,
    pub block_trailer: Option<blocks::Trailer<'a>>,
}

impl<'a> Mt940<'a> {
    pub fn new(data: &'a str) -> Self {
        Mt940 {
            data: data,
            block_basic: None,
            block_application: None,
            block_user: None,
            block_text: None,
            block_trailer: None,
        }
    }

    pub fn parse(&mut self) {
        self.parse_blocks();
        println!("self = {:#?}", self);
    }

    fn parse_blocks(&mut self) {
        let block_start_index: Vec<usize> = self.data.match_indices("{").map(|(i, _)| i).collect();
        let block_end_index: Vec<usize> = self.data.match_indices("}").map(|(i, _)| i).collect();
        let blocks = block_start_index.iter().zip(block_end_index.iter());

        for (i, (start, end)) in blocks.enumerate() {
            let block_id = 1 + i as i8;
            let block_data = self.strip_block(block_id, start, end);

            match block_id {
                1 => {
                    self.block_basic = Some(blocks::Basic::new(block_id, block_data));
                }
                2 => {
                    self.block_application = Some(blocks::Application::new(block_id, block_data));
                }
                3 => {
                    self.block_user = Some(blocks::User::new(block_id, block_data));
                }
                4 => {
                    self.block_text = Some(blocks::Text::new(block_id, block_data));
                }
                5 => {
                    self.block_trailer = Some(blocks::Trailer::new(block_id, block_data));
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
        self.data[*start..=*end]
            .strip_prefix(&prefix)
            .unwrap()
            .strip_suffix(suffix)
            .unwrap()
    }
}
