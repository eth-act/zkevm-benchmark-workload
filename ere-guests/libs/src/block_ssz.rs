use alloy_consensus::TxEip4844;
use alloy_eips::eip4895;
use alloy_primitives::{Address, B64, B256, BlockNumber, Bloom, Bytes, U256};

#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct Block {
    pub header: Header,
    pub body: BlockBody,
}

impl
    From<
        alloy_consensus::Block<
            alloy_consensus::EthereumTxEnvelope<TxEip4844>,
            alloy_consensus::Header,
        >,
    > for Block
{
    fn from(
        block: alloy_consensus::Block<
            alloy_consensus::EthereumTxEnvelope<TxEip4844>,
            alloy_consensus::Header,
        >,
    ) -> Self {
        Self {
            header: block.header.into(),
            body: block.body.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct BlockBody {
    pub transactions: Vec<EthereumTxEnvelope>,
    pub ommers: Vec<Header>,
    pub withdrawals: Option<Vec<Withdrawal>>,
}

impl From<alloy_consensus::BlockBody<alloy_consensus::EthereumTxEnvelope<TxEip4844>>>
    for BlockBody
{
    fn from(
        body: alloy_consensus::BlockBody<alloy_consensus::EthereumTxEnvelope<TxEip4844>>,
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

#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct Header {
    pub parent_hash: B256,
    pub ommers_hash: B256,
    pub beneficiary: Address,
    pub state_root: B256,
    pub transactions_root: B256,
    pub receipts_root: B256,
    pub logs_bloom: Bloom,
    pub difficulty: U256,
    pub number: BlockNumber,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: u64,
    pub extra_data: Bytes,
    pub mix_hash: B256,
    pub nonce: B64,
    pub base_fee_per_gas: Option<u64>,
    pub withdrawals_root: Option<B256>,
    pub blob_gas_used: Option<u64>,
    pub excess_blob_gas: Option<u64>,
    pub parent_beacon_block_root: Option<B256>,
    pub requests_hash: Option<B256>,
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

#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct Withdrawal {
    pub index: u64,
    pub validator_index: u64,
    pub address: Address,
    pub amount: u64,
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

// Support a reduced set of transaction types for the block body to avoid pulling a lot of complexity.
// Reference: https://github.com/alloy-rs/alloy/blob/63ca195f29309c40e1c807865fdfc4deb0d3b8ff/crates/consensus/src/transaction/envelope.rs#L165-L187
#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
#[ssz(enum_behaviour = "union")]
pub enum EthereumTxEnvelope {
    Legacy(SignedTxLegacy),
}

impl From<alloy_consensus::EthereumTxEnvelope<TxEip4844>> for EthereumTxEnvelope {
    fn from(tx: alloy_consensus::EthereumTxEnvelope<TxEip4844>) -> Self {
        match tx {
            alloy_consensus::EthereumTxEnvelope::Legacy(tx) => {
                EthereumTxEnvelope::Legacy(tx.into())
            }
            _ => {
                panic!("Unsupported transaction type in block body: {:?}", tx);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct SignedTxLegacy {
    tx: TxLegacy,
    signature: Signature,
}

impl From<alloy_consensus::Signed<alloy_consensus::TxLegacy>> for SignedTxLegacy {
    fn from(signed_tx: alloy_consensus::Signed<alloy_consensus::TxLegacy>) -> Self {
        Self {
            tx: signed_tx.tx().clone().into(),
            signature: signed_tx.signature().clone().into(),
        }
    }
}

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

#[derive(Debug, PartialEq, Eq, ssz_derive::Encode, ssz_derive::Decode)]
pub struct TxLegacy {
    pub chain_id: Option<ChainId>,
    pub nonce: u64,
    pub gas_price: u128,
    pub gas_limit: u64,
    pub to: Address,
    pub value: U256,
    pub input: Bytes,
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
                alloy_primitives::TxKind::Call(addr) => addr.into(),
            },
            value: tx.value,
            input: tx.input,
        }
    }
}

pub type ChainId = u64;

#[cfg(test)]
mod tests {
    use ssz::{Decode, Encode};

    use crate::{
        BincodeBlock,
        block_ssz::{Block, Header},
    };

    #[test]
    fn test_block_ssz_encode_decode() {
        let tx_json = r#"
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

        let decoded: BincodeBlock = serde_json::from_str(tx_json).unwrap();
        let block: Block = decoded.0.into();

        let ssz_bytes = block.as_ssz_bytes();
        assert!(!ssz_bytes.is_empty());

        let block_ssz_decoded: Block = Block::from_ssz_bytes(&ssz_bytes).unwrap();
        assert_eq!(block, block_ssz_decoded);
    }
}
