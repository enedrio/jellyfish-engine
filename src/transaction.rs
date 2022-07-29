use std::{fmt, str::FromStr};

use serde::Deserialize;

pub enum TxType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

impl std::str::FromStr for TxType {
    type Err = String;

    fn from_str(input: &str) -> Result<TxType, Self::Err> {
        match input {
            "deposit" => Ok(TxType::Deposit),
            "withdrawal" => Ok(TxType::Withdrawal),
            "dispute" => Ok(TxType::Dispute),
            "resolve" => Ok(TxType::Resolve),
            "chargeback" => Ok(TxType::Chargeback),
            _ => Err("Unknown Transaction Type".to_string()),
        }
    }
}

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize, Clone)]
pub struct Transaction {
    #[serde(rename = "type")]
    tx_type: String,
    #[serde(rename = "client")]
    client_id: u32,
    #[serde(rename = "tx")]
    tx_id: u32,
    amount: Option<f32>,
    #[serde(skip)]
    disputed: bool,
}

impl Transaction {
    pub fn id(&self) -> u32 {
        self.tx_id
    }

    // getset crate could be used here instead
    pub fn client_id(&self) -> u32 {
        self.client_id
    }

    pub fn tx_type(&self) -> Result<TxType, String> {
        TxType::from_str(&self.tx_type)
    }

    pub fn amount(&self) -> Option<f32> {
        self.amount
    }

    // if the transaction is already under dispute,
    // this function returns an error.
    pub fn dispute(&mut self) -> Result<(), String> {
        if self.disputed {
            Err("Already disputed".to_string())
        } else {
            self.disputed = true;
            Ok(())
        }
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
