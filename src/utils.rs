use chrono::prelude::*;
use chrono::NaiveDate;
use iso_4217::*; // currency
use iso3166_1::*;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
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
    type Error = &'static str;

    fn try_from(transaction_type: &str) -> Result<Self, Self::Error> {
        match transaction_type {
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
            _ => Err("We really shouldn't have reached this, too bad!"),
        }
    }
}

#[derive(Debug)]
pub enum CreditDebit {
    Credit,
    Debit,
    CreditReversal,
    DebitReversal,
}

impl TryFrom<&str> for CreditDebit {
    type Error = &'static str;

    fn try_from(credit_or_debit: &str) -> Result<Self, Self::Error> {
        match credit_or_debit {
            "C" => Ok(CreditDebit::Credit),
            "D" => Ok(CreditDebit::Debit),
            _ => Err("We really shouldn't have reached this, too bad!"),
        }
    }
}

#[derive(Debug)]
pub enum BalanceType {
    Final,
    Intermediary,
}

#[derive(Debug, PartialEq)]
pub enum FundsCode {
    SwiftTransfer,
    NonSwiftTransfer,
    FirstAdvice,
}

impl TryFrom<&str> for FundsCode {
    type Error = &'static str;

    fn try_from(funds_code: &str) -> Result<Self, Self::Error> {
        match funds_code {
            "S" => Ok(FundsCode::SwiftTransfer),
            "N" => Ok(FundsCode::NonSwiftTransfer),
            "F" => Ok(FundsCode::FirstAdvice),
            _ => Err("We really shouldn't have reached this, too bad!"),
        }
    }
}


#[derive(Debug)]
pub struct BusinessIdentifierCode<'a> {
    pub business_party_prefix: &'a str,
    pub country_code: &'a str,
    pub business_party_suffix: &'a str,
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
    pub bic_code: BusinessIdentifierCode<'a>,
    pub terminal_code: &'a str, // try to make this a char?
    pub branch_code: &'a str,
}

impl<'a> LogicalTerminalAddress<'a> {
    pub fn new(data: &'a str) -> Self { // consider moving this back to block.rs as its not yet used anywhere else
        let bic_code = BusinessIdentifierCode::new(&data[..8]);

        Self {
            bic_code,
            terminal_code: &data[8..9],
            branch_code: &data[9..],
        }
    }
}

#[derive(Debug)]
pub struct Balance {
    pub credit_or_debit: CreditDebit,
    pub date: NaiveDate,
    pub currency: CurrencyCode,
    pub amount: f64,
}

impl Balance {
    pub fn new(value: &str) -> Self {
        let credit_or_debit = CreditDebit::try_from(&value[..1]).unwrap();
        let date = naive_date_from_swift_date(&value[1..7]);
        let currency = CurrencyCode::try_from(&value[7..10]).unwrap();
        let amount = float_from_swift_amount(&value[10..]);

        Self {
            credit_or_debit,
            date,
            currency,
            amount,
        }
    }
}

pub fn naive_date_from_swift_date(date: &str) -> NaiveDate {
    if date.len() == 6 {
        NaiveDate::from_ymd(
            2000 + date[..2].parse::<i32>().unwrap(),
            date[2..4].parse::<u32>().unwrap(),
            date[4..6].parse::<u32>().unwrap(),
        )
    } else {
        NaiveDate::from_ymd(
            chrono::Utc::now().year(),
            date[..2].parse::<u32>().unwrap(),
            date[2..].parse::<u32>().unwrap(),
        )
    }
}

pub fn float_from_swift_amount(amount: &str) -> f64 {
    amount.replace(',', ".").parse::<f64>().unwrap()
}
