use std::path::Path;
use ere_dockerized::ErezkVM;
use reth_stateless::ExecutionWitness;
use walkdir::WalkDir;
use zkvm_interface::Input;
use witness_generator::BlockAndWitness;
use crate::guest_programs::{GuestIO, OutputVerifier, OutputVerifierResult};

/// Verifies the output of the program.
#[derive(Debug, Clone)]
pub struct ProgramOutputVerifier;

impl OutputVerifier for ProgramOutputVerifier {
    fn check_serialized(&self, zkvm: ErezkVM, bytes: &[u8]) -> anyhow::Result<OutputVerifierResult> {
        match zkvm {
            ErezkVM::Risc0 => match bytes.is_empty()
            {
                true => Ok(OutputVerifierResult::Match),
                false => Ok(OutputVerifierResult::Mismatch(format!(
                    "Expected empty output, got {bytes:?}",
                ))),
            },
            _ => todo!("Output verification not implemented for this zkVM"),
        }
    }
}

/// Generate inputs for the trie bench guest program.
pub fn trie_bench_inputs(
    input_folder: &Path
) -> anyhow::Result<Vec<GuestIO<(), ProgramOutputVerifier>>> {
    let guest_inputs = read_benchmark_fixtures_folder(input_folder)?
        .into_iter()
        .map(|bw| {
            Ok(GuestIO {
                name: bw.name,
                input: write_stdin(&bw.block_and_witness.witness)?,
                output: ProgramOutputVerifier,
                metadata: (),
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(guest_inputs)
}

/// Reads the benchmark fixtures folder and returns a list of block and witness pairs.
fn read_benchmark_fixtures_folder(path: &Path) -> anyhow::Result<Vec<BlockAndWitness>> {
    WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .collect::<anyhow::Result<Vec<_>, _>>()?
        .into_iter()
        .map(|entry| {
            if entry.file_type().is_file() {
                let content = std::fs::read(entry.path())?;
                let bw: BlockAndWitness = serde_json::from_slice(&content).map_err(|e| {
                    anyhow::anyhow!("Failed to parse {}: {}", entry.path().display(), e)
                })?;
                Ok(bw)
            } else {
                anyhow::bail!("Invalid input folder structure: expected files only")
            }
        })
        .collect::<anyhow::Result<Vec<BlockAndWitness>>>()
}

fn write_stdin(ew: &ExecutionWitness) -> anyhow::Result<Input> {
    let mut stdin = Input::new();
    stdin.write(ew.clone());

    Ok(stdin)
}