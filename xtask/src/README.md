# xtask – Workspace Patch Manager

This is a helper CLI for managing `[patch.crates-io]` entries in the workspace `Cargo.toml`.

## Usage

- `cargo sp1` - Apply SP1 patches
- `cargo risc0` - Apply RISC0 patches  
- `cargo pico` - Apply Pico patches
- `cargo zisk` - Apply Zisk patches
- `cargo zkm` - Apply zkMIPS patches

## EreDockerized Integration

With the new EreDockerized system, most patch management is handled automatically within Docker containers. The xtask is primarily used for local development and testing scenarios where manual patch application may be needed.
