use crate::utils::*;
use chrono::prelude::*;
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
#[derive(Debug, PartialEq, Serialize)]
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
#[derive(Debug, PartialEq, Serialize)]
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

        if value[index..index + 4].chars().all(char::is_numeric) {
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
        } else if &value[index..=index] == "C" {
            index += 1;
            CreditDebit::Credit
        } else if &value[index..=index] == "D" {
            index += 1;
            CreditDebit::Debit
        } else {
            panic!("Credit/Debit type not found or not recognized");
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

        let funds_code = if value[index..=index]
            .chars()
            .map(|x| x.to_string())
            .any(|x| x == "S" || x == "N" || x == "F")
        {
            FundsCode::try_from(&value[index..=index]).unwrap()
        } else {
            panic!("FundsCode type not found or not recognized");
        };

        index += 1;

        let transaction_type = if funds_code == FundsCode::SwiftTransfer {
            None
        } else {
            Some(TransactionType::try_from(&value[index..index + 3]).unwrap())
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
#[derive(Debug, PartialEq, Serialize)]
pub struct BookedFunds {
    pub balance_type: BalanceType,
    pub balance_data: Balance,
}

impl BookedFunds {
    pub fn new(balance_type: BalanceType, balance_data: &str) -> Self {
        Self {
            balance_type,
            balance_data: Balance::new(balance_data),
        }
    }
}

// Tag64
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
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
        if value.len() != 3 {
            panic!("ServiceIdentifier '{value}' has unexpected length")
        }

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
        if value.len() != 4 {
            panic!("BankingPriority '{value}' has unexpected length")
        }

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
        if value.len() > 16 {
            panic!("Invalid MessageUserReference length");
        }

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
    pub fn new(value: &str) -> Self {
        Self {
            validation_flag: ValidationFlag::try_from(value).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
// Tag424
pub struct RelatedReference<'a> {
    pub related_reference: &'a str,
}

impl<'a> RelatedReference<'a> {
    pub fn new(value: &'a str) -> Self {
        if value.len() > 16 {
            panic!("Invalid RelatedReference length");
        }

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
        if value.len() != 3 {
            panic!("ServiceTypeIdentifier '{value}' has unexpected length")
        }

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
        if value.len() > 34 {
            panic!("Invalid PaymentReleaseInformationReceiver length");
        }

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
    pub fn new(value: &'a str) -> Self {
        let codeword = &value[1..4];
        let additional_information = &value[4..].strip_prefix('\\').unwrap_or("");

        Self {
            codeword: SanctionScreenType::try_from(codeword).unwrap(),
            additional_information,
        }
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
    fn test_transaction_reference_number() {
        let reference = TransactionReferenceNumber::new("3996-11-11111111");

        assert_eq!(reference.transaction_reference_number, "3996-11-11111111");
    }

    #[test]
    fn test_account_identification() {
        let account = AccountIdentification::new("DABADKKK/111111-11111111");

        assert_eq!(account.account_identification, "DABADKKK/111111-11111111");
    }

    #[test]
    fn test_statement_number() {
        let statement = StatementNumber::new("00001/001");

        assert_eq!(statement.statement_number, 1);
        assert_eq!(statement.sequence_number, 1);
    }

    #[test]
    fn test_opening_balance() {
        let opening_balance = OpeningBalance::new(BalanceType::Final, "C090924EUR54484,04");

        assert_eq!(
            opening_balance.balance_data.credit_or_debit,
            CreditDebit::Credit
        );
        assert_eq!(
            opening_balance.balance_data.date,
            NaiveDate::from_ymd(2009, 9, 24)
        );
        assert_eq!(
            opening_balance.balance_data.currency,
            Currency::EUR
        );
        assert_eq!(opening_balance.balance_data.amount, 54484.04);
    }

    #[test]
    fn test_booked_funds() {
        let booked_funds = BookedFunds::new(BalanceType::Final, "C090924EUR54484,04");

        assert_eq!(
            booked_funds.balance_data.credit_or_debit,
            CreditDebit::Credit
        );
        assert_eq!(
            booked_funds.balance_data.date,
            NaiveDate::from_ymd(2009, 9, 24)
        );
        assert_eq!(
            booked_funds.balance_data.currency,
            Currency::EUR
        );
        assert_eq!(booked_funds.balance_data.amount, 54484.04);
    }

    #[test]
    fn test_closing_available_funds() {
        let closing_available_funds = ClosingAvailableBalance::new("C090924EUR54484,04");

        assert_eq!(
            closing_available_funds.balance_data.credit_or_debit,
            CreditDebit::Credit
        );
        assert_eq!(
            closing_available_funds.balance_data.date,
            NaiveDate::from_ymd(2009, 9, 24)
        );
        assert_eq!(
            closing_available_funds.balance_data.currency,
            Currency::EUR
        );
        assert_eq!(closing_available_funds.balance_data.amount, 54484.04);
    }

    #[test]
    fn test_information_to_account_owner() {
        let information_to_account_owner =
            InformationToAccountOwner::new("Fees according to advice");

        assert_eq!(
            information_to_account_owner.information_to_account_owner,
            "Fees according to advice"
        );
    }

    #[test]
    fn test_statement_line() {
        let statement_line = StatementLine::new("0909290929DR55,00NMSC0000000000000269//1234");

        assert_eq!(statement_line.value_date, NaiveDate::from_ymd(2009, 9, 29));
        assert_eq!(statement_line.entry_date, NaiveDate::from_ymd(2022, 9, 29));
        assert_eq!(statement_line.amount, 55.0);
        assert_eq!(statement_line.funds_code, FundsCode::NonSwiftTransfer);
        assert_eq!(statement_line.transaction_type, Some(TransactionType::MSC));
        assert_eq!(statement_line.account_owner_reference, "0000000000000269");
        assert_eq!(
            statement_line.account_servicing_insitution_reference,
            Some("//1234")
        );
        assert_eq!(statement_line.supplementary_details, None);
    }

    #[test]
    fn test_statement_line_credit() {
        let statement_line = StatementLine::new("0909290929C55,00NMSC0000000000000269//1234");

        assert_eq!(statement_line.debit_or_credit, CreditDebit::Credit);
    }

    #[test]
    fn test_statement_line_debit() {
        let statement_line = StatementLine::new("0909290929D55,00NMSC0000000000000269//1234");

        assert_eq!(statement_line.debit_or_credit, CreditDebit::Debit);
    }

    #[test]
    fn test_statement_line_credit_reversal() {
        let statement_line = StatementLine::new("0909290929CR55,00NMSC0000000000000269//1234");

        assert_eq!(statement_line.debit_or_credit, CreditDebit::CreditReversal);
    }

    #[test]
    fn test_statement_line_debit_reversal() {
        let statement_line = StatementLine::new("0909290929DR55,00NMSC0000000000000269//1234");

        assert_eq!(statement_line.debit_or_credit, CreditDebit::DebitReversal);
    }

    #[test]
    #[should_panic(expected = "Credit/Debit type not found or not recognized")]
    fn test_statement_line_missing_credit_or_debit() {
        StatementLine::new("090929092955,00NMSC0000000000000269//1234");
    }

    #[test]
    #[should_panic(expected = "FundsCode type not found or not recognized")]
    fn test_statement_line_missing_funds_code() {
        StatementLine::new("0909290929DR55,00MSC0000000000000269//1234");
    }
}
