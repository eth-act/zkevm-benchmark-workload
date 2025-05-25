use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use witness_generator::{generate_stateless_witness, BlocksAndWitnesses};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci-program");

fn main() {
    let client = ProverClient::from_env();

    let generated_corpuses = generate_stateless_witness::generate();
    for corpus in generated_corpuses {
        if !corpus
            .name
            .contains("fork_Cancun-blockchain_test-absent_target_False-opcode_CALLCODE")
        {
            continue;
        }

        let last_block_with_witness = match corpus.blocks_and_witnesses.last() {
            Some(last_block_with_witness) => last_block_with_witness,
            None => panic!("unexpected test with no blocks {}", &corpus.name),
        };

        let mut stdin = SP1Stdin::new();
        stdin.write(&last_block_with_witness);
        stdin.write(&corpus.network);

        let (_, report) = client.execute(FIBONACCI_ELF, &stdin).run().unwrap();

        panic!("name {}", &corpus.name);
    }
}
