use serde::{Deserialize};
use subxt::ext::{
    sp_core::sr25519::{Public, Signature},
};
use codec::{Decode, Encode};

const EXPECTED_FILE_PATH: &str = "./transactions.json";

#[derive(Deserialize)]
pub struct TransactionInput {
    pub sender: Public,
    pub recipient: Public,
    pub amount: u128,
    pub signature: Signature,
}

pub fn process_json_file() -> Vec<TransactionInput> {
    let file_contents = std::fs::read_to_string(EXPECTED_FILE_PATH).expect("Failed to read file");
    let data: Vec<TransactionInput> = serde_json::from_str(&file_contents).expect("Failed to parse JSON");
    data
}