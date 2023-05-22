# Risc-roll
### "Trustlessly offload computations of Substrate chains with zk proofs, without the zk learning curve"

This repository serves as a Proof of Concept (POC) for *partial* zkrollups of Substrate chains. Our goal is to explore the possibilities of enabling Substrate/rust developers to build zk rollups in *plain rust*, eliminating the need for writing circuits. By leveraging the power of Risc0 and Rollup architecture, we aim to create an efficient and scalable solution for Substrate extrinsics.

*This project was developed over the course of one month at the 2023 Consensus Web3athon*

## Goals
Our project has the following objectives:

- Empowering Substrate/Rust Developers: We want to provide an alternative approach for Substrate developers to build zk rollups using the familiar Rust programming language, rather than dealing with circuit design.

- Understanding Risc0 and Rollup Architecture: Through this project, we will dive deep into the Risc0 technology and explore its potential for building robust and scalable rollup systems.

- Performance Evaluation: We will evaluate the performance of Risc0 proving for various computations and assess whether it meets the requirements for processing Substrate extrinsics efficiently.

## Scope
To maintain focus and deliver a meaningful POC, our project has the following scope:

- No Custom VM or Substrate Executor
  - Most zk rollups implement the VM of their chain for a number of reasons. We have a few reasons for not doing this, one of them being hacakthon scope. Instead, our approach is to execute pallet code or a simplification of a given pallet's code inside the zkvm.
- This is **not** production ready, and makes no claims to be a proper rollup.
- The project does not use recursive proofs, as this is not currently supported in Risc0. Support for this will come soon, and we plan to implement it when ready.
- Only one extrinsic(transaction) type is supported: balance transfers. Later, we could develop functionality to generate guest code out of Substrate pallets.
- There is no prover or sequencer network as of the time of writing.

## Project Overview
Our project comprises the following components:

- Offchain Prover: This component onchain state and requested transactions as input, computes the transactions, and outputs the STARK proof and updated state. As of this writing, the onchain state consists of a set of account balances, the requested transactions constist of a set of balance transfers.

  The input occurs through CLI. This portion receives the transactions and verifies the signatures associated with each.

  The transaction processing and proving occurs inside of Risc0's zkvm. The output consists of a Risc0 `journal` and `receipt`. The `receipt` contains the STARK proof, and the `journal` contains values we've committed to, i.e. our updated onchain state values.

  Finally, the host receives the `receipt` and `journal` from the Risc0 zkvm, constructs a transaction with them, and sends it to a locally running Substrate node, via `Subxt`.

- Substrate Node with Verification Pallet: This is a substrate template node with a custom verification pallet. The verification pallet is responsible for validating the STARK proof sent by the offchain prover, ensuring the integrity and correctness of the computed transfers, and then updating the chain's state in totality with the results included with the proof.

![image](https://github.com/justinFrevert/substrate-web3athon-2023/assets/81839854/c84f8819-57a8-46a8-8232-bcab2da2480e)

## Performance
TBD: requires testing on stronger machine

Macbook i7 16GB RAM
- 3 transfer extrinsics: 12 secs
- 50 transfer extrinsics: 28 secs

## Future Improvements
This project is first and foremost a POC and learning exercise. However, we believe that a more accessible verifiable computing environment will open the door to many new types of projects. We're interested in ways to more deeply ingrain the technology into Substrate tools. The following are ideas we'd like to explore in the future.

- Rollup any pallet
  - Generate Risc0 guest code to rollup (most)any pallet. This would allow substrate developers to freely and selectively offload portions of their chain.
- Faster proving times
  - Through optimizations and natural progression of Risc0's technology the speed of proving transactions will improve.
- Recursive proofs for batch transactions
- Sequencer and RPC endpoint for prover
- Improve verifier
  - Enshrine the rollup fully by trusting the incoming proof in the onchain transaction verification process
  - Improve onchain state update by updating state through a lower-level API.

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
../../target/release/rollup-host
```

