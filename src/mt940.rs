use crate::blocks;

#[derive(Debug)]
pub struct Mt940<'a> {
    pub data: &'a str,
    pub block1: Option<blocks::Block1<'a>>,
    pub block2: Option<blocks::Block2<'a>>,
    pub block3: Option<blocks::Block3<'a>>,
    pub block4: Option<blocks::Block4<'a>>,
    pub block5: Option<blocks::Block5<'a>>,
}

impl<'a> Mt940<'a> {
    pub fn new(data: &'a str) -> Self {
        Mt940 {
            data: data,
            block1: None,
            block2: None,
            block3: None,
            block4: None,
            block5: None,
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
                    self.block1 = Some(blocks::Block1::new(block_id, block_data));
                }
                2 => {
                    self.block2 = Some(blocks::Block2::new(block_id, block_data));
                }
                3 => {
                    self.block3 = Some(blocks::Block3::new(block_id, block_data));
                }
                4 => {
                    self.block4 = Some(blocks::Block4::new(block_id, block_data));
                }
                5 => {
                    self.block5 = Some(blocks::Block5::new(block_id, block_data));
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
