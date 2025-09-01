//! SP1 panic guest program

#![no_main]

sp1_zkvm::entrypoint!(main);
pub fn main() {
    panic!("The ticker is eth")
}
