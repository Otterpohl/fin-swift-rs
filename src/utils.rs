use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime};
use eyre::{eyre, Result};
use iso3166_1::alpha2; // country
use iso_currency::Currency;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum SwiftType {
    Mt940,
}

impl TryFrom<&str> for SwiftType {
    type Error = eyre::Error;

    fn try_from(input: &str) -> Result<Self> {
        match input {
            "940" => Ok(Self::Mt940),
            _ => return Err(eyre!(
                "Swift Type is either missing or the value '{input}' is not valid"
            )),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[cfg(not(tarpaulin_include))]
#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum TransactionType {
    BNK,
    BOE,
    BRF,
    CAR,
    CAS,
    CHG,
    CHK,
    CLR,
    CMI,
    CMN,
    CMP,
    CMS,
    CMT,
    CMZ,
    COL,
    COM,
    CPN,
    DCR,
    DDT,
    DIS,
    DIV,
    EQA,
    EXT,
    FEX,
    INT,
    LBX,
    LDP,
    MAR,
    MAT,
    MGT,
    MSC,
    NWI,
    ODC,
    OPT,
    PCH,
    POP,
    PRN,
    REC,
    RED,
    RIG,
    RTI,
    SAL,
    SEC,
    SLE,
    STO,
    STP,
    SUB,
    SWP,
    TAX,
    TCK,
    TCM,
    TRA,
    TRF,
    TRN,
    UWC,
    VDA,
    WAR,
}

impl TryFrom<&str> for TransactionType {
    type Error = eyre::Error;

    #[cfg(not(tarpaulin_include))]
    fn try_from(input: &str) -> Result<Self> {
        match input {
            "BNK" => Ok(Self::BNK),
            "BOE" => Ok(Self::BOE),
            "BRF" => Ok(Self::BRF),
            "CAR" => Ok(Self::CAR),
            "CAS" => Ok(Self::CAS),
            "CHG" => Ok(Self::CHG),
            "CHK" => Ok(Self::CHK),
            "CLR" => Ok(Self::CLR),
            "CMI" => Ok(Self::CMI),
            "CMN" => Ok(Self::CMN),
            "CMP" => Ok(Self::CMP),
            "CMS" => Ok(Self::CMS),
            "CMT" => Ok(Self::CMT),
            "CMZ" => Ok(Self::CMZ),
            "COL" => Ok(Self::COL),
            "COM" => Ok(Self::COM),
            "CPN" => Ok(Self::CPN),
            "DCR" => Ok(Self::DCR),
            "DDT" => Ok(Self::DDT),
            "DIS" => Ok(Self::DIS),
            "DIV" => Ok(Self::DIV),
            "EQA" => Ok(Self::EQA),
            "EXT" => Ok(Self::EXT),
            "FEX" => Ok(Self::FEX),
            "INT" => Ok(Self::INT),
            "LBX" => Ok(Self::LBX),
            "LDP" => Ok(Self::LDP),
            "MAR" => Ok(Self::MAR),
            "MAT" => Ok(Self::MAT),
            "MGT" => Ok(Self::MGT),
            "MSC" => Ok(Self::MSC),
            "NWI" => Ok(Self::NWI),
            "ODC" => Ok(Self::ODC),
            "OPT" => Ok(Self::OPT),
            "PCH" => Ok(Self::PCH),
            "POP" => Ok(Self::POP),
            "PRN" => Ok(Self::PRN),
            "REC" => Ok(Self::REC),
            "RED" => Ok(Self::RED),
            "RIG" => Ok(Self::RIG),
            "RTI" => Ok(Self::RTI),
            "SAL" => Ok(Self::SAL),
            "SEC" => Ok(Self::SEC),
            "SLE" => Ok(Self::SLE),
            "STO" => Ok(Self::STO),
            "STP" => Ok(Self::STP),
            "SUB" => Ok(Self::SUB),
            "SWP" => Ok(Self::SWP),
            "TAX" => Ok(Self::TAX),
            "TCK" => Ok(Self::TCK),
            "TCM" => Ok(Self::TCM),
            "TRA" => Ok(Self::TRA),
            "TRF" => Ok(Self::TRF),
            "TRN" => Ok(Self::TRN),
            "UWC" => Ok(Self::UWC),
            "VDA" => Ok(Self::VDA),
            "WAR" => Ok(Self::WAR),
            _ => return Err(eyre!(
                "Transaction Type is either missing or the value '{input}' is not valid"
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum IO {
    Input,
    Output,
}

impl TryFrom<&str> for IO {
    type Error = eyre::Error;

    #[cfg(not(tarpaulin_include))]
    fn try_from(input: &str) -> Result<Self> {
        match input {
            "I" => Ok(Self::Input),
            "O" => Ok(Self::Output),
            _ => return Err(eyre!(
                "IO is either missing or the value '{input}' is not valid"
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum ApplicationId {
    F,
    A,
    L,
}

impl TryFrom<&str> for ApplicationId {
    type Error = eyre::Error;

    #[cfg(not(tarpaulin_include))]
    fn try_from(input: &str) -> Result<Self> {
        match input {
            "F" => Ok(Self::F),
            "A" => Ok(Self::A),
            "L" => Ok(Self::L),
            _ => return Err(eyre!(
                "Application Id is either missing or the value '{input}' is not valid"
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum ServiceId {
    FinGpa,
    AckNak,
}

impl TryFrom<&str> for ServiceId {
    type Error = eyre::Error;

    #[cfg(not(tarpaulin_include))]
    fn try_from(input: &str) -> Result<Self> {
        match input {
            "21" => Ok(Self::FinGpa),
            "01" => Ok(Self::AckNak),
            _ => return Err(eyre!(
                "Service Id is either missing or the value '{input}' is not valid"
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum CreditDebit {
    Credit,
    Debit,
    CreditReversal,
    DebitReversal,
}

impl CreditDebit {
    pub fn value(&self) -> String {
        match self {
            Self::Credit => "C".to_string(),
            Self::Debit => "D".to_string(),
            Self::CreditReversal => "CR".to_string(),
            Self::DebitReversal => "DR".to_string(),
        }
    }
}

impl TryFrom<&str> for CreditDebit {
    type Error = eyre::Error;

    #[cfg(not(tarpaulin_include))]
    fn try_from(input: &str) -> Result<Self> {
        match input {
            "CR" | "RC" => Ok(Self::CreditReversal),
            "DR" | "RD" => Ok(Self::DebitReversal),
            "C" => Ok(Self::Credit),
            "D" => Ok(Self::Debit),
            _ => return Err(eyre!(
                "Credit Debit is either missing or the value '{input}' is not valid"
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum BalanceType {
    Final,
    Intermediary,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum FundsCode {
    SwiftTransfer,
    NonSwiftTransfer,
    FirstAdvice,
}

impl TryFrom<&str> for FundsCode {
    type Error = eyre::Error;

    #[cfg(not(tarpaulin_include))]
    fn try_from(input: &str) -> Result<Self> {
        match input {
            "S" => Ok(Self::SwiftTransfer),
            "N" => Ok(Self::NonSwiftTransfer),
            "F" => Ok(Self::FirstAdvice),
            _ => return Err(eyre!(
                "Funds Code is either missing or the value '{input}' is not valid"
            )),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum ValidationFlag {
    REMIT,
    RFDD,
    STP,
}

impl TryFrom<&str> for ValidationFlag {
    type Error = eyre::Error;

    #[cfg(not(tarpaulin_include))]
    fn try_from(input: &str) -> Result<Self> {
        match input {
            "REMIT" => Ok(Self::REMIT),
            "RFDD" => Ok(Self::RFDD),
            "STP" => Ok(Self::STP),
            _ => return Err(eyre!(
                "Validation Flag value is either missing or the value '{input}' is not valid"
            )),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum SanctionScreenType {
    AOK,
    FPO,
    NOK,
}

impl TryFrom<&str> for SanctionScreenType {
    type Error = eyre::Error;

    #[cfg(not(tarpaulin_include))]
    fn try_from(input: &str) -> Result<Self> {
        match input {
            "AOK" => Ok(Self::AOK),
            "FPO" => Ok(Self::FPO),
            "NOK" => Ok(Self::NOK),
            _ => return Err(eyre!(
                "Sanction Screen Type is either missing or the value '{input}' is not valid"
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct BusinessIdentifierCode<'a> {
    pub business_party_prefix: &'a str,
    pub country_code: &'a str,
    pub business_party_suffix: &'a str,
}

impl<'a> BusinessIdentifierCode<'a> {
    fn new(input: &'a str) -> Result<Self> {
        let business_party_prefix = &input[..4];
        let country_code = alpha2(&input[4..6])
            .ok_or_else(|| {
                eyre!(
                    "Country code is either missing or the value '{}' is not valid",
                    &input[4..6]
                )
            })?
            .alpha2;
        let business_party_suffix = &input[6..];

        Ok(Self {
            business_party_prefix,
            country_code,
            business_party_suffix,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct LogicalTerminalAddress<'a> {
    pub bic_code: BusinessIdentifierCode<'a>,
    pub terminal_code: &'a str, // try to make this a char?
    pub branch_code: &'a str,
}

impl<'a> LogicalTerminalAddress<'a> {
    pub fn new(input: &'a str) -> Result<Self> {
        let bic_code = BusinessIdentifierCode::new(&input[..8])?;

        Ok(Self {
            bic_code,
            terminal_code: &input[8..9],
            branch_code: &input[9..],
        })
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Balance {
    pub credit_or_debit: CreditDebit,
    pub date: NaiveDate,
    pub currency: Currency,
    pub amount: f64,
}

impl Balance {
    pub fn new(input: &str) -> Result<Self> {
        let credit_or_debit = CreditDebit::try_from(&input[..1])?;
        let date = naive_date_from_swift_date(&input[1..7])?;
        let currency = Currency::from_code(&input[7..10]).ok_or_else(|| {
            eyre!(
                "currency code is either missing or the value '{}' is not valid",
                &input[7..10]
            )
        })?;
        let amount = float_from_swift_amount(&input[10..])?;

        Ok(Self {
            credit_or_debit,
            date,
            currency,
            amount,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct MessageInputReference<'a> {
    pub date: NaiveDate,
    pub lt_identifier: &'a str,
    pub branch_code: &'a str,
    pub session_number: i16,
    pub sequence_number: i16,
}

impl<'a> MessageInputReference<'a> {
    pub fn new(input: &'a str) -> Result<Self> {
        let date = naive_date_from_swift_date(&input[..6])?;
        let lt_identifier = &input[6..18];
        let branch_code = &input[18..21];
        let session_number = input[21..25].parse::<i16>()?;
        let sequence_number = input[25..].parse::<i16>()?;

        Ok(Self {
            date,
            lt_identifier,
            branch_code,
            session_number,
            sequence_number,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct AddressInformation<'a> {
    pub time_of_crediting: NaiveTime,
    pub time_of_debiting: NaiveTime,
    pub country_code: &'a str,
    pub internal_posting_reference: &'a str,
}

impl<'a> AddressInformation<'a> {
    pub fn new(input: &'a str) -> Result<Self> {
        let segments: Vec<&str> = input.trim().split(' ').collect();

        let time_of_crediting = naive_time_from_swift_time(segments[0])?;
        let time_of_debiting = naive_time_from_swift_time(segments[1])?;

        let country_code = alpha2(segments[2])
            .ok_or_else(|| {
                eyre!(
                    "Country code is either missing or the value '{}' is not valid",
                    &input[4..6]
                )
            })?
            .alpha2;
        let internal_posting_reference = segments[3];

        Ok(Self {
            time_of_crediting,
            time_of_debiting,
            country_code,
            internal_posting_reference,
        })
    }
}

pub fn naive_time_from_swift_time(time: &str) -> Result<chrono::NaiveTime> {
    Ok(chrono::NaiveTime::from_hms(
        time[..2].parse::<u32>()?,
        time[2..4].parse::<u32>()?,
        time[4..].parse::<u32>()?,
    ))
}

pub fn naive_date_from_swift_date(date: &str) -> Result<NaiveDate> {
    if date.len() == 4 {
        Ok(NaiveDate::from_ymd(
            chrono::Utc::now().year(),
            date[..2].parse::<u32>()?,
            date[2..].parse::<u32>()?,
        ))
    } else if date.len() == 6 {
        return Ok(NaiveDate::from_ymd(
            2000 + date[..2].parse::<i32>()?,
            date[2..4].parse::<u32>()?,
            date[4..6].parse::<u32>()?,
        ))
    } else if date.len() == 8 {
        return Ok(NaiveDate::from_ymd(
            date[..4].parse::<i32>()?,
            date[4..6].parse::<u32>()?,
            date[6..8].parse::<u32>()?,
        ))
    } else {
        return Err(eyre!("Invalid swift date provided"))
    }
}

pub fn naive_date_time_from_swift_date_time(date_time: &str) -> Result<NaiveDateTime> {
    Ok(NaiveDateTime::new(
        naive_date_from_swift_date(&date_time[..6])?,
        NaiveTime::from_hms_milli(
            date_time[6..8].parse::<u32>()?,
            date_time[8..10].parse::<u32>()?,
            date_time[10..12].parse::<u32>()?,
            date_time[12..].parse::<u32>()?,
        ),
    ))
}

pub fn float_from_swift_amount(amount: &str) -> Result<f64> {
    Ok(amount.replace(',', ".").parse::<f64>()?)
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike};

    use super::*;

    #[test]
    fn test_amount_with_scale() -> Result<()> {
        assert_eq!(float_from_swift_amount("379,29")?, 379.29);
        Ok(())
    }

    #[test]
    fn test_amount_without_scale() -> Result<()> {
        assert_eq!(float_from_swift_amount("379,")?, 379.0);
        Ok(())
    }

    #[test]
    fn test_amount_without_comma() -> Result<()> {
        assert_eq!(float_from_swift_amount("379.")?, 379.0);
        Ok(())
    }

    #[test]
    fn test_date_long_year() -> Result<()> {
        let date = naive_date_from_swift_date("20090924")?;

        assert_eq!(date.year(), 2009);
        assert_eq!(date.month(), 9);
        assert_eq!(date.day(), 24);
        Ok(())
    }

    #[test]
    fn test_date_short_year() -> Result<()> {
        let date = naive_date_from_swift_date("090924")?;

        assert_eq!(date.year(), 2009);
        assert_eq!(date.month(), 9);
        assert_eq!(date.day(), 24);
        Ok(())
    }

    #[test]
    fn test_date_no_year() -> Result<()> {
        let date = naive_date_from_swift_date("0924").unwrap();

        assert_eq!(date.year(), chrono::Utc::now().year());
        assert_eq!(date.month(), 9);
        assert_eq!(date.day(), 24);
        Ok(())
    }

    #[test]
    fn test_time() -> Result<()> {
        let time = naive_time_from_swift_time("121413").unwrap();

        assert_eq!(time.hour(), 12);
        assert_eq!(time.minute(), 14);
        assert_eq!(time.second(), 13);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Invalid swift date provided")]
    fn test_date_bad_data() {
        naive_date_from_swift_date("").unwrap();
    }

    #[test]
    fn test_datetime() -> Result<()> {
        let datetime = naive_date_time_from_swift_date_time("18071715301204").unwrap();
        assert_eq!(datetime.year(), 2018);
        assert_eq!(datetime.month(), 7);
        assert_eq!(datetime.day(), 17);
        assert_eq!(datetime.hour(), 15);
        assert_eq!(datetime.minute(), 30);
        assert_eq!(datetime.second(), 12);
        Ok(())
    }

    #[test]
    fn test_business_identifier_code() -> Result<()> {
        let bic_code = BusinessIdentifierCode::new("ASNBNL21")?;

        assert_eq!(bic_code.business_party_prefix, "ASNB");
        assert_eq!(bic_code.country_code, "NL");
        assert_eq!(bic_code.business_party_suffix, "21");
        Ok(())
    }

    #[test]
    fn test_logical_terminal_address() -> Result<()> {
        let lta = LogicalTerminalAddress::new("ASNBNL21XXXX")?;

        assert_eq!(lta.bic_code, BusinessIdentifierCode::new("ASNBNL21")?);
        assert_eq!(lta.terminal_code, "X");
        assert_eq!(lta.branch_code, "XXX");
        Ok(())
    }

    #[test]
    fn test_credit_or_debit() -> Result<()> {
        assert_eq!(CreditDebit::try_from("C")?, CreditDebit::Credit);
        assert_eq!(CreditDebit::try_from("D")?, CreditDebit::Debit);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Credit Debit is either missing or the value 'A' is not valid")]
    fn test_credit_or_debit_bad_data() {
        CreditDebit::try_from("A").unwrap();
    }

    #[test]
    fn test_currency_code() -> Result<()> {
        assert_eq!(Currency::from_code("EUR").unwrap(), Currency::EUR);
        Ok(())
    }

    #[test]
    fn test_funds_code() -> Result<()> {
        assert_eq!(FundsCode::try_from("S")?, FundsCode::SwiftTransfer);
        assert_eq!(FundsCode::try_from("N")?, FundsCode::NonSwiftTransfer);
        assert_eq!(FundsCode::try_from("F")?, FundsCode::FirstAdvice);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Funds Code is either missing or the value 'T' is not valid")]
    fn test_funds_code_bad_data() {
        FundsCode::try_from("T").unwrap();
    }

    #[test]
    fn test_balance() -> Result<()> {
        let balance = Balance::new("C090930EUR53189,31")?;

        assert_eq!(balance.credit_or_debit, CreditDebit::Credit);
        assert_eq!(balance.date, NaiveDate::from_ymd(2009, 9, 30));
        assert_eq!(balance.currency, Currency::EUR);
        assert_eq!(balance.amount, 53189.31);
        Ok(())
    }

    #[test]
    fn test_message_input_reference() -> Result<()> {
        let mir = MessageInputReference::new("120811BANKBEBBAXXX2222123456")?;
        assert_eq!(mir.date.year(), 2012);
        assert_eq!(mir.date.month(), 8);
        assert_eq!(mir.date.day(), 11);
        assert_eq!(mir.lt_identifier, "BANKBEBBAXXX");
        assert_eq!(mir.branch_code, "222");
        assert_eq!(mir.session_number, 2123);
        assert_eq!(mir.sequence_number, 456);

        Ok(())
    }

    #[test]
    fn test_address_information() -> Result<()> {
        let ai = AddressInformation::new(" 121413 121413 DE BANKDECDA123")?;

        assert_eq!(ai.time_of_crediting.hour(), 12);
        assert_eq!(ai.time_of_crediting.minute(), 14);
        assert_eq!(ai.time_of_crediting.second(), 13);
        assert_eq!(ai.time_of_debiting.hour(), 12);
        assert_eq!(ai.time_of_debiting.minute(), 14);
        assert_eq!(ai.time_of_debiting.second(), 13);
        assert_eq!(ai.country_code, "DE");
        assert_eq!(ai.internal_posting_reference, "BANKDECDA123");
        Ok(())
    }
}
