use crate::utils::*;
use chrono::prelude::*;

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
            .collect::<Vec<u32>>();

        Self {
            statement_number: statement_sequence_number[0],
            sequence_number: statement_sequence_number[1],
        }
    }
}

// Tag60F
#[derive(Debug)]
pub struct OpeningBalance {
    pub balance_type: BalanceType,
    pub balance_data: Balance,
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
    pub value_date: NaiveDate,
    pub entry_date: NaiveDate,
    pub debit_or_credit: CreditDebit,
    pub amount: f64,
    pub funds_code: FundsCode,
    pub transaction_type: Option<TransactionType>,
    pub account_owner_reference: &'a str,
    pub account_servicing_insitution_reference: Option<&'a str>,
    pub supplementary_details: Option<&'a str>,
}

impl<'a> StatementLine<'a> {
    pub fn new(value: &'a str) -> Self {
        // we will use this to track where in the string we
        // should start parsing from each time we get a value
        let mut index = 0;

        let value_date = naive_date_from_swift_date(&value[index..index + 6]);
        let mut entry_date = value_date;

        index += 6;

        if value[index..index + 4].chars().all(|x| x.is_numeric()) {
            entry_date = naive_date_from_swift_date(&value[index..index + 4]);
            index += 4;
        }

        // god i hate this shit, i wish i was better at rust
        let debit_or_credit = if ["CR", "RC"].iter().any(|x| *x == &value[index..index + 2]) {
                index += 2;
                CreditDebit::CreditReversal
        } else if ["DR", "RD"].iter().any(|x| *x == &value[index..index + 2]) {
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

        for c in value[index..index + 15].chars().map(|x| x.to_string()) {
            if c.parse::<u8>().is_ok() || c == "," {
                amount_string.push_str(&c);
            } else {
                break;
            }
        }

        let amount = float_from_swift_amount(&amount_string);

        // float will truncate the 0 and so the len will be 1 char short, check the string instead!
        index += amount_string.to_string().len();

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

        let account_servicing_insitution_reference = if value[index..].starts_with("NONREF") {
            index += 6;
            Some("NONREF")
        } else if value[index..].is_empty() {
            None
        } else {
            Some(&value[index..])
        };

        let supplementary_details = if account_servicing_insitution_reference == Some("NONREF") {
            Some(&value[index..])
        } else {
            None
        };

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
    pub balance_type: BalanceType,
    pub balance: Balance,
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
    pub balance_data: Balance,
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
