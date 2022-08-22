use crate::utils::{
    float_from_swift_amount, naive_date_from_swift_date, Balance, BalanceType, CreditDebit,
    FundsCode, SanctionScreenType, TransactionType, ValidationFlag,
};
use chrono::NaiveDate;
use eyre::Result;
use serde::Serialize;

// Tag20
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
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
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
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
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub struct StatementNumber {
    pub statement_number: u32,
    pub sequence_number: u32,
}

impl StatementNumber {
    pub fn new(value: &str) -> Result<Self> {
        let statement_sequence_number = value
            .split('/')
            .map(|x| x.strip_prefix('0').unwrap_or(x).parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            statement_number: statement_sequence_number[0],
            sequence_number: statement_sequence_number[1],
        })
    }
}

// Tag60F
#[derive(Debug, PartialEq, Serialize)]
pub struct OpeningBalance {
    pub balance_type: BalanceType,
    pub balance_data: Balance,
}

impl OpeningBalance {
    pub fn new(balance_type: BalanceType, balance_data: &str) -> Result<Self> {
        Ok(Self {
            balance_type,
            balance_data: Balance::new(balance_data)?,
        })
    }
}

// Tag61
#[derive(Debug, PartialEq, Serialize)]
pub struct StatementLine<'a> {
    pub value_date: NaiveDate,
    pub entry_date: NaiveDate,
    pub debit_or_credit: CreditDebit,
    pub amount: f64,
    pub funds_code: FundsCode,
    pub transaction_type: Option<TransactionType>,
    pub account_owner_reference: &'a str,
    pub account_servicing_insitution_reference: Option<&'a str>, // TODO typo
    pub supplementary_details: Option<&'a str>,
}

impl<'a> StatementLine<'a> {
    pub fn new(value: &'a str) -> Result<Self> {
        // we will use this to track where in the string we
        // should start parsing from each time we get a value
        let mut index = 0;

        let value_date = naive_date_from_swift_date(&value[index..index + 6])?;
        let mut entry_date = value_date;

        index += 6;

        if value[index..index + 4].chars().all(char::is_numeric) {
            entry_date = naive_date_from_swift_date(&value[index..index + 4])?;
            index += 4;
        }

        let debit_or_credit = CreditDebit::try_from(&value[index..index + 2])
            .unwrap_or(CreditDebit::try_from(&value[index..=index])?);

        index += debit_or_credit.value().len();

        let mut amount_string = "".to_string();

        for c in value[index..index + 15].chars().map(|x| x.to_string()) {
            if c.parse::<u8>().is_ok() || c == "," {
                amount_string.push_str(&c);
            } else {
                break;
            }
        }

        let amount = float_from_swift_amount(&amount_string)?;

        // float will truncate the 0 and so the len will be 1 char short, check the string instead!
        index += amount_string.to_string().len();

        let funds_code = FundsCode::try_from(&value[index..=index])?;

        index += 1;

        let transaction_type = if funds_code == FundsCode::SwiftTransfer {
            None
        } else {
            Some(TransactionType::try_from(&value[index..index + 3])?)
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

        Ok(Self {
            value_date,
            entry_date,
            debit_or_credit,
            amount,
            funds_code,
            transaction_type,
            account_owner_reference,
            account_servicing_insitution_reference,
            supplementary_details,
        })
    }
}

// Tag62F
#[derive(Debug, PartialEq, Serialize)]
pub struct BookedFunds {
    pub balance_type: BalanceType,
    pub balance_data: Balance,
}

impl BookedFunds {
    pub fn new(balance_type: BalanceType, balance_data: &str) -> Result<Self> {
        Ok(Self {
            balance_type,
            balance_data: Balance::new(balance_data)?,
        })
    }
}

// Tag64
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct ClosingAvailableBalance {
    pub balance_data: Balance,
}

impl ClosingAvailableBalance {
    pub fn new(value: &str) -> Result<Self> {
        Ok(Self {
            balance_data: Balance::new(value)?,
        })
    }
}

// Tag86
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
// Tag103
pub struct ServiceIdentifier<'a> {
    pub service_identifier: &'a str,
}

impl<'a> ServiceIdentifier<'a> {
    pub fn new(value: &'a str) -> Self {
        assert!(
            value.len() == 3,
            "ServiceIdentifier '{value}' is an unexpected length"
        );

        Self {
            service_identifier: value,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
// Tag113
pub struct BankingPriority<'a> {
    pub banking_priority: &'a str,
}

impl<'a> BankingPriority<'a> {
    pub fn new(value: &'a str) -> Self {
        assert!(
            value.len() == 4,
            "BankingPriority '{value}' is an unexpected length"
        );

        Self {
            banking_priority: value,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
// Tag108
pub struct MessageUserReference<'a> {
    pub message_user_reference: &'a str,
}

impl<'a> MessageUserReference<'a> {
    pub fn new(value: &'a str) -> Self {
        assert!(
            value.len() <= 16,
            "MessageUserReference '{value}' is an unexpected length"
        );

        Self {
            message_user_reference: value,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
// Tag119
pub struct Validation {
    pub validation_flag: ValidationFlag,
}

impl Validation {
    pub fn new(value: &str) -> Result<Self> {
        Ok(Self {
            validation_flag: ValidationFlag::try_from(value)?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
// Tag424
pub struct RelatedReference<'a> {
    pub related_reference: &'a str,
}

impl<'a> RelatedReference<'a> {
    pub fn new(value: &'a str) -> Self {
        assert!(
            value.len() <= 16,
            "RelatedReference '{value}' is an unexpected length"
        );

        Self {
            related_reference: value,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
// Tag111
pub struct ServiceTypeIdentifier<'a> {
    pub service_type_identifier: &'a str,
}

impl<'a> ServiceTypeIdentifier<'a> {
    pub fn new(value: &'a str) -> Self {
        assert!(
            value.len() == 3,
            "ServiceTypeIdentifier '{value}' is an unexpected length"
        );

        Self {
            service_type_identifier: value,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
// Tag165
pub struct PaymentReleaseInformationReceiver<'a> {
    pub payment_release_information_receiver: &'a str,
}

impl<'a> PaymentReleaseInformationReceiver<'a> {
    pub fn new(value: &'a str) -> Self {
        assert!(
            value.len() <= 34,
            "PaymentReleaseInformationReceiver '{value}' is an unexpected length"
        );

        Self {
            payment_release_information_receiver: value,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
// Tag433
pub struct SanctionsScreeningInformation<'a> {
    pub codeword: SanctionScreenType,
    pub additional_information: &'a str, // this should be an option!
}

impl<'a> SanctionsScreeningInformation<'a> {
    pub fn new(value: &'a str) -> Result<Self> {
        let codeword = &value[1..4];
        let additional_information = value[4..].strip_prefix('\\').unwrap_or("");

        Ok(Self {
            codeword: SanctionScreenType::try_from(codeword)?,
            additional_information,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
// Tag434
pub struct PaymentControlsInformation<'a> {
    pub codeword: &'a str,
    pub additional_information: &'a str,
}

impl<'a> PaymentControlsInformation<'a> {
    pub fn new(value: &'a str) -> Self {
        let codeword = &value[1..4];
        let additional_information = &value[4..].strip_prefix('\\').unwrap_or("");

        Self {
            codeword,
            additional_information,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iso_currency::Currency;

    #[test]
    fn test_transaction_reference_number() -> Result<()> {
        assert_eq!(
            TransactionReferenceNumber::new("3996-11-11111111").transaction_reference_number,
            "3996-11-11111111"
        ); // TODO do we need to parse this or is it just free text?
        Ok(())
    }

    #[test]
    fn test_account_identification() -> Result<()> {
        assert_eq!(
            AccountIdentification::new("DABADKKK/111111-11111111").account_identification,
            "DABADKKK/111111-11111111"
        );
        Ok(())
    }

    #[test]
    fn test_statement_number() -> Result<()> {
        let statement = StatementNumber::new("00001/001")?;

        assert_eq!(statement.statement_number, 1);
        assert_eq!(statement.sequence_number, 1);
        Ok(())
    }

    #[test]
    fn test_opening_balance() -> Result<()> {
        let opening_balance = OpeningBalance::new(BalanceType::Final, "C090924EUR54484,04")?;

        assert_eq!(
            opening_balance.balance_data.credit_or_debit,
            CreditDebit::Credit
        );
        assert_eq!(
            opening_balance.balance_data.date,
            NaiveDate::from_ymd(2009, 9, 24)
        );
        assert_eq!(opening_balance.balance_data.currency, Currency::EUR);
        assert_eq!(opening_balance.balance_data.amount, 54484.04);
        Ok(())
    }

    #[test]
    fn test_booked_funds() -> Result<()> {
        let booked_funds = BookedFunds::new(BalanceType::Final, "C090924EUR54484,04")?;

        assert_eq!(
            booked_funds.balance_data.credit_or_debit,
            CreditDebit::Credit
        );
        assert_eq!(
            booked_funds.balance_data.date,
            NaiveDate::from_ymd(2009, 9, 24)
        );
        assert_eq!(booked_funds.balance_data.currency, Currency::EUR);
        assert_eq!(booked_funds.balance_data.amount, 54484.04);
        Ok(())
    }

    #[test]
    fn test_closing_available_funds() -> Result<()> {
        let closing_available_funds = ClosingAvailableBalance::new("C090924EUR54484,04")?;

        assert_eq!(
            closing_available_funds.balance_data.credit_or_debit,
            CreditDebit::Credit
        );
        assert_eq!(
            closing_available_funds.balance_data.date,
            NaiveDate::from_ymd(2009, 9, 24)
        );
        assert_eq!(closing_available_funds.balance_data.currency, Currency::EUR);
        assert_eq!(closing_available_funds.balance_data.amount, 54484.04);
        Ok(())
    }

    #[test]
    fn test_information_to_account_owner() -> Result<()> {
        assert_eq!(
            InformationToAccountOwner::new("Fees according to advice").information_to_account_owner,
            "Fees according to advice"
        );
        Ok(())
    }

    #[test]
    fn test_statement_line() -> Result<()> {
        let sl = StatementLine::new("0909290929DR55,00NMSC0000000000000269//1234")?;

        assert_eq!(sl.value_date, NaiveDate::from_ymd(2009, 9, 29));
        assert_eq!(sl.entry_date, NaiveDate::from_ymd(2022, 9, 29));
        assert_eq!(sl.amount, 55.0);
        assert_eq!(sl.funds_code, FundsCode::NonSwiftTransfer);
        assert_eq!(sl.transaction_type, Some(TransactionType::MSC));
        assert_eq!(sl.account_owner_reference, "0000000000000269");
        assert_eq!(sl.account_servicing_insitution_reference, Some("//1234"));
        assert_eq!(sl.supplementary_details, None);
        Ok(())
    }

    #[test]
    fn test_statement_line_credit() -> Result<()> {
        let sl = StatementLine::new("0909290929C55,00NMSC0000000000000269//1234")?;

        assert_eq!(sl.debit_or_credit, CreditDebit::Credit);
        Ok(())
    }

    #[test]
    fn test_statement_line_debit() -> Result<()> {
        let sl = StatementLine::new("0909290929D55,00NMSC0000000000000269//1234")?;

        assert_eq!(sl.debit_or_credit, CreditDebit::Debit);
        Ok(())
    }

    #[test]
    fn test_statement_line_credit_reversal() -> Result<()> {
        let sl = StatementLine::new("0909290929CR55,00NMSC0000000000000269//1234")?;

        assert_eq!(sl.debit_or_credit, CreditDebit::CreditReversal);
        Ok(())
    }

    #[test]
    fn test_statement_line_debit_reversal() -> Result<()> {
        let sl = StatementLine::new("0909290929DR55,00NMSC0000000000000269//1234")?;

        assert_eq!(sl.debit_or_credit, CreditDebit::DebitReversal);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Credit Debit is either missing or the value '5' is not valid")]
    fn test_statement_line_missing_credit_or_debit() {
        StatementLine::new("090929092955,00NMSC0000000000000269//1234").unwrap();
    }

    #[test]
    #[should_panic(expected = "Funds Code is either missing or the value 'M' is not valid")]
    fn test_statement_line_missing_funds_code() {
        StatementLine::new("0909290929DR55,00MSC0000000000000269//1234").unwrap();
    }
}
