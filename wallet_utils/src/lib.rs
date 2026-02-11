use ethers::{
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, TransactionRequest, U256}
};
use std::{sync::Arc};

pub async fn send_transaction(
    rpc_url: &str,
    private_key: &str,
    to: Address,
    value: U256,
    data: Option<Vec<u8>>
) -> Result<String, Box<dyn std::error::Error>> {
    // provider
    let provider = Provider::<Http>::try_from(rpc_url).unwrap();

    // walet
    let wallet: LocalWallet = private_key.parse().unwrap();
    let wallet = wallet.with_chain_id(11155111u64);

    let client = Arc::new(
        ethers::middleware::SignerMiddleware::new(
            provider,
            wallet,
        )
    );

    let mut tx = TransactionRequest::new().to(to).value(value);
    if let Some(d) = data {
        tx = tx.data(d);
    }

    let pending = client.send_transaction(tx, None).await?;
    let hash = format!("{:#x}", pending.tx_hash());

    Ok(hash)
}