# Precompile Patches

This directory contains patches for dependencies used within the zkVM guest programs (e.g., `succinct-guest`, `zkm-guest`).

## Purpose

Certain dependencies, particularly cryptographic libraries or those interacting heavily with low-level primitives, may require modifications to be:

1. **Compatible:** Ensure they work correctly within the specific constraints and environment of a zkVM. (Usually this means `no_std`)
2. **Efficient:** Optimize their performance when executed inside the zkVM, as standard implementations might be unnecessarily costly in a ZK context. (Usually this is done by having a circuit implementation of the algorithm and exposing that as a precompile that the patches will call)

These patches apply the necessary modifications directly to the source code of the dependencies before the guest programs are compiled.

## Structure

Patch configurations are defined in TOML files, named after the corresponding zkVM platform:

- `sp1.toml`: Defines patches applied when building for the SP1 platform.
- `risc0.toml`: Defines patches applied when building for the Risc Zero platform.
- `pico.toml`: Defines patches applied when building for the Pico platform.
- `zisk.toml`: Defines patches applied when building for the Zisk platform.

These TOML files specify which crates need patching and point to the repositories containing the modified source.

## Application

The application of these patches is automated via the workspace's `xtask` runner.

As mentioned in the main `README.md`, running `cargo <zkvm-name>` (e.g., `cargo sp1`, `cargo risc0`) will trigger the `xtask` for that specific zkVM. This task reads the corresponding `.toml` file (`sp1.toml`, `risc0.toml`, etc.) and applies the specified patches to the relevant dependencies within the `[patch.crates-io]` section of the workspace `Cargo.toml`.

Since the `xtask` integrates with cargo, you can chain standard cargo commands after the zkVM name. For instance, to ensure patches are applied (if needed by the xtask) and then build the corresponding host program, you could run:

```bash
# Example for SP1 host
cargo sp1 build --release -p ere-hosts 

# Example for Risc Zero host
cargo risc0 build --release -p ere-hosts
```

**Note:** With the new EreDockerized system, manual patching is generally not required as Docker containers handle the patched environments automatically.
