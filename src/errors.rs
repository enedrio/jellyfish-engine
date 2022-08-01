use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("Could not process because client with id `{0}` is locked")]
    ClientIsLocked(u16),
    #[error("Could not lock client with id `{0}`")]
    ClientLockFailed(u16),
    #[error("Could not unlock client with id `{0}`")]
    ClientUnlockFailed(u16),
    #[error(
        "requested amount ({amount:?}) is not available in client account with id {client_id:?}"
    )]
    AmountNotAvailable { client_id: u16, amount: f64 },
    #[error("requested amount ({amount:?}) is not held in client account with id {client_id:?}")]
    AmountNotHeld { client_id: u16, amount: f64 },
    #[error("client does not exist")]
    ClientDoesNotExist,
    #[error("transaction can't be created because it already exists")]
    TransactionExistsAlready,
    #[error("invalid transaction record")]
    InvalidTransactionRecord,
    #[error("the dispute's transaction or client id is invalid")]
    InvalidDispute,
    #[error("the resolve's transaction or client id is invalid")]
    InvalidResolve,
    #[error("the chargeback's transaction or client id is invalid")]
    InvalidChargeback,
    #[error("ignoring unknown transaction type")]
    UnknownTransactionType,
}
