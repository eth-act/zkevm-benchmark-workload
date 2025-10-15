//! SSZ-encoded representations of Ethereum block structures.
//!
//! This module provides SSZ (Simple Serialize) compatible structures for Ethereum blocks.
//!
//! Note: SSZ is not currently used in the execution layer. For this reason, it doesn't yet have native support in
//! Reth, so we define our own SSZ-compatible structures here. Whenever Reth adds native SSZ support,
//! we can consider removing this module and using the native types directly.

use alloc::vec::Vec;
use alloy_eips::eip4895;
use alloy_primitives::{Address, B64, B256, BlockNumber, Bloom, Bytes, U256};

/// SSZ-serializable representation of an Ethereum block.
#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct Block {
    header: Header,
    body: BlockBody,
}

impl
    From<
        alloy_consensus::Block<
            alloy_consensus::EthereumTxEnvelope<alloy_consensus::TxEip4844>,
            alloy_consensus::Header,
        >,
    > for Block
{
    fn from(
        block: alloy_consensus::Block<
            alloy_consensus::EthereumTxEnvelope<alloy_consensus::TxEip4844>,
            alloy_consensus::Header,
        >,
    ) -> Self {
        Self {
            header: block.header.into(),
            body: block.body.into(),
        }
    }
}

/// SSZ-serializable representation of a block body.
#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct BlockBody {
    transactions: Vec<EthereumTxEnvelope>,
    ommers: Vec<Header>,
    withdrawals: Option<Vec<Withdrawal>>,
}

impl
    From<
        alloy_consensus::BlockBody<alloy_consensus::EthereumTxEnvelope<alloy_consensus::TxEip4844>>,
    > for BlockBody
{
    fn from(
        body: alloy_consensus::BlockBody<
            alloy_consensus::EthereumTxEnvelope<alloy_consensus::TxEip4844>,
        >,
    ) -> Self {
        Self {
            transactions: body
                .transactions
                .into_iter()
                .map(EthereumTxEnvelope::from)
                .collect(),
            ommers: body.ommers.into_iter().map(Header::from).collect(),
            withdrawals: body
                .withdrawals
                .map(|ws| ws.into_iter().map(Withdrawal::from).collect()),
        }
    }
}

/// SSZ-serializable representation of an Ethereum block header.
#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct Header {
    parent_hash: B256,
    ommers_hash: B256,
    beneficiary: Address,
    state_root: B256,
    transactions_root: B256,
    receipts_root: B256,
    logs_bloom: Bloom,
    difficulty: U256,
    number: BlockNumber,
    gas_limit: u64,
    gas_used: u64,
    timestamp: u64,
    extra_data: Bytes,
    mix_hash: B256,
    nonce: B64,
    base_fee_per_gas: Option<u64>,
    withdrawals_root: Option<B256>,
    blob_gas_used: Option<u64>,
    excess_blob_gas: Option<u64>,
    parent_beacon_block_root: Option<B256>,
    requests_hash: Option<B256>,
}

impl From<alloy_consensus::Header> for Header {
    fn from(header: alloy_consensus::Header) -> Self {
        Self {
            parent_hash: header.parent_hash,
            ommers_hash: header.ommers_hash,
            beneficiary: header.beneficiary,
            state_root: header.state_root,
            transactions_root: header.transactions_root,
            receipts_root: header.receipts_root,
            logs_bloom: header.logs_bloom,
            difficulty: header.difficulty,
            number: header.number,
            gas_limit: header.gas_limit,
            gas_used: header.gas_used,
            timestamp: header.timestamp,
            extra_data: header.extra_data,
            mix_hash: header.mix_hash,
            nonce: header.nonce,
            base_fee_per_gas: header.base_fee_per_gas,
            withdrawals_root: header.withdrawals_root,
            blob_gas_used: header.blob_gas_used,
            excess_blob_gas: header.excess_blob_gas,
            parent_beacon_block_root: header.parent_beacon_block_root,
            requests_hash: header.requests_hash,
        }
    }
}

/// SSZ-serializable representation of a validator withdrawal.
#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct Withdrawal {
    index: u64,
    validator_index: u64,
    address: Address,
    amount: u64,
}

impl From<eip4895::Withdrawal> for Withdrawal {
    fn from(withdrawal: eip4895::Withdrawal) -> Self {
        Self {
            index: withdrawal.index,
            validator_index: withdrawal.validator_index,
            address: withdrawal.address,
            amount: withdrawal.amount,
        }
    }
}

/// SSZ-serializable transaction envelope supporting a subset of Ethereum transaction types.
#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
#[ssz(enum_behaviour = "union")]
pub enum EthereumTxEnvelope {
    /// Legacy transaction type.
    Legacy(SignedTx<TxLegacy>),
    /// EIP-1559 transaction type.
    Eip1559(SignedTx<TxEip1559>),
    /// EIP-4844 transaction type.
    Eip4844(SignedTx<TxEip4844>),
    /// EIP-2930 transaction type.
    Eip2930(SignedTx<TxEip2930>),
    /// EIP-7702 transaction type.
    Eip7702(SignedTx<TxEip7702>),
}

impl From<alloy_consensus::EthereumTxEnvelope<alloy_consensus::TxEip4844>> for EthereumTxEnvelope {
    fn from(tx: alloy_consensus::EthereumTxEnvelope<alloy_consensus::TxEip4844>) -> Self {
        match tx {
            alloy_consensus::EthereumTxEnvelope::Legacy(tx) => Self::Legacy(tx.into()),
            alloy_consensus::EthereumTxEnvelope::Eip1559(tx) => Self::Eip1559(tx.into()),
            alloy_consensus::EthereumTxEnvelope::Eip4844(tx) => Self::Eip4844(tx.into()),
            alloy_consensus::EthereumTxEnvelope::Eip2930(tx) => Self::Eip2930(tx.into()),
            alloy_consensus::EthereumTxEnvelope::Eip7702(tx) => Self::Eip7702(tx.into()),
        }
    }
}

/// SSZ-serializable representation of a signed legacy Ethereum transaction.
#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct SignedTx<Tx: ssz::Encode + ssz::Decode> {
    tx: Tx,
    signature: Signature,
}

impl From<alloy_consensus::Signed<alloy_consensus::TxLegacy>> for SignedTx<TxLegacy> {
    fn from(signed_tx: alloy_consensus::Signed<alloy_consensus::TxLegacy>) -> Self {
        Self {
            tx: signed_tx.tx().clone().into(),
            signature: (*signed_tx.signature()).into(),
        }
    }
}

impl From<alloy_consensus::Signed<alloy_consensus::TxEip1559>> for SignedTx<TxEip1559> {
    fn from(signed_tx: alloy_consensus::Signed<alloy_consensus::TxEip1559>) -> Self {
        Self {
            tx: signed_tx.tx().clone().into(),
            signature: (*signed_tx.signature()).into(),
        }
    }
}

impl From<alloy_consensus::Signed<alloy_consensus::TxEip4844>> for SignedTx<TxEip4844> {
    fn from(signed_tx: alloy_consensus::Signed<alloy_consensus::TxEip4844>) -> Self {
        Self {
            tx: signed_tx.tx().clone().into(),
            signature: (*signed_tx.signature()).into(),
        }
    }
}

impl From<alloy_consensus::Signed<alloy_consensus::TxEip2930>> for SignedTx<TxEip2930> {
    fn from(signed_tx: alloy_consensus::Signed<alloy_consensus::TxEip2930>) -> Self {
        Self {
            tx: signed_tx.tx().clone().into(),
            signature: (*signed_tx.signature()).into(),
        }
    }
}

impl From<alloy_consensus::Signed<alloy_consensus::TxEip7702>> for SignedTx<TxEip7702> {
    fn from(signed_tx: alloy_consensus::Signed<alloy_consensus::TxEip7702>) -> Self {
        Self {
            tx: signed_tx.tx().clone().into(),
            signature: (*signed_tx.signature()).into(),
        }
    }
}

/// SSZ-serializable representation of an ECDSA signature.
#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct Signature {
    y_parity: bool,
    r: U256,
    s: U256,
}

impl From<alloy_primitives::Signature> for Signature {
    fn from(signature: alloy_primitives::Signature) -> Self {
        Self {
            y_parity: signature.v(),
            r: signature.r(),
            s: signature.s(),
        }
    }
}

/// SSZ-serializable representation of a legacy Ethereum transaction.
#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct TxLegacy {
    chain_id: Option<ChainId>,
    nonce: u64,
    gas_price: u128,
    gas_limit: u64,
    to: Address,
    value: U256,
    input: Bytes,
}

impl From<alloy_consensus::TxLegacy> for TxLegacy {
    fn from(tx: alloy_consensus::TxLegacy) -> Self {
        Self {
            chain_id: tx.chain_id,
            nonce: tx.nonce,
            gas_price: tx.gas_price,
            gas_limit: tx.gas_limit,
            to: match tx.to {
                alloy_primitives::TxKind::Create => Address::default(),
                alloy_primitives::TxKind::Call(addr) => addr,
            },
            value: tx.value,
            input: tx.input,
        }
    }
}

/// SSZ-serializable representation of an EIP-1559 transaction.
#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct TxEip1559 {
    chain_id: ChainId,
    nonce: u64,
    gas_limit: u64,
    max_fee_per_gas: u128,
    max_priority_fee_per_gas: u128,
    to: Address,
    value: U256,
    access_list: Vec<AccessListItem>,
    input: Bytes,
}

impl From<alloy_consensus::transaction::TxEip1559> for TxEip1559 {
    fn from(value: alloy_consensus::transaction::TxEip1559) -> Self {
        Self {
            chain_id: value.chain_id,
            nonce: value.nonce,
            gas_limit: value.gas_limit,
            max_fee_per_gas: value.max_fee_per_gas,
            max_priority_fee_per_gas: value.max_priority_fee_per_gas,
            to: match value.to {
                alloy_primitives::TxKind::Create => Address::default(),
                alloy_primitives::TxKind::Call(addr) => addr,
            },
            value: value.value,
            access_list: value
                .access_list
                .iter()
                .map(|al| AccessListItem {
                    address: al.address,
                    storage_keys: al.storage_keys.clone(),
                })
                .collect(),
            input: value.input,
        }
    }
}

/// SSZ-serializable representation of an access list.
#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct AccessListItem {
    address: Address,
    storage_keys: Vec<B256>,
}

/// SSZ-serializable representation of an EIP-4844 transaction.
#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct TxEip4844 {
    chain_id: ChainId,
    nonce: u64,
    gas_limit: u64,
    max_fee_per_gas: u128,
    max_priority_fee_per_gas: u128,
    to: Address,
    value: U256,
    access_list: Vec<AccessListItem>,
    blob_versioned_hashes: Vec<B256>,
    max_fee_per_blob_gas: u128,
    input: Bytes,
}

impl From<alloy_consensus::transaction::TxEip4844> for TxEip4844 {
    fn from(value: alloy_consensus::transaction::TxEip4844) -> Self {
        Self {
            chain_id: value.chain_id,
            nonce: value.nonce,
            gas_limit: value.gas_limit,
            max_fee_per_gas: value.max_fee_per_gas,
            max_priority_fee_per_gas: value.max_priority_fee_per_gas,
            to: value.to,
            value: value.value,
            access_list: value
                .access_list
                .iter()
                .map(|al| AccessListItem {
                    address: al.address,
                    storage_keys: al.storage_keys.clone(),
                })
                .collect(),
            blob_versioned_hashes: value.blob_versioned_hashes,
            max_fee_per_blob_gas: value.max_fee_per_blob_gas,
            input: value.input,
        }
    }
}

/// SSZ-serializable representation of an EIP-2930 transaction.
#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct TxEip2930 {
    chain_id: ChainId,
    nonce: u64,
    gas_price: u128,
    gas_limit: u64,
    to: Address,
    value: U256,
    access_list: Vec<AccessListItem>,
    input: Bytes,
}

impl From<alloy_consensus::transaction::TxEip2930> for TxEip2930 {
    fn from(value: alloy_consensus::transaction::TxEip2930) -> Self {
        Self {
            chain_id: value.chain_id,
            nonce: value.nonce,
            gas_price: value.gas_price,
            gas_limit: value.gas_limit,
            to: match value.to {
                alloy_primitives::TxKind::Create => Address::default(),
                alloy_primitives::TxKind::Call(addr) => addr,
            },
            value: value.value,
            access_list: value
                .access_list
                .iter()
                .map(|al| AccessListItem {
                    address: al.address,
                    storage_keys: al.storage_keys.clone(),
                })
                .collect(),
            input: value.input,
        }
    }
}

/// SSZ-serializable representation of an EIP-7702 transaction.
#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct TxEip7702 {
    chain_id: ChainId,
    nonce: u64,
    gas_limit: u64,
    max_fee_per_gas: u128,
    max_priority_fee_per_gas: u128,
    to: Address,
    value: U256,
    access_list: Vec<AccessListItem>,
    authorization_list: Vec<SignedAuthorization>,
    input: Bytes,
}

impl From<alloy_consensus::transaction::TxEip7702> for TxEip7702 {
    fn from(value: alloy_consensus::transaction::TxEip7702) -> Self {
        Self {
            chain_id: value.chain_id,
            nonce: value.nonce,
            gas_limit: value.gas_limit,
            max_fee_per_gas: value.max_fee_per_gas,
            max_priority_fee_per_gas: value.max_priority_fee_per_gas,
            to: value.to,
            value: value.value,
            access_list: value
                .access_list
                .iter()
                .map(|al| AccessListItem {
                    address: al.address,
                    storage_keys: al.storage_keys.clone(),
                })
                .collect(),
            authorization_list: value
                .authorization_list
                .into_iter()
                .map(SignedAuthorization::from)
                .collect(),
            input: value.input,
        }
    }
}

/// SSZ-serializable representation of an authorization for an EIP-7702 transaction.
#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct SignedAuthorization {
    inner: Authorization,
    y_parity: bool,
    r: U256,
    s: U256,
}

impl From<alloy_eips::eip7702::SignedAuthorization> for SignedAuthorization {
    fn from(auth: alloy_eips::eip7702::SignedAuthorization) -> Self {
        Self {
            inner: auth.inner().clone().into(),
            y_parity: auth.signature().unwrap().v(),
            r: auth.signature().unwrap().r(),
            s: auth.signature().unwrap().s(),
        }
    }
}

/// SSZ-serializable representation of an authorization.
#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct Authorization {
    chain_id: U256,
    address: Address,
    nonce: u64,
}

impl From<alloy_eips::eip7702::Authorization> for Authorization {
    fn from(auth: alloy_eips::eip7702::Authorization) -> Self {
        Self {
            chain_id: auth.chain_id,
            address: auth.address,
            nonce: auth.nonce,
        }
    }
}

/// Type alias for Ethereum chain identifiers.
pub type ChainId = u64;

#[cfg(test)]
mod tests {
    use ssz::{Decode, Encode};

    use crate::{BincodeBlock, block_ssz::Block};

    #[test]
    fn test_block_ssz_encode_decode() {
        // Sample block data in JSON format for testing
        let block_json = r#"
        {
            "header": {
                "parent_hash": "0x5448165948733a50620ce604351e52218152fce74695792bb63042af34731072",
                "ommers_hash": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
                "beneficiary": "0x2adc25665018aa1fe0e6bc666dac8fc2697ff9ba",
                "state_root": "0x275620cf6a1271bf8cae4edadda0076897f09cd2bef8533ea7e7e13ba8d8e225",
                "transactions_root": "0x7c610e7810983ef78836bef4c3beb6aec3131a7589898d46904d302c76ea4836",
                "receipts_root": "0x6ebeb82e2fd4ad8ef581ba011ed8590752fbb658e86bb4f29d186cba3f7b1357",
                "withdrawals_root": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
                "logs_bloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
                "difficulty": "0x0",
                "number": 2,
                "gas_limit": 100000000000,
                "gas_used": 1000000,
                "timestamp": 24,
                "mix_hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "nonce": "0x0000000000000000",
                "base_fee_per_gas": 7,
                "blob_gas_used": 0,
                "excess_blob_gas": 0,
                "parent_beacon_block_root": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "requests_hash": "0xe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "extra_data": "0x"
            },
            "body": {
                "transactions": [
                {
                    "signature": {
                    "r": "0x8f29ffe2060a6e48c5fd6c1e225d53638b64602fd1ffdab6896f867d4a58d5e0",
                    "s": "0x1901323b25372c41b46c46e1c63f4bb246a3e22b9c61536c45ed19008cbbd0b8",
                    "yParity": "0x0",
                    "v": "0x0"
                    },
                    "transaction": {
                    "Legacy": {
                        "chain_id": "0x1",
                        "nonce": 0,
                        "gas_price": 10,
                        "gas_limit": 1000000,
                        "to": "0x0000000000000000000000000000000000001100",
                        "value": "0x0",
                        "input": "0x"
                    }
                    }
                }
                ],
                "ommers": [],
                "withdrawals": []
            }
        }"#;

        // Parse the JSON into our bincode-compatible format
        let bincode_block: BincodeBlock =
            serde_json::from_str(block_json).expect("Failed to parse test block JSON");

        // Convert to our SSZ-compatible block format
        let ssz_block: Block = bincode_block.0.into();

        // Encode the block using SSZ
        let ssz_bytes = ssz_block.as_ssz_bytes();
        assert!(!ssz_bytes.is_empty(), "SSZ encoding should not be empty");

        // Decode the block back from SSZ bytes
        let decoded_block: Block =
            Block::from_ssz_bytes(&ssz_bytes).expect("Failed to decode SSZ bytes back to Block");

        // Verify that round-trip encoding/decoding preserves the data
        assert_eq!(
            ssz_block, decoded_block,
            "Round-trip encoding should preserve block data"
        );
    }
}
