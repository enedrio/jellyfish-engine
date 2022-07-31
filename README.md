# Jellyfish Engine

## What is this
The Jellyfish Engine 
* reads a series of transactions from an input file in csv format
* updates or creates clients (that only exist for the lifetime of the program)
* and outputs the resulting client "database" as csv to stdout.


## Usage

The engine can be run with 
`cargo run -- data.csv`

## Tests
tests can be run with `cargo test`


## Open Questions
I assume that only deposit transactions can be disputed.
I furthermore assume that the transaction should stay in place, but marked as charged back.