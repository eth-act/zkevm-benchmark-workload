//! Serde helpers for Beacon API and JSON-RPC quantity fields.

use std::{fmt, str::FromStr};

use alloy_primitives::U256;
use serde::de::{self, Visitor};

pub(crate) fn de_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_any(U64Visitor)
}

pub(crate) fn de_u256<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_any(U256Visitor)
}

struct U64Visitor;

impl Visitor<'_> for U64Visitor {
    type Value = u64;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a decimal or 0x-prefixed u64")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(value)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        parse_u64(value).map_err(E::custom)
    }
}

struct U256Visitor;

impl Visitor<'_> for U256Visitor {
    type Value = U256;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a decimal or 0x-prefixed uint256")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(U256::from(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        parse_u256(value).map_err(E::custom)
    }
}

pub(crate) fn parse_u64(value: &str) -> anyhow::Result<u64> {
    let value = value.trim();
    if let Some(hex) = value.strip_prefix("0x") {
        return Ok(u64::from_str_radix(hex, 16)?);
    }
    Ok(value.parse()?)
}

pub(crate) fn parse_u256(value: &str) -> anyhow::Result<U256> {
    let value = value.trim();
    if let Some(hex) = value.strip_prefix("0x") {
        return Ok(U256::from_str_radix(hex, 16)?);
    }
    Ok(U256::from_str(value)?)
}

pub(crate) fn hex_quantity(value: u64) -> String {
    format!("0x{value:x}")
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize)]
    struct U64Value {
        #[serde(deserialize_with = "de_u64")]
        value: u64,
    }

    #[derive(Debug, Deserialize)]
    struct U256Value {
        #[serde(deserialize_with = "de_u256")]
        value: U256,
    }

    #[test]
    fn parses_decimal_and_hex_u64() {
        let decimal: U64Value = serde_json::from_str(r#"{"value":"42"}"#).unwrap();
        let hex: U64Value = serde_json::from_str(r#"{"value":"0x2a"}"#).unwrap();

        assert_eq!(decimal.value, 42);
        assert_eq!(hex.value, 42);
    }

    #[test]
    fn parses_decimal_and_hex_u256() {
        let decimal: U256Value = serde_json::from_str(r#"{"value":"42"}"#).unwrap();
        let hex: U256Value = serde_json::from_str(r#"{"value":"0x2a"}"#).unwrap();

        assert_eq!(decimal.value, U256::from(42));
        assert_eq!(hex.value, U256::from(42));
    }
}
