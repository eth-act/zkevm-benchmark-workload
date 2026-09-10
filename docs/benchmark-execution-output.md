# Benchmark Execution Output

This is the detailed output reference for `ere-hosts`: metrics files, hardware metadata, proof files, serialized input dumps, and workload metadata.

For accepted input schemas, see [Benchmark Execution Inputs](benchmark-execution-inputs.md). For common execution commands, see [Benchmark Execution](benchmark-execution.md).

## Output Destinations

Default output locations:

- Metrics output folder: `zkevm-metrics/`
- Verification proof folder: `zkevm-fixtures-proofs/`
- Zisk profile output folder: `zisk-profiles/`

Proofs are only saved when `--save-proofs <PATH>` is provided with `--action prove`.

Serialized guest inputs are only saved when `--dump-inputs <PATH>` is provided. These dumps are zkVM-independent, so each fixture input is written once even if multiple zkVMs are selected.

## Metrics Layout

Completed execution, proving, and estimation runs retain their measurements when public values do not match the fixture.
The runner logs a warning and sets `output_matched: false` in the selected success payload.
Estimator errors and panics produce `cost_estimation.crashed`.
Existing execution, proving, and verification error handling remains unchanged.

For `stateless-validator` runs, metrics are written under:

```text
zkevm-metrics/
  hardware.json
  <execution-client>-<execution-client-version>/
    <zkvm>-<sdk-version>/
      <fixture-name>.json
```

Each fixture metrics file is a single pretty-printed `BenchmarkRun` JSON object. The `zkevm-metrics` library helper `BenchmarkRun::to_json` serializes a list of runs, but the CLI output files under `zkevm-metrics/` contain one object per file.

## Hardware JSON

`hardware.json` contains detected host hardware:

```json
{
  "cpu_model": "AMD Ryzen 7 PRO 7840U w/ Radeon 780M Graphics",
  "total_ram_gib": 30,
  "gpus": [
    {
      "model": "NVIDIA ..."
    }
  ]
}
```

GPU information is detected through `nvidia-smi` when available.

## BenchmarkRun JSON

A successful execution metrics file has this shape:

```json
{
  "name": "eest__tests_foo_py_test_case_param__block0",
  "timestamp_completed": "2026-05-25T12:34:56.789Z",
  "metadata": {
    "fixture_format": "eest",
    "original_test_name": "tests/foo.py::test_case[param]",
    "source_path": "blockchain_tests/for_amsterdam/compute/mcopy.json",
    "block_index": 0,
    "network": "Amsterdam",
    "chain_id": 1,
    "block_number": 1,
    "block_used_gas": 16,
    "opcode_count": {
      "PUSH1": 5,
      "SSTORE": 2
    },
    "target_opcode": "MCOPY"
  },
  "execution": {
    "success": {
      "output_matched": true,
      "execution_duration": {
        "secs": 12,
        "nanos": 327837000
      }
    }
  }
}
```

Optional top-level fields are omitted when they are not populated:

- `execution` is present for `--action execute`.
- `proving` is present for `--action prove`.
- `verification` is present for `--action verify`.
- `cost_estimation` is present for `--action estimate-cost`.

Several action fields can coexist in one file. Each action replaces only its own payload.
Existing results for other actions remain intact. `--force-rerun` follows the same merge behavior.
Without that flag, an existing result for the selected action causes a skip, including a recorded crash.
`timestamp_completed` identifies the latest completed action update.
Updates replace the file atomically. Invalid existing JSON stops the update without overwriting the file.
Actions that target the same fixture file must run sequentially across processes.

Execution success no longer contains `total_num_cycles` or `region_cycles`.
Archived metrics are not rewritten in bulk. Unmodified action payloads retain their original fields during a merge.

Success variants:

```json
{
  "execution": {
    "success": {
      "output_matched": true,
      "execution_duration": {
        "secs": 0,
        "nanos": 1000000
      }
    }
  }
}
```

```json
{
  "proving": {
    "success": {
      "output_matched": true,
      "proof_size": 256,
      "proving_time_ms": 2000,
      "verification_time_ms": 200
    }
  }
}
```

```json
{
  "verification": {
    "success": {
      "proof_size": 256,
      "verification_time_ms": 200
    }
  }
}
```

Crash variants use the same enum wrapper with `crashed`:

```json
{
  "execution": {
    "crashed": {
      "reason": "error or panic message"
    }
  }
}
```

## Cost Estimation

A cost result uses the following structure. The hashes below are illustrative placeholders.

```json
{
  "cost_estimation": {
    "success": {
      "output_matched": true,
      "cost": {"opcode": 100, "syscall": 20, "system": 30},
      "peak_heap_bytes": null,
      "context": {
        "execution_client": "reth",
        "execution_client_version": "0.1.0-rc.3",
        "zkvm": "sp1",
        "sdk_version": "v6.4.0",
        "ere_revision": "5023513",
        "elf_sha256": "<SHA-256 of actual ELF bytes>",
        "input_sha256": "<SHA-256 of raw input bytes>",
        "estimator_settings": {"heap_start": "_end"}
      }
    }
  }
}
```

The runner preserves upstream component names and unsigned 64-bit values. Each backend defines its own cost model.
These values estimate proving work. They are not prices, execution cycles, or measured proving time.

| zkVM | Unit | Components |
| --- | --- | --- |
| OpenVM | Unpadded trace cells, summed across segments | `rv64`, `precompile`, `system` |
| SP1 | `3 * trace_area + complexity` (ten times SP1 gas) | `opcode`, `syscall`, `system` |
| ZisK | Trace cells | `base`, `precompile`, `memory`, `opcode`, `main` |

Component names and units follow [Ere v0.17.0](https://github.com/eth-act/ere/tree/v0.17.0/crates/prover).
Raw costs must not be compared across zkVMs, SDK versions, Ere revisions, or estimator settings.

`peak_heap_bytes` is an estimator measurement of guest heap memory. It is not host RAM or a precise allocator high-water mark.
OpenVM and ZisK estimate the span of nonzero heap bytes. SP1 measures guest memory above its heap symbol.
Missing symbols or unreadable heap memory can produce `null`. A missing measurement is not zero.

The context records effective settings that affect measurement:

- `heap_start`: `ERE_COST_ESTIMATION_HEAP_START`, default `_end` for OpenVM/SP1 or `_heap_bottom` for ZisK.
- `heap_end`: ZisK's `ERE_COST_ESTIMATION_HEAP_END`, default `_heap_top`.
- `segment_memory_bytes`: OpenVM's `ERE_OPENVM_SEGMENT_MEMORY`, default `15569256448` bytes (14.5 GiB).

Invalid OpenVM segment-memory values use the upstream default. Scheduling concurrency does not change the recorded cost model.
ELF hashes identify custom artifacts even when the client version comes from the pinned catalog.

## Metadata By Workload

The `metadata` field is workload-specific:

- Canonical EEST stateless-validator fixtures write EEST provenance and block metadata.
- Verification preserves existing fixture metadata. A new verification-only record writes `null`. A later fixture action fills it.

Canonical EEST metadata has this shape:

```json
{
  "fixture_format": "eest",
  "original_test_name": "tests/foo.py::test_case[param]",
  "source_path": "blockchain_tests/for_amsterdam/compute/mcopy.json",
  "block_index": 0,
  "network": "Amsterdam",
  "chain_id": 1,
  "block_number": 1,
  "block_used_gas": 16,
  "opcode_count": {
    "PUSH1": 5,
    "SSTORE": 2
  },
  "target_opcode": "MCOPY"
}
```

`block_number` and `block_used_gas` are `null` when the source fixture does not provide those values. `opcode_count` is the opcode tally of the benchmarked block taken from `_info.metadata.opcode_count_per_block`, and `target_opcode` is the opcode the benchmark stresses taken from `_info.metadata.target_opcode`. Either key is omitted when the source fixture supplies no value.

## Proofs And Verification

Generate and save proofs:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    --action prove \
    --save-proofs my-proofs \
    stateless-validator --execution-client reth \
    --input-folder /path/to/eest-fixtures
```

Verify proofs from a local folder:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    --action verify \
    --proofs-folder my-proofs \
    stateless-validator --execution-client reth
```

Verify proofs from a remote archive:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    --action verify \
    --proofs-url https://example.com/proofs.tar.gz \
    stateless-validator --execution-client reth
```

When `--proofs-url` is used, the archive is downloaded, extracted to a temporary directory, and cleaned up after verification.

## Input Dumps

Dump the raw serialized guest inputs used for a run:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    --dump-inputs debug-inputs \
    stateless-validator --execution-client reth \
    --input-folder /path/to/eest-fixtures
```

Use these dumps to inspect the canonical `statelessInputBytes` passed to the guest after fixture loading.
