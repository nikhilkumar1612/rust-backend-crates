use ethers::{
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet},
    types::{Address, TransactionRequest, U256, H256}
};
use std::{sync::Arc};

pub async fn send_transaction(
    rpc_url: &str,
    private_key: &str,
    to: Address,
    value: U256,
    data: Option<Vec<u8>>
) -> Result<H256, Box<dyn std::error::Error>> {
    // provider
    let provider = Provider::<Http>::try_from(rpc_url).unwrap();

    // walet
    let wallet: LocalWallet = private_key.parse().unwrap();

    let client = Arc::new(
        ethers::middleware::SignerMiddleware::new(
            provider,
            wallet
        )
    );

    let mut tx = TransactionRequest::new().to(to).value(value);
    if let Some(d) = data {
        tx = tx.data(d);
    }

    let pending = client.send_transaction(tx, None).await?;

    let receipt = pending.await?;

    let receipt = receipt.ok_or_else(|| anyhow::anyhow!("tx dropped"))?;

    Ok(receipt.transaction_hash)
}