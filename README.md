# Jellyfish Engine

## What is this
The Jellyfish Engine 
* reads a series of transactions from an input file in csv format
* updates or creates clients (that only exist for the lifetime of the program)
* and outputs the resulting client "database" as csv to stdout.


## Usage
The engine can be run with 
`cargo run -- test_data.csv > accounts.csv`

## Tests
Unit tests can be run with `cargo test`
An e2e test run can be done with the `test_data.csv`.
It contains example transactions of all kinds.
Just run `cargo run -- test_data.csv > accounts.csv`
and inspect the accounts.csv output file.
There should also appear an error log on the console output,
indicating that there was an invalid transaction.


## Open Questions & Assumptions
I assume that only deposit transactions can be disputed (The Engine logs an error if any other transaction type is disputed).
I furthermore assume that charged back transactions should stay in memory, but marked as charged back.
Another assumption is that the amount that is beeing transfered per transaction is relatively low.
At least low enough so that `f64` has enough precision to handle the arithmetic "correctly".
This could be made more rock solid in another iteration by implementing integer arithemtic 
for the internal calculations.


## Logging
The application sends error logs to `stderr` output.
Have a look at the `errors::TransactionError` enum for possible Errors.

## Safety & Robustness
This line in main.rs:33:
```rust
let data = String::from_utf8(wtr.into_inner().expect("Failed to flush the csv writer"))
        .expect("Invalid Utf-8 output from csv writer");
```
can cause the engine to panic. Since this is very unlikely I left it like this for the first iteration.
It could be improved with a generic error crate like `anyhow` 
to communicate `csv::Error`, `FromUtf8Error`, `IntoInnerError` 
as a single severe Error type or by creating another 
Error Enum with `thiserror` for that case.

All possible errors in the transaction logic are handled and emitted as error logs.
If there is invald csv the main function returns early without sending the updated accounts to the output.
This behaviour could easily be changed by handling the `transaction?` statement in `main.rs:19`.
My assumption was that an invalid csv should not be treated lightly but as a hard error.
Invalid transaction information on the other hand should be logged, 
but the other valid transactions should still happen.


## Efficiency
As far as I understand the docs from the `csv` crate the `csv::Reader` is buffered automatically,
so it should be the same as using BufReader on the file. But I have not tested the memory 
footprint explicitly, yet.
