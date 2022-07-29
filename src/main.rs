use std::collections::HashMap;
use std::io::Read;

mod client;
mod transaction;

use client::Client;
use transaction::Transaction;

fn main() -> Result<(), csv::Error> {
    let mut transactions = HashMap::new();
    let mut clients = HashMap::new();

    let mut file = std::fs::File::open("data.csv")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut reader = csv::Reader::from_reader(contents.as_bytes());

    for transaction in reader.deserialize() {
        let transaction: Transaction = transaction?;
        println!("{}", &transaction);
        // check if transaction client exists, create client if not
        if !clients.contains_key(&transaction.client_id()) {
            clients.insert(
                transaction.client_id(),
                Client::from_id(transaction.client_id()),
            );
        }

        // process transaction

        // and log it for possible dispute
        transactions.insert(transaction.id(), transaction);
    }
    for client in clients.values() {
        println!("{}", client);
    }

    Ok(())
}
