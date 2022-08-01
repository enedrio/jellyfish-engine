use std::io::{self, Write};

mod client;
mod client_transaction_handler;
mod errors;
mod transaction;

use client::Client;
use client_transaction_handler::ClientTransactionHandler;

use csv::{ReaderBuilder, Trim};

fn parse_transactions<T>(input: T, handler: &mut ClientTransactionHandler) -> Result<(), csv::Error>
where
    T: std::io::Read,
{
    let mut reader = ReaderBuilder::new().trim(Trim::All).from_reader(input);
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
    let file = std::fs::File::open(file_path)?;
    parse_transactions(file, &mut handler)?;
    output_clients_to_stdout(&handler)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_transactions;
    use crate::client_transaction_handler::ClientTransactionHandler;
    #[test]
    fn it_can_handle_white_space_in_csv() {
        let data = "type, client, tx,amount\ndeposit, 1, 1, 1.0\n";
        let mut handler = ClientTransactionHandler::new();
        parse_transactions(data.as_bytes(), &mut handler).unwrap();
        assert_eq!(handler.clients().get(&1).unwrap().total(), 1.0);
    }

    #[test]
    fn it_() {
        let data = "type, client, tx,amount\ndeposit, 1, 1, 1.0\n";
        let mut handler = ClientTransactionHandler::new();
        parse_transactions(data.as_bytes(), &mut handler).unwrap();
        assert_eq!(handler.clients().get(&1).unwrap().total(), 1.0);
    }
}
