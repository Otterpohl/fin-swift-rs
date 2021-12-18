use chrono::prelude::*;
use std::convert::TryFrom;
use iso_4217::*; // currency

// https://www.paiementor.com/swift-mt940-format-specifications/

#[derive(Debug)]
enum CreditDebit {
    Credit,
    Debit,
    CreditReversal,
    DebitReversal,
}

#[derive(Debug)]
pub enum BalanceType {
    Final,
    //Intermediary,
}

#[derive(Debug)]
pub enum FundsCode {
    SwiftTransfer,
    NonSwiftTransfer,
    FirstAdvice,
}

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

#[derive(Debug)]
pub struct Balance {
    credit_or_debit: CreditDebit,
    date: NaiveDate,
    currency: CurrencyCode,
    amount: f64
}

impl Balance {
    pub fn new(value: &str) -> Self {
        let credit_or_debit = match &value[..1] {
            "C" => CreditDebit::Credit,
            "D" => CreditDebit::Debit,
            _ => panic!("We really shouldn't have reached this, too bad!"),
        };
        let date = NaiveDate::from_ymd( // TODO: account for 1900??
            2000 + value[1..3].parse::<i32>().unwrap(),
            value[3..5].parse::<u32>().unwrap(), 
            value[5..7].parse::<u32>().unwrap()
        );
        let currency: CurrencyCode = TryFrom::try_from(&value[7..10]).unwrap();
        let amount = value[10..].replace(',', ".").parse::<f64>().unwrap();

        Self {
            credit_or_debit,
            date,
            currency,
            amount,
        }
    }
}

// Tag20
#[derive(Debug)]
pub struct TransactionReferenceNumber<'a> { 
    pub transaction_reference_number: &'a str,
}

impl<'a> TransactionReferenceNumber<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            transaction_reference_number: value,
        }
    }
}

// Tag25
#[derive(Debug)]
pub struct AccountIdentification<'a> {
    pub account_identification: &'a str,
}

impl<'a> AccountIdentification<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            account_identification: value,
        }
    }
}

// Tag28C
#[derive(Debug)]
pub struct StatementNumber {
    pub statement_number: u32,
    pub sequence_number: u32,
}

impl StatementNumber {
    pub fn new(value: &str) -> Self {
        let statement_sequence_number = value
            .split('/')
            .map(|x| x
                .strip_prefix('0')
                .unwrap_or(x)
                .parse::<u32>()
                .unwrap())
            .collect::<Vec<_>>();

        Self {
            statement_number: statement_sequence_number[0],
            sequence_number: statement_sequence_number[1],
        }
    }
}

// Tag60F
#[derive(Debug)]
pub struct OpeningBalance {
    balance_type: BalanceType,
    balance_data: Balance,
}

impl OpeningBalance {
    pub fn new(balance_type: BalanceType, balance_data: &str) -> Self {
        Self {
            balance_type,
            balance_data: Balance::new(balance_data),
        }
    }
}

// Tag61
#[derive(Debug)]
pub struct StatementLine<'a> {
    value_date: NaiveDate,
    entry_date: NaiveDate,
    debit_or_credit: CreditDebit,
    amount: f64,
    funds_code: FundsCode,
    transaction_type: TransactionType,
    account_owner_reference: &'a str,
    account_servicing_insitution_reference: Option<&'a str>,
    supplementary_details: Option<&'a str>,
}

impl<'a> StatementLine<'a> {
    pub fn new(value: &'a str) -> Self {
        let mut next_start_index = 0;

        // 090925 0925 DR 583,92 N MSC
        // 1110030403010139//1234
        let value_date = NaiveDate::from_ymd( // TODO: account for 1900??
            2000 + value[next_start_index..2].parse::<i32>().unwrap(),
            value[2..4].parse::<u32>().unwrap(), 
            value[4..6].parse::<u32>().unwrap()
        );

        let mut entry_date = value_date;

        next_start_index = 6;

        if value[6..10].chars().all(|x| x.is_numeric()) {
            entry_date = NaiveDate::from_ymd( // TODO: account for 1900??
                chrono::Utc::now().year(),
            value[6..8].parse::<u32>().unwrap(), 
            value[8..10].parse::<u32>().unwrap()
            );

            next_start_index = 10;
        }

        // god i hate this shit, i wish i was better at rust
        let debit_or_credit = if &value[next_start_index..next_start_index + 2] == "CR" || 
            &value[next_start_index..next_start_index + 2] == "RC" {
            next_start_index = 12;
            CreditDebit::CreditReversal
        }
        else if &value[next_start_index..next_start_index + 2] == "DR" || 
            &value[next_start_index..next_start_index + 2] == "RD" {
            next_start_index = 12;
            CreditDebit::DebitReversal
        }
        else if &value[next_start_index..next_start_index + 1] == "C" {
            next_start_index = 11;
            CreditDebit::Credit
        }
        else if &value[next_start_index..next_start_index + 1] == "D" {
            next_start_index = 11;
            CreditDebit::Debit
        }
        else {
            panic!("We really shouldn't have reached this, too bad!");
        };

        let mut amount_string = "".to_string();

        for c in value[next_start_index..next_start_index + 15].chars() {
            if c.to_string().parse::<u8>().is_ok() || c.to_string() == "," {
                amount_string.push_str(&c.to_string());
            }
            else{
                break
            }
        }

        let amount = amount_string.replace(',', ".").parse::<f64>().unwrap();
        next_start_index = next_start_index + amount.to_string().len();

        // fold here maybe?
        let funds_code = if value[next_start_index..next_start_index + 1].chars().any(|x| 
            x.to_string() == "S" ||
            x.to_string() == "N" ||
            x.to_string() == "F") {

            match &value[next_start_index..next_start_index + 1] {
                "S" => FundsCode::SwiftTransfer,
                "N" => FundsCode::NonSwiftTransfer,
                "F" => FundsCode::FirstAdvice,
                _ => panic!("We really shouldn't have reached this, too bad!"),
            }
        }
        else {
            panic!("We really shouldn't have reached this, too bad!");
        };

        next_start_index = next_start_index + 1;

        let transaction_type = match &value[next_start_index..next_start_index + 3] { // try from this shit
            "BNK" => TransactionType::BNK,
            "BOE" => TransactionType::BOE,
            "BRF" => TransactionType::BRF,
            "CAR" => TransactionType::CAR,
            "CAS" => TransactionType::CAS,
            "CHG" => TransactionType::CHG,
            "CHK" => TransactionType::CHK,
            "CLR" => TransactionType::CLR,
            "CMI" => TransactionType::CMI,
            "CMN" => TransactionType::CMN,
            "CMP" => TransactionType::CMP,
            "CMS" => TransactionType::CMS,
            "CMT" => TransactionType::CMT,
            "CMZ" => TransactionType::CMZ,
            "COL" => TransactionType::COL,
            "COM" => TransactionType::COM,
            "CPN" => TransactionType::CPN,
            "DCR" => TransactionType::DCR,
            "DDT" => TransactionType::DDT,
            "DIS" => TransactionType::DIS,
            "DIV" => TransactionType::DIV,
            "EQA" => TransactionType::EQA,
            "EXT" => TransactionType::EXT,
            "FEX" => TransactionType::FEX,
            "INT" => TransactionType::INT,
            "LBX" => TransactionType::LBX,
            "LDP" => TransactionType::LDP,
            "MAR" => TransactionType::MAR,
            "MAT" => TransactionType::MAT,
            "MGT" => TransactionType::MGT,
            "MSC" => TransactionType::MSC,
            "NWI" => TransactionType::NWI,
            "ODC" => TransactionType::ODC,
            "OPT" => TransactionType::OPT,
            "PCH" => TransactionType::PCH,
            "POP" => TransactionType::POP,
            "PRN" => TransactionType::PRN,
            "REC" => TransactionType::REC,
            "RED" => TransactionType::RED,
            "RIG" => TransactionType::RIG,
            "RTI" => TransactionType::RTI,
            "SAL" => TransactionType::SAL,
            "SEC" => TransactionType::SEC,
            "SLE" => TransactionType::SLE,
            "STO" => TransactionType::STO,
            "STP" => TransactionType::STP,
            "SUB" => TransactionType::SUB,
            "SWP" => TransactionType::SWP,
            "TAX" => TransactionType::TAX,
            "TCK" => TransactionType::TCK,
            "TCM" => TransactionType::TCM,
            "TRA" => TransactionType::TRA,
            "TRF" => TransactionType::TRF,
            "TRN" => TransactionType::TRN,
            "UWC" => TransactionType::UWC,
            "VDA" => TransactionType::VDA,
            "WAR" => TransactionType::WAR,
             _ => panic!("We really shouldn't have reached this, too bad!"),
        };

        next_start_index = next_start_index + 3;

        let account_owner_reference = &value[next_start_index..next_start_index + 16];

        next_start_index = next_start_index + 16;

        
        let mut supplementary_details = None;

        let account_servicing_insitution_reference =  if value[next_start_index..].starts_with("NONREF") {
            Some("NONREF")
        } 
        else {
            Some(&value[next_start_index..])
        };

        if account_servicing_insitution_reference == Some("NONREF") {
            next_start_index = next_start_index + 6;
            supplementary_details = Some(&value[next_start_index..]);
        }

        Self {
            value_date,
            entry_date,
            debit_or_credit,
            amount,
            funds_code,
            transaction_type,
            account_owner_reference,
            account_servicing_insitution_reference,
            supplementary_details,
        }
    }
}

// Tag62F
#[derive(Debug)]
pub struct BookedFunds {
    balance_type: BalanceType,
    balance: Balance,
}

impl BookedFunds {
    pub fn new(balance_type: BalanceType, balance_data: &str) -> Self {
        Self {
            balance_type,
            balance: Balance::new(balance_data),
        }
    }
}

// Tag64
#[derive(Debug)]
pub struct ClosingAvailableBalance<'a> {
    pub closing_available_balance: &'a str,
}

impl<'a> ClosingAvailableBalance<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            closing_available_balance: value,
        }
    }
}

// Tag86
#[derive(Debug)]
pub struct InformationToAccountOwner<'a> {
    pub information_to_account_owner: &'a str,
}

impl<'a> InformationToAccountOwner<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            information_to_account_owner: value,
        }
    }
}
