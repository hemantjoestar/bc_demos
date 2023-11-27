// Reduce noise during dev
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_assignments)]
use bc_demos::ethers_utils::*;
use bc_demos::imports::*;
use bc_demos::utils::tracing_utils::*;

// abigen!(SP_ERC20_ARB, "src/bin/abi/SP_ERC20_shanghai.json");
abigen!(SP_ERC20_ARB, "src/bin/abi/SP_ERC20_merge.json");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_vec: HashMap<String, String> = dotenvy::vars().collect();
    tracing();

    // -------------------------
    // Section start: Client RPC
    // -------------------------

    let (client, provider) = get_signer_to_external_rpc(
        env_vec.get("SIGNER_PRIVATE_KEY_1").unwrap(),
        env_vec.get("BLAST_ARBITRUM_TESTNET_RPC").unwrap(),
    )
    .await
    .context("RPC ERROR")?;

    let client = std::sync::Arc::new(client);

    // -------------------------
    // Section end: Client RPC
    // -------------------------

    // -------------------------
    // Section start: Deploy contract
    // -------------------------

    // let (sp_erc20_instance, receipt) = SP_ERC20_ARB::deploy(
    //     std::sync::Arc::clone(&client),
    //     Address::from_str(env_vec.get("SIGNER_PUB_ADDR_1").context("key error")?)
    //         .context("addr error")?,
    // )?
    // .send_with_receipt()
    // .await
    // .context("deploy failed")?;
    // sp_erc20_instance.owner();

    // info!(?receipt.transaction_hash);
    // info!("{}", sp_erc20_instance.address());

    // -------------------------
    // Section end: Deploy contract
    // -------------------------

    // -------------------------
    // Section start: query balance
    // -------------------------

    let sp_erc20_instance = SP_ERC20_ARB::new(
        Address::from_str("0xdd94505D0391d2c87f3bdcBaD6365E12f670CEdE")?,
        std::sync::Arc::clone(&client),
    );

    // dbg!(sp_erc20_instance.abi().functions());
    use ethers::abi::FunctionExt;
    for func in sp_erc20_instance.abi().functions() {
        // println!("{:?}", func.selector());
        println!("{:?}", hex::encode(func.selector()));
    }
    panic!("end");
    // -------------------------
    // Section end: query balance
    // -------------------------

    let balance = sp_erc20_instance
        .balance_of(Address::from_str(
            env_vec.get("SIGNER_PUB_ADDR_1").unwrap(),
        )?)
        .call()
        .await?;
    info!(?balance);
    // -------------------------
    // Section start: simple transaction
    // -------------------------
    //0xdd94505D0391d2c87f3bdcBaD6365E12f670CEdE
    // sp_erc20_instance
    //     .transfer(
    //         Address::from_str(env_vec.get("SIGNER_PUB_ADDR_2").unwrap())?,
    //         Into::<U256>::into(89),
    //     )
    //     .send()
    //     .await?
    //     .log_msg("pending simple transaction")
    //     .interval(std::time::Duration::from_millis(500))
    //     .confirmations(2)
    //     .await?
    //     .ok_or(anyhow!("tx_dropped"))?;

    // -------------------------
    // Section end: simple transaction
    // -------------------------

    // -------------------------
    // Section start: multicall
    // -------------------------

    let random_wallets = get_random_wallets(1);
    let mut vec_calls = Vec::new();
    for wallet in random_wallets.iter() {
        vec_calls.push(sp_erc20_instance.transfer(
            Address::from_str(env_vec.get("SIGNER_PUB_ADDR_2").unwrap())?,
            // wallet.address(),
            U256::from(rand::thread_rng().gen_range(1..2)),
        ));
    }
    let mut multicall = Multicall::new(
        client.clone(),
        Some(H160::from_str(env_vec.get("MULTICALL_3").unwrap())?),
    )
    .await?;
    // multicall.add_calls(true, vec_calls);
    multicall
        .add_call(
            sp_erc20_instance.transfer(
                Address::from_str(env_vec.get("SIGNER_PUB_ADDR_2").unwrap())?,
                Into::<U256>::into(89),
            ),
            false,
        )
        .add_call(
            sp_erc20_instance.transfer(
                Address::from_str(env_vec.get("SIGNER_PUB_ADDR_2").unwrap())?,
                Into::<U256>::into(89),
            ),
            false,
        );
    multicall
        .send()
        .await?
        .log_msg("pending multicall")
        .interval(std::time::Duration::from_millis(500))
        .confirmations(2)
        .await?
        .ok_or(anyhow!("multicall_error"))?;
    let balance = sp_erc20_instance
        .balance_of(Address::from_str(
            env_vec.get("SIGNER_PUB_ADDR_1").unwrap(),
        )?)
        .call()
        .await?;
    info!(?balance);
    // -------------------------
    // Section end: multicall
    // -------------------------
    Ok(())
}
