use crate::block::*;

// https://www.paiementor.com/swift-mt950-statement-message-detailed-analysis/


#[derive(Debug)]
pub struct MT940<'a> {
    pub basic: Basic<'a>,
    pub application: Application<'a>,
    pub user: User<'a>,
    pub text: Text<'a>,
    pub trailer: Trailer<'a>,
}

impl<'a> MT940<'a> {
    pub fn new(message_data: &'a str) -> Self {
        let mut block_1 = None;
        let mut block_2 = None;
        let mut block_3 = None;
        let mut block_4 = None;
        let mut block_5 = None;

        let block_start: Vec<usize> = message_data
            .match_indices('{')
            .map(|(i, _)| i)
            .collect();
        let block_end: Vec<usize> = message_data
            .match_indices('}')
            .map(|(i, _)| i)
            .collect();
        let block_segments = block_start.iter().zip(block_end.iter());

        for (i, (start, end)) in block_segments.enumerate() {
            let block_id = 1 + i as i8;

            let prefix = format!("{{{}:", block_id);
            let suffix = match block_id {
                4 => "-}",
                1 | 2 | 3 | 5 => "}",
                _ => {
                    panic!("unexpected block_id `{}`", block_id)
                }
            };

            let block_data = message_data[*start..=*end]
                .strip_prefix(&prefix)
                .unwrap()
                .strip_suffix(suffix)
                .unwrap();

            match block_id {
                1 => {
                    block_1 = Some(Basic::new(block_data));
                }
                2 => {
                    block_2 = Some(Application::new(block_data));
                }
                3 => {
                    block_3 = Some(User::new(block_data));
                }
                4 => {
                    block_4 = Some(Text::new(block_data));
                }
                5 => {
                    // TODO: if it is zero here then lets not even create an empty struct?
                    block_5 = Some(Trailer::new(block_data));
                }
                _ => {
                    panic!("We really shouldn't have reached this, too bad!");
                }
            }
        }

        Self {
            basic: block_1.unwrap(),
            application: block_2.unwrap(),
            user: block_3.unwrap(),
            text: block_4.unwrap(),
            trailer: block_5.unwrap(),
        }
    }
}
