use crate::block::*;
use regex::Regex;

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

        let block_regex = Regex::new(r"(?m)(\{\d:)").unwrap();
        let block_start: Vec<usize> = block_regex
            .captures_iter(message_data)
            .map(|x| x.get(0).unwrap().start())
            .collect();
            
        let mut block_end: Vec<usize> = block_start
            .clone()
            .into_iter()
            .map(|x| if x == 0 { x } else { x - 1 })
            .collect();

        block_end.remove(0);
        block_end.push(message_data.len() - 1);

        let block_segments = block_start.iter().zip(block_end.iter());

        for (i, (start, end)) in block_segments.enumerate() {
            let block_id = 1 + i as i8;

            let prefix = format!("{{{}:", block_id); // TODO, clean up by using new string interpolation
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
                    unreachable!();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_parser() {
        let data = MT940::new(
            "{1:F01ASNBNL21XXXX0000000000}{2:O940ASNBNL21XXXXN}{3:}{4:
                         :20:0000000000
                         :25:NL81ASNB9999999999
                         :28C:3/1
                         :60F:C200103EUR379,29
                         :62F:C200103EUR379,29
                         -}{5:}",
        );

        let block_basic = Basic::new("F01ASNBNL21XXXX0000000000");
        let block_application = Application::new("O940ASNBNL21XXXXN");
        let block_user = User::new("");
        let block_text = Text::new(
            ":20:0000000000
                       :25:NL81ASNB9999999999
                       :28C:3/1
                       :60F:C200103EUR379,29
                       :62F:C200103EUR379,29",
        );
        let block_trailer = Trailer::new("");

        assert_eq!(data.basic, block_basic);
        assert_eq!(data.application, block_application);
        assert_eq!(data.user, block_user);
        assert_eq!(data.text, block_text);
        assert_eq!(data.trailer, block_trailer);
    }

    #[test]
    #[should_panic(expected = "unexpected block_id `6`")]
    fn test_message_wrong_id() {
        MT940::new(
            "{1:F01ASNBNL21XXXX0000000000}{2:O940ASNBNL21XXXXN}{3:}{4:
                         :20:0000000000
                         :25:NL81ASNB9999999999
                         :28C:3/1
                         :60F:C200103EUR379,29
                         :62F:C200103EUR379,29
                         -}{5:}{6:}",
        );
    }
}
