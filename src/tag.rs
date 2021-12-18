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
        let amount = money_from_swift_amount(&value[10..]);

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
        let mut next_index = 0;

        let value_date = naive_date_from_swift_date(&value[next_index..6]);
        let mut entry_date = value_date;

        next_index += 6;

        if value[next_index..next_index + 4]
            .chars()
            .all(|x| x.is_numeric())
        {
            entry_date = naive_date_from_swift_date(&value[6..10]);
            next_index += 4;
        }

        // god i hate this shit, i wish i was better at rust
        let debit_or_credit = if &value[next_index..next_index + 2] == "CR"
            || &value[next_index..next_index + 2] == "RC"
        {
            next_index += 2;
            CreditDebit::CreditReversal
        } else if &value[next_index..next_index + 2] == "DR"
            || &value[next_index..next_index + 2] == "RD"
        {
            next_index += 2;
            CreditDebit::DebitReversal
        } else if &value[next_index..next_index + 1] == "C" {
            next_index += 1;
            CreditDebit::Credit
        } else if &value[next_index..next_index + 1] == "D" {
            next_index += 1;
            CreditDebit::Debit
        } else {
            panic!("We really shouldn't have reached this, too bad!");
        };

        let mut amount_string = "".to_string();

        for c in value[next_index..next_index + 15].chars() {
            if c.to_string().parse::<u8>().is_ok() || c.to_string() == "," {
                amount_string.push_str(&c.to_string());
            } else {
                break;
            }
        }

        let amount = money_from_swift_amount(&amount_string);
        next_index += amount.to_string().len();

        // fold here maybe?
        let funds_code = if value[next_index..next_index + 1]
            .chars()
            .any(|x| x.to_string() == "S" || x.to_string() == "N" || x.to_string() == "F")
        {
            FundsCode::try_from(&value[next_index..next_index + 1]).unwrap()
        } else {
            panic!("We really shouldn't have reached this, too bad!");
        };

        next_index += 1;

        let transaction_type = if funds_code != FundsCode::SwiftTransfer {
            Some(TransactionType::try_from(&value[next_index..next_index + 3]).unwrap())
        } else {
            None
        };

        if transaction_type.is_some() {
            next_index += 3;
        }

        let account_owner_reference = &value[next_index..next_index + 16];

        next_index += 16;

        let mut supplementary_details = None;

        let account_servicing_insitution_reference = if value[next_index..].starts_with("NONREF") {
            Some("NONREF")
        } else {
            Some(&value[next_index..])
        };

        if account_servicing_insitution_reference == Some("NONREF") {
            next_index += 6;
            supplementary_details = Some(&value[next_index..]);
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
