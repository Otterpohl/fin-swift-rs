use chrono::prelude::*;
use chrono::NaiveDate;
use eyre::{eyre, Result};
use iso3166_1::alpha2; // country
use iso_currency::Currency;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub enum SwiftType {
    Mt940,
}

impl TryFrom<&str> for SwiftType {
    type Error = eyre::Error;

    fn try_from(input: &str) -> Result<Self> {
        match input {
            "940" => Ok(SwiftType::Mt940),
            _ => Err(eyre!(
                "Swift Type is either missing or the value '{input}' is not valid"
            )),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
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

    fn try_from(input: &str) -> Result<Self> {
        match input {
            "BNK" => Ok(TransactionType::BNK),
            "BOE" => Ok(TransactionType::BOE),
            "BRF" => Ok(TransactionType::BRF),
            "CAR" => Ok(TransactionType::CAR),
            "CAS" => Ok(TransactionType::CAS),
            "CHG" => Ok(TransactionType::CHG),
            "CHK" => Ok(TransactionType::CHK),
            "CLR" => Ok(TransactionType::CLR),
            "CMI" => Ok(TransactionType::CMI),
            "CMN" => Ok(TransactionType::CMN),
            "CMP" => Ok(TransactionType::CMP),
            "CMS" => Ok(TransactionType::CMS),
            "CMT" => Ok(TransactionType::CMT),
            "CMZ" => Ok(TransactionType::CMZ),
            "COL" => Ok(TransactionType::COL),
            "COM" => Ok(TransactionType::COM),
            "CPN" => Ok(TransactionType::CPN),
            "DCR" => Ok(TransactionType::DCR),
            "DDT" => Ok(TransactionType::DDT),
            "DIS" => Ok(TransactionType::DIS),
            "DIV" => Ok(TransactionType::DIV),
            "EQA" => Ok(TransactionType::EQA),
            "EXT" => Ok(TransactionType::EXT),
            "FEX" => Ok(TransactionType::FEX),
            "INT" => Ok(TransactionType::INT),
            "LBX" => Ok(TransactionType::LBX),
            "LDP" => Ok(TransactionType::LDP),
            "MAR" => Ok(TransactionType::MAR),
            "MAT" => Ok(TransactionType::MAT),
            "MGT" => Ok(TransactionType::MGT),
            "MSC" => Ok(TransactionType::MSC),
            "NWI" => Ok(TransactionType::NWI),
            "ODC" => Ok(TransactionType::ODC),
            "OPT" => Ok(TransactionType::OPT),
            "PCH" => Ok(TransactionType::PCH),
            "POP" => Ok(TransactionType::POP),
            "PRN" => Ok(TransactionType::PRN),
            "REC" => Ok(TransactionType::REC),
            "RED" => Ok(TransactionType::RED),
            "RIG" => Ok(TransactionType::RIG),
            "RTI" => Ok(TransactionType::RTI),
            "SAL" => Ok(TransactionType::SAL),
            "SEC" => Ok(TransactionType::SEC),
            "SLE" => Ok(TransactionType::SLE),
            "STO" => Ok(TransactionType::STO),
            "STP" => Ok(TransactionType::STP),
            "SUB" => Ok(TransactionType::SUB),
            "SWP" => Ok(TransactionType::SWP),
            "TAX" => Ok(TransactionType::TAX),
            "TCK" => Ok(TransactionType::TCK),
            "TCM" => Ok(TransactionType::TCM),
            "TRA" => Ok(TransactionType::TRA),
            "TRF" => Ok(TransactionType::TRF),
            "TRN" => Ok(TransactionType::TRN),
            "UWC" => Ok(TransactionType::UWC),
            "VDA" => Ok(TransactionType::VDA),
            "WAR" => Ok(TransactionType::WAR),
            _ => Err(eyre!(
                "Transaction Type is either missing or the value '{input}' is not valid"
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub enum IO {
    Input,
    Output,
}

impl TryFrom<&str> for IO {
    type Error = eyre::Error;

    fn try_from(input: &str) -> Result<Self> {
        match input {
            "I" => Ok(IO::Input),
            "O" => Ok(IO::Output),
            _ => Err(eyre!(
                "IO is either missing or the value '{input}' is not valid"
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub enum ApplicationId {
    F,
    A,
    L,
}

impl TryFrom<&str> for ApplicationId {
    type Error = eyre::Error;

    fn try_from(input: &str) -> Result<Self> {
        match input {
            "F" => Ok(ApplicationId::F),
            "A" => Ok(ApplicationId::A),
            "L" => Ok(ApplicationId::L),
            _ => Err(eyre!(
                "Application Id is either missing or the value '{input}' is not valid"
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub enum ServiceId {
    FinGpa,
    AckNak,
}

impl TryFrom<&str> for ServiceId {
    type Error = eyre::Error;

    fn try_from(input: &str) -> Result<Self> {
        match input {
            "21" => Ok(ServiceId::FinGpa),
            "01" => Ok(ServiceId::AckNak),
            _ => Err(eyre!(
                "Service Id is either missing or the value '{input}' is not valid"
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub enum CreditDebit {
    Credit,
    Debit,
    CreditReversal,
    DebitReversal,
}

impl CreditDebit {
    pub fn value(self) -> String {
        match self {
            CreditDebit::Credit => "C".to_string(),
            CreditDebit::Debit => "D".to_string(),
            CreditDebit::CreditReversal => "CR".to_string(),
            CreditDebit::DebitReversal => "DR".to_string(),
        }
    }
}

impl TryFrom<&str> for CreditDebit {
    type Error = eyre::Error;

    fn try_from(input: &str) -> Result<Self> {
        match input {
            "CR" | "RC" => Ok(CreditDebit::CreditReversal),
            "DR" | "RD" => Ok(CreditDebit::DebitReversal),
            "C" => Ok(CreditDebit::Credit),
            "D" => Ok(CreditDebit::Debit),
            _ => Err(eyre!(
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

    fn try_from(input: &str) -> Result<Self> {
        match input {
            "S" => Ok(FundsCode::SwiftTransfer),
            "N" => Ok(FundsCode::NonSwiftTransfer),
            "F" => Ok(FundsCode::FirstAdvice),
            _ => Err(eyre!(
                "Funds Code is either missing or the value '{input}' is not valid"
            )),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
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
            "REMIT" => Ok(ValidationFlag::REMIT),
            "RFDD" => Ok(ValidationFlag::RFDD),
            "STP" => Ok(ValidationFlag::STP),
            _ => Err(eyre!(
                "Validation Flag value is either missing or the value '{input}' is not valid"
            )),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
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
            "AOK" => Ok(SanctionScreenType::AOK),
            "FPO" => Ok(SanctionScreenType::FPO),
            "NOK" => Ok(SanctionScreenType::NOK),
            _ => Err(eyre!(
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

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
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
        let currency = Currency::from_code(&input[7..10])
            .ok_or_else(|| eyre!("currency code does not exist"))?; // fix
        let amount = float_from_swift_amount(&input[10..])?;

        Ok(Self {
            credit_or_debit,
            date,
            currency,
            amount,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
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
        Ok(NaiveDate::from_ymd(
            2000 + date[..2].parse::<i32>()?,
            date[2..4].parse::<u32>()?,
            date[4..6].parse::<u32>()?,
        ))
    } else if date.len() == 8 {
        Ok(NaiveDate::from_ymd(
            date[..4].parse::<i32>()?,
            date[4..6].parse::<u32>()?,
            date[6..8].parse::<u32>()?,
        ))
    } else {
        Err(eyre!("Invalid swift date provided"))
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
    use super::*;

    #[test]
    fn test_amount_with_scale() {
        let amount = float_from_swift_amount("379,29").unwrap();

        assert_eq!(amount, 379.29)
    }

    #[test]
    fn test_amount_without_scale() {
        let amount = float_from_swift_amount("379,").unwrap();

        assert_eq!(amount, 379.0)
    }

    #[test]
    fn test_amount_without_comma() {
        let amount = float_from_swift_amount("379.").unwrap();

        assert_eq!(amount, 379.0);
    }

    #[test]
    fn test_date_long_year() {
        let date = naive_date_from_swift_date("20090924").unwrap();

        assert_eq!(date.year(), 2009);
        assert_eq!(date.month(), 9);
        assert_eq!(date.day(), 24);
    }

    #[test]
    fn test_date_short_year() {
        let date = naive_date_from_swift_date("090924").unwrap();

        assert_eq!(date.year(), 2009);
        assert_eq!(date.month(), 9);
        assert_eq!(date.day(), 24);
    }

    #[test]
    fn test_date_no_year() {
        let date = naive_date_from_swift_date("0924").unwrap();

        assert_eq!(date.year(), chrono::Utc::now().year());
        assert_eq!(date.month(), 9);
        assert_eq!(date.day(), 24);
    }

    #[test]
    fn test_time() {
        let time = naive_time_from_swift_time("121413").unwrap();

        assert_eq!(time.hour(), 12);
        assert_eq!(time.minute(), 14);
        assert_eq!(time.second(), 13);
    }

    #[test]
    #[should_panic(expected = "Invalid swift date provided")]
    fn test_date_bad() {
        naive_date_from_swift_date("").unwrap();
    }

    #[test]
    fn test_business_identifier_code() {
        let bic_code = BusinessIdentifierCode::new("ASNBNL21").unwrap();

        assert_eq!(bic_code.business_party_prefix, "ASNB");
        assert_eq!(bic_code.country_code, "NL");
        assert_eq!(bic_code.business_party_suffix, "21");
    }

    #[test]
    fn test_logical_terminal_address() {
        let logical_terminal_address = LogicalTerminalAddress::new("ASNBNL21XXXX").unwrap();

        assert_eq!(
            logical_terminal_address.bic_code,
            BusinessIdentifierCode::new("ASNBNL21").unwrap()
        );
        assert_eq!(logical_terminal_address.terminal_code, "X");
        assert_eq!(logical_terminal_address.branch_code, "XXX");
    }

    #[test]
    fn test_credit_or_debit_credit() {
        let credit = CreditDebit::try_from("C").unwrap();
        assert_eq!(credit, CreditDebit::Credit);
    }

    #[test]
    fn test_credit_or_debit_debit() {
        let debit = CreditDebit::try_from("D").unwrap();
        assert_eq!(debit, CreditDebit::Debit);
    }

    #[test]
    #[should_panic(expected = "Credit Debit is either missing or the value 'A' is not valid")]
    fn test_credit_or_debit() {
        CreditDebit::try_from("A").unwrap();
    }

    #[test]
    fn test_currency_code() {
        let currency_code = Currency::from_code("EUR").unwrap();

        assert_eq!(currency_code, Currency::EUR);
    }

    #[test]
    fn test_funds_code() {
        let swift_transfer = FundsCode::try_from("S").unwrap();
        assert_eq!(swift_transfer, FundsCode::SwiftTransfer);

        let swift_transfer = FundsCode::try_from("N").unwrap();
        assert_eq!(swift_transfer, FundsCode::NonSwiftTransfer);

        let swift_transfer = FundsCode::try_from("F").unwrap();
        assert_eq!(swift_transfer, FundsCode::FirstAdvice);
    }

    #[test]
    #[should_panic(expected = "Funds Code is either missing or the value 'T' is not valid")]
    fn test_funds_code_bad() {
        FundsCode::try_from("T").unwrap();
    }

    #[test]
    fn test_balance() {
        let balance = Balance::new("C090930EUR53189,31").unwrap();

        assert_eq!(balance.credit_or_debit, CreditDebit::Credit);
        assert_eq!(balance.date, NaiveDate::from_ymd(2009, 9, 30));
        assert_eq!(balance.currency, Currency::EUR);
        assert_eq!(balance.amount, 53189.31);
    }
}
