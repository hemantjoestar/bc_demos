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
async fn main() -> anyhow::Result<()> {
    tracing();
    // dotenv().context(".env file not found")?;
    let env_vec: HashMap<String, String> = dotenvy::vars().collect();
    info!("{}", env_vec.get("SIGNER_PUB_ADDR_1").context("env error")?);
    let data_to_commit = "your_data".to_string().into_bytes();
    let (client, provider) = get_signer_to_external_rpc(
        env_vec.get("SIGNER_PRIVATE_KEY_1").unwrap(),
        env_vec.get("BLAST_ARBITRUM_TESTNET_RPC").unwrap(),
    )
    .await?;
    let client = std::sync::Arc::new(client);
    let wallet: LocalWallet =
        "380eb0f3d505f087e438eca80bc4df9a7faa24f868e69fc0440261a0fc0567dc".parse()?;
    "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646".parse::<Address>()?;

    let wei_balance = provider
        .get_balance(
            client.address(),
            // Address::from_str(),
            // raw str env takes as string and will try ens
            // H160::from_slice(&hex::decode(&std::env::var("SIGNER_PUB_ADDR_1")?)?),
            None,
        )
        .await?;
    info!(?wei_balance);
    // Encode the data you want to commit to the blockchain
    let data_to_commit_hex = hex::encode(data_to_commit);
    Ok(())
}
fn encode_data() {
    todo!()
}
async fn get_signer_to_external_rpc(
    pvt_key: &str,
    rpc_url: &str,
) -> anyhow::Result<(
    SignerMiddleware<Provider<Http>, LocalWallet>,
    Provider<Http>,
)> {
    let provider = Provider::try_from(rpc_url)?.interval(std::time::Duration::from_millis(10u64));
    let client = SignerMiddleware::new(
        provider.clone(),
        LocalWallet::from_str(pvt_key)?.with_chain_id(provider.get_chainid().await?.as_u64()),
    );
    Ok((client, provider))
}
// info!(
//     "{}",
//     env_vec
//         .get("SIGNER_PUB_ADDR_1")
//         .ok_or_else(|| anyhow!("env error"))?
// );
