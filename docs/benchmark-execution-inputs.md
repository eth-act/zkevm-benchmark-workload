# Benchmark Execution Inputs

This is the detailed input reference for `ere-hosts`, especially the `stateless-validator` guest program.

For the operator workflow and common commands, see [Benchmark Execution](benchmark-execution.md). For generated fixture workflows, see [Fixture Generation](fixture-generation.md).

## Input Discovery

Default input location:

```text
zkevm-fixtures-input/
```

Override it with `stateless-validator --input-folder <PATH>`.

`ere-hosts` accepts either:

- A directory containing fixture JSON files.
- A single `.json` fixture file.

When the input is a directory, `ere-hosts` recursively loads `.json` files and skips any file under a `.meta/` directory.

When the input folder contains an EEST `blockchain_tests/` subdirectory, only that subtree is used for `stateless-validator` inputs. EEST bundles that only contain `blockchain_tests_engine/`, `blockchain_tests_engine_x/`, or `blockchain_tests_sync/` are rejected because those fixture formats do not contain the canonical stateless validator bytes.

## Fixture Selection

`--fixture <PREFIX>` filters selected fixtures by fixture-name prefix:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    stateless-validator --execution-client reth \
    --fixture test_sha256.py::test_sha256 \
    --fixture test_memory.py::test_mcopy
```

For direct EEST fixtures, the prefix may match either the sanitized generated fixture name or the original EEST test name.

## Accepted Fixture Formats

`stateless-validator` accepts two fixture JSON formats:

- Legacy generated fixtures with a top-level `stateless_input` field.
- Direct EEST `blockchain_tests` fixtures whose executable blocks contain `statelessInputBytes` and `statelessOutputBytes`.

Use direct EEST inputs for Reth and Ethrex. Zesu accepts only inputs whose decoded canonical chain configuration selects `ProtocolFork::Amsterdam` (the Glamsterdam guest path). Legacy generated fixtures are accepted only when `--execution-client zilkworm` is selected.

## Legacy Generated Fixtures

Legacy generated fixtures are the repo-native files produced by `witness-generator-cli` from RPC, raw-input, or legacy EEST generation. `ere-hosts` detects this format by the presence of a top-level `stateless_input` field. Each file contains one fixture object.

```json
{
  "name": "rpc_block_23743854",
  "stateless_input": {
    "block": {
      "...": "Ethereum block data"
    },
    "witness": {
      "...": "execution witness data"
    },
    "chain_config": {
      "...": "chain configuration"
    }
  },
  "success": true
}
```

Fields:

- `name`: Fixture name. This becomes the metrics file name stem.
- `stateless_input`: A serialized `stateless::StatelessInput` containing `block`, `witness`, and `chain_config`.
- `success`: Whether the fixture represents a valid block validation.

Generated repo fixtures are converted into Zilkworm's unified-RLP guest input format before execution. Metrics metadata for these fixtures is `{"block_used_gas": <u64>}`. Selecting a legacy fixture with Reth, Ethrex, or Zesu returns a format error directing the operator to canonical fixtures.

## Direct EEST Blockchain Tests Fixtures

Direct EEST inputs must be `blockchain_tests` fixtures whose executable blocks contain canonical stateless validator input and output bytes.

```json
{
  "tests/foo.py::test_case[param]": {
    "network": "Amsterdam",
    "config": {
      "chainid": "0x01"
    },
    "blocks": [
      {
        "statelessInputBytes": "0x000102",
        "statelessOutputBytes": "0xaabb",
        "blockHeader": {
          "number": "0x01",
          "gasUsed": "0x10"
        }
      }
    ]
  }
}
```

Rules:

- The file is a JSON object keyed by original EEST test name.
- Each test case must include `network`, `config.chainid`, and `blocks`.
- Additional EEST fields may be present and are ignored by `ere-hosts`.
- `config.chainid`, `blockHeader.number`, `blocknumber`, and `blockHeader.gasUsed` may be decimal strings or `0x`-prefixed hex strings.
- A block without `statelessInputBytes` is skipped.
- A block with empty `statelessInputBytes` is skipped.
- A block with non-empty `statelessInputBytes` must also have `statelessOutputBytes`.
- `statelessInputBytes` and `statelessOutputBytes` are hex strings with or without `0x`; after removing the prefix they must contain an even number of hex digits.

Each accepted EEST block becomes one benchmark fixture. `ere-hosts` sends `statelessInputBytes` directly to the selected guest program without EL-specific host conversion. The expected public values are the raw SSZ `statelessOutputBytes` emitted by the v0.13 stateless validator guests.

The generated fixture name is derived from the original test name, source path, and block index. Output metadata preserves those original EEST fields; see [Benchmark Execution Output](benchmark-execution-output.md#metadata-by-workload).
