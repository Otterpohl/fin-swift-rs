use crate::tag;
use crate::utils::*;
use iso3166_1::*;
use regex::Regex;

// TODO: add enum for block data

#[derive(Debug)]
pub struct BusinessIdentifierCode<'a> {
    business_party_prefix: &'a str,
    country_code: &'a str,
    business_party_suffix: &'a str,
}

impl<'a> BusinessIdentifierCode<'a> {
    fn new(data: &'a str) -> Self {
        let business_party_prefix = &data[0..4];
        let country_code = alpha2(&data[4..6]).unwrap().alpha2;
        let business_party_suffix = &data[6..];

        Self {
            business_party_prefix,
            country_code,
            business_party_suffix,
        }
    }
}

#[derive(Debug)]
pub struct LogicalTerminalAddress<'a> {
    bic_code: BusinessIdentifierCode<'a>,
    terminal_code: &'a str, // try to make this a char?
    branch_code: &'a str,
}

impl<'a> LogicalTerminalAddress<'a> {
    fn new(data: &'a str) -> Self {
        let bic_code = BusinessIdentifierCode::new(&data[..8]);

        Self {
            bic_code,
            terminal_code: &data[8..9],
            branch_code: &data[9..],
        }
    }
}

// https://www.paiementor.com/swift-mt-message-block-1-basic-header-description
// https://www2.swift.com/knowledgecentre/publications/us9m_20180720/?topic=ajc.htm#genajc
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
            n @ "F" | n @ "A" | n @ "L" => n,
            n => {
                panic!("unexpected application_id `{}` in Basic block", n)
            }
        };

        let service_id = match &block_data[1..3] {
            n @ "01" | n @ "21" => n,
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

// Contains the text of the message
#[derive(Debug)]
pub struct Text<'a> {
    pub tag_20: tag::TransactionReferenceNumber<'a>,
    pub tag_25: tag::AccountIdentification<'a>,
    pub tag_28c: tag::StatementNumber,
    pub tag_60f: tag::OpeningBalance,
    pub tag_61: Vec<tag::StatementLine<'a>>,
    pub tag_62f: tag::BookedFunds,
    pub tag_64: Option<tag::ClosingAvailableBalance>,
    pub tag_86: Vec<tag::InformationToAccountOwner<'a>>,
}

impl<'a> Text<'a> {
    pub fn new(block_data: &'a str) -> Self {
        let mut transaction_reference_number = None;
        let mut tag_account_identification = None;
        let mut statement_number = None;
        let mut opening_balance = None;
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
                    transaction_reference_number =
                        Some(tag::TransactionReferenceNumber::new(value));
                }
                "25" => {
                    tag_account_identification = Some(tag::AccountIdentification::new(value));
                }
                "28C" => {
                    statement_number = Some(tag::StatementNumber::new(value));
                }
                "60F" => {
                    opening_balance = Some(tag::OpeningBalance::new(BalanceType::Final, value));
                }
                "60M" => {
                    opening_balance =
                        Some(tag::OpeningBalance::new(BalanceType::Intermediary, value));
                }
                "62F" => {
                    booked_funds_final = Some(tag::BookedFunds::new(BalanceType::Final, value));
                }
                "62M" => {
                    booked_funds_final =
                        Some(tag::BookedFunds::new(BalanceType::Intermediary, value));
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
