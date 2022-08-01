use crate::errors::TransactionError;
use serde::Deserialize;
use std::{
    fmt::{self, Display},
    str::FromStr,
};

#[derive(Debug)]
pub enum TxType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

impl Display for TxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for TxType {
    type Err = TransactionError;

    fn from_str(input: &str) -> Result<TxType, Self::Err> {
        match input {
            "deposit" => Ok(TxType::Deposit),
            "withdrawal" => Ok(TxType::Withdrawal),
            "dispute" => Ok(TxType::Dispute),
            "resolve" => Ok(TxType::Resolve),
            "chargeback" => Ok(TxType::Chargeback),
            _ => Err(TransactionError::UnknownTransactionType),
        }
    }
}

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Transaction {
    #[serde(rename = "type")]
    tx_type: String,
    #[serde(rename = "client")]
    client_id: u16,
    #[serde(rename = "tx")]
    tx_id: u32,
    amount: Option<f32>,
    #[serde(skip)]
    disputed: bool,
    #[serde(skip)]
    charged_back: bool,
}

impl Transaction {
    pub fn new(tx_type: String, client_id: u16, tx_id: u32, amount: Option<f32>) -> Self {
        Self {
            tx_type,
            client_id,
            tx_id,
            amount,
            disputed: false,
            charged_back: false,
        }
    }

    pub fn id(&self) -> u32 {
        self.tx_id
    }

    // getset crate could be used here instead
    pub fn client_id(&self) -> u16 {
        self.client_id
    }

    pub fn tx_type(&self) -> Result<TxType, TransactionError> {
        TxType::from_str(&self.tx_type)
    }

    pub fn amount(&self) -> Option<f32> {
        self.amount
    }

    // if the transaction is already under dispute,
    // this function returns an error.
    pub fn dispute(&mut self) -> Result<(), TransactionError> {
        if self.disputed {
            Err(TransactionError::InvalidDispute)
        } else {
            self.disputed = true;
            Ok(())
        }
    }

    // if the transaction is already under dispute,
    // this function returns an error.
    pub fn resolve(&mut self) -> Result<(), TransactionError> {
        if !self.disputed {
            Err(TransactionError::InvalidResolve)
        } else {
            self.disputed = false;
            Ok(())
        }
    }

    pub fn chargeback(&mut self) -> Result<(), TransactionError> {
        if self.charged_back {
            Err(TransactionError::InvalidChargeback)
        } else {
            self.charged_back = true;
            Ok(())
        }
    }

    pub fn charged_back(&self) -> bool {
        self.charged_back
    }

    pub fn disputed(&self) -> bool {
        self.disputed
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transaction {}: {} for client {} with amount {}",
            self.id(),
            self.tx_type,
            self.client_id,
            self.amount.unwrap_or_default()
        )
    }
}
