use crate::utils::*;
use chrono::prelude::*;
use iso_4217::*; // currency

#[derive(Debug)]
pub struct Balance {
    credit_or_debit: CreditDebit,
    date: NaiveDate,
    currency: CurrencyCode,
    amount: f64,
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
    pub fn new(account_identification: &'a str) -> Self {
        Self {
            account_identification,
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
            .map(|x| x.strip_prefix('0').unwrap_or(x).parse::<u32>().unwrap())
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
    transaction_type: Option<TransactionType>,
    account_owner_reference: &'a str,
    account_servicing_insitution_reference: Option<&'a str>,
    supplementary_details: Option<&'a str>,
}

impl<'a> StatementLine<'a> {
    pub fn new(value: &'a str) -> Self {
        // we will use this to track where in the string we
        // should start parsing from each time we get a value
        let mut index = 0;

        let value_date = naive_date_from_swift_date(&value[index..index + 6]);
        let mut entry_date = value_date;

        index += 6;

        if value[index..index + 4]
            .chars()
            .all(|x| x.is_numeric())
        {
            entry_date = naive_date_from_swift_date(&value[index..index + 4]);
            index += 4;
        }

        // god i hate this shit, i wish i was better at rust
        let debit_or_credit = if &value[index..index + 2] == "CR"
            || &value[index..index + 2] == "RC"
        {
            index += 2;
            CreditDebit::CreditReversal
        } else if &value[index..index + 2] == "DR"
            || &value[index..index + 2] == "RD"
        {
            index += 2;
            CreditDebit::DebitReversal
        } else if &value[index..index + 1] == "C" {
            index += 1;
            CreditDebit::Credit
        } else if &value[index..index + 1] == "D" {
            index += 1;
            CreditDebit::Debit
        } else {
            panic!("We really shouldn't have reached this, too bad!");
        };

        let mut amount_string = "".to_string();

        for c in value[index..index + 15].chars() {
            if c.to_string().parse::<u8>().is_ok() || c.to_string() == "," {
                amount_string.push_str(&c.to_string());
            } else {
                break;
            }
        }

        let amount = float_from_swift_amount(&amount_string);

        // float will truncate the 0 and so the len will be 1 char short, check the string instead!
        index += amount_string.to_string().len();

        // fold here maybe?
        let funds_code = if value[index..index + 1]
            .chars()
            .map(|x| x.to_string())
            .any(|x| x == "S" || x == "N" || x == "F")
        {
            FundsCode::try_from(&value[index..index + 1]).unwrap()
        } else {
            panic!("We really shouldn't have reached this, too bad!");
        };

        index += 1;

        let transaction_type = if funds_code != FundsCode::SwiftTransfer {
            Some(TransactionType::try_from(&value[index..index + 3]).unwrap())
        } else {
            None
        };

        if transaction_type.is_some() {
            index += 3;
        }

        let account_owner_reference = if value[index..].len() > 16 {
            &value[index..index + 16]
        } else {
            &value[index..]
        };

        index += account_owner_reference.len();

        let mut supplementary_details = None;

        let account_servicing_insitution_reference = if value[index..].starts_with("NONREF") {
            Some("NONREF")
        } else if value[index..].is_empty() {
            None
        } else {
            Some(&value[index..])
        };

        if account_servicing_insitution_reference == Some("NONREF") {
            index += 6;
            supplementary_details = Some(&value[index..]);
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
pub struct ClosingAvailableBalance {
    balance_data: Balance,
}

impl ClosingAvailableBalance {
    pub fn new(value: &str) -> Self {
        Self {
            balance_data: Balance::new(value),
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
