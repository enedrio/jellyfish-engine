use std::io::Read;

mod app_state;
mod client;
mod client_transaction_handler;
mod transaction;

use client::Client;
use transaction::Transaction;

use app_state::AppState;

use crate::client_transaction_handler::ClientTransactionHandler;

fn main() -> Result<(), csv::Error> {
    let file_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "data.csv".to_string());
    let mut app_state = AppState::new();
    let mut file = std::fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut reader = csv::Reader::from_reader(contents.as_bytes());

    for transaction in reader.deserialize() {
        let transaction: Transaction = transaction?;
        // println!("{}", &transaction);
        // check if transaction client exists, create client if not
        app_state
            .add_transaction(transaction)
            .expect("Something went wrong while adding the transaction");
    }
    println!("client,available,held,total,locked");
    for client in app_state.clients().values() {
        println!("{}", client.as_csv());
    }

    Ok(())
}
