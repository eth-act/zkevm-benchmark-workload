# EEST Test Fixtures

These two `blockchain_test_engine` fixtures come unchanged from the `fixtures_zkevm.tar.gz` bundle of
[`tests-zkevm@v21.0.1`](https://github.com/ethereum/execution-specs/releases/tag/tests-zkevm%40v21.0.1).
Their paths below `blockchain_tests_engine/` match the bundle.

| Fixture | Expected result |
| --- | --- |
| `for_amsterdam/ported_static/stShift/sar00/sar00.json` | valid block |
| `for_amsterdam/prague/eip6110_deposits/modified_contract/invalid_layout_with_swapped_decodable_offsets.json` | invalid block |

Two tests use them:

- `vendored_eest_fixtures_decode_with_guest_schema` in
  [`fixtures.rs`](../../crates/benchmark-runner/src/stateless_validator/fixtures.rs) loads the fixtures and
  decodes their stateless input and output bytes with the ere-guests schema. It runs with the default
  `cargo test`.
- [`e2e.rs`](../../crates/ere-hosts/tests/e2e.rs) executes and cost-estimates the fixtures with the Reth and
  Ethrex guests on SP1 and requires every result to match the fixture output. It needs Docker:

  ```bash
  cargo test -p ere-hosts --test e2e -- --ignored
  ```

## Refreshing

When the workspace moves to a new `tests-zkevm` release, replace these files with the same tests from the new
bundle. Keep one valid and one invalid block. Then update the fixture names and gas values in the loader test,
and run both tests.

Pick tests that every client in `e2e.rs` validates correctly. Leave out the three block access list tests
listed in [paradigmxyz/stateless#48](https://github.com/paradigmxyz/stateless/pull/48): revm and alloy-evm bugs
make Reth `0.1.0-rc.4` reject their valid blocks.
