use crate::imports::*;
pub use ethers::prelude::*;

pub async fn get_signer_to_external_rpc(
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
pub fn get_random_wallets(count: usize) -> Vec<LocalWallet> {
    let mut vec = Vec::<LocalWallet>::new();
    for _ in 0..count {
        vec.push(LocalWallet::new(&mut rand::thread_rng()));
    }
    vec
}
