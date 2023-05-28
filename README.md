# Risc-roll
### "Trustlessly offload computations of Substrate chains with zk proofs, without the zk learning curve"

This repository serves as a Proof of Concept (POC) for a prover which can serve as a zkrollup of a Substrate-based chain.  Our goal is to explore the possibilities of enabling Substrate/rust developers to leverage zk proofs generally, and not necessarily within the context of rollups, *without the difficulty of circuit design*, and at any level throughout the Substrate stack.

We try to exemplify this through a zk-prover of an extrinsic: balance transfers, for use in a zkrollup. Substrate developers are in a unique position to selectively roll up parts of their chain and enshrine the rollup results simply and in pasteable code similar to the pallet that we show due to Substrate's modularity. In the future, it may be possible to rollup any substrate pallet through some process that generates the appropriate guest code for it. In lieau of such a process, we show that this is a simple process to perform manually.

*This project was developed over the course of one month at the 2023 Consensus Web3athon*

## Goals
Our project has the following objectives:

- Empowering Substrate/Rust Developers: We want to provide an alternative approach for Substrate developers to build zk rollups using the familiar Rust programming language, rather than dealing with circuit design.

- Understanding Risc0 and Rollup Architecture: Through this project, we will dive deep into the Risc0 technology and explore its potential for building robust and scalable rollup systems.

- Performance Evaluation: We will evaluate the performance of Risc0 proving for various computations and assess whether it meets the requirements for processing Substrate extrinsics efficiently.

## Scope
To maintain focus and deliver a meaningful POC, our project has the following scope:

- No Custom VM or Substrate Executor
  - Some zk rollups implement the VM of their chain for a number of reasons. We have a few reasons for not doing this, one of them being hacakthon scope. There is value instead in a different approach of emulating pallet code in the guest, which is the approach we've taken here.
- For the hackathon scope, we did not include a sequencer to this project, and without that, the transactions are front-runnable
- This is **not** production ready, and makes no claims to be a proper rollup.
- The project does not use recursive proofs, as this is not currently supported in Risc0. Support for this will come soon, and we plan to implement it when ready.
- Only one extrinsic(transaction) type is supported: balance transfers. Later, we could develop functionality to generate guest code out of Substrate pallets.
- There is no prover or sequencer network as of the time of writing.

## Project Overview
Our project comprises the following components:

- Prover: This component onchain state and requested transactions as input, computes the transactions, and outputs the STARK proof and updated state. As of this writing, the onchain state consists of a set of account balances, the requested transactions constist of a set of balance transfers.

  The input is given via local json file: `transactions.json`. The host-side of the prover receives the transactions and verifies the signatures associated with each.

  The transaction processing and proving occurs inside of Risc0's zkvm. The output consists of a Risc0 `journal` and `receipt`. The `receipt` contains the STARK proof, and the `journal` contains values we've committed to, i.e. our updated onchain state values.

  Finally, the host receives the `receipt` and `journal` from the Risc0 zkvm, constructs a transaction for the custom pallet with them, and sends it to a locally running Substrate node which has the custom pallet, via `Subxt`.

  You can find this code in `./provers/transfer`. The code which runs inside of the Risc0 zkvm/guest can be found in `./provers/transfer/methods/guest/src/bin/transfer.rs`

- Substrate Node with Verification Pallet: This is a substrate template node with a custom verification pallet. The verification pallet is responsible for validating the STARK proof sent by the prover, ensuring the integrity and correctness of the computed transfers, and then updating the chain's state in totality with the results included with the proof.

The typical Substrate structure of `./node`, `./runtime`, and `./pallets` exists in this project. The custom pallet can be found in `./pallets/template`

![architecture](https://github.com/justinFrevert/risc-roll/assets/81839854/e799dcd9-2a38-4063-b605-f5f294e1d7f5)

## Performance
Some initial testing(not actual benchmarks)
Macbook i7 16GB RAM(without `metal` feature)
- 3 transfer extrinsics: 12 secs
- 30 transfer extrinsics: 21 secs
- 50 transfer extrinsics: 28 secs

## Development
Local development is based around the main workflow of sending proofs to the locally running Substrate node.

### Pre-requisites
Install Rust
Substrate

### Start local node
1. In the root of the directory:
```shell
cargo build --release
```
2. Start the node with local dev options:
```shell
./target/release/node-template --dev
```

### Run Prover
1. Navigate to prover directory
```shell
cd ./provers/transfer
```
2. Build:
```shell
cargo build --release
```
3. Run prover
```shell
../../target/release/prover-host
```

The prover will prove the transactions in `./provers/transfer/transactions.json`, send the proofs to the chain, which will verify and change the balances state, if the proof is verified.

When making changes: ensure you keep the image id and subxt metadata up-to-date to avoid errors. See `provers/transfer/README.md`
