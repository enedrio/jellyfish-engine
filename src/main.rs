use std::io::Read;

mod client;
mod client_transaction_handler;
mod transaction;

use client::Client;
use client_transaction_handler::ClientTransactionHandler;
use transaction::Transaction;

use csv::{ReaderBuilder, Trim};

fn read_csv_from_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_transactions(csv_data: String) -> Result<(), csv::Error> {
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(csv_data.as_bytes());

    let mut handler = ClientTransactionHandler::new();
    for transaction in reader.deserialize() {
        handler
            .add_transaction(transaction?)
            .expect("Something went wrong while adding the transaction");
    }

    // quick hack for client csv output
    println!("client,available,held,total,locked");
    for client in handler.clients().values() {
        println!("{}", client.as_csv());
    }
    Ok(())
}

fn main() -> Result<(), csv::Error> {
    let file_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "data.csv".to_string());
    // TODO: Replace this unwrap_or_else with a sensible error message.
    let csv_data = read_csv_from_file(&file_path)?;
    parse_transactions(csv_data)?;
    Ok(())
}
