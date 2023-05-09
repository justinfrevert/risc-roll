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

#![no_main]
#![no_std]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // TODO: Need to figure out how to get this working... can't currently send u128s
    let sender = env::read::<u128>();
    let sender = env::read::<u128>();
    let recipient = env::read::<u128>();
    let transfer_amount = env::read::<u128>();

    let sender_new_balance = sender.checked_sub(transfer_amount);
    if sender_new_balance.is_none() {
        panic!("Insufficient balance to transfer")
    }
    let recipient_new_balance = recipient.checked_add(transfer_amount);
    if recipient_new_balance.is_none() {
        panic!("Recipient overflow")
    }

    env::commit(&sender_new_balance.unwrap().to_le_bytes());
    env::commit(&recipient_new_balance.unwrap().to_le_bytes());
}
