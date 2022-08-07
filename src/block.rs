use crate::tag::{
    AccountIdentification, BankingPriority, BookedFunds, ClosingAvailableBalance,
    InformationToAccountOwner, MessageUserReference, OpeningBalance, PaymentControlsInformation,
    PaymentReleaseInformationReceiver, RelatedReference, SanctionsScreeningInformation,
    ServiceIdentifier, ServiceTypeIdentifier, StatementLine, StatementNumber,
    TransactionReferenceNumber, Validation,
};
use crate::utils::{
    naive_date_time_from_swift_date_time, AddressInformation, BalanceType, LogicalTerminalAddress,
    MessageInputReference,
};
use anyhow::{anyhow, Ok, Result};
use chrono::NaiveDateTime;
use eyre::{eyre, Result};
use regex::Regex;
use serde::Serialize;
use uuid::Uuid;

// https://www.paiementor.com/swift-mt-message-block-1-basic-header-description
// https://www2.swift.com/knowledgecentre/publications/us9m_20180720/?topic=ajc.htm#genajc

// Block 1
// Fundamental reference for any particular message
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Basic<'a> {
    pub application_id: &'a str,
    pub service_id: &'a str,
    pub source_address: LogicalTerminalAddress<'a>,
    pub session_number: &'a str,
    pub sequence_number: &'a str,
}

impl<'a> Basic<'a> {
    pub fn new(block_data: &'a str) -> Result<Self> {
        let application_id = match &block_data[..1] {
            n @ ("F" | "A" | "L") => n,
            n => {
                panic!("unexpected application_id `{n}` in Basic block")
            }
        };

        let service_id = match &block_data[1..3] {
            n @ ("01" | "21") => n,
            n => {
                panic!("unexpected service_id `{n}` in Basic block")
            }
        };

        let source_address = LogicalTerminalAddress::new(&block_data[3..15])?;

        Ok(Self {
            application_id,
            service_id,
            source_address,
            session_number: &block_data[15..19], // TODO: try parse these as numbers
            sequence_number: &block_data[19..],  // TODO: try parse these as numbers
        })
    }
}

// Block 2
// Information about the message itself
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Application<'a> {
    pub input_output_id: &'a str,
    pub message_type: &'a str,
    pub destination_address: LogicalTerminalAddress<'a>,
    pub priority: Option<&'a str>,
    pub delivery_monitoring: Option<&'a str>,
    pub obsolescence_period: Option<&'a str>,
}

impl<'a> Application<'a> {
    pub fn new(block_data: &'a str) -> Result<Self> {
        let input_output_id = match &block_data[..1] {
            n @ ("I" | "O") => {
                // struct this?
                n
            }
            n => {
                panic!("unexpected input_output_id `{n}` in Application block")
            }
        };

        let message_type = match &block_data[1..4] {
            n @ "940" => n,
            n => {
                panic!("unexpected message_type `{n}`")
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

        let destination_address = LogicalTerminalAddress::new(&block_data[4..16])?;

        Ok(Self {
            input_output_id,
            message_type,
            destination_address,
            priority,
            delivery_monitoring,
            obsolescence_period,
        })
    }
}

// Block 3
// Allows users to provide their own reference
// https://www.paiementor.com/swift-mt-message-block-3-user-header-description/
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct User<'a> {
    pub tag_103: Option<ServiceIdentifier<'a>>,
    pub tag_113: Option<BankingPriority<'a>>,
    pub tag_108: Option<MessageUserReference<'a>>,
    pub tag_119: Option<Validation>,
    pub tag_423: Option<NaiveDateTime>,
    pub tag_106: Option<MessageInputReference<'a>>,
    pub tag_424: Option<RelatedReference<'a>>,
    pub tag_111: Option<ServiceTypeIdentifier<'a>>,
    pub tag_121: Option<Uuid>,
    pub tag_115: Option<AddressInformation<'a>>,
    pub tag_165: Option<PaymentReleaseInformationReceiver<'a>>,
    pub tag_433: Option<SanctionsScreeningInformation<'a>>,
    pub tag_434: Option<PaymentControlsInformation<'a>>,
}

impl<'a> User<'a> {
    pub fn new(block_data: &'a str) -> Result<Self> {
        let mut service_identifier = None;
        let mut banking_priority = None;
        let mut message_user_reference = None;
        let mut validation = None;
        let mut balance_checkpoint_date = None;
        let mut message_input_reference = None;
        let mut related_reference = None;
        let mut service_type_identifier = None;
        let mut unique_transaction_reference = None;
        let mut address_information = None;
        let mut payment_release_information_receiver = None;
        let mut sanctions_screening_information = None;
        let mut payment_controls_information = None;

        let block_start: Vec<usize> = block_data.match_indices('{').map(|(i, _)| i + 1).collect();
        let block_end: Vec<usize> = block_data.match_indices('}').map(|(i, _)| i).collect();
        let block_segments = block_start.iter().zip(block_end.iter());

        for (start, end) in block_segments {
            let section = &block_data[*start..*end];
            let index = section
                .chars()
                .position(|c| c == ':')
                .ok_or_else(|| eyre!("missing ':' in block"))?;
            let tag = &section[..index];
            let value = &section[index + 1..];

            match tag {
                "103" => {
                    service_identifier = Some(ServiceIdentifier::new(value));
                }
                "113" => {
                    banking_priority = Some(BankingPriority::new(value));
                }
                "108" => {
                    message_user_reference = Some(MessageUserReference::new(value));
                }
                "119" => {
                    validation = Some(Validation::new(value)?);
                }
                "423" => {
                    balance_checkpoint_date = Some(naive_date_time_from_swift_date_time(value)?);
                }
                "106" => {
                    message_input_reference = Some(MessageInputReference::new(value)?);
                }
                "424" => {
                    related_reference = Some(RelatedReference::new(value));
                }
                "111" => {
                    service_type_identifier = Some(ServiceTypeIdentifier::new(value));
                }
                "121" => {
                    unique_transaction_reference = Some(Uuid::parse_str(value)?);
                }
                "115" => {
                    address_information = Some(AddressInformation::new(value)?);
                }
                "165" => {
                    payment_release_information_receiver =
                        Some(PaymentReleaseInformationReceiver::new(value));
                }
                "433" => {
                    sanctions_screening_information =
                        Some(SanctionsScreeningInformation::new(value)?);
                }
                "434" => {
                    payment_controls_information = Some(PaymentControlsInformation::new(value));
                }
                _ => {
                    Err(eyre!("unexpected tag `{tag}` in User block"))?;
                }
            }
        }

        Ok(Self {
            tag_103: service_identifier,
            tag_113: banking_priority,
            tag_108: message_user_reference,
            tag_119: validation,
            tag_423: balance_checkpoint_date,
            tag_106: message_input_reference,
            tag_424: related_reference,
            tag_111: service_type_identifier,
            tag_121: unique_transaction_reference,
            tag_115: address_information,
            tag_165: payment_release_information_receiver,
            tag_433: sanctions_screening_information,
            tag_434: payment_controls_information,
        })
    }
}

// Block 4
// Contains the text of the message
#[derive(Debug, PartialEq, Serialize)]
pub struct Text<'a> {
    pub tag_20: TransactionReferenceNumber<'a>,
    pub tag_25: AccountIdentification<'a>,
    pub tag_28c: StatementNumber,
    pub tag_60: OpeningBalance,
    pub tag_61: Vec<StatementLine<'a>>,
    pub tag_62: BookedFunds,
    pub tag_64: Option<ClosingAvailableBalance>,
    pub tag_86: Vec<InformationToAccountOwner<'a>>,
}

impl<'a> Text<'a> {
    pub fn new(block_data: &'a str) -> Result<Self> {
        let mut txn_ref_num = None;
        let mut account_id = None;
        let mut statement_num = None;
        let mut opening_balance = None;
        let mut booked_funds = None;
        let mut statement_line: Vec<StatementLine> = vec![];
        let mut information_to_account_owner: Vec<InformationToAccountOwner> = vec![];
        let mut closing_available_balance = None;

        let tag_regex = Regex::new(r"(?m)(?:(\d{2}|\d{2}[A-Z]):.+)")?;

        for tag in tag_regex.captures_iter(block_data) {
            let block_key = tag
                .get(1)
                .ok_or_else(|| eyre!("block does not contain a key"))?
                .as_str();
            let block_data = tag
                .get(0)
                .ok_or_else(|| eyre!("block does not contain a value"))?
                .as_str();
            let value = block_data[block_key.len()..block_data.len()]
                .trim_matches(|c| c == ':' || c == '\r');

            match block_key {
                "20" => {
                    txn_ref_num = Some(TransactionReferenceNumber::new(value));
                }
                "25" => {
                    account_id = Some(AccountIdentification::new(value));
                }
                "28C" => {
                    statement_num = Some(StatementNumber::new(value)?);
                }
                "60F" => {
                    opening_balance = Some(OpeningBalance::new(BalanceType::Final, value)?);
                }
                "60M" => {
                    opening_balance = Some(OpeningBalance::new(BalanceType::Intermediary, value)?);
                }
                "62F" => {
                    booked_funds = Some(BookedFunds::new(BalanceType::Final, value)?);
                }
                "62M" => {
                    booked_funds = Some(BookedFunds::new(BalanceType::Intermediary, value)?);
                }
                "61" => {
                    statement_line.push(StatementLine::new(value)?);
                }
                "86" => {
                    information_to_account_owner.push(InformationToAccountOwner::new(value));
                }
                "64" => {
                    closing_available_balance = Some(ClosingAvailableBalance::new(value)?);
                }
                _ => {
                    Err(eyre!("unexpected block key `{block_key}` in Basic block"))?;
                }
            };
        }

        let txn_ref_num =
            txn_ref_num.ok_or_else(|| eyre!("missing transaction reference number (tag 20)"))?;
        let account_id =
            account_id.ok_or_else(|| eyre!("missing account identification (tag 25"))?;
        let statement_num =
            statement_num.ok_or_else(|| eyre!("missing statement number (tag 28C"))?;
        let opening_balance =
            opening_balance.ok_or_else(|| eyre!("missing opening balance (tag 60"))?;
        let booked_funds = booked_funds.ok_or_else(|| eyre!("missing booked funds (tag 62"))?;

        Ok(Self {
            tag_20: txn_ref_num,
            tag_25: account_id,
            tag_28c: statement_num,
            tag_60: opening_balance,
            tag_61: statement_line,
            tag_62: booked_funds,
            tag_64: closing_available_balance,
            tag_86: information_to_account_owner,
        })
    }
}

// Block 5
// Indicates special circumstances that relate to message handling or contains security information
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Trailer<'a> {
    pub data: Option<&'a str>,
}

impl<'a> Trailer<'a> {
    pub fn new(block_data: &'a str) -> Self {
        let mut data = None;

        if !block_data.is_empty() {
            data = Some(block_data);
        }

        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tag::*;
    use crate::utils::*;

    #[test]
    fn test_block_basic() {
        let block_data = "F01ASNBNL21XXXX0000000000";
        let data = Basic::new(block_data).unwrap();
        let source_address = LogicalTerminalAddress::new(&block_data[3..15]).unwrap();

        assert_eq!(data.application_id, ApplicationId::F);
        assert_eq!(data.service_id, ServiceId::AckNak);
        assert_eq!(data.source_address, source_address);
        assert_eq!(data.session_number, 0);
        assert_eq!(data.sequence_number, 0);
    }

    #[test]
    #[should_panic(expected = "Application Id is either missing or the value 'T' is not valid")]
    fn test_block_basic_application_id() {
        Basic::new("T01ASNBNL21XXXX0000000000").unwrap();
    }

    #[test]
    #[should_panic(expected = "Service Id is either missing or the value '02' is not valid")]
    fn test_block_service_id() {
        Basic::new("F02ASNBNL21XXXX0000000000").unwrap();
    }

    #[test]
    fn test_block_application() {
        let block_data = "O940ASNBNL21XXXXN3123";
        let data = Application::new(block_data).unwrap();
        let destination_address = LogicalTerminalAddress::new(&block_data[4..16]).unwrap();

        assert_eq!(data.input_output_id, IO::Output);
        assert_eq!(data.message_type, SwiftType::Mt940);
        assert_eq!(data.destination_address, destination_address);
        assert_eq!(data.priority, Some("N"));
        assert_eq!(data.delivery_monitoring, Some("3"));
        assert_eq!(data.obsolescence_period, Some("123"));
    }

    #[test]
    #[should_panic(expected = "IO is either missing or the value 'B' is not valid")]
    fn test_block_application_input_output_id() {
        Application::new("B940ASNBNL21XXXXN").unwrap();
    }

    #[test]
    #[should_panic(expected = "Swift Type is either missing or the value '537' is not valid")]
    fn test_block_application_message_type() {
        Application::new("O537ASNBNL21XXXXN").unwrap();
    }

    #[test]
    fn test_block_user() {
        let data = User::new("3:{103:CAD}{113:xxxx}{119:STP}{108:2RDRQDHM3WO}{423:18071715301204}{111:DER}{106:120811BANKBEBBAXXX2222123456}{424:PQAB1234}{121:180f1e65-90e0-44d5-a49a-92b55eb3025f}{165:DERASDFQWERTY}{115: 121413 121413 DE BANKDECDA123}{433:/AOK}{434:/FPO}").unwrap();

        assert_eq!(data.tag_103.unwrap().service_identifier, "CAD");
        assert_eq!(data.tag_113.unwrap().banking_priority, "xxxx");
        assert_eq!(data.tag_108.unwrap().message_user_reference, "2RDRQDHM3WO");
        assert_eq!(data.tag_119.unwrap().validation_flag, ValidationFlag::STP);
        assert_eq!(
            data.tag_423.unwrap(),
            naive_date_time_from_swift_date_time("18071715301204").unwrap()
        );
        assert_eq!(
            data.tag_106.unwrap().date,
            naive_date_from_swift_date("120811").unwrap()
        );
        assert_eq!(data.tag_106.unwrap().lt_identifier, "BANKBEBBAXXX");
        assert_eq!(data.tag_106.unwrap().branch_code, "222");
        assert_eq!(data.tag_106.unwrap().session_number, 2123);
        assert_eq!(data.tag_106.unwrap().sequence_number, 456);
        assert_eq!(data.tag_424.unwrap().related_reference, "PQAB1234");
        assert_eq!(data.tag_111.unwrap().service_type_identifier, "DER");
        assert_eq!(
            data.tag_121.unwrap().to_string(),
            "180f1e65-90e0-44d5-a49a-92b55eb3025f"
        );
        assert_eq!(
            data.tag_115.unwrap().time_of_crediting,
            naive_time_from_swift_time("121413").unwrap()
        );
        assert_eq!(
            data.tag_115.unwrap().time_of_debiting,
            naive_time_from_swift_time("121413").unwrap()
        );
        assert_eq!(data.tag_115.unwrap().country_code, "DE");
        assert_eq!(
            data.tag_115.unwrap().internal_posting_reference,
            "BANKDECDA123"
        );
        assert_eq!(
            data.tag_165.unwrap().payment_release_information_receiver,
            "DERASDFQWERTY"
        );
        assert_eq!(data.tag_433.unwrap().codeword, SanctionScreenType::AOK);
        assert_eq!(data.tag_433.unwrap().additional_information, "");
        assert_eq!(data.tag_434.unwrap().codeword, "FPO");
        assert_eq!(data.tag_434.unwrap().additional_information, "");
    }

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
        )
        .unwrap();

        let tag_20 = TransactionReferenceNumber::new("3996-11-11111111");
        let tag_25 = AccountIdentification::new("DABADKKK/111111-11111111");
        let tag_28c = StatementNumber::new("00001/001").unwrap();
        let tag_64 = ClosingAvailableBalance::new("C090930EUR53189,31").unwrap();

        let mut tag_61: Vec<StatementLine> = vec![];
        let mut tag_86: Vec<InformationToAccountOwner> = vec![];
        tag_61.push(StatementLine::new("0909250925DR583,92NMSC1110030403010139//1234").unwrap());
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
        )
        .unwrap();

        let tag_60 = OpeningBalance::new(BalanceType::Final, "C090924EUR54484,04").unwrap();
        let tag_62 = BookedFunds::new(BalanceType::Final, "C090930EUR53126,94").unwrap();

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
        )
        .unwrap();

        let tag_60 = OpeningBalance::new(BalanceType::Intermediary, "C090924EUR54484,04").unwrap();
        let tag_62 = BookedFunds::new(BalanceType::Intermediary, "C090930EUR53126,94").unwrap();

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
        )
        .unwrap();
    }

    #[test]
    fn test_block_trailer() {}
}
