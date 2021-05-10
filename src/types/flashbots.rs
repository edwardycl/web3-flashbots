use crate::types::{Bytes, H256, U256};
use serde::{Deserialize, Deserializer};

/// Response type for `eth_callBundle` request
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize)]
pub struct CallBundleResponse {
    #[serde(deserialize_with = "u256_from_dec_str")]
    bundle_gas_price: U256,
    bundle_hash: H256,
    #[serde(deserialize_with = "u256_from_dec_str")]
    coinbase_diff: U256,
    #[serde(deserialize_with = "u256_from_dec_str")]
    eth_sent_to_coinbase: U256,
    #[serde(deserialize_with = "u256_from_dec_str")]
    gas_fees: U256,
    results: Vec<TransactionSimulation>,
    state_block_number: u64,
    total_gas_used: u64,
}

/// Transaction simulation result
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize)]
pub struct TransactionSimulation {
    gas_used: u64,
    tx_hash: H256,
    value: Option<Bytes>,
    error: Option<String>,
    revert: Option<String>, // TODO: parse encoded string
}

fn u256_from_dec_str<'de, D>(de: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    U256::from_dec_str(&s).map_err(serde::de::Error::custom)
}
