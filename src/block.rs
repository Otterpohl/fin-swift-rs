use crate::tag;
use regex::Regex;

// TODO: add enum for block data

#[derive(Debug)]
pub struct LogicalTerminalAddress<'a> {
    bic: &'a str,
    terminal_code: &'a str, // try to make this a char?
    branch_code: &'a str,
}

impl<'a> LogicalTerminalAddress<'a> {
    fn new(data: &'a str) -> Self {
        LogicalTerminalAddress {
            bic: &data[..8],
            terminal_code: &data[8..9],
            branch_code: &data[9..],
        }
    }
}

// https://www.paiementor.com/swift-mt-message-block-1-basic-header-description
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
        // i really dont like this.
        let application_id = match &block_data[..1] {
            n @ "F" | n @ "A" | n @ "L" => { 
                n
            }
            n => {
                panic!("unexpected application_id `{}` in Basic block",n)
            }
        };

        let service_id = match &block_data[1..3] {
            n @ "01" | n @ "21" => {
                n
            }
            n => {
                 panic!("unexpected service_id `{}` in Basic block",n)
             }
        };

        let source_address = LogicalTerminalAddress::new(&block_data[3..15]);

        Basic {
            application_id,
            service_id,
            source_address,
            session_number : &block_data[15..19],
            sequence_number : &block_data[19..],
        }
    }
}

// Information about the message itself
#[derive(Debug)]
pub struct Application<'a> {
    input_output_id: &'a str,
    message_type: &'a str,
    destination_address: LogicalTerminalAddress<'a>,
    priority: Option<&'a str>,
    delivery_monitoring: Option<&'a str>,
    obsolescence_period: Option<&'a str>,
}

impl<'a> Application<'a> {
    pub fn new(block_data: &'a str) -> Self {

        let input_output_id = match &block_data[..1] {
            n @ "I" | n @ "O" => { 
                n 
            }
            n => {
                panic!("unexpected input_output_id `{}` in Application block",n)
            }
        };

        let message_type = match &block_data[1..4] {
            n @ "940" => { 
                n
            }
            n => {
                panic!("unexpected message_type `{}`",n)
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
        
        Application {
            input_output_id,
            message_type,
            destination_address,
            priority,
            delivery_monitoring,
            obsolescence_period,
        }
    }
}

// Allows users to provide their own reference
#[derive(Debug)]
pub struct User<'a> {
    pub data: Option<&'a str>,
}

impl<'a> User<'a> {
    pub fn new(block_data: &'a str) -> Self {
        let mut data = None;

        if block_data.len() > 0 {
            data = Some(block_data)
        }

        User {
            data,
        }
    }
}

// Contains the text of the message
#[derive(Debug)]
pub struct Text<'a> {
    pub tag_20: tag::TransactionReferenceNumber<'a>,
    pub tag_25: tag::AccountIdentification<'a>,
    pub tag_28c: tag::StatementNumber<'a>,
    pub tag_60f: tag::OpeningBalanceFinal<'a>,
    pub tag_62f: tag::BookedFundsFinal<'a>,
    pub tag_61: Vec<tag::StatementLine<'a>>,
    pub tag_86: Vec<tag::InformationToAccountOwner<'a>>,
    pub tag_64: Option<tag::ClosingAvailableBalance<'a>>,
}

impl<'a> Text<'a> {
    pub fn new(block_data: &'a str) -> Self {
        let mut transaction_reference_number = None;
        let mut tag_account_identification = None;
        let mut statement_number = None;
        let mut opening_balance_final = None;
        let mut booked_funds_final = None;
        let mut statement_line: Vec<tag::StatementLine> = vec![];
        let mut information_to_account_owner: Vec<tag::InformationToAccountOwner> = vec![];
        let mut closing_available_balance = None;
        
        let tag_regex = Regex::new(r"(?m)(?:(\d\d|\d\d[A-Z]):.+)").unwrap();

        for tag in tag_regex.captures_iter(block_data) {
            let block_key = tag.get(1).unwrap().as_str();
            let block_data = tag.get(0).unwrap().as_str();
            let value = block_data[block_key.len()..block_data.len()]
                    .trim_matches(|c| c == ':' || c == '\r');

            match block_key {
                "20" => {
                    transaction_reference_number = Some(tag::TransactionReferenceNumber::new(value));
                }
                "25" => {
                    tag_account_identification = Some(tag::AccountIdentification::new(value));
                }
                "28C" => {
                    statement_number = Some(tag::StatementNumber::new(value));
                }
                "60F" => {
                    opening_balance_final = Some(tag::OpeningBalanceFinal::new(value));
                }
                "62F" => {
                    booked_funds_final = Some(tag::BookedFundsFinal::new(value));
                }
                "61" => {
                    statement_line.push(tag::StatementLine::new(value));
                }
                "86" => {
                    information_to_account_owner.push(tag::InformationToAccountOwner::new(value));
                }
                "64" => {
                    closing_available_balance = Some(tag::ClosingAvailableBalance::new(value));
                }
                _ => {
                    panic!("We really shouldn't have reached this, too bad!");
                }
            };
        }

        Text {
            tag_20: transaction_reference_number.unwrap(),
            tag_25: tag_account_identification.unwrap(),
            tag_28c: statement_number.unwrap(),
            tag_60f: opening_balance_final.unwrap(),
            tag_62f: booked_funds_final.unwrap(),
            tag_61: statement_line,
            tag_86: information_to_account_owner,
            tag_64: closing_available_balance,
        }
    }
}

// Indicates special circumstances that relate to message handling or contains security information
#[derive(Debug)]
pub struct Trailer<'a> {
    pub data: Option<&'a str>,
}

impl<'a> Trailer<'a> {
    pub fn new(block_data: &'a str) -> Self {
        let mut data = None;

        if block_data.len() > 0 {
            data = Some(block_data)
        }
        
        Trailer {
            data,
        }
    }
}
