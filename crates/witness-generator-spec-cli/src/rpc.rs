//! Minimal Beacon API and JSON-RPC client/types for network witness generation.

use alloy_primitives::{Address, B256, Bytes, FixedBytes};
use anyhow::{Context, bail};
use reqwest::{
    Client, RequestBuilder,
    header::{HeaderName, HeaderValue},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};

use crate::{
    NetworkWitnessConfig,
    serde_helpers::{de_u64, hex_quantity, parse_u64},
};

#[derive(Debug, Clone)]
pub(crate) struct RpcClient {
    config: NetworkWitnessConfig,
    http: Client,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ExecutionPayloadEnvelopeResponse {
    pub(crate) data: SignedExecutionPayloadEnvelope,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SignedExecutionPayloadEnvelope {
    pub(crate) message: ExecutionPayloadEnvelope,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ExecutionPayloadEnvelope {
    pub(crate) payload: BeaconExecutionPayload,
    pub(crate) execution_requests: ExecutionRequestsJson,
    pub(crate) parent_beacon_block_root: B256,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct BeaconExecutionPayload {
    pub(crate) parent_hash: B256,
    pub(crate) fee_recipient: Address,
    pub(crate) state_root: B256,
    pub(crate) receipts_root: B256,
    pub(crate) logs_bloom: FixedBytes<256>,
    pub(crate) prev_randao: B256,
    #[serde(deserialize_with = "de_u64")]
    pub(crate) block_number: u64,
    #[serde(deserialize_with = "de_u64")]
    pub(crate) gas_limit: u64,
    #[serde(deserialize_with = "de_u64")]
    pub(crate) gas_used: u64,
    #[serde(deserialize_with = "de_u64")]
    pub(crate) timestamp: u64,
    pub(crate) extra_data: Bytes,
    #[serde(deserialize_with = "crate::serde_helpers::de_u256")]
    pub(crate) base_fee_per_gas: alloy_primitives::U256,
    pub(crate) block_hash: B256,
    #[serde(default)]
    pub(crate) transactions: Vec<Bytes>,
    #[serde(default)]
    pub(crate) withdrawals: Vec<WithdrawalJson>,
    #[serde(deserialize_with = "de_u64")]
    pub(crate) blob_gas_used: u64,
    #[serde(deserialize_with = "de_u64")]
    pub(crate) excess_blob_gas: u64,
    pub(crate) block_access_list: Bytes,
    #[serde(deserialize_with = "de_u64")]
    pub(crate) slot_number: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct WithdrawalJson {
    #[serde(deserialize_with = "de_u64")]
    pub(crate) index: u64,
    #[serde(deserialize_with = "de_u64")]
    pub(crate) validator_index: u64,
    pub(crate) address: Address,
    #[serde(deserialize_with = "de_u64")]
    pub(crate) amount: u64,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct ExecutionRequestsJson {
    #[serde(default)]
    pub(crate) deposits: Vec<DepositRequestJson>,
    #[serde(default)]
    pub(crate) withdrawals: Vec<WithdrawalRequestJson>,
    #[serde(default)]
    pub(crate) consolidations: Vec<ConsolidationRequestJson>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct DepositRequestJson {
    pub(crate) pubkey: FixedBytes<48>,
    pub(crate) withdrawal_credentials: B256,
    #[serde(deserialize_with = "de_u64")]
    pub(crate) amount: u64,
    pub(crate) signature: FixedBytes<96>,
    #[serde(deserialize_with = "de_u64")]
    pub(crate) index: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct WithdrawalRequestJson {
    pub(crate) source_address: Address,
    pub(crate) validator_pubkey: FixedBytes<48>,
    #[serde(deserialize_with = "de_u64")]
    pub(crate) amount: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ConsolidationRequestJson {
    pub(crate) source_address: Address,
    pub(crate) source_pubkey: FixedBytes<48>,
    pub(crate) target_pubkey: FixedBytes<48>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RpcExecutionWitness {
    #[serde(default)]
    pub(crate) state: Vec<Bytes>,
    #[serde(default)]
    #[allow(dead_code)]
    pub(crate) keys: Vec<Bytes>,
    #[serde(default)]
    pub(crate) codes: Vec<Bytes>,
    #[serde(default)]
    pub(crate) headers: Vec<Bytes>,
}

#[derive(Debug, Clone)]
pub(crate) struct ElBlock {
    pub(crate) hash: B256,
    pub(crate) number: u64,
    pub(crate) timestamp: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct ElBlockRpc {
    hash: B256,
    #[serde(deserialize_with = "de_u64")]
    number: u64,
    #[serde(deserialize_with = "de_u64")]
    timestamp: u64,
}

#[derive(Debug, Clone)]
pub(crate) struct BeaconGenesis {
    pub(crate) genesis_time: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct BeaconGenesisResponse {
    data: BeaconGenesisData,
}

#[derive(Debug, Clone, Deserialize)]
struct BeaconGenesisData {
    #[serde(deserialize_with = "de_u64")]
    genesis_time: u64,
}

#[derive(Debug, Clone)]
pub(crate) struct BeaconSpec {
    pub(crate) seconds_per_slot: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct BeaconSpecResponse {
    data: BeaconSpecData,
}

#[derive(Debug, Clone, Deserialize)]
struct BeaconSpecData {
    #[serde(rename = "SECONDS_PER_SLOT", deserialize_with = "de_u64")]
    seconds_per_slot: u64,
}

#[derive(Debug, Serialize)]
struct JsonRpcRequest<'a> {
    jsonrpc: &'static str,
    id: u64,
    method: &'a str,
    params: Value,
}

#[derive(Debug, Deserialize)]
struct JsonRpcResponse<T> {
    result: Option<T>,
    error: Option<JsonRpcError>,
}

#[derive(Debug, Deserialize)]
struct JsonRpcError {
    code: i64,
    message: String,
}

impl RpcClient {
    pub(crate) const fn new(config: NetworkWitnessConfig, http: Client) -> Self {
        Self { config, http }
    }

    pub(crate) async fn execution_payload_envelope(
        &self,
        block_id: &str,
    ) -> anyhow::Result<ExecutionPayloadEnvelope> {
        let url = execution_payload_envelope_url(&self.config.cl_endpoint, block_id);
        let response: ExecutionPayloadEnvelopeResponse = self
            .send_get_with_headers(&url, &self.config.cl_headers)
            .await
            .with_context(|| format!("failed to fetch execution payload envelope `{block_id}`"))?;
        Ok(response.data.message)
    }

    pub(crate) async fn beacon_genesis(&self) -> anyhow::Result<BeaconGenesis> {
        let url = format!(
            "{}/eth/v1/beacon/genesis",
            trim_endpoint(&self.config.cl_endpoint)
        );
        let response: BeaconGenesisResponse = self
            .send_get_with_headers(&url, &self.config.cl_headers)
            .await
            .context("failed to fetch CL genesis")?;
        Ok(BeaconGenesis {
            genesis_time: response.data.genesis_time,
        })
    }

    pub(crate) async fn beacon_spec(&self) -> anyhow::Result<BeaconSpec> {
        let url = format!(
            "{}/eth/v1/config/spec",
            trim_endpoint(&self.config.cl_endpoint)
        );
        let response: BeaconSpecResponse = self
            .send_get_with_headers(&url, &self.config.cl_headers)
            .await
            .context("failed to fetch CL spec")?;
        Ok(BeaconSpec {
            seconds_per_slot: response.data.seconds_per_slot,
        })
    }

    pub(crate) async fn debug_execution_witness_by_block_hash(
        &self,
        block_hash: B256,
    ) -> anyhow::Result<RpcExecutionWitness> {
        self.el_rpc(
            "debug_executionWitnessByBlockHash",
            json!([block_hash.to_string()]),
        )
        .await
    }

    pub(crate) async fn eth_chain_id(&self) -> anyhow::Result<u64> {
        let chain_id: String = self.el_rpc("eth_chainId", json!([])).await?;
        parse_u64(&chain_id).context("failed to parse eth_chainId")
    }

    pub(crate) async fn eth_block_by_number(&self, number: u64) -> anyhow::Result<ElBlock> {
        let result: Option<ElBlockRpc> = self
            .el_rpc("eth_getBlockByNumber", json!([hex_quantity(number), false]))
            .await?;
        let block = result.with_context(|| format!("EL block #{number} not found"))?;
        Ok(ElBlock {
            hash: block.hash,
            number: block.number,
            timestamp: block.timestamp,
        })
    }

    async fn el_rpc<T>(&self, method: &str, params: Value) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        let request = JsonRpcRequest {
            jsonrpc: "2.0",
            id: 1,
            method,
            params,
        };
        let url = trim_endpoint(&self.config.el_endpoint).to_owned();
        let builder = self.http.post(url).json(&request);
        let response: JsonRpcResponse<T> = apply_headers(builder, &self.config.el_headers)?
            .send()
            .await
            .with_context(|| format!("JSON-RPC method `{method}` failed"))?
            .error_for_status()
            .with_context(|| format!("JSON-RPC method `{method}` returned HTTP error"))?
            .json()
            .await
            .with_context(|| format!("failed to decode JSON-RPC response for `{method}`"))?;

        if let Some(error) = response.error {
            bail!(
                "JSON-RPC method `{method}` returned error {}: {}",
                error.code,
                error.message
            );
        }
        response
            .result
            .with_context(|| format!("JSON-RPC method `{method}` returned no result"))
    }

    async fn send_get_with_headers<T>(
        &self,
        url: &str,
        headers: &[(String, String)],
    ) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        let builder = self.http.get(url);
        apply_headers(builder, headers)?
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .context("failed to decode JSON response")
    }
}

fn apply_headers(
    mut builder: RequestBuilder,
    headers: &[(String, String)],
) -> anyhow::Result<RequestBuilder> {
    for (name, value) in headers {
        let name = HeaderName::from_bytes(name.as_bytes())
            .with_context(|| format!("invalid HTTP header name `{name}`"))?;
        let value = HeaderValue::from_str(value)
            .with_context(|| format!("invalid value for HTTP header `{name}`"))?;
        builder = builder.header(name, value);
    }
    Ok(builder)
}

fn trim_endpoint(endpoint: &str) -> &str {
    endpoint.trim_end_matches('/')
}

fn execution_payload_envelope_url(endpoint: &str, block_id: &str) -> String {
    format!(
        "{}/eth/v1/beacon/execution_payload_envelopes/{}",
        trim_endpoint(endpoint),
        block_id,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_plural_execution_payload_envelope_url() {
        assert_eq!(
            execution_payload_envelope_url("http://127.0.0.1:3500/", "head"),
            "http://127.0.0.1:3500/eth/v1/beacon/execution_payload_envelopes/head",
        );
    }

    #[test]
    fn parses_execution_requests_with_mixed_quantities() {
        let json = serde_json::json!({
            "deposits": [{
                "pubkey": format!("0x{}", "11".repeat(48)),
                "withdrawal_credentials": format!("0x{}", "22".repeat(32)),
                "amount": "32000000000",
                "signature": format!("0x{}", "33".repeat(96)),
                "index": "0x2a"
            }],
            "withdrawals": [{
                "source_address": format!("0x{}", "44".repeat(20)),
                "validator_pubkey": format!("0x{}", "55".repeat(48)),
                "amount": "0x01"
            }],
            "consolidations": [{
                "source_address": format!("0x{}", "66".repeat(20)),
                "source_pubkey": format!("0x{}", "77".repeat(48)),
                "target_pubkey": format!("0x{}", "88".repeat(48))
            }]
        });

        let parsed: ExecutionRequestsJson = serde_json::from_value(json).unwrap();

        assert_eq!(parsed.deposits[0].amount, 32_000_000_000);
        assert_eq!(parsed.deposits[0].index, 42);
        assert_eq!(parsed.withdrawals[0].amount, 1);
        assert_eq!(parsed.consolidations.len(), 1);
    }

    #[test]
    fn parses_execution_payload_envelope_with_amsterdam_fields() {
        let response = serde_json::json!({
            "version": "gloas",
            "execution_optimistic": false,
            "finalized": false,
            "data": {
                "message": {
                    "payload": {
                        "parent_hash": format!("0x{}", "01".repeat(32)),
                        "fee_recipient": format!("0x{}", "02".repeat(20)),
                        "state_root": format!("0x{}", "03".repeat(32)),
                        "receipts_root": format!("0x{}", "04".repeat(32)),
                        "logs_bloom": format!("0x{}", "00".repeat(256)),
                        "prev_randao": format!("0x{}", "05".repeat(32)),
                        "block_number": "11",
                        "gas_limit": "30000000",
                        "gas_used": "21000",
                        "timestamp": "1000",
                        "extra_data": "0x",
                        "base_fee_per_gas": "0x7",
                        "block_hash": format!("0x{}", "06".repeat(32)),
                        "transactions": [],
                        "withdrawals": [],
                        "blob_gas_used": "0",
                        "excess_blob_gas": "0",
                        "block_access_list": "0xc0",
                        "slot_number": "64"
                    },
                    "execution_requests": {},
                    "builder_index": "0",
                    "beacon_block_root": format!("0x{}", "bb".repeat(32)),
                    "parent_beacon_block_root": format!("0x{}", "aa".repeat(32))
                }
            }
        });

        let parsed: ExecutionPayloadEnvelopeResponse = serde_json::from_value(response).unwrap();

        assert_eq!(parsed.data.message.payload.block_number, 11);
        assert_eq!(parsed.data.message.payload.slot_number, 64);
        assert_eq!(
            parsed.data.message.parent_beacon_block_root,
            B256::repeat_byte(0xaa)
        );
    }
}
