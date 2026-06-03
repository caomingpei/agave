//! Creates the initial empty EpochRewards sysvar
use {
    solana_account::AccountSharedData,
    solana_genesis_config::GenesisConfig,
    solana_sdk_ids::sysvar,
    solana_sysvar::{
        epoch_rewards::{self, EpochRewards},
        SysvarSerialize,
    },
    std::sync::Arc,
};

pub fn add_genesis_account(genesis_config: &mut GenesisConfig) -> u64 {
    let data = vec![0; EpochRewards::size_of()];
    let lamports = std::cmp::max(genesis_config.rent.minimum_balance(data.len()), 1);

    let account = AccountSharedData::create_from_existing_shared_data(
        lamports,
        Arc::new(data),
        sysvar::id(),
        false,
        u64::MAX,
    );

    genesis_config.add_account(epoch_rewards::id(), account);

    lamports
}
