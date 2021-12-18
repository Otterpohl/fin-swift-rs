use std::fmt;
use chrono::prelude::*;
use chrono::NaiveDate;

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

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            TransactionType::BNK => "Securities Related Item – Bank fees",
            TransactionType::BOE => "Bill of exchange",
            TransactionType::BRF => "Brokerage fee",
            TransactionType::CAR => "Securities Related Item – Corporate Actions Related (Should only be used when no specific corporate action event code is available)",
            TransactionType::CAS => "Securities Related Item – Cash in Lieu",
            TransactionType::CHG => "Charges and other expenses",
            TransactionType::CHK => "Cheques",
            TransactionType::CLR => "Cash letters/Cheques remittance",
            TransactionType::CMI => "Cash management item – No detail",
            TransactionType::CMN => "Cash management item – Notional pooling",
            TransactionType::CMP => "Compensation claims",
            TransactionType::CMS => "Cash management item – Sweeping",
            TransactionType::CMT => "Cash management item -Topping",
            TransactionType::CMZ => "Cash management item – Zero balancing",
            TransactionType::COL => "Collections (used when entering a principal amount)",
            TransactionType::COM => "Commission",
            TransactionType::CPN => "Securities Related Item – Coupon payments",
            TransactionType::DCR => "Documentary credit (used when entering a principal amount)",
            TransactionType::DDT => "Direct Debit Item",
            TransactionType::DIS => "Securities Related Item – Gains disbursement",
            TransactionType::DIV => "Securities Related Item – Dividends",
            TransactionType::EQA => "Equivalent amount",
            TransactionType::EXT => "Securities Related Item – External transfer for own account",
            TransactionType::FEX => "Foreign exchange",
            TransactionType::INT => "Interest",
            TransactionType::LBX => "Lock box",
            TransactionType::LDP => "Loan deposit",
            TransactionType::MAR => "Securities Related Item – Margin payments/Receipts",
            TransactionType::MAT => "Securities Related Item – Maturity",
            TransactionType::MGT => "Securities Related Item – Management fees",
            TransactionType::MSC => "Miscellaneous",
            TransactionType::NWI => "Securities Related Item – New issues distribution",
            TransactionType::ODC => "Overdraft charge",
            TransactionType::OPT => "Securities Related Item – Options",
            TransactionType::PCH => "Securities Related Item – Purchase (including STIF and Time deposits)",
            TransactionType::POP => "Securities Related Item – Pair-off proceeds",
            TransactionType::PRN => "Securities Related Item – Principal pay-down/pay-up",
            TransactionType::REC => "Securities Related Item – Tax reclaim",
            TransactionType::RED => "Securities Related Item – Redemption/Withdrawal",
            TransactionType::RIG => "Securities Related Item – Rights",
            TransactionType::RTI => "Returned item",
            TransactionType::SAL => "Securities Related Item – Sale (including STIF and Time deposits)",
            TransactionType::SEC => "Securities (used when entering a principal amount)",
            TransactionType::SLE => "Securities Related Item – Securities lending related",
            TransactionType::STO => "Standing order",
            TransactionType::STP => "Securities Related Item – Stamp duty",
            TransactionType::SUB => "Securities Related Item – Subscription",
            TransactionType::SWP => "Securities Related Item – SWAP payment",
            TransactionType::TAX => "Securities Related Item – Withholding tax payment",
            TransactionType::TCK => "Travellers cheques",
            TransactionType::TCM => "Securities Related Item – Tripartite collateral management",
            TransactionType::TRA => "Securities Related Item – Internal transfer for own account",
            TransactionType::TRF => "Transfer",
            TransactionType::TRN => "Securities Related Item – Transaction fee",
            TransactionType::UWC => "Securities Related Item – Underwriting commission",
            TransactionType::VDA => "Value date adjustment (used with an entry made to withdraw an incorrectly dated entry – it will be followed by the correct entry with the relevant code)",
            TransactionType::WAR => "Securities Related Item – Warrant",
        };
        write!(f, "({:?}, {})", self, description)
    }
}


pub fn naive_date_from_swift_date(date: &str) -> NaiveDate {
    if date.len() == 6 {
        NaiveDate::from_ymd(
            2000 + date[..2].parse::<i32>().unwrap(),
            date[2..4].parse::<u32>().unwrap(), 
            date[4..6].parse::<u32>().unwrap()
        )
    }
    else {
        NaiveDate::from_ymd(
            chrono::Utc::now().year(),
            date[..2].parse::<u32>().unwrap(), 
            date[2..].parse::<u32>().unwrap()
        )
    }
    
    
}

pub fn money_from_swift_amount(amount: &str) -> f64 {
    amount.replace(',', ".").parse::<f64>().unwrap()
}

