# Transfer Prover
Example for Substrate balances transfer extrinsic executed within Risc0 zkvm guest. This contains simplified code which emulates Substrate balances transfers, which runs within the Risc0 guest. The rest of the code consists of host-code, which:
- Assists in signed transaction creation
- Accepts signed transactons via CLI
- Sends transactions to Risc0 guest
- Retrieves Risc0 journal and receipts, and sends to Substrate pallet via Subxt

## Getting chain metadata
The chain metadata changes with any alterations in the runtime. The chain metadata in `metadata.scale` needs to be updated each time. To do this:

Start local node
run
```shell
subxt metadata -f bytes > metadata.scale
```

## Image ID
The Substrate pallet knows and trusts the image id of this guest. After each change, the image id will need to be updated here `pallets/template/src/common.rs`, and the substrate runtime also rebuilt(run `cargo build --release` in project root). 

## Transactions
In the current state, the transactions available in `./transactions.json` are sent into the guest for proving. Later, this would be passed in after recieving via JSON-RPC.

## Generating transactions to prove
By default, the existing transations in `transations.json` are passed into the node. To generate more, use the signing tool via `sign` subcommand and paste into the `transactions.json` array:
```shell
# Generate a transfer from alice to bob for 500 balance
./target/release/prover-host sign -s //Alice -t 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty -a 500000000000000
# Generate a transfer from bob to charlie for 1000000 balance
./target/release/prover-host sign -s //Bob -t 5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y -a 1000000000000000000
```
The results are not strongly verified, so a transaction can be pasted multiple times for testing convenience