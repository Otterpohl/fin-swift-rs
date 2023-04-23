use crate::block::{Application, Basic, Text, Trailer, User};
use eyre::{eyre, Result};
use regex::Regex;
use serde::Serialize;

// https://www.paiementor.com/swift-mt950-statement-message-detailed-analysis/

#[derive(Debug, Serialize)]
pub struct MT940<'a> {
    pub basic: Basic<'a>,
    pub application: Application<'a>,
    pub user: User<'a>,
    pub text: Text<'a>,
    pub trailer: Trailer<'a>,
}

impl<'a> MT940<'a> {
    pub fn new(message_data: &'a str) -> Result<Self> {
        let mut block_1 = None;
        let mut block_2 = None;
        let mut block_3 = None;
        let mut block_4 = None;
        let mut block_5 = None;

        let block_regex = Regex::new(r"(?m)(\{\d:)")?;
        let block_start: Vec<usize> = block_regex
            .captures_iter(message_data)
            .map(|x| return x.get(0).expect("").start())
            .collect();

        let mut block_end: Vec<usize> = block_start
            .iter()
            .map(|&x| if x == 0 { x } else { x - 1 })
            .collect();

        block_end.remove(0);
        block_end.push(message_data.len() - 1);

        let block_segments = block_start.iter().zip(block_end.iter());

        for (i, (start, end)) in block_segments.enumerate() {
            let block_id = 1_usize + i;

            let prefix = format!("{{{block_id}:");
            let suffix = match block_id {
                4 => Ok("-}"),
                1 | 2 | 3 | 5 => Ok("}"),
                _ => Err(eyre!("unexpected block_id `{block_id}`")),
            }?;

            let block_data = message_data[*start..=*end]
                .strip_prefix(&prefix)
                .ok_or_else(|| eyre!("prefix '{prefix}' not found in block"))?
                .strip_suffix(suffix)
                .ok_or_else(|| eyre!("suffix '{suffix}' not found in block"))?;

            match block_id {
                1 => {
                    block_1 = Some(Basic::new(block_data)?);
                }
                2 => {
                    block_2 = Some(Application::new(block_data)?);
                }
                3 => {
                    block_3 = Some(User::new(block_data)?);
                }
                4 => {
                    block_4 = Some(Text::new(block_data)?);
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

        let block_1 = block_1.ok_or_else(|| eyre!("block 1 not found"))?;
        let block_2 = block_2.ok_or_else(|| eyre!("block 2 not found"))?;
        let block_3 = block_3.ok_or_else(|| eyre!("block 3 not found"))?;
        let block_4 = block_4.ok_or_else(|| eyre!("block 4 not found"))?;
        let block_5 = block_5.ok_or_else(|| eyre!("block 5 not found"))?;

        Ok(Self {
            basic: block_1,
            application: block_2,
            user: block_3,
            text: block_4,
            trailer: block_5,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        )
        .unwrap();
    }
}
