use crate::tag::*;
use crate::utils::*;
use regex::Regex;

// https://www.paiementor.com/swift-mt-message-block-1-basic-header-description
// https://www2.swift.com/knowledgecentre/publications/us9m_20180720/?topic=ajc.htm#genajc

// Block 1
// Fundamental reference for any particular message
#[derive(Debug)]
pub struct Basic<'a> {
    pub application_id: &'a str,
    pub service_id: &'a str,
    pub source_address: LogicalTerminalAddress<'a>,
    pub session_number: &'a str,
    pub sequence_number: &'a str,
}

impl<'a> Basic<'a> {
    pub fn new(block_data: &'a str) -> Self {
        let application_id = match &block_data[..1] {
            n @ ("F" | "A" | "L") => n,
            n => {
                panic!("unexpected application_id `{}` in Basic block", n)
            }
        };

        let service_id = match &block_data[1..3] {
            n @ ("01" | "21") => n,
            n => {
                panic!("unexpected service_id `{}` in Basic block", n)
            }
        };

        let source_address = LogicalTerminalAddress::new(&block_data[3..15]);

        Self {
            application_id,
            service_id,
            source_address,
            session_number: &block_data[15..19],
            sequence_number: &block_data[19..],
        }
    }
}

// Block 2
// Information about the message itself
#[derive(Debug)]
pub struct Application<'a> {
    pub input_output_id: &'a str,
    pub message_type: &'a str,
    pub destination_address: LogicalTerminalAddress<'a>,
    pub priority: Option<&'a str>,
    pub delivery_monitoring: Option<&'a str>,
    pub obsolescence_period: Option<&'a str>,
}

impl<'a> Application<'a> {
    pub fn new(block_data: &'a str) -> Self {
        let input_output_id = match &block_data[..1] {
            n @ "I" | n @ "O" => {
                // struct this?
                n
            }
            n => {
                panic!("unexpected input_output_id `{}` in Application block", n)
            }
        };

        let message_type = match &block_data[1..4] {
            n @ "940" => n,
            n => {
                panic!("unexpected message_type `{}`", n)
            }
        };

        let mut priority = None;
        let mut delivery_monitoring = None;
        let mut obsolescence_period = None;

        if block_data.len() >= 16 {
            priority = Some(&block_data[16..17]);
        }

        if block_data.len() >= 18 {
            delivery_monitoring = Some(&block_data[17..18]);
        }

        if block_data.len() >= 21 {
            obsolescence_period = Some(&block_data[18..]);
        }

        let destination_address = LogicalTerminalAddress::new(&block_data[4..16]);

        Self {
            input_output_id,
            message_type,
            destination_address,
            priority,
            delivery_monitoring,
            obsolescence_period,
        }
    }
}

// Block 3
// Allows users to provide their own reference
#[derive(Debug)]
pub struct User<'a> {
    pub data: Option<&'a str>,
}

impl<'a> User<'a> {
    pub fn new(block_data: &'a str) -> Self {
        let mut data = None;

        if !block_data.is_empty() {
            data = Some(block_data)
        }

        Self { data }
    }
}

// Block 4
// Contains the text of the message
#[derive(Debug)]
pub struct Text<'a> {
    pub tag_20: TransactionReferenceNumber<'a>,
    pub tag_25: AccountIdentification<'a>,
    pub tag_28c: StatementNumber,
    pub tag_60f: OpeningBalance,
    pub tag_61: Vec<StatementLine<'a>>,
    pub tag_62f: BookedFunds,
    pub tag_64: Option<ClosingAvailableBalance>,
    pub tag_86: Vec<InformationToAccountOwner<'a>>,
}

impl<'a> Text<'a> {
    pub fn new(block_data: &'a str) -> Self {
        let mut transaction_reference_number = None;
        let mut tag_account_identification = None;
        let mut statement_number = None;
        let mut opening_balance = None;
        let mut booked_funds_final = None;
        let mut statement_line: Vec<StatementLine> = vec![];
        let mut information_to_account_owner: Vec<InformationToAccountOwner> = vec![];
        let mut closing_available_balance = None;

        let tag_regex = Regex::new(r"(?m)(?:(\d{2}|\d{2}[A-Z]):.+)").unwrap();

        for tag in tag_regex.captures_iter(block_data) {
            let block_key = tag.get(1).unwrap().as_str();
            let block_data = tag.get(0).unwrap().as_str();
            let value = block_data[block_key.len()..block_data.len()]
                .trim_matches(|c| c == ':' || c == '\r');

            match block_key {
                "20" => {
                    transaction_reference_number = Some(TransactionReferenceNumber::new(value));
                }
                "25" => {
                    tag_account_identification = Some(AccountIdentification::new(value));
                }
                "28C" => {
                    statement_number = Some(StatementNumber::new(value));
                }
                "60F" => {
                    opening_balance = Some(OpeningBalance::new(BalanceType::Final, value));
                }
                "60M" => {
                    opening_balance = Some(OpeningBalance::new(BalanceType::Intermediary, value));
                }
                "62F" => {
                    booked_funds_final = Some(BookedFunds::new(BalanceType::Final, value));
                }
                "62M" => {
                    booked_funds_final = Some(BookedFunds::new(BalanceType::Intermediary, value));
                }
                "61" => {
                    statement_line.push(StatementLine::new(value));
                }
                "86" => {
                    information_to_account_owner.push(InformationToAccountOwner::new(value));
                }
                "64" => {
                    closing_available_balance = Some(ClosingAvailableBalance::new(value));
                }
                _ => {
                    panic!("We really shouldn't have reached this, too bad!");
                }
            };
        }

        Self {
            tag_20: transaction_reference_number.unwrap(),
            tag_25: tag_account_identification.unwrap(),
            tag_28c: statement_number.unwrap(),
            tag_60f: opening_balance.unwrap(),
            tag_61: statement_line,
            tag_62f: booked_funds_final.unwrap(),
            tag_64: closing_available_balance,
            tag_86: information_to_account_owner,
        }
    }
}

// Block 5
// Indicates special circumstances that relate to message handling or contains security information
#[derive(Debug)]
pub struct Trailer<'a> {
    pub data: Option<&'a str>,
}

impl<'a> Trailer<'a> {
    pub fn new(block_data: &'a str) -> Self {
        let mut data = None;

        if !block_data.is_empty() {
            data = Some(block_data)
        }

        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_basic() {
        let block_data = "F01ASNBNL21XXXX0000000000";
        let data = Basic::new(block_data);
        let source_address = LogicalTerminalAddress::new(&block_data[3..15]);

        assert_eq!(data.application_id, "F");
        assert_eq!(data.service_id, "01");
        assert_eq!(data.source_address, source_address);
        assert_eq!(data.session_number, "0000");
        assert_eq!(data.sequence_number, "000000");
    }

    #[test]
    #[should_panic(expected = "unexpected application_id `T` in Basic block")]
    fn test_block_basic_application_id() {
        Basic::new("T01ASNBNL21XXXX0000000000");
    }

    #[test]
    #[should_panic(expected = "unexpected service_id `02` in Basic block")]
    fn test_block_service_id() {
        Basic::new("F02ASNBNL21XXXX0000000000");
    }

    #[test]
    fn test_block_application() {
        let block_data = "O940ASNBNL21XXXXN3123";
        let data = Application::new(block_data);
        let destination_address = LogicalTerminalAddress::new(&block_data[4..16]);

        assert_eq!(data.input_output_id, "O");
        assert_eq!(data.message_type, "940");
        assert_eq!(data.destination_address, destination_address);
        assert_eq!(data.priority, Some("N"));
        assert_eq!(data.delivery_monitoring, Some("3"));
        assert_eq!(data.obsolescence_period, Some("123"));
    }

    #[test]
    #[should_panic(expected = "unexpected input_output_id `B` in Application block")]
    fn test_block_application_input_output_id() {
        Application::new("B940ASNBNL21XXXXN");
    }

    #[test]
    #[should_panic(expected = "unexpected message_type `537`")]
    fn test_block_application_message_type() {
        Application::new("O537ASNBNL21XXXXN");
    }

    #[test]
    fn test_block_user() {}

    #[test]
    fn test_block_text() {
        let data = Text::new(
            ":20:3996-11-11111111
:25:DABADKKK/111111-11111111
:28C:00001/001
:60F:C090924EUR54484,04
:61:0909250925DR583,92NMSC1110030403010139//1234
:86:11100304030101391234
:86:Fees according to advice
:62F:C090930EUR53126,94
:64:C090930EUR53189,31",
        );

        let tag_20 = TransactionReferenceNumber::new("3996-11-11111111");
        let tag_25 = AccountIdentification::new("DABADKKK/111111-11111111");
        let tag_28c = StatementNumber::new("00001/001");
        let tag_64 = ClosingAvailableBalance::new("C090930EUR53189,31");

        let mut tag_61: Vec<StatementLine> = vec![];
        let mut tag_86: Vec<InformationToAccountOwner> = vec![];
        tag_61.push(StatementLine::new(
            "0909250925DR583,92NMSC1110030403010139//1234",
        ));
        tag_86.push(InformationToAccountOwner::new("11100304030101391234"));
        tag_86.push(InformationToAccountOwner::new("Fees according to advice"));

        assert_eq!(data.tag_20, tag_20);
        assert_eq!(data.tag_25, tag_25);
        assert_eq!(data.tag_28c, tag_28c);

        assert_eq!(data.tag_61, tag_61);
        assert_eq!(data.tag_86, tag_86);
        assert_eq!(data.tag_64.unwrap(), tag_64);
    }

    #[test]
    fn test_block_text_final() {
        let data = Text::new(
            ":20:3996-11-11111111
:25:DABADKKK/111111-11111111
:28C:00001/001
:60F:C090924EUR54484,04
:61:0909250925DR583,92NMSC1110030403010139//1234
:86:11100304030101391234
:86:Fees according to advice
:62F:C090930EUR53126,94
:64:C090930EUR53189,31",
        );

        let tag_60 = OpeningBalance::new(BalanceType::Final, "C090924EUR54484,04");
        let tag_62 = BookedFunds::new(BalanceType::Final, "C090930EUR53126,94");

        assert_eq!(data.tag_60, tag_60);
        assert_eq!(data.tag_62, tag_62);
    }

    #[test]
    fn test_block_text_intermediary() {
        let data = Text::new(
            ":20:3996-11-11111111
:25:DABADKKK/111111-11111111
:28C:00001/001
:60M:C090924EUR54484,04
:61:0909250925DR583,92NMSC1110030403010139//1234
:86:11100304030101391234
:86:Fees according to advice
:62M:C090930EUR53126,94
:64:C090930EUR53189,31",
        );

        let tag_60 = OpeningBalance::new(BalanceType::Intermediary, "C090924EUR54484,04");
        let tag_62 = BookedFunds::new(BalanceType::Intermediary, "C090930EUR53126,94");

        assert_eq!(data.tag_60, tag_60);
        assert_eq!(data.tag_62, tag_62);
    }

    #[test]
    #[should_panic(expected = "unexpected block key `69M` in Basic block")]
    fn test_block_text_wrong_tag() {
        Text::new(
            ":20:3996-11-11111111
:25:DABADKKK/111111-11111111
:28C:00001/001
:69M:C090924EUR54484,04
:61:0909250925DR583,92NMSC1110030403010139//1234
:86:11100304030101391234
:86:Fees according to advice
:62M:C090930EUR53126,94
:64:C090930EUR53189,31",
        );
    }

    #[test]
    fn test_block_trailer() {}
}
