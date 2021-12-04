use crate::tag;
use regex::Regex;

// https://www.paiementor.com/swift-mt-message-block-1-basic-header-
// Fundamental reference for any particular message
#[derive(Debug)]
pub struct Basic<'a> {
    pub application_id: &'a str,
    pub service_id: &'a str,
    pub logical_terminal_address: &'a str,
    pub session_number: &'a str,
    pub sequence_number: &'a str,
}

impl<'a> Basic<'a> {
    pub fn new(block_data: &'a str) -> Self {
        Basic {
            application_id : &block_data[..1],
            service_id : &block_data[1..3],
            logical_terminal_address : &block_data[3..15],
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
    destination_address: &'a str,
    priority: Option<&'a str>,
    delivery_monitoring: Option<&'a str>,
    obsolescence_period: Option<&'a str>,
}

impl<'a> Application<'a> {
    pub fn new(block_data: &'a str) -> Self {

        let mut priority = None;
        let mut delivery_monitoring = None;
        let mut obsolescence_period = None;
        eprintln!("block_data.len() = {:#?}", block_data.len());
        
        if block_data.len() >= 16 {
            priority = Some(&block_data[16..17]);
        }
        
        if block_data.len() >= 18 {
            delivery_monitoring = Some(&block_data[17..18]);
        }

        if block_data.len() >= 21 {
            obsolescence_period = Some(&block_data[18..]);
        }
        
        Application {
            input_output_id: &block_data[..1],
            message_type: &block_data[1..4],
            destination_address: &block_data[4..16],
            priority: priority,
            delivery_monitoring: delivery_monitoring,
            obsolescence_period: obsolescence_period,
        }
    }
}

// Allows users to provide their own reference
#[derive(Debug)]
pub struct User<'a> {
    pub data: &'a str,
}

impl<'a> User<'a> {
    pub fn new(block_data: &'a str) -> Self {
        User {
            data: block_data,
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
    pub tag_64: tag::ClosingAvailableBalance<'a>,
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
            tag_64: closing_available_balance.unwrap(),
        }
    }
}

// Indicates special circumstances that relate to message handling or contains security information
#[derive(Debug)]
pub struct Trailer<'a> {
    pub data: &'a str,
}

impl<'a> Trailer<'a> {
    pub fn new(block_data: &'a str) -> Self {
        Trailer {
            data: block_data,
        }
    }
}
