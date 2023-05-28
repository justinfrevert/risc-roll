// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod input;
mod cli;
mod tx_prover;

use cli::{Cli, SubCommand::{Sign, Run}};
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Some(Sign { suri, to, amount  }) => {
            // Signer mode for convenient transaction signing
            input::display_signature(suri, to, amount)
        },
        Some(Run { transactions_file_path, .. }) => {
            // Run the code
            tx_prover::prove_transactions(transactions_file_path).await;
        },
        // TODO: I feel like I am not using clap default arguments properly here... I would think there would be a way to avoid to have to do it manually this
        None => {
            tx_prover::prove_transactions("./transactions.json".to_string()).await
        }
    }
}
