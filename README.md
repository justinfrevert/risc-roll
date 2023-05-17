# Substrate with Risc0 Web3athon Project

Welcome to the Substrate with Risc0 Web3athon project! This repository serves as a Proof of Concept (POC) for zkrollup-like systems built using Risc0. Our goal is to explore the possibilities of enabling Substrate/rust developers to build zk rollups in rust, eliminating the need for writing circuits. By leveraging the power of Risc0 and Rollup architecture, we aim to create an efficient and scalable solution for Substrate extrinsics.

## Goals

Our project has the following objectives:

- Empowering Substrate/Rust Developers: We want to provide an alternative approach for Substrate developers to build zk rollups using the familiar Rust programming language, rather than dealing with circuit design.

- Understanding Risc0 and Rollup Architecture: Through this project, we will dive deep into the Risc0 technology and explore its potential for building robust and scalable rollup systems.

- Performance Evaluation: We will evaluate the performance of Risc0 proving for various computations and assess whether it meets the requirements for processing Substrate extrinsics efficiently.

## Scope

To maintain focus and deliver a meaningful POC, our project has the following scope:

- No Custom VM or Substrate Executor: We won't be implementing a custom virtual machine or reimagining Substrate's executor for proving arbitrary extrinsics. Instead, our focus is on demonstrating the proving, verification, and balance transfer capabilities.

## Project Overview

Our project comprises the following components:

- Offchain Prover: This module takes a representation of transactions as input and computes the transfers. It generates a proof using the Risc0 technology, allowing for efficient verification. Additionally, it provides the updated balances after the transfers.

- Substrate Node with Verification Pallet: We have developed a Substrate node integrated with a verification pallet. The verification pallet is responsible for validating the STARK proof sent by the offchain prover, ensuring the integrity and correctness of the computed transfers.

![image](https://github.com/justinFrevert/substrate-web3athon-2023/assets/81839854/c84f8819-57a8-46a8-8232-bcab2da2480e)
