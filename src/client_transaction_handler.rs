use crate::errors::TransactionError;
use crate::transaction::{Transaction, TxType};
use crate::Client;
use std::collections::HashMap;

/// The ClientTransactionHandler implements the core logic of the jellyfish engine.
/// it handles transactions and updates client objects according to the requirements.
pub struct ClientTransactionHandler {
    transactions: HashMap<u32, Transaction>,
    clients: HashMap<u16, Client>,
}

impl ClientTransactionHandler {
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
            clients: HashMap::new(),
        }
    }

    /// Adds a transaction to the internal transaction map.
    fn log_transaction(&mut self, t: Transaction) -> Result<(), TransactionError> {
        if let std::collections::hash_map::Entry::Vacant(e) = self.transactions.entry(t.id()) {
            e.insert(t);
            Ok(())
        } else {
            Err(TransactionError::TransactionExistsAlready)
        }
    }

    /// Parses the transaction type and reacts appropriately.
    pub fn add_transaction(&mut self, t: Transaction) -> Result<(), TransactionError> {
        // Create client if it does not exist yet

        self.clients
            .entry(t.client_id())
            .or_insert_with(|| Client::from_id(t.client_id()));

        let client = self
            .clients
            .get_mut(&t.client_id())
            .ok_or(TransactionError::ClientDoesNotExist)?;

        match t.tx_type()? {
            TxType::Deposit => {
                let amount = t.amount().unwrap();
                client.deposit(amount)?;
                self.log_transaction(t)?;
                Ok(())
            }
            TxType::Withdrawal => {
                let amount = t.amount().unwrap();
                client.withdraw(amount)?;
                self.log_transaction(t)?;
                Ok(())
            }
            TxType::Dispute => {
                self.dispute_transaction(t.id())?;
                Ok(())
            }
            TxType::Resolve => {
                self.resolve_transaction(t.id())?;
                Ok(())
            }
            TxType::Chargeback => {
                self.chargeback_transaction(t.id())?;
                Ok(())
            }
        }
    }

    fn dispute_transaction(&mut self, id: u32) -> Result<(), TransactionError> {
        let tx = self
            .transactions
            .get_mut(&id)
            .ok_or(TransactionError::InvalidDispute)?;

        let client = self
            .clients
            .get_mut(&tx.client_id())
            .ok_or(TransactionError::InvalidDispute)?;

        tx.dispute()?;

        client.dispute(
            tx.amount()
                .ok_or(TransactionError::InvalidTransactionRecord)? as f64,
        )?;
        Ok(())
    }

    fn resolve_transaction(&mut self, id: u32) -> Result<(), TransactionError> {
        let tx = self
            .transactions
            .get_mut(&id)
            .ok_or(TransactionError::InvalidResolve)?;

        let client = self
            .clients
            .get_mut(&tx.client_id())
            .ok_or(TransactionError::InvalidResolve)?;
        tx.resolve()?;

        client.resolve(
            tx.amount()
                .ok_or(TransactionError::InvalidTransactionRecord)? as f64,
        )?;
        Ok(())
    }

    fn chargeback_transaction(&mut self, id: u32) -> Result<(), TransactionError> {
        let tx = self
            .transactions
            .get_mut(&id)
            .ok_or(TransactionError::InvalidResolve)?;

        let client = self
            .clients
            .get_mut(&tx.client_id())
            .ok_or(TransactionError::InvalidResolve)?;
        tx.resolve()?;
        tx.chargeback()?;

        client.chargeback(
            tx.amount()
                .ok_or(TransactionError::InvalidTransactionRecord)? as f64,
        )?;
        Ok(())
    }

    pub fn clients(&self) -> &HashMap<u16, Client> {
        // TODO: maybe use iterator over clients as return value instead
        &self.clients
    }
}

#[cfg(test)]
mod tests {
    use super::ClientTransactionHandler;
    use crate::transaction::{Transaction, TxType};

    #[test]
    fn it_adds_transactions_to_the_log() {
        let mut handler = ClientTransactionHandler::new();
        let tx_id = 2;
        let tx_type = (TxType::Deposit).to_string().to_ascii_lowercase();
        let t = Transaction::new(tx_type, 1, tx_id, Some(1.0));
        assert!(!handler.transactions.contains_key(&tx_id));
        handler.add_transaction(t.clone()).unwrap();
        assert!(handler.transactions.contains_key(&2));
        assert_eq!(handler.transactions.get(&2).unwrap(), &t);
    }

    #[test]
    fn it_creates_a_client_when_it_does_not_exist() {
        let mut handler = ClientTransactionHandler::new();
        let client_id = 1;
        let tx_type = (TxType::Deposit).to_string().to_ascii_lowercase();
        let t = Transaction::new(tx_type, client_id, 2, Some(1.0));
        assert!(!handler.clients().contains_key(&client_id));
        handler.add_transaction(t).unwrap();
        assert!(handler.clients().contains_key(&client_id));
    }

    #[test]
    fn it_deposits_the_given_amount_to_a_clients_account() {
        let mut handler = ClientTransactionHandler::new();
        let client_id = 1;
        let tx_type = (TxType::Deposit).to_string().to_ascii_lowercase();
        let t = Transaction::new(tx_type, client_id, 2, Some(1.0));
        handler.add_transaction(t).unwrap();
        assert_eq!(handler.clients().get(&client_id).unwrap().total(), 1.0);
    }

    #[test]
    fn it_withdraws_the_given_amount_from_a_clients_account() {
        let mut handler = ClientTransactionHandler::new();
        let client_id = 1;
        // create client by adding a deposit transaction
        let tx_type = (TxType::Deposit).to_string().to_ascii_lowercase();
        let t = Transaction::new(tx_type, client_id, 2, Some(1.0));
        handler.add_transaction(t).unwrap();

        // withdraw same amount
        let tx_type = (TxType::Withdrawal).to_string().to_ascii_lowercase();
        let t = Transaction::new(tx_type, client_id, 3, Some(1.0));
        handler.add_transaction(t).unwrap();

        assert_eq!(handler.clients().get(&client_id).unwrap().total(), 0.0);
    }

    #[test]
    fn withdrawal_fails_if_account_has_not_enough_funds() {
        let mut handler = ClientTransactionHandler::new();
        let client_id = 1;
        // create client by adding a deposit transaction
        let tx_type = (TxType::Deposit).to_string().to_ascii_lowercase();
        let t = Transaction::new(tx_type, client_id, 2, Some(1.0));
        handler.add_transaction(t).unwrap();

        // withdraw same amount
        let tx_type = (TxType::Withdrawal).to_string().to_ascii_lowercase();
        let t = Transaction::new(tx_type, client_id, 3, Some(2.0));
        assert!(handler.add_transaction(t).is_err());

        // the client balance is unchanged afterwards
        assert_eq!(handler.clients().get(&client_id).unwrap().total(), 1.0);
    }

    #[test]
    fn an_existing_transaction_can_be_disputed() {
        let mut handler = ClientTransactionHandler::new();
        let client_id = 1;
        let tx_id = 2;
        // create client by adding a deposit transaction
        let tx_type = (TxType::Deposit).to_string().to_ascii_lowercase();
        let t = Transaction::new(tx_type, client_id, tx_id, Some(1.0));
        handler.add_transaction(t).unwrap();

        let tx_type = (TxType::Dispute).to_string().to_ascii_lowercase();
        let t = Transaction::new(tx_type, client_id, tx_id, None);
        handler.add_transaction(t).unwrap();

        assert!(handler.transactions.get(&tx_id).unwrap().disputed());
        assert_eq!(handler.clients().get(&client_id).unwrap().total(), 1.0);
        assert_eq!(handler.clients().get(&client_id).unwrap().held(), 1.0);
        assert_eq!(handler.clients().get(&client_id).unwrap().available(), 0.0);
    }

    #[test]
    fn disputing_non_existant_transactions_fails() {
        let mut handler = ClientTransactionHandler::new();
        let client_id = 1;
        let tx_id = 2;
        // create client by adding a deposit transaction
        let tx_type = (TxType::Dispute).to_string().to_ascii_lowercase();
        let t = Transaction::new(tx_type, client_id, tx_id, None);
        assert!(handler.add_transaction(t).is_err());
    }

    #[test]
    fn an_existing_dispute_can_be_resolved() {
        let mut handler = ClientTransactionHandler::new();
        let client_id = 1;
        let tx_id = 2;
        // create client by adding a deposit transaction
        let tx_type = (TxType::Deposit).to_string().to_ascii_lowercase();
        let t = Transaction::new(tx_type, client_id, tx_id, Some(1.0));
        handler.add_transaction(t).unwrap();

        let tx_type = (TxType::Dispute).to_string().to_ascii_lowercase();
        let t = Transaction::new(tx_type, client_id, tx_id, None);
        handler.add_transaction(t).unwrap();

        let tx_type = (TxType::Resolve).to_string().to_ascii_lowercase();
        let t = Transaction::new(tx_type, client_id, tx_id, None);
        handler.add_transaction(t).unwrap();

        assert!(!handler.transactions.get(&tx_id).unwrap().disputed());
        assert_eq!(handler.clients().get(&client_id).unwrap().total(), 1.0);
        assert_eq!(handler.clients().get(&client_id).unwrap().held(), 0.0);
        assert_eq!(handler.clients().get(&client_id).unwrap().available(), 1.0);
    }

    #[test]
    fn resolving_non_existant_disputes_fails() {
        // The Error can be ignored according to the requirements,
        // but its consistent, to always report failure through errors.

        let mut handler = ClientTransactionHandler::new();
        let client_id = 1;
        let tx_id = 2;
        // create client by adding a deposit transaction
        let tx_type = (TxType::Deposit).to_string().to_ascii_lowercase();
        let t = Transaction::new(tx_type, client_id, tx_id, Some(1.0));
        handler.add_transaction(t).unwrap();

        let tx_type = (TxType::Resolve).to_string().to_ascii_lowercase();
        let t = Transaction::new(tx_type, client_id, tx_id, None);
        assert!(handler.add_transaction(t).is_err());
    }

    #[test]
    fn an_existing_dispute_can_be_charged_back() {
        let mut handler = ClientTransactionHandler::new();
        let client_id = 1;
        let tx_id = 2;
        // create client by adding a deposit transaction
        let tx_type = (TxType::Deposit).to_string().to_ascii_lowercase();
        let t = Transaction::new(tx_type, client_id, tx_id, Some(1.0));
        handler.add_transaction(t).unwrap();

        let tx_type = (TxType::Dispute).to_string().to_ascii_lowercase();
        let t = Transaction::new(tx_type, client_id, tx_id, None);
        handler.add_transaction(t).unwrap();

        let tx_type = (TxType::Chargeback).to_string().to_ascii_lowercase();
        let t = Transaction::new(tx_type, client_id, tx_id, None);
        handler.add_transaction(t).unwrap();

        assert!(!handler.transactions.get(&tx_id).unwrap().disputed());
        assert!(handler.transactions.get(&tx_id).unwrap().charged_back());
        assert_eq!(handler.clients().get(&client_id).unwrap().total(), 0.0);
        assert_eq!(handler.clients().get(&client_id).unwrap().held(), 0.0);
        assert_eq!(handler.clients().get(&client_id).unwrap().available(), 0.0);
    }
}
