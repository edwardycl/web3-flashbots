use crate::types::{Bytes, H256, U256};
use serde::{Deserialize, Deserializer};

/// Response type for `eth_callBundle` request
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize)]
pub struct CallBundleResponse {
    /// Bundle gas price
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub bundle_gas_price: U256,
    /// Bundle hash
    pub bundle_hash: H256,
    /// Coinbase diff (gas fees + eth sent to coinbase)
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub coinbase_diff: U256,
    /// Eth sent to coinbase
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub eth_sent_to_coinbase: U256,
    /// Gas fees in eth
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub gas_fees: U256,
    /// Transaction simulation results
    pub results: Vec<TransactionSimulation>,
    /// State block number used for simulation
    pub state_block_number: u64,
    /// Total gas used
    pub total_gas_used: u64,
}

/// Transaction simulation result
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize)]
pub struct TransactionSimulation {
    /// Gas used
    pub gas_used: u64,
    /// Transaction hash
    pub tx_hash: H256,
    /// Value
    pub value: Option<Bytes>,
    /// Error message (e.g. "execution reverted")
    pub error: Option<String>,
    /// Revert message
    pub revert: Option<String>, // TODO: parse encoded string
}

fn u256_from_dec_str<'de, D>(de: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    U256::from_dec_str(&s).map_err(serde::de::Error::custom)
}
