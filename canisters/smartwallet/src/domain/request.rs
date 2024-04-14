use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct TransferRequest {
    pub addresses: Vec<String>,
    pub amounts: Vec<u64>,
}
