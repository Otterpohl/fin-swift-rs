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
pub struct StatementNumber<'a> {
    pub data: &'a str,
}

impl<'a> StatementNumber<'a> {
    pub fn new(value: &'a str) -> Self {
        StatementNumber {
            data: value,
        }
    }
}

// Tag60F
#[derive(Debug)]
pub struct OpeningBalanceFinal<'a> { // do we need a separate struct just for F and M?
    pub data: &'a str,
}

impl<'a> OpeningBalanceFinal<'a> {
    pub fn new(value: &'a str) -> Self {
        OpeningBalanceFinal {
            data: value,
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
