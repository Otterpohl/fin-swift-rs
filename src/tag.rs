use chrono::prelude::*;
use std::convert::TryFrom;
use iso_4217::*;
use std::fmt;

// https://www.paiementor.com/swift-mt940-format-specifications/

// Tag20
#[derive(Debug)]
pub struct TransactionReferenceNumber<'a> { 
    pub data: &'a str,
}

impl<'a> TransactionReferenceNumber<'a> {
    pub fn new(value: &'a str) -> Self {
        TransactionReferenceNumber {
            data: value,
        }
    }
}

#[derive(Debug)]
enum CreditOrDebit {
    Credit,
    Debit,
}

// Tag25
#[derive(Debug)]
pub struct AccountIdentification<'a> {
    pub data: &'a str,
}

impl<'a> AccountIdentification<'a> {
    pub fn new(value: &'a str) -> Self {
        AccountIdentification {
            data: value,
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

        StatementNumber {
            statement_number: statement_sequence_number[0],
            sequence_number: statement_sequence_number[1],
        }
    }
}

// Tag60F
#[derive(Debug)]
pub struct OpeningBalanceFinal { // do we need a separate struct just for F and M?
    credit_or_debit: CreditOrDebit,
    date: NaiveDate,
    currency: CurrencyCode,
    amount: f64
}

impl OpeningBalanceFinal {
    pub fn new(value: &str) -> Self {
        // TODO: account for 1900??
        let credit_or_debit = match &value[..1] {
            "C" => {CreditOrDebit::Credit}
            "D" => {CreditOrDebit::Debit}
            _ => {panic!("We really shouldn't have reached this, too bad!");}
        };
        let date = NaiveDate::from_ymd(
            2000 + value[1..3].parse::<i32>().unwrap(),
            value[3..5].parse::<u32>().unwrap(), 
            value[5..7].parse::<u32>().unwrap()
        );
        let currency: CurrencyCode = TryFrom::try_from(&value[7..10]).unwrap();
        let amount = value[10..].replace(',', '.'.to_string().as_ref()).parse::<f64>().unwrap();

        OpeningBalanceFinal {
            credit_or_debit,
            date,
            currency,
            amount,
        }
    }
}

// Tag61
#[derive(Debug)]
pub struct StatementLine<'a> {
    pub data: &'a str,
}

impl<'a> StatementLine<'a> {
    pub fn new(value: &'a str) -> Self {
        StatementLine {
            data: value,
        }
    }
}

// Tag62F
#[derive(Debug)]
pub struct BookedFundsFinal<'a> { // do we need a separate struct just for F and M?
    pub data: &'a str,
}

impl<'a> BookedFundsFinal<'a> {
    pub fn new(value: &'a str) -> Self {
        BookedFundsFinal {
            data: value,
        }
    }
}

// Tag64
#[derive(Debug)]
pub struct ClosingAvailableBalance<'a> {
    pub data: &'a str,
}

impl<'a> ClosingAvailableBalance<'a> {
    pub fn new(value: &'a str) -> Self {
        ClosingAvailableBalance {
            data: value,
        }
    }
}

// Tag86
#[derive(Debug)]
pub struct InformationToAccountOwner<'a> {
    pub data: &'a str,
}

impl<'a> InformationToAccountOwner<'a> {
    pub fn new(value: &'a str) -> Self {
        InformationToAccountOwner {
            data: value,
        }
    }
}
