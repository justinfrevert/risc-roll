# Substrate with Risc0 Web3athon Project
### "Trustlessly offload computations of Substrate chains with zk proofs, without the zk learning curve"


Welcome to the Substrate with Risc0 Web3athon project! This repository serves as a Proof of Concept (POC) for partial zkrollups of Substrate chains, built using Risc0. Our goal is to explore the possibilities of enabling Substrate/rust developers to build zk rollups in rust, eliminating the need for writing circuits. By leveraging the power of Risc0 and Rollup architecture, we aim to create an efficient and scalable solution for Substrate extrinsics.

## Goals

Our project has the following objectives:

- Empowering Substrate/Rust Developers: We want to provide an alternative approach for Substrate developers to build zk rollups using the familiar Rust programming language, rather than dealing with circuit design.

- Understanding Risc0 and Rollup Architecture: Through this project, we will dive deep into the Risc0 technology and explore its potential for building robust and scalable rollup systems.

- Performance Evaluation: We will evaluate the performance of Risc0 proving for various computations and assess whether it meets the requirements for processing Substrate extrinsics efficiently.

## Scope

To maintain focus and deliver a meaningful POC, our project has the following scope:

- No Custom VM or Substrate Executor: We won't be implementing a custom virtual machine or reimagining Substrate's executor for proving arbitrary extrinsics. Instead, our focus is on demonstrating the proving, verification, and balance transfer capabilities. 
  - A Substrate extrinsic analogue to this approach might be some system or tooling which can convert Substrate pallets or traits to a Risc0 zkvm-friendly format 💡.
- This is not production ready, and makes no claims to be a proper rollup.
- The project does not use recursive proofs, as this is not currently supported in Risc0. 

## Project Overview

Our project comprises the following components:

- Offchain Prover: This module takes a representation of transactions as input and computes the transfers. It generates a proof of these operations. Additionally, it provides the updated balances after the transfers.

- Substrate Node with Verification Pallet: We have developed a Substrate node integrated with a verification pallet. The verification pallet is responsible for validating the STARK proof sent by the offchain prover, ensuring the integrity and correctness of the computed transfers.

The offchain component communicates with the substrate node using Subxt. The primary communications are RPC queries for state information and extrinsic submissions for submitting the proofs + their relevant data.

![image](https://github.com/justinFrevert/substrate-web3athon-2023/assets/81839854/c84f8819-57a8-46a8-8232-bcab2da2480e)


## Performance
The prover performance is promising, given that the Risc0 project is extremely early. They are likely to improve the prover speed significantly as time goes on. 

Macbook i7
- 50 transfer extrinsics: 28 secs

## Future Improvements
As of writing, this project is largely a POC and learning exercise first and foremost. However, we believe that a more accessible verifiable computing environment will open the door to many new types of projects. We're interested in ways to more deeply ingrain the technology into Substrate tools. The following are ideas we'd like to explore in the future

- Rollup any pallet
  - Generate Risc0 guest code to rollup (most)any pallet. This would allow substrate developers to freely and selectively offload heavy portions of their chain.
- Faster proving times
  - Through optimizations and natural progression of Risc0's technology the speed of proving transactions will improve.
