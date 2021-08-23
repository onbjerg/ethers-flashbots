use ethers_core::types::{H160, U256, U64};
use serde::{de, Deserialize};
use serde_json::Value;
use std::str::FromStr;

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

pub fn deserialize_optional_h160<'de, D>(deserializer: D) -> Result<Option<H160>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => {
            if s.as_str() == "0x" {
                return Ok(None);
            }

            Some(H160::from_str(s.as_str()).map_err(de::Error::custom)?)
        }
        _ => return Err(de::Error::custom("expected a hexadecimal string")),
    })
}
