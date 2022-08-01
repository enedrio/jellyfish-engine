use std::io::{self, Read, Write};

mod client;
mod client_transaction_handler;
mod errors;
mod transaction;

use client::Client;
use client_transaction_handler::ClientTransactionHandler;

use csv::{ReaderBuilder, Trim};

fn read_csv_from_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_transactions(
    csv_data: String,
    handler: &mut ClientTransactionHandler,
) -> Result<(), csv::Error> {
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(csv_data.as_bytes());

    for transaction in reader.deserialize() {
        let result = handler.add_transaction(transaction?);
        if let Err(err) = result {
            log::error!("{}", err);
        }
    }
    Ok(())
}

fn output_clients_to_stdout(handler: &ClientTransactionHandler) -> Result<(), csv::Error> {
    let mut wtr = csv::Writer::from_writer(vec![]);
    for client in handler.clients().values() {
        wtr.serialize(client)?;
    }

    let data = String::from_utf8(wtr.into_inner().expect("Failed to flush the csv writer"))
        .expect("Invalid Utf-8 output from csv writer");

    io::stdout().write_all(data.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), csv::Error> {
    env_logger::init();
    let file_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "data.csv".to_string());
    let mut handler = ClientTransactionHandler::new();
    let csv_data = read_csv_from_file(&file_path)?;
    parse_transactions(csv_data, &mut handler)?;
    output_clients_to_stdout(&handler)?;
    Ok(())
}
