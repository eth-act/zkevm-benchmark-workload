# Benchmark Execution Inputs

This is the detailed input reference for the `ere-hosts stateless-validator` workload. For commands and proof operations, see [Benchmark Execution](benchmark-execution.md).

## Action-Aware Input Requirement

Execute, estimate-cost, and prove actions require:

```text
stateless-validator --input-folder <PATH>
```

There is no default input location. The path must exist and may identify:

- One EEST `blockchain_test_engine` or `blockchain_test` `.json` fixture file.
- A directory of such files.
- An EEST fixture checkout or archive root containing `blockchain_tests_engine/` or `blockchain_tests/`.

Verification does not need fixture input. If `--input-folder` is supplied with `--action verify`, the option is accepted and ignored for backward compatibility, including when its path no longer exists.

## Discovery

Directory input is walked recursively in sorted filename order. Only `.json` files are considered, and files below `.meta/` are excluded.

If the input path contains `blockchain_tests_engine/`, the runner reads only that subtree. Otherwise, it reads `blockchain_tests/`. `tests-zkevm` bundles ship both formats; `tests-zkevm-benchmark` bundles ship only `blockchain_test`.

The runner rejects an EEST bundle that has neither subdirectory. It recognizes a bundle by a `blockchain_tests_engine_x/` or `blockchain_tests_sync/` subdirectory, or by a `.meta/index.json` that lists `fixture_formats`. It walks any other directory as is.

An empty directory retains the existing discovery behavior: it produces no fixture paths.

Batch archives exported by `witness-generator-spec-cli` use this layout. After
extracting one, pass the extraction root as `--input-folder`; discovery selects
its `blockchain_tests_engine/` subtree and ignores `.meta/manifest.json`.

## Canonical EEST Schema

A `blockchain_test_engine` fixture is a JSON object whose `engineNewPayloads` entries contain `statelessInputBytes` and `statelessOutputBytes`:

```json
{
  "tests/foo.py::test_case[param]": {
    "network": "Amsterdam",
    "config": {
      "chainid": "0x01"
    },
    "engineNewPayloads": [
      {
        "params": [
          {
            "blockNumber": "0x01",
            "gasUsed": "0x10"
          }
        ],
        "statelessInputBytes": "0x150102",
        "statelessOutputBytes": "0xaabb"
      }
    ],
    "_info": {
      "metadata": {
        "opcode_count_per_block": [
          {
            "PUSH1": 5,
            "SSTORE": 2
          }
        ],
        "target_opcode": "MCOPY"
      }
    }
  }
}
```

Rules:

- The file is a JSON object keyed by the original EEST test name.
- Each test case includes `network`, `config.chainid`, and a block list: `engineNewPayloads` in `blockchain_test_engine`, `blocks` in `blockchain_test`. Other EEST fields are ignored.
- The block number and gas used come from `params[0].blockNumber` and `params[0].gasUsed` in `blockchain_test_engine`, and from `blockHeader.number` and `blockHeader.gasUsed` in `blockchain_test`. Both are optional.
- `config.chainid`, the block number, and the gas used may be decimal strings or `0x`-prefixed hexadecimal strings.
- Only the last block of a test case is loaded, because EEST benchmark tests place the worst case block last and use any preceding blocks for setup.
- A block without `statelessInputBytes` is skipped. A block with empty `statelessInputBytes` is loaded, because EEST uses it as a conformance case.
- A block with `statelessInputBytes` must also contain `statelessOutputBytes`.
- Both byte fields are hexadecimal strings with an optional `0x` prefix and an even number of hexadecimal digits after that prefix.
- `_info.metadata.opcode_count_per_block` holds one opcode-count map per block in order, so its last entry describes the loaded block. If the array length differs from the block count, the loader logs a warning and omits opcode counts for that test case. The fixture is still loaded, and guest input/output validation still applies.
- `_info.metadata.target_opcode` names the opcode a benchmark stresses. Benchmarks that stress no single opcode omit it.

Each loaded block becomes one benchmark fixture, named from the original EEST test name and block index. Colliding names get a numeric suffix. The original test name remains available for fixture-prefix selection.

## Execution-Client Routing

Reth, Ethrex, Zesu, and Nimbus receive `statelessInputBytes` unchanged on stdin and compare public values with `statelessOutputBytes`.
Reth `0.1.0-rc.3` and Ethrex `27.0.0` support all three zkVMs.
Zesu `tests-glamsterdam-devnet@v8.1.4` and Nimbus `v0.1.0-alpha` support ZisK only.

Fixture deserialization remains independent of the selected execution client.
Client-specific availability is checked before artifact resolution or guest
execution.

## Fixture Selection

Repeat `--fixture <PREFIX>` to select one or more fixture-name prefixes:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    stateless-validator --execution-client reth \
    --input-folder /path/to/eest-fixtures \
    --fixture test_sha256.py::test_sha256 \
    --fixture test_memory.py::test_mcopy
```

A prefix may match either the sanitized fixture name or the original EEST test name. A `.json` suffix is ignored during prefix normalization, repeated prefixes are deduplicated, and empty prefixes are rejected.

## Metadata, Existing Outputs, And Public Values

Benchmark metadata preserves the fixture format, original test name, source path, block index, network, chain ID, block number, gas used, the block's opcode count, and the target opcode. See [Benchmark Execution Output](benchmark-execution-output.md#metadata-by-workload) for the serialized shape.

Unless `--force-rerun` is set, fixture preparation skips cases that already contain a result for the selected action. Execution, estimation, and proving compare the guest's public values with the fixture's raw `statelessOutputBytes`; proof verification retains the existing stored-proof verification behavior.

## Legacy Format Rejection

JSON with a top-level `stateless_input` field is rejected before canonical deserialization with this migration error:

```text
legacy fixture format with top-level stateless_input is no longer supported; provide an EEST blockchain_test_engine or blockchain_test fixture containing statelessInputBytes and statelessOutputBytes
```

The old fixture generator crates and image are discontinued. Existing fixture, metric, proof, and published image files remain untouched, but legacy fixture JSON must be replaced with canonical EEST `blockchain_test_engine` or `blockchain_test` input before it can be benchmarked.
