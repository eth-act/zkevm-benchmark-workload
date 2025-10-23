//! Airbender guest program

#![no_std]
#![no_main]
#![no_builtins]
#![allow(incomplete_features)]
#![feature(allocator_api)]
#![feature(generic_const_exprs)]

extern crate alloc;

use alloc::vec::Vec;
use core::{array, iter::repeat_with};
use reth_guest::{
    guest::ethereum_guest,
    sdk::{SDK, ScopeMarker},
};
use riscv_common::{csr_read_word, zksync_os_finish_success};

mod airbender_rt;

#[allow(missing_debug_implementations)]
struct AirbenderSDK;

impl SDK for AirbenderSDK {
    fn read_input() -> Vec<u8> {
        let len = csr_read_word() as usize;
        repeat_with(csr_read_word)
            .take(len.div_ceil(4))
            .flat_map(|word| word.to_le_bytes())
            .take(len)
            .collect()
    }

    fn commit_output(output: [u8; 32]) {
        let words = array::from_fn(|i| u32::from_le_bytes(array::from_fn(|j| output[4 * i + j])));
        zksync_os_finish_success(&words);
    }

    fn cycle_scope(_: ScopeMarker, _: &str) {}
}

/// Entry point.
pub fn main() {
    ethereum_guest::<AirbenderSDK>();
}
