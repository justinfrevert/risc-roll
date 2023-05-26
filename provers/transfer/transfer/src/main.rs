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
use input::{process_json_file, TransactionInput};
use methods::{TRANSFER_ELF, TRANSFER_ID};
use sp_core::Pair as Pairt;
use risc0_zkvm::{
    serde::to_vec,
    Executor, ExecutorEnv, SegmentReceipt, SessionReceipt,
};
use subxt::{
	config::WithExtrinsicParams,
	ext::{
		sp_core::{
			sr25519::{Pair as SubxtPair, Public, Signature},
			Pair as SubxtPairT,
		},
		sp_runtime::{AccountId32, traits::Verify},
	},
	tx::{BaseExtrinsicParams, PairSigner, PlainTip},
	OnlineClient, PolkadotConfig, SubstrateConfig,
};
use codec::{Encode, Decode};
use std::time::Instant;

// // Runtime types, etc
#[subxt::subxt(runtime_metadata_path = "./metadata.scale")]
pub mod substrate_node {}

use crate::substrate_node::runtime_types::{
	frame_system::AccountInfo, pallet_balances::AccountData,
};

type ApiType = OnlineClient<
	WithExtrinsicParams<SubstrateConfig, BaseExtrinsicParams<SubstrateConfig, PlainTip>>,
>;

// TODO: should be the signed bytes of the call being made
const MESSAGE: &str = "hello";

async fn account_query(api: &ApiType, account: AccountId32)  -> Result<Option<AccountInfo<u32, AccountData<u128>>>, subxt::Error> {
    let query = substrate_node::storage().system().account(&account);
    let query_result = api.storage().fetch(&query, None).await;    
	query_result
}

#[tokio::main]
async fn main() {
    let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();

    println!("Preparing transactions...");
    let transfers = process_json_file();

    if transfers.is_empty() {
        panic!("Transactions must not be empty!");
    }

	let signatures_valid = transfers.iter().all(| TransactionInput {sender, recipient, amount, signature}| {
		let is_valid = Signature::verify(&signature, MESSAGE.as_bytes(), &sender);

		if !is_valid {
			println!("Could not verify signature for sender: {:?} recipient: {:?}, balance: {:?} ",
			sender,
			recipient,
			amount)
		}
		is_valid
	});

    if !signatures_valid {
        panic!("Invalid signatures; could not process transactions");
    }

    let mut accounts_set = std::collections::HashSet::new();

    // Check all transfers and get list of accounts we need to check balances for(any account involved here)
    transfers.iter().for_each(| TransactionInput { sender, recipient, .. }| {
        accounts_set.insert(sender);
        accounts_set.insert(recipient);
    });

    // Fill balances
    let mut balances = vec![];
    // On-chain representation of accounts
    let mut accounts_decoded = vec![];

    for account in accounts_set.clone() {
        let account: AccountId32 = account.clone().into();
        accounts_decoded.push(account.clone());
        let balance_query_result = account_query(&api, account.clone()).await;
        let free_balance = balance_query_result.unwrap().map_or(0, |balance| balance.data.free);
        balances.push(free_balance);
    }

    let accounts_set: Vec<Public> = accounts_set.into_iter().map(|p| *p).collect();

    // Avoid sending the full accounts into the vm, we'll just look them up based on the order of balances
    let transfers_with_indexed_accounts = transfers.into_iter().map(| TransactionInput { sender, recipient, amount, .. }| {
        let sender_index: usize = accounts_set.clone().into_iter().position(|r| r == sender).unwrap();
        let recipient_index: usize = accounts_set.clone().into_iter().position(|r| r == recipient).unwrap();
        (sender_index, recipient_index, amount.into())
    }).collect();

    let receipt = transfer_batch(
        balances,
        transfers_with_indexed_accounts
    );

    // Verify receipt, panic if it's wrong
    receipt.verify(TRANSFER_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );

    let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
    let restored_key = SubxtPair::from_string("0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a", None).unwrap();
    let signer = PairSigner::new(restored_key);

    println!("transfer id {:?} (if you updated guest, this needs to be pasted into pallet image id)", TRANSFER_ID);

    // The segment receipts that SCALE can understand
    let substrate_session_receipt = receipt.segments.into_iter().map(| SegmentReceipt { seal, index }| {
        (seal, index)
    }).collect();

    println!("Sending tx");
    api
        .tx()
        .sign_and_submit_then_watch_default(
            &substrate_node::tx().template_module().rollup_transfers(
                accounts_decoded,
                substrate_session_receipt,
                receipt.journal
            ),
            &signer
        )
        .await.unwrap()
        .wait_for_finalized()
        .await.unwrap();
    println!("Done");
}

// Compute the transfer inside the zkvm
fn transfer_batch(balances: Vec<u128>, transfers_with_indexed_accounts: Vec<(usize, usize, u128)>) -> SessionReceipt {
    // "compatible" here meaning u128s are converted to bytes for the vm to be able to use
    let compatible_balances: Vec<[u8; 16]> = balances.iter().map(|balance| {
        balance.to_be_bytes()
    }).collect();

    let compatible_transfers_with_indexed_accounts: Vec<(usize, usize, [u8; 16])> = transfers_with_indexed_accounts.into_iter().map(|(sender, recipient, balance)| {
        (sender, recipient, balance.to_be_bytes())
    }).collect();

    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&compatible_balances).unwrap())
        .add_input(&to_vec(&compatible_transfers_with_indexed_accounts).unwrap())
        .build();

    // First, we make an executor, loading the 'multiply' ELF binary.
    let mut exec = Executor::from_elf(env, TRANSFER_ELF).unwrap();

    println!("Now running transfer txes in guest");
    let guest_start_time = Instant::now();
    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();
    let elapsed = guest_start_time.elapsed();
    println!("Guest done proving txes in {:?} sec {:?} ms", elapsed.as_secs(), elapsed.subsec_millis());

    receipt
}

