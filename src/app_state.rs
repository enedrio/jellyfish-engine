use crate::client_transaction_handler::ClientTransactionHandler;
use crate::transaction::Transaction;
use crate::Client;
use std::collections::HashMap;

/// The AppState is a placeholder for the application or database state.
/// It implements the ClientTransactionHandler Trait which acts like an api to the
/// core logic of the program.
pub struct AppState {
    transactions: HashMap<u32, Transaction>,
    clients: HashMap<u32, Client>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
            clients: HashMap::new(),
        }
    }

    /// Adds a transaction to the internal transaction map.
    fn log_transaction(&mut self, t: Transaction) -> Result<(), String> {
        if self.transactions.contains_key(&t.id()) {
            Err("Transaction exists already".to_string())
        } else {
            self.transactions.insert(t.id(), t);
            Ok(())
        }
    }

    /// Parses the transaction type and reacts appropriatly.
    fn handle_transaction(&mut self, t: &Transaction) -> Result<(), String> {
        if !self.clients.contains_key(&t.client_id()) {
            self.clients
                .insert(t.client_id(), Client::from_id(t.client_id()));
        }

        /// TODO: Parse and handle transaction type.
        Ok(())
    }

    fn dispute_transaction(&mut self, id: u32) -> Result<(), String> {
        if let Some(t) = self.transactions.get_mut(&id) {
            t.dispute()?;
            if let Some(c) = self.clients.get_mut(&t.client_id()) {
                // TODO: dispute amount of transaction in client
                Ok(())
            } else {
                Err("Client does not exist".to_string())
            }
        } else {
            Err("Transaction does not exist".to_string())
        }
    }
}

impl ClientTransactionHandler for AppState {
    fn add_transaction(&mut self, t: Transaction) -> Result<(), String> {
        self.handle_transaction(&t)?;
        self.log_transaction(t)?;
        Ok(())
    }

    fn clients(&self) -> HashMap<u32, Client> {
        // TODO: maybe use iterator over clients as return value instead
        self.clients.clone()
    }
}
