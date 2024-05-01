// The stable store Size for Wallet
pub const WALLET_SIZE: u64 = 128;

// The stable store Size for Staking record
pub const STAKING_RECORD_SIZE: u64 = 128;

pub const METADATA_SIZE: u64 = 4096;

/// A bitcoin Transaction size will be much than 1m, so we set its stable store size limit to 2m
pub const TRANSACTION_LOG_SIZE: u64 = 2_048_000;

/// The daily limit for a wallet in satoshi
pub const DAILY_LIMIET_SATOSHI: u64 = 10_000_000_000;
