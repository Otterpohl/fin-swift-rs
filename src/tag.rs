use chrono::prelude::*;
use std::convert::TryFrom;
use iso_4217::*;

// https://www.paiementor.com/swift-mt940-format-specifications/

#[derive(Debug)]
enum CreditOrDebit {
    Credit,
    Debit,
}

#[derive(Debug)]
pub enum BalanceType {
    Final,
    //Intermediary,
}

#[derive(Debug)]
pub struct Balance {
    credit_or_debit: CreditOrDebit,
    date: NaiveDate,
    currency: CurrencyCode,
    amount: f64
}

impl Balance {
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
        TransactionReferenceNumber {
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
        AccountIdentification {
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

        StatementNumber {
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
        OpeningBalance {
            balance_type,
            balance_data: Balance::new(balance_data),
        }
    }
}

// Tag61
#[derive(Debug)]
pub struct StatementLine<'a> {
    pub statement_line: &'a str,
}

impl<'a> StatementLine<'a> {
    pub fn new(value: &'a str) -> Self {
        StatementLine {
            statement_line: value,
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
        BookedFunds {
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
        ClosingAvailableBalance {
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
        InformationToAccountOwner {
            information_to_account_owner: value,
        }
    }
}
