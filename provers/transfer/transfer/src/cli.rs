use clap::{Parser, Subcommand};
use subxt::ext::{
    sp_core::sr25519::Public,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<SubCommand>,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Run code to sign and output desired transactions only. This is basically for local testing, and not a proper way to sequence transactions for passing in.
    Sign {
        #[clap(short, help = "Secret key of sender, which wil also be used to generate account id for \"from\" field")]
        suri: String,
        #[clap(short, help = "Hex string of public key of receiver")]
        to: Public,
        #[clap(short, help = "Transfer amount")]
        amount: u128,
    },
    /// Run the prover
    Run,
}
