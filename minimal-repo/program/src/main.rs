//! SP1 guest program

#![no_main]

extern crate alloc;

use alloc::sync::Arc;

use reth_stateless::{fork_spec::ForkSpec, validation::stateless_validation, ClientInput};

sp1_zkvm::entrypoint!(main);

use alloc::alloc::{alloc, Layout};
pub fn main() {
    println!("Attempting to allocate just over 2GB using direct alloc");

    // Try to allocate 2GB + 1 byte
    let size = 0x78000001; // 1 more byte than the bump allocator allows

    println!("Creating layout for {} bytes", size);
    let layout = Layout::from_size_align(size, 8).unwrap();

    println!("Calling alloc() - this should panic with 'Memory limit exceeded'");
    let ptr = unsafe { alloc(layout) };

    // Adding this line, so that the compiler doesn't optimize out the alloc
    if ptr.is_null() {
        println!("Allocation returned null pointer");
    } else {
        println!("Allocation succeeded unexpectedly at address: {:p}", ptr);
        // Try to write to verify the allocation
        unsafe { ptr.write(0x42) };
        println!("Successfully wrote to allocated memory");
    }
}
