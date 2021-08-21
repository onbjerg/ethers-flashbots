use ethers_core::types::{U256, U64, H160};
use serde::{de, Deserialize};
use serde_json::Value;
use std::str::FromStr;

pub fn deserialize_h160<'de, D>(deserializer: D) -> Result<H160, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => {
            if s.as_str() == "0x" {
                return Ok(H160::zero());
            }

            H160::from_str(s.as_str()).map_err(de::Error::custom)?
        }
        Value::Number(num) => H160::from_low_u64_ne(
            num.as_u64()
                .ok_or_else(|| de::Error::custom("Invalid number"))?,
        ),
        _ => return Err(de::Error::custom("wrong type")),
    })
}

pub fn deserialize_u64<'de, D>(deserializer: D) -> Result<U64, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => {
            if s.as_str() == "0x" {
                return Ok(U64::zero());
            }

            U64::from(u64::from_str(s.as_str()).map_err(de::Error::custom)?)
        }
        Value::Number(num) => U64::from(
            num.as_u64()
                .ok_or_else(|| de::Error::custom("Invalid number"))?,
        ),
        _ => return Err(de::Error::custom("wrong type")),
    })
}

pub fn deserialize_u256<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => {
            if s.as_str() == "0x" {
                return Ok(U256::zero());
            }

            U256::from(u64::from_str(s.as_str()).map_err(de::Error::custom)?)
        }
        Value::Number(num) => U256::from(
            num.as_u64()
                .ok_or_else(|| de::Error::custom("Invalid number"))?,
        ),
        _ => return Err(de::Error::custom("wrong type")),
    })
}
