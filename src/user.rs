use crate::utils::deserialize_u256;
use ethers::core::types::U256;
use serde::Deserialize;

/// Represents stats for a searcher.
#[derive(Deserialize, Debug)]
pub struct UserStats {
    /// Whether the searcher is high priority or not.
    pub is_high_priority: bool,
    /// The total amount of payments made to miners.
    #[serde(deserialize_with = "deserialize_u256")]
    pub all_time_miner_payments: U256,
    /// The total amount of gas simulated in bundles.
    #[serde(deserialize_with = "deserialize_u256")]
    pub all_time_gas_simulated: U256,
    /// The total amount of payments made to miners in the last 7 days.
    #[serde(deserialize_with = "deserialize_u256")]
    pub last_7d_miner_payments: U256,
    /// The total amount of gas simulated in bundles the last 7 days.
    #[serde(deserialize_with = "deserialize_u256")]
    pub last_7d_gas_simulated: U256,
    /// The total amount of payments made to miners in the last day.
    #[serde(deserialize_with = "deserialize_u256")]
    pub last_1d_miner_payments: U256,
    /// The total amount of gas simulated in bundles in the last day.
    #[serde(deserialize_with = "deserialize_u256")]
    pub last_1d_gas_simulated: U256,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_stats_deserialize() {
        let user_stats: UserStats = serde_json::from_str(
            r#"{
  "is_high_priority": true,
  "all_time_miner_payments": "1280749594841588639",
  "all_time_gas_simulated": "30049470846",
  "last_7d_miner_payments": "1280749594841588639",
  "last_7d_gas_simulated": "30049470846",
  "last_1d_miner_payments": "142305510537954293",
  "last_1d_gas_simulated": "2731770076"
}"#,
        )
        .unwrap();

        assert_eq!(user_stats.is_high_priority, true);
        assert_eq!(
            user_stats.all_time_miner_payments,
            U256::from_dec_str("1280749594841588639").unwrap()
        );
        assert_eq!(
            user_stats.all_time_gas_simulated,
            U256::from_dec_str("30049470846").unwrap()
        );
        assert_eq!(
            user_stats.last_7d_miner_payments,
            U256::from_dec_str("1280749594841588639").unwrap()
        );
        assert_eq!(
            user_stats.last_7d_gas_simulated,
            U256::from_dec_str("30049470846").unwrap()
        );
        assert_eq!(
            user_stats.last_1d_miner_payments,
            U256::from_dec_str("142305510537954293").unwrap()
        );
        assert_eq!(
            user_stats.last_1d_gas_simulated,
            U256::from_dec_str("2731770076").unwrap()
        );
    }
}
