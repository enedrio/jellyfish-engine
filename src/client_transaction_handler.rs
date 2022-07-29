/// This trait is the public interface for one part of the business logic.
/// It allows for a clear separation of io logic from data handling.
/// We don't care about csv from this point on.
use crate::{client::Client, Transaction};
use std::collections::HashMap;

/// Implementors of the ClientTransactionHandler Trait allow
/// to keep track of incoming Transactions and update their
/// client's balances accordingly.
/// New clients get created on their first transaction.
/// Transactions are logged so that they can be disputed,
/// in case an errouneous transaction was created.
pub trait ClientTransactionHandler {
    /// handle an incoming transaction. Update clients according to the transaction type.
    fn add_transaction(&mut self, t: Transaction) -> Result<(), String>;
    fn clients(&self) -> HashMap<u32, Client>;
}
