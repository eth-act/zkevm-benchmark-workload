//! Risc0 guest program

extern crate alloc;

use risc0_zkvm::guest::env;
use alloy_consensus::Header;
use alloy_primitives::{Address, B256, FixedBytes, U256, keccak256};
use reth_primitives_traits::SealedHeader;
use reth_stateless::validation::StatelessValidationError;
use reth_stateless::{ExecutionWitness, StatelessTrie};
use reth_stateless::trie::StatelessSparseTrie;
use sparsestate::SparseState;
use std::collections::HashMap;

fn create_stateless_trie<T: StatelessTrie>(
    witness: &ExecutionWitness,
    state_root: B256,
) -> T {
    println!("create_stateless_trie {}", std::any::type_name::<T>());
    let start = env::cycle_count();
    let trie = T::new(&witness, state_root).unwrap().0;
    let end = env::cycle_count();
    eprintln!("create_stateless_trie {} (cycle tracker): {}", std::any::type_name::<T>(), end - start);
    trie
}

fn account_bench<T: StatelessTrie>(trie: &mut T, witness: &ExecutionWitness) {
    let addresses: Vec<Address> = witness
        .keys
        .iter()
        .filter(|key| key.len() == 20)
        .map(|key| Address::from(FixedBytes::<20>::from_slice(key)))
        .collect();

    println!("account_bench {}", std::any::type_name::<T>());
    let start = env::cycle_count();
    for address in addresses.iter() {
        let r = trie.account(address.clone());
        if r.is_err() {
            panic!("account_bench {}: {:?}", std::any::type_name::<T>(), r);
        }
    }
    let end = env::cycle_count();
    eprintln!("account_bench {} (cycle tracker): {}", std::any::type_name::<T>(), end - start);
}

fn storage_bench<T: StatelessTrie>(trie: &mut T, witness: &ExecutionWitness) {
    let storage = build_storage_hash_map(&witness);

    println!("storage_bench {}", std::any::type_name::<T>());
    let start = env::cycle_count();
    for (address, slots) in storage.iter() {
        for slot in slots {
            println!("storage_bench {} : {}", address, slot);
            let r = trie.storage(address.clone(), slot.clone());
            if r.is_err() {
                panic!("storage_bench {}: {:?}", std::any::type_name::<T>(), r);
            }
        }
    }
    let end = env::cycle_count();
    eprintln!("storage_bench {} (cycle tracker): {}", std::any::type_name::<T>(), end - start);
}

fn run_trie_bench<T: StatelessTrie>(witness: &ExecutionWitness, state_root: B256) {
    let mut trie = create_stateless_trie::<T>(witness, state_root);
    account_bench::<T>(&mut trie, witness);
    storage_bench::<T>(&mut trie, witness);
}

/// Entry point.
pub fn main() {
    let witness = env::read::<ExecutionWitness>();

    let state_root = get_state_root(&witness);

    run_trie_bench::<StatelessSparseTrie>(&witness, state_root);
    run_trie_bench::<SparseState>(&witness, state_root);
}

pub fn get_state_root(witness: &ExecutionWitness) -> B256 {
    let mut ancestor_headers: Vec<_> = witness
        .headers
        .iter()
        .map(|bytes| {
            let hash = keccak256(bytes);
            alloy_rlp::decode_exact::<Header>(bytes)
                .map(|h| SealedHeader::new(h, hash))
                .map_err(|_| StatelessValidationError::HeaderDeserializationFailed)
        })
        .collect::<Result<_, _>>()
        .unwrap();
    // Sort the headers by their block number to ensure that they are in
    // ascending order.
    ancestor_headers.sort_by_key(|header| header.number);

    // There should be at least one ancestor header.
    // The edge case here would be the genesis block, but we do not create proofs for the genesis
    // block.
    let parent = match ancestor_headers.last() {
        Some(prev_header) => prev_header,
        None => panic!("Parent not in ancestor headers"),
    };

    parent.state_root
}

pub fn build_storage_hash_map(witness: &ExecutionWitness) -> HashMap<Address, Vec<U256>> {
    use std::collections::HashMap;

    let mut storage = HashMap::new();
    if witness.keys.len() > 0 {
        let mut current_address = Address::default();
        for k in witness.keys.iter() {
            if k.len() == 20 {
                current_address = Address::from(FixedBytes::<20>::from_slice(&k));
                storage.insert(current_address, vec![]);
            } else if k.len() == 32 {
                match storage.get_mut(&current_address) {
                    None => {
                        panic!("Account not found");
                    }
                    Some(value) => {
                        value.push(U256::from_be_slice(k.as_ref()));
                    }
                }
            } else {
                panic!("Invalid key length");
            }
        }
    }
    storage
}
