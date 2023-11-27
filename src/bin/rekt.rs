// Reduce noise during dev
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_assignments)]
// Admin Wallet Address : 0x10CdBeab9eABC80A8895C7D7a5d8aaAabCfA1c12
// Admin Private Key : b00712674d0a685a0df16dc11b5479e864980ca2c370e809e4b90b1695f065c0
// Polygon Contract Address : 0xC143D3d0e5D9054eC02d6dD8D4116c6fD5Ef06c6
// BNB Testnet Contract Address : 0xa58683b7473A61E56C2C0eB2912c90F667EDcA50
// Sepolia Testnet Contract Address : 0xa58683b7473A61E56C2C0eB2912c90F667EDcA50
// read, write, multi

use bc_demos::ethers_utils::*;
use bc_demos::imports::*;
use bc_demos::utils::tracing_utils::*;

static POLYGON_ADDRESS: &str = "0xC143D3d0e5D9054eC02d6dD8D4116c6fD5Ef06c6";
static SEPOLIA_ADDRESS: &str = "0xa58683b7473A61E56C2C0eB2912c90F667EDcA50";
static OWNER: &str = "b00712674d0a685a0df16dc11b5479e864980ca2c370e809e4b90b1695f065c0";
use rand::Rng;

abigen!(TREKT, "src/bin/abi/rekt_erc20.json");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // panic!("end");
    let env_vec: HashMap<String, String> = dotenvy::vars().collect();
    tracing();

    let (client, provider) =
        get_signer_to_external_rpc(OWNER, env_vec.get("BLAST_POLYGON_TESTNET_RPC").unwrap())
            .await
            .context("RPC")?;

    let client = std::sync::Arc::new(client);
    let rekt_polygon_instance = TREKT::new(
        Address::from_str(POLYGON_ADDRESS)?,
        std::sync::Arc::clone(&client),
    );

    // Owner verification
    // let owner_pub_derived = OWNER.parse::<LocalWallet>()?.address();
    // info!(?owner_pub_derived);

    // // get owner address from contract
    // let contract_owner = rekt_polygon_instance.owner().call().await?;
    // info!(?contract_owner);
    // ensure!(
    //     owner_pub_derived == contract_owner,
    //     "owner pub addr didnt match"
    // );

    // generate 10 random receiver address
    let random_wallets = get_random_wallets(250);
    // pick one and send tokens too
    let lucky = &random_wallets[rand::thread_rng().gen_range(0..random_wallets.len())];
    //check balance
    let lucky_balance = rekt_polygon_instance
        .balance_of(lucky.address())
        .call()
        .await?;
    info!(?lucky_balance);
    //send tokens

    let whole_amount: u64 = 77;
    let decimals = rekt_polygon_instance.decimals().call().await?;
    let decimal_amount = U256::from(whole_amount) * U256::exp10(decimals as usize);
    // let transfer_receipt = rekt_polygon_instance
    //     .transfer(lucky.address(), Into::<U256>::into(decimal_amount))
    //     .send()
    //     .await?
    //     .log_msg("pending lucky transfer")
    //     .interval(std::time::Duration::from_millis(500))
    //     .confirmations(2)
    //     .await?
    //     .ok_or(anyhow!("tx_dropped"))?;

    // send tokens to all using multicall
    // let mut vec_calls = Vec::new();
    let mut tx_hashes = Vec::new();
    // Fetch the current nonce
    let mut nonce = client.get_transaction_count(client.address(), None).await?;
    for wallet in random_wallets.iter() {
        info! {"nonce = {}",nonce};
        let tx = rekt_polygon_instance
            .transfer(
                wallet.address(),
                U256::from(rand::thread_rng().gen_range(0..1000)),
            )
            .gas_price(U256::from(4) * U256::exp10(9))
            .nonce(nonce); // Set the nonce manually;

        let pending_tx = tx.send().await?;
        tx_hashes.push(pending_tx.tx_hash());
        nonce += 1.into();
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        // vec_calls.push(rekt_polygon_instance.transfer(
        //     wallet.address(),
        //     U256::from(rand::thread_rng().gen_range(0..1000)),
        // ));
    }
    // panic!("stop");

    // let mut multicall = Multicall::new(
    //     client.clone(),
    //     Some(H160::from_str(env_vec.get("MULTICALL_3").unwrap())?),
    // )
    // .await?;
    // multicall.add_calls(false, vec_calls);

    // // let multicall_tx = multicall.send().await?;
    // // info!(?multicall_tx);
    // let multicall_receipt = multicall
    //     .send()
    //     .await?
    //     .log_msg("pending multicall")
    //     .interval(std::time::Duration::from_millis(500))
    //     .confirmations(2)
    //     .await?
    //     .ok_or(anyhow!("multicall_error"))?;
    // info!(?multicall_receipt.transaction_hash);
    // send from one random user to another. requires that the wallet have some native token to do
    // so. chek if have else seed
    // panic!("stop");

    Ok(())
}
