# Substrate with Risc0 Web3athon Project

The project provides a POC for zkrollup-like systems built using Risc0. 

## Goals
- Explore ways to enable Substrate/rust developers to build zk rollups in rust as opposed to writing circuits
- Learn about Risc0 and Rollup architecture
- Learn performance of Risc0 proving of various computations and whether it is sufficient for Substrate extrinsics

## Scope
- There is no custom VM or reimplementation of Substrate's executor to allow proving of arbitrary extrinsics
- The project will only demonstrate proving, verifying, and performing balance transfers

## Overview
The project consists of:
- An offchain prover, which takes some representation of transactions as input, and computes the transfers, outputting a proof as well as the new balances
- A substrate node, with a pallet that performs verification of the STARK proof sent by the prover
