# Precompile Patches

This directory contains patches for zkVM precompiles used within the zkVM guest programs.

## Structure

Patch configurations are defined in TOML files, named after the corresponding zkVM platform:

- `sp1.toml`: Defines patches applied when building for the SP1 platform.
- `risc0.toml`: Defines patches applied when building for the Risc Zero platform.
- `pico.toml`: Defines patches applied when building for the Pico platform.
- `zisk.toml`: Defines patches applied when building for the Zisk platform.
- `openvm.toml`: Defines patches applied when building for the OpenVM platform.
- `airbender.toml`: Defines patches applied when building for the Airbender platform.
- `zkm.toml`: Defines patches applied when building for the zkMIPS platform.

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

# Example for OpenVM host
cargo openvm build --release -p ere-hosts
```

**Note:** It is not necessary to call these commands manually, since `ere-host` automatically applies the patches as part of its execution process depending on which zkVM is being targeted.
