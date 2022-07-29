use std::fmt;

use serde::Deserialize;

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
    amount: f32,
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
            self.amount
        )
    }
}
