// Reduce noise during dev
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_assignments)]

use bc_demos::ethers_utils::*;
use bc_demos::imports::*;
use bc_demos::utils::tracing_utils::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing();

    let env_vec: HashMap<String, String> = dotenvy::vars().collect();

    let (client, provider) = get_signer_to_external_rpc(
        env_vec.get("SIGNER_PRIVATE_KEY_1").unwrap(),
        env_vec.get("BLAST_ARBITRUM_TESTNET_RPC").unwrap(),
    )
    .await?;

    let client = std::sync::Arc::new(client);
    // let wei_balance = provider.get_balance(client.address(), None).await?;
    // info!(?wei_balance);

    let tx_request = Eip1559TransactionRequest::new()
        .to(H160::zero())
        .data(Bytes::from_static(b"0xSpleen says hi to all"));

    let receipt = client
        .send_transaction(tx_request, None)
        .await?
        .log_msg("pending message")
        .interval(std::time::Duration::from_millis(500))
        .confirmations(2)
        .await?
        .ok_or(anyhow!("tx_dropped"))?;
    info!(?receipt);
    // let receipt_deserialized = serde_json::to_string(&receipt).expect("Failed to deserialize");
    // info!(receipt_deserialized);
    Ok(())
}
