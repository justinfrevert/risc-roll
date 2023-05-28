use serde::{Deserialize, Serialize};
// use serde::json;
use subxt::ext::{
    sp_core::{
        Pair as PairT,
        sr25519::{Pair, Public, Signature}
    },
};
use codec::{Decode, Encode};

const EXPECTED_FILE_PATH: &str = "./transactions.json";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionInput {
    pub sender: Public,
    pub recipient: Public,
    pub amount: u128,
    pub signature: Signature,
}

#[derive(Deserialize, Debug, Decode, Encode)]
pub struct UnsignedTransactionInput {
    pub sender: Public,
    pub recipient: Public,
    pub amount: u128,
}

// Create a signed transaction for display only. Purely for convenience of local testing
// pub fn display_signature(unsigned_tx: UnsignedTransactionInput, signer: String) {
pub fn display_signature(signer: String, recipient: Public, amount: u128) {
    let pair = Pair::from_string(&signer, None).unwrap();
    // This is a bit of convenience: we'll assume the signer is sender.
    let sender = pair.public();
    let unsigned_tx = UnsignedTransactionInput { sender, recipient, amount };
    let signature = pair.sign(unsigned_tx.encode().as_ref());

    let tx = TransactionInput {
        sender, recipient, signature, amount
    };

    let json_output = serde_json::to_string_pretty(&tx).unwrap();

    println!("{}\n(Hint: Paste this in transactions.json)", json_output);
}

pub fn process_json_file(file_path: String) -> Vec<TransactionInput> {
    let file_contents = std::fs::read_to_string(file_path).expect("Failed to read file");
    let data: Vec<TransactionInput> = serde_json::from_str(&file_contents).expect("Failed to parse JSON");
    data
}