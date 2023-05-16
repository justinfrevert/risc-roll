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

use methods::{TRANSFER_ELF, TRANSFER_ID};

use risc0_zkvm::{serde::to_vec, Executor, ExecutorEnv, SegmentReceipt, SessionReceipt};

use clap::Parser;
use subxt::{
	config::WithExtrinsicParams,
	ext::{
		sp_core::{
			sr25519::{verify_batch, Pair as SubxtPair, Public, Signature},
			Pair as SubxtPairT,
		},
		sp_runtime::AccountId32,
	},
	tx::{BaseExtrinsicParams, PairSigner, PlainTip},
	OnlineClient, PolkadotConfig, SubstrateConfig,
};

use codec::{Encode, Decode};
use sp_keyring::AccountKeyring;

// // Runtime types, etc
#[subxt::subxt(runtime_metadata_path = "./metadata.scale")]
pub mod substrate_node {}

use crate::substrate_node::runtime_types::{
	frame_system::AccountInfo, pallet_balances::AccountData,
};

type ApiType = OnlineClient<
	WithExtrinsicParams<SubstrateConfig, BaseExtrinsicParams<SubstrateConfig, PlainTip>>,
>;

type ApiType = OnlineClient<WithExtrinsicParams<SubstrateConfig, BaseExtrinsicParams<SubstrateConfig, PlainTip>>>;

async fn account_query(api: &ApiType, account: AccountId32)  -> Result<Option<AccountInfo<u32, AccountData<u128>>>, subxt::Error> {
    let query = substrate_node::storage().system().account(&account);
    let query_result = api.storage().fetch(&query, None).await;    
	query_result
}

#[tokio::main]
async fn main() {
    let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();

    // TODO: get from input instead later
    let transfers = get_from_json();

	transfers.iter().map(|_| {
		let sender_pair =
		SubxtPair::from_seed(&array_bytes::hex2array_unchecked(&args.sender_hex_seed));
		let sender_public_key = sender_pair.public();
		let receiver_public_key = Public::from_raw(array_bytes::hex2array_unchecked(&args.receiver));
		let transfer_amount = args.amount;
		let message: [u8; 12] = array_bytes::hex2array_unchecked(&"68656c6c6f2c20776f726c64");
	
		let signature = Signature::from_raw(array_bytes::hex2array_unchecked(&args.sig));
		if !verify_batch(vec![&message], vec![&signature], vec![&sender_public_key.into()]) {
			panic!("Signature verification failed");
		}
	}).collect();


    // TODO: Maybe later we can send single integers representing accounts into the vm as an index
    let mut accounts_set = std::collections::HashSet::new();

    // Check all transfers and get list of accounts we need to check balances for(any account involved here)
    transfers.iter().for_each(|(sender, recipient, _)| {
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

    // Avoid sending the full accountsi nto the vm, we'll just send an index of each account in `accounts_set`, which is the same order as `balances`.
    // So, the vm can use the indices to lookup account(balance) info from `balances`
    let transfers_with_indexed_accounts = transfers.into_iter().map(|(sender, recipient, balance)| {
        let sender_index: usize = accounts_set.clone().into_iter().position(|r| r == sender).unwrap();
        let recipient_index: usize = accounts_set.clone().into_iter().position(|r| r == recipient).unwrap();
        (sender_index, recipient_index, balance)
    }).collect();

    let receipt = transfer_batch(
        balances,
        transfers_with_indexed_accounts
    );

    // Verify receipt, panic if it's wrong
    receipt.verify(TRANSFER_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );

    // TODO: Below needs update to use changes to receipts in 0.14.0
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
                // alice().public().into(),
                // bob().public().into(),
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
        // TODO: Figure out how to end u128s to guest here
        .add_input(&to_vec(&compatible_balances).unwrap())
        .add_input(&to_vec(&compatible_transfers_with_indexed_accounts).unwrap())
        .build();

    // First, we make an executor, loading the 'multiply' ELF binary.
    let mut exec = Executor::from_elf(env, TRANSFER_ELF).unwrap();

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();
    receipt
}

