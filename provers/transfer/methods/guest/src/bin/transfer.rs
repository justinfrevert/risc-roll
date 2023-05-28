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
use sp_std::vec::Vec;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let balances_bytes = env::read::<Vec<[u8; 16]>>();
    let transfers_with_indexed_accounts_bytes = env::read::<Vec<(usize, usize, [u8; 16])>>();

    let mut balances: Vec<u128> = balances_bytes.clone().into_iter().map(|balance| {
        u128::from_be_bytes(balance)
    }).collect();

    let transfers_with_indexed_accounts: Vec<(usize, usize, u128)> =
        transfers_with_indexed_accounts_bytes.clone().into_iter().map(|(sender_index, recipient_index, balance)| {
            (sender_index, recipient_index, u128::from_be_bytes(balance))
        }).collect();

    transfers_with_indexed_accounts.into_iter().for_each(|(sender_index, recipient_index, transfer_balance)| {
        let sender_balance = balances[sender_index];
        let recipient_balance = balances[recipient_index];

        // TODO: This shouldn't fail on bad transactions, we should take the bad transactions out
        balances[sender_index] = sender_balance.checked_sub(transfer_balance).expect("Insufficient balance for transfer");
        balances[recipient_index] =  recipient_balance.checked_add(transfer_balance).unwrap();
    });

    let new_balances_bytes: Vec<[u8; 16]> = balances.into_iter().map(|b| b.to_be_bytes()).collect();
    
    env::commit(&(
        // Old balances
        balances_bytes,
        // New balances
        new_balances_bytes
    ))
}
