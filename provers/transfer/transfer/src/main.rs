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

use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Executor, ExecutorEnv, SegmentReceipt, SessionReceipt,
};

use subxt::{
	config::WithExtrinsicParams,
	ext::{
		sp_core::{sr25519::Pair as SubxtPair, Pair as SubxtPairT},
		sp_runtime::{AccountId32, MultiAddress},
	},
	SubstrateConfig,
	tx::{BaseExtrinsicParams, PairSigner, PlainTip},
	OnlineClient, PolkadotConfig,
};

// // Runtime types, etc
#[subxt::subxt(runtime_metadata_path = "./metadata.scale")]
pub mod substrate_node {}

use crate::substrate_node::runtime_types::{
    frame_system::AccountInfo,
    pallet_balances::AccountData
};


type ApiType = OnlineClient<WithExtrinsicParams<SubstrateConfig, BaseExtrinsicParams<SubstrateConfig, PlainTip>>>;

fn alice() -> subxt::ext::sp_core::sr25519::Pair  {
    SubxtPair::from_string("0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a", None).unwrap()
}

fn bob() -> subxt::ext::sp_core::sr25519::Pair  {
    SubxtPair::from_string("0x398f0c28f98885e046333d4a41c19cee4c37368a9832c6502f6cfd182e2aef89", None).unwrap()
}

async fn account_query(api: &ApiType, account: AccountId32)  -> Result<Option<AccountInfo<u32, AccountData<u128>>>, subxt::Error> {
    let query = substrate_node::storage().system().account(&account);
    let query_result = api.storage().fetch(&query, None).await;
	query_result
}

#[tokio::main]
async fn main() {
    let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();

    // Retrieve alice and bob balances as they will serve as our sender and recipient, respectively
    let alice_result = account_query(&api, alice().public().into()).await;
    let bob_result = account_query(&api, bob().public().into()).await;

    let alice_free_balance = alice_result.unwrap().unwrap().data.free;
    let bob_free_balance = bob_result.unwrap().unwrap().data.free;

    let transfer_amount = 500_u128;

    println!("sender balance: {:?} recipient balance {:?}", alice_free_balance, bob_free_balance);
    let (receipt, _) = transfer(
        alice_free_balance,
        bob_free_balance,
        transfer_amount
    );

    // Verify receipt, panic if it's wrong
    receipt.verify(TRANSFER_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );

    // TODO: Below needs update to use changes to receipts in 0.14.0
    let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
    let restored_key = SubxtPair::from_string("0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a", None).unwrap();
    let signer = PairSigner::new(restored_key);

    println!("transfer id {:?}", TRANSFER_ID);

    // The segment receipts that SCALE can understand
    let substrate_session_receipt = receipt.segments.into_iter().map(| SegmentReceipt { seal, index }| {
        (seal, index)
    }).collect();

    println!("Sending tx");
    api
        .tx()
        .sign_and_submit_then_watch_default(
            &substrate_node::tx().template_module().rollup_transfer(
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
fn transfer(sender: u128, recipient: u128, transfer_amount: u128) -> (
    SessionReceipt,
    ([u8; 16], [u8; 16], [u8; 16], [u8; 16])
) {
    let env = ExecutorEnv::builder()
        // TODO: Figure out how to end u128s to guest here
        .add_input(&to_vec(&sender.to_be_bytes()).unwrap())
        .add_input(&to_vec(&recipient.to_be_bytes()).unwrap())
        .add_input(&to_vec(&500_u128.to_be_bytes()).unwrap())
        .build();

    // First, we make an executor, loading the 'multiply' ELF binary.
    let mut exec = Executor::from_elf(env, TRANSFER_ELF).unwrap();

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();

    let c: ([u8; 16], [u8; 16], [u8; 16], [u8; 16]) = from_slice(&receipt.journal).expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );

    let sender_result = u128::from_be_bytes(c.1);
    let recipient_result = u128::from_be_bytes(c.3);

    // Print an assertion
    println!("Transfer result sender {:?}, recipient: {:?}", sender_result, recipient_result);

    (receipt, c)
}
