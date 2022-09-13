use std::{convert::TryFrom, time::Duration};

use ethers::{
    abi::Address,
    prelude::U256,
    providers::{Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::TransactionRequest,
    utils::Ganache,
};
use eyre::{ContextCompat, Result};
use hex::ToHex;

#[tokio::main]
async fn main() -> Result<()> {
    //Spawn a ganache instance
    let mnemonic = "dog bus oil money legal snowboard cat town snow water house apartment";
    let ganache = Ganache::new().mnemonic(mnemonic).spawn();
    println!("HTTP endpoint: {}", ganache.endpoint());

    // Get first wallet by ganache
    let wallet: LocalWallet = ganache.keys()[0].clone().into();
    let wallet_address = wallet.address();
    println!(
        "Default wallet address: {}",
        wallet_address.encode_hex::<String>()
    );

    // A provider Eth JsonRPC client
    let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));

    //Query balance by address
    let first_balance = provider.get_balance(wallet_address, None).await?;
    println!("Wallet first address balance: {} Eth", first_balance);

    // Create a transaction to transfer 1000 wei to 'other address'
    let other_address = "0xe7E6c88Ad1BAb6508a251B7995f44fB1C5E3dCF7".parse::<Address>()?;
    let tx = TransactionRequest::pay(other_address, U256::from(1000u64)).from(wallet_address);
    //Send tx and wait for receipt
    let receipt = provider
        .send_transaction(tx, None)
        .await?
        .log_msg("Pending transfer")
        .await?
        .context("Missing receipt")?;

    println!(
        "Tx mined in block {}",
        receipt.block_number.context("Cant get block nr")?
    );

    println!(
        "Balance of {} {}",
        other_address,
        provider.get_balance(other_address, None).await?
    );

    Ok(())
}
