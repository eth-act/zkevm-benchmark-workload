//! Assembly of canonical Amsterdam stateless input bytes.

use alloy_consensus::{Header, Transaction as AlloyTransaction, TxEnvelope};
use alloy_eips::eip2718::Decodable2718;
use alloy_primitives::{B256, Bytes};
use alloy_rlp::Decodable;
use anyhow::{Context, ensure};
use libssz::SszEncode;
use libssz_types::SszList;
use stateless_validator_common::new_payload_request::{
    BlockAccessList, ConsolidationRequest, ConsolidationRequests, DepositRequest, DepositRequests,
    ExecutionPayloadV4, ExecutionRequests, ExtraData, NewPayloadRequestAmsterdam,
    Transaction as PayloadTransaction, Transactions, VersionedHashes, Withdrawal,
    WithdrawalRequest, WithdrawalRequests, Withdrawals,
};

use crate::{
    rpc::{
        BeaconExecutionPayload, ConsolidationRequestJson, DepositRequestJson,
        ExecutionPayloadEnvelope, ExecutionRequestsJson, RpcExecutionWitness, WithdrawalJson,
        WithdrawalRequestJson,
    },
    schema::{self, PublicKeys, SszExecutionWitness, SszStatelessInput},
};

/// Canonical stateless guest input generated from network RPC data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedInput {
    /// Bytes encoded as `0x0001 || SSZ(SszStatelessInput)`.
    pub bytes: Vec<u8>,
    /// Execution block hash.
    pub block_hash: B256,
    /// Execution block number.
    pub block_number: u64,
    /// Consensus slot number copied from the Amsterdam execution payload.
    pub slot_number: u64,
    /// Execution chain id.
    pub chain_id: u64,
}

pub(crate) fn build_generated_input(
    envelope: ExecutionPayloadEnvelope,
    witness: RpcExecutionWitness,
    chain_id: u64,
) -> anyhow::Result<GeneratedInput> {
    let payload = &envelope.payload;
    let slot_number = payload.slot_number;
    let block_hash = payload.block_hash;
    let block_number = payload.block_number;
    let transaction_artifacts = decode_transaction_artifacts(&payload.transactions)?;

    let new_payload_request = NewPayloadRequestAmsterdam {
        execution_payload: convert_execution_payload(payload)?,
        versioned_hashes: transaction_artifacts.versioned_hashes,
        parent_beacon_block_root: envelope.parent_beacon_block_root.0,
        execution_requests: convert_execution_requests(&envelope.execution_requests)?,
    };

    let witness = convert_witness(witness, block_number)?;
    let chain_config = schema::amsterdam_chain_config(chain_id)?;
    let public_keys = PublicKeys::try_from(transaction_artifacts.public_keys)
        .map_err(|err| anyhow::anyhow!("public_keys exceed SSZ bound: {err:?}"))?;

    let stateless_input = SszStatelessInput {
        new_payload_request,
        witness,
        chain_config,
        public_keys,
    };

    let stateless_input_bytes = stateless_input.to_ssz();
    let mut bytes = Vec::with_capacity(2 + stateless_input_bytes.len());
    bytes.extend_from_slice(&schema::STATELESS_INPUT_SCHEMA_ID_BYTES);
    bytes.extend(stateless_input_bytes);

    Ok(GeneratedInput {
        bytes,
        block_hash,
        block_number,
        slot_number,
        chain_id,
    })
}

fn convert_execution_payload(
    payload: &BeaconExecutionPayload,
) -> anyhow::Result<ExecutionPayloadV4> {
    let transactions = payload
        .transactions
        .iter()
        .enumerate()
        .map(|(i, tx)| {
            PayloadTransaction::try_from(tx.to_vec())
                .map_err(|err| anyhow::anyhow!("transaction #{i} exceeds SSZ bound: {err:?}"))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    let withdrawals = payload
        .withdrawals
        .iter()
        .map(convert_withdrawal)
        .collect::<Vec<_>>();

    Ok(ExecutionPayloadV4 {
        parent_hash: payload.parent_hash.0,
        fee_recipient: payload.fee_recipient.into_array(),
        state_root: payload.state_root.0,
        receipts_root: payload.receipts_root.0,
        logs_bloom: payload.logs_bloom.0,
        prev_randao: payload.prev_randao.0,
        block_number: payload.block_number,
        gas_limit: payload.gas_limit,
        gas_used: payload.gas_used,
        timestamp: payload.timestamp,
        extra_data: ExtraData::try_from(payload.extra_data.to_vec())
            .map_err(|err| anyhow::anyhow!("extra_data exceeds SSZ bound: {err:?}"))?,
        base_fee_per_gas: payload.base_fee_per_gas.to_le_bytes(),
        block_hash: payload.block_hash.0,
        transactions: Transactions::try_from(transactions)
            .map_err(|err| anyhow::anyhow!("transactions exceed SSZ bound: {err:?}"))?,
        withdrawals: Withdrawals::try_from(withdrawals)
            .map_err(|err| anyhow::anyhow!("withdrawals exceed SSZ bound: {err:?}"))?,
        blob_gas_used: payload.blob_gas_used,
        excess_blob_gas: payload.excess_blob_gas,
        block_access_list: BlockAccessList::try_from(payload.block_access_list.to_vec())
            .map_err(|err| anyhow::anyhow!("block_access_list exceeds SSZ bound: {err:?}"))?,
        slot_number: payload.slot_number,
    })
}

const fn convert_withdrawal(withdrawal: &WithdrawalJson) -> Withdrawal {
    Withdrawal {
        index: withdrawal.index,
        validator_index: withdrawal.validator_index,
        address: withdrawal.address.into_array(),
        amount: withdrawal.amount,
    }
}

fn convert_execution_requests(
    requests: &ExecutionRequestsJson,
) -> anyhow::Result<ExecutionRequests> {
    let deposits = requests
        .deposits
        .iter()
        .map(convert_deposit_request)
        .collect::<Vec<_>>();
    let withdrawals = requests
        .withdrawals
        .iter()
        .map(convert_withdrawal_request)
        .collect::<Vec<_>>();
    let consolidations = requests
        .consolidations
        .iter()
        .map(convert_consolidation_request)
        .collect::<Vec<_>>();

    Ok(ExecutionRequests {
        deposits: DepositRequests::try_from(deposits)
            .map_err(|err| anyhow::anyhow!("deposit requests exceed SSZ bound: {err:?}"))?,
        withdrawals: WithdrawalRequests::try_from(withdrawals)
            .map_err(|err| anyhow::anyhow!("withdrawal requests exceed SSZ bound: {err:?}"))?,
        consolidations: ConsolidationRequests::try_from(consolidations)
            .map_err(|err| anyhow::anyhow!("consolidation requests exceed SSZ bound: {err:?}"))?,
    })
}

const fn convert_deposit_request(request: &DepositRequestJson) -> DepositRequest {
    DepositRequest {
        pubkey: request.pubkey.0,
        withdrawal_credentials: request.withdrawal_credentials.0,
        amount: request.amount,
        signature: request.signature.0,
        index: request.index,
    }
}

const fn convert_withdrawal_request(request: &WithdrawalRequestJson) -> WithdrawalRequest {
    WithdrawalRequest {
        source_address: request.source_address.into_array(),
        validator_pubkey: request.validator_pubkey.0,
        amount: request.amount,
    }
}

const fn convert_consolidation_request(request: &ConsolidationRequestJson) -> ConsolidationRequest {
    ConsolidationRequest {
        source_address: request.source_address.into_array(),
        source_pubkey: request.source_pubkey.0,
        target_pubkey: request.target_pubkey.0,
    }
}

fn convert_witness(
    witness: RpcExecutionWitness,
    block_number: u64,
) -> anyhow::Result<SszExecutionWitness> {
    let headers = normalize_headers(witness.headers, block_number)?;
    Ok(SszExecutionWitness {
        state: bytes_to_nested_ssz_list::<
            { schema::MAX_BYTES_PER_WITNESS_NODE },
            { schema::MAX_WITNESS_NODES },
        >(witness.state, "witness.state")?,
        codes: bytes_to_nested_ssz_list::<
            { schema::MAX_BYTES_PER_CODE },
            { schema::MAX_WITNESS_CODES },
        >(witness.codes, "witness.codes")?,
        headers: bytes_to_nested_ssz_list::<
            { schema::MAX_BYTES_PER_HEADER },
            { schema::MAX_WITNESS_HEADERS },
        >(headers, "witness.headers")?,
    })
}

fn bytes_to_nested_ssz_list<const MAX_BYTES: usize, const MAX_ITEMS: usize>(
    values: Vec<Bytes>,
    label: &str,
) -> anyhow::Result<SszList<SszList<u8, MAX_BYTES>, MAX_ITEMS>> {
    let values = values
        .into_iter()
        .enumerate()
        .map(|(i, bytes)| {
            SszList::<u8, MAX_BYTES>::try_from(bytes.to_vec())
                .map_err(|err| anyhow::anyhow!("{label}[{i}] exceeds SSZ byte bound: {err:?}"))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    SszList::<SszList<u8, MAX_BYTES>, MAX_ITEMS>::try_from(values)
        .map_err(|err| anyhow::anyhow!("{label} exceeds SSZ item bound: {err:?}"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TransactionArtifacts {
    public_keys: Vec<[u8; 65]>,
    versioned_hashes: VersionedHashes,
}

fn decode_transaction_artifacts(transactions: &[Bytes]) -> anyhow::Result<TransactionArtifacts> {
    let mut public_keys = Vec::with_capacity(transactions.len());
    let mut versioned_hashes = Vec::new();

    for (i, tx) in transactions.iter().enumerate() {
        let envelope = TxEnvelope::decode_2718_exact(tx.as_ref())
            .with_context(|| format!("failed to decode transaction #{i}"))?;
        let public_key = envelope
            .signature()
            .recover_from_prehash(&envelope.signature_hash())
            .map(|key| key.to_encoded_point(false).as_bytes().try_into().unwrap())
            .with_context(|| format!("failed to recover public key for transaction #{i}"))?;
        public_keys.push(public_key);

        if let Some(hashes) = envelope.blob_versioned_hashes() {
            versioned_hashes.extend(hashes.iter().map(|hash| hash.0));
        }
    }

    let versioned_hashes = VersionedHashes::try_from(versioned_hashes)
        .map_err(|err| anyhow::anyhow!("versioned_hashes exceed SSZ bound: {err:?}"))?;

    Ok(TransactionArtifacts {
        public_keys,
        versioned_hashes,
    })
}

#[cfg(test)]
fn recover_public_keys(transactions: &[Bytes]) -> anyhow::Result<Vec<[u8; 65]>> {
    Ok(decode_transaction_artifacts(transactions)?.public_keys)
}

#[cfg(test)]
fn decode_versioned_hashes(transactions: &[Bytes]) -> anyhow::Result<VersionedHashes> {
    Ok(decode_transaction_artifacts(transactions)?.versioned_hashes)
}

fn normalize_headers(headers: Vec<Bytes>, block_number: u64) -> anyhow::Result<Vec<Bytes>> {
    ensure!(
        block_number > 0,
        "cannot require parent header for genesis block"
    );
    let parent_number = block_number - 1;
    let mut headers = headers
        .into_iter()
        .enumerate()
        .map(|(i, bytes)| {
            let number = decode_header_number(&bytes)
                .with_context(|| format!("failed to decode witness header #{i}"))?;
            Ok((number, bytes))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    headers.sort_by_key(|(number, _)| *number);

    for pair in headers.windows(2) {
        let prev = pair[0].0;
        let next = pair[1].0;
        ensure!(next != prev, "duplicate witness header for block #{next}");
        ensure!(
            next == prev + 1,
            "witness headers are not contiguous: block #{prev} followed by block #{next}",
        );
    }

    ensure!(
        headers
            .last()
            .map(|(number, _)| *number)
            .is_some_and(|number| number == parent_number),
        "witness parent header for block #{parent_number} is absent",
    );

    Ok(headers.into_iter().map(|(_, bytes)| bytes).collect())
}

fn decode_header_number(bytes: &[u8]) -> anyhow::Result<u64> {
    let mut slice = bytes;
    let header = Header::decode(&mut slice).context("invalid RLP header")?;
    ensure!(slice.is_empty(), "RLP header has trailing bytes");
    Ok(header.number)
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{b256, hex};
    use alloy_rlp::Encodable;

    use super::*;

    #[test]
    fn extracts_versioned_hashes_from_blob_transaction() {
        let raw = hex::decode(
            "0x03f9011d83aa36a7820fa28477359400852e90edd0008252089411e9ca82a3a762b4b5bd264d4173a242e7a770648080c08504a817c800f8a5a0012ec3d6f66766bedb002a190126b3549fce0047de0d4c25cffce0dc1c57921aa00152d8e24762ff22b1cfd9f8c0683786a7ca63ba49973818b3d1e9512cd2cec4a0013b98c6c83e066d5b14af2b85199e3d4fc7d1e778dd53130d180f5077e2d1c7a001148b495d6e859114e670ca54fb6e2657f0cbae5b08063605093a4b3dc9f8f1a0011ac212f13c5dff2b2c6b600a79635103d6f580a4221079951181b25c7e654901a0c8de4cced43169f9aa3d36506363b2d2c44f6c49fc1fd91ea114c86f3757077ea01e11fdd0d1934eda0492606ee0bb80a7bf8f35cc5f86ec60fe5031ba48bfd544",
        )
        .unwrap();

        let hashes = decode_versioned_hashes(&[Bytes::from(raw)]).unwrap();

        assert_eq!(
            &*hashes,
            &[
                b256!("012ec3d6f66766bedb002a190126b3549fce0047de0d4c25cffce0dc1c57921a").0,
                b256!("0152d8e24762ff22b1cfd9f8c0683786a7ca63ba49973818b3d1e9512cd2cec4").0,
                b256!("013b98c6c83e066d5b14af2b85199e3d4fc7d1e778dd53130d180f5077e2d1c7").0,
                b256!("01148b495d6e859114e670ca54fb6e2657f0cbae5b08063605093a4b3dc9f8f1").0,
                b256!("011ac212f13c5dff2b2c6b600a79635103d6f580a4221079951181b25c7e6549").0,
            ]
        );
    }

    #[test]
    fn recovers_public_key_from_raw_transaction() {
        let raw = hex::decode(
            "f86e81fa843127403882f61894db8d964741c53e55df9c2d4e9414c6c96482874e870aa87bee538000808360306ca03aa421df67a101c45ff9cb06ce28f518a5d8d8dbb76a79361280071909650a27a05a447ff053c4ae601cfe81859b58d5603f2d0a73481c50f348089032feb0b073",
        )
        .unwrap();

        let public_keys = recover_public_keys(&[Bytes::from(raw)]).unwrap();

        assert_eq!(public_keys.len(), 1);
        assert_eq!(public_keys[0][0], 0x04);
    }

    #[test]
    fn normalizes_headers_to_oldest_first_and_parent_last() {
        let header_8 = rlp_header(8);
        let header_9 = rlp_header(9);

        let normalized = normalize_headers(vec![header_9.clone(), header_8.clone()], 10).unwrap();

        assert_eq!(normalized, vec![header_8, header_9]);
    }

    #[test]
    fn rejects_missing_parent_header() {
        let err = normalize_headers(vec![rlp_header(7), rlp_header(8)], 10).unwrap_err();

        assert!(
            err.to_string()
                .contains("witness parent header for block #9 is absent")
        );
    }

    #[test]
    fn rejects_non_contiguous_headers() {
        let err = normalize_headers(vec![rlp_header(7), rlp_header(9)], 10).unwrap_err();

        assert!(err.to_string().contains("not contiguous"));
    }

    #[test]
    fn witness_lists_enforce_bounds() {
        let oversized = Bytes::from(vec![0_u8; schema::MAX_BYTES_PER_HEADER + 1]);
        let err = bytes_to_nested_ssz_list::<
            { schema::MAX_BYTES_PER_HEADER },
            { schema::MAX_WITNESS_HEADERS },
        >(vec![oversized], "headers")
        .unwrap_err();

        assert!(err.to_string().contains("exceeds SSZ byte bound"));
    }

    #[test]
    fn fixture_json_builds_deterministic_stateless_input_bytes() {
        let parent_header = rlp_header(9);
        let envelope_response = serde_json::json!({
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
                        "block_number": "10",
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
        let witness_json = serde_json::json!({
            "state": ["0x80"],
            "keys": ["0x01"],
            "codes": ["0x"],
            "headers": [format!("0x{}", hex::encode(&parent_header))]
        });
        let parsed: crate::rpc::ExecutionPayloadEnvelopeResponse =
            serde_json::from_value(envelope_response).unwrap();
        let envelope = parsed.data.message;
        let witness: RpcExecutionWitness = serde_json::from_value(witness_json).unwrap();

        let first = build_generated_input(envelope.clone(), witness.clone(), 1).unwrap();
        let second = build_generated_input(envelope, witness, 1).unwrap();

        assert_eq!(first.bytes, second.bytes);
        assert_eq!(&first.bytes[..2], &schema::STATELESS_INPUT_SCHEMA_ID_BYTES);
        assert_eq!(first.block_number, 10);
        assert_eq!(first.slot_number, 64);
        assert_eq!(first.chain_id, 1);
    }

    fn rlp_header(number: u64) -> Bytes {
        let header = Header {
            number,
            base_fee_per_gas: Some(7),
            withdrawals_root: Some(B256::ZERO),
            blob_gas_used: Some(0),
            excess_blob_gas: Some(0),
            parent_beacon_block_root: Some(B256::ZERO),
            requests_hash: Some(B256::ZERO),
            block_access_list_hash: Some(B256::ZERO),
            slot_number: Some(number + 1),
            ..Default::default()
        };
        let mut bytes = Vec::new();
        header.encode(&mut bytes);
        Bytes::from(bytes)
    }
}
