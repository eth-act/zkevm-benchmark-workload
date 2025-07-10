use risc0_zkvm::guest::env;
use revm::ExecuteEvm;
use revm::{
    primitives::{U256, Address, FixedBytes},
    context::Context,
    context::TxEnv,
    database::{CacheDB, EmptyDBTyped, StateBuilder},
    state::{AccountInfo, Bytecode},
    MainContext, MainBuilder,
};
use std::collections::HashMap;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use profile::track_cycles;

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    nonce: String,
    #[serde(rename = "gasPrice")]
    gas_price: String,
    #[serde(rename = "gasLimit")]
    gas_limit: Vec<String>,
    to: String,
    value: Vec<String>,
    data: Vec<String>,
    sender: String,
    #[serde(rename = "secretKey")]
    secret_key: String,
}

#[derive(Serialize, Deserialize)]
struct ExecutionOutput {
    success: bool,
    gas_used: u64,
    output: Option<Vec<u8>>,
    error: Option<String>,
    cycles_used: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestCase {
    env: Environment,
    pre: HashMap<String, Account>,
    transaction: Transaction,
    post: HashMap<String, Vec<PostState>>,
    config: Config,
}

#[derive(Debug, Serialize, Deserialize)]
struct Environment {
    #[serde(rename = "currentCoinbase")]
    current_coinbase: String,
    #[serde(rename = "currentGasLimit")]
    current_gas_limit: String,
    #[serde(rename = "currentNumber")]
    current_number: String,
    #[serde(rename = "currentTimestamp")]
    current_timestamp: String,
    #[serde(rename = "currentDifficulty")]
    current_difficulty: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    nonce: String,
    balance: String,
    code: String,
    storage: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PostState {
    hash: String,
    logs: String,
    txbytes: String,
    indexes: HashMap<String, u64>,
    state: HashMap<String, Account>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(rename = "chainid")]
    chain_id: String,
}

fn parse_hex_to_address(hex_str: &str) -> Address {
    // Remove "0x" prefix if present
    let clean_hex = if hex_str.starts_with("0x") {
        &hex_str[2..]
    } else {
        hex_str
    };
    
    // Parse the hex string to bytes
    let bytes = hex::decode(clean_hex).expect("Invalid hex string");
    
    // Convert to Address (20 bytes)
    let mut addr_bytes = [0u8; 20];
    if bytes.len() <= 20 {
        addr_bytes[20 - bytes.len()..].copy_from_slice(&bytes);
    } else {
        addr_bytes.copy_from_slice(&bytes[bytes.len() - 20..]);
    }
    
    Address::from(addr_bytes)
}

fn main() {
    // Get tx from env
    let tx_data: TxEnv = env::read();
    let test_case: TestCase = env::read();

    // Create a new state instead of reading from env since State doesn't implement DeserializeOwned
    let cache_db = CacheDB::new(EmptyDBTyped::<std::convert::Infallible>::default());
    let mut state = StateBuilder::new_with_database(cache_db).build();

    // Helper function to parse hex string to U256
    fn parse_hex_to_u256(hex_str: &str) -> U256 {
        let clean_hex = if hex_str.starts_with("0x") {
            &hex_str[2..]
        } else {
            hex_str
        };
        U256::from_str_radix(clean_hex, 16).expect("Invalid hex string")
    }

    // Helper function to parse hex string to u64
    fn parse_hex_to_u64(hex_str: &str) -> u64 {
        let clean_hex = if hex_str.starts_with("0x") {
            &hex_str[2..]
        } else {
            hex_str
        };
        u64::from_str_radix(clean_hex, 16).expect("Invalid hex string")
    }

    // Helper function to parse hex string to bytes
    fn parse_hex_to_bytes(hex_str: &str) -> Vec<u8> {
        let clean_hex = if hex_str.starts_with("0x") {
            &hex_str[2..]
        } else {
            hex_str
        };
        hex::decode(clean_hex).expect("Invalid hex string")
    }

    // Insert accounts from test case
    for (addr, account) in &test_case.pre {
        state.insert_account(parse_hex_to_address(addr), AccountInfo{
            balance: parse_hex_to_u256(&account.balance),
            nonce: parse_hex_to_u64(&account.nonce),
            code: Some(Bytecode::new_raw(parse_hex_to_bytes(&account.code).into())),
            code_hash: FixedBytes::from_str("0x0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
        });
    }

    // Use Context::mainnet() method
    let mut evm = Context::mainnet()
        .with_db(&mut state)
        .modify_block_chained(|block| {
            block.beneficiary = parse_hex_to_address(&test_case.env.current_coinbase);
            block.gas_limit = parse_hex_to_u64(&test_case.env.current_gas_limit);
            block.number = parse_hex_to_u256(&test_case.env.current_number);
            block.timestamp = parse_hex_to_u256(&test_case.env.current_timestamp);
            block.difficulty = parse_hex_to_u256(&test_case.env.current_difficulty);
        })
        .build_mainnet();

    // Run EVM with tracer and capture cycles
    let (result, cycles_used) = track_cycles!("evm_transact_one", {
        evm.transact_one(tx_data)
    });

    // Extract relevant information from the result
    let output = match result {
        Ok(execution_result) => {
            ExecutionOutput {
                success: true,
                gas_used: execution_result.gas_used(),
                output: execution_result.output().map(|o| o.to_vec()),
                error: None,
                cycles_used,
            }
        }
        Err(evm_error) => {
            ExecutionOutput {
                success: false,
                gas_used: 0,
                output: None,
                error: Some(format!("{:?}", evm_error)),
                cycles_used,
            }
        }
    };
    
    // Commit the serializable output
    env::commit(&output);
}