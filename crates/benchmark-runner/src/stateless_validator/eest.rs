use anyhow::{bail, Context, Result};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, path::Path};

const EEST_SAFE_NAME_MAX_LEN: usize = 80;

#[derive(Debug, Clone)]
pub(crate) struct EestStatelessFixture {
    pub(crate) name: String,
    pub(crate) original_test_name: String,
    pub(crate) source_path: String,
    pub(crate) block_index: usize,
    pub(crate) network: String,
    pub(crate) chain_id: u64,
    pub(crate) block_number: Option<u64>,
    pub(crate) block_used_gas: Option<u64>,
    pub(crate) stateless_input_bytes: Vec<u8>,
    pub(crate) stateless_output_bytes: Vec<u8>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EestBlockchainTest {
    network: String,
    config: EestConfig,
    blocks: Vec<EestBlock>,
}

#[derive(Debug, Deserialize)]
struct EestConfig {
    chainid: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EestBlock {
    #[serde(default)]
    stateless_input_bytes: Option<String>,
    #[serde(default)]
    stateless_output_bytes: Option<String>,
    #[serde(default)]
    block_header: Option<EestBlockHeader>,
    #[serde(default, rename = "blocknumber")]
    block_number: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EestBlockHeader {
    #[serde(default)]
    number: Option<String>,
    #[serde(default)]
    gas_used: Option<String>,
}

pub(crate) fn load_eest_benchmark_fixtures(
    value: serde_json::Value,
    path: &Path,
    input_root: &Path,
) -> Result<Vec<EestStatelessFixture>> {
    let cases: BTreeMap<String, EestBlockchainTest> =
        serde_json::from_value(value).with_context(|| {
            format!(
                "Fixture {} is neither a legacy benchmark fixture nor an EEST blockchain test",
                path.display()
            )
        })?;

    let source_path = relative_source_path(path, input_root);
    let mut fixtures = Vec::new();

    for (test_name, case) in cases {
        let chain_id = parse_json_u64(&case.config.chainid)
            .with_context(|| format!("Failed to parse chainid for EEST test {test_name}"))?;

        for (block_index, block) in case.blocks.into_iter().enumerate() {
            let Some(input_hex) = block.stateless_input_bytes else {
                continue;
            };
            let stateless_input_bytes = decode_hex_bytes("statelessInputBytes", &input_hex)
                .with_context(|| {
                    format!(
                        "Failed to decode statelessInputBytes for EEST test {test_name} block {block_index}"
                    )
                })?;
            if stateless_input_bytes.is_empty() {
                continue;
            }

            let output_hex = block.stateless_output_bytes.with_context(|| {
                format!(
                    "EEST test {test_name} block {block_index} has statelessInputBytes but no statelessOutputBytes"
                )
            })?;
            let stateless_output_bytes = decode_hex_bytes("statelessOutputBytes", &output_hex)
                .with_context(|| {
                    format!(
                        "Failed to decode statelessOutputBytes for EEST test {test_name} block {block_index}"
                    )
                })?;
            let block_number = parse_optional_json_u64(
                block
                    .block_header
                    .as_ref()
                    .and_then(|header| header.number.as_deref())
                    .or(block.block_number.as_deref()),
            )
            .with_context(|| {
                format!(
                    "Failed to parse block number for EEST test {test_name} block {block_index}"
                )
            })?;
            let block_used_gas = parse_optional_json_u64(
                block
                    .block_header
                    .as_ref()
                    .and_then(|header| header.gas_used.as_deref()),
            )
            .with_context(|| {
                format!("Failed to parse gas used for EEST test {test_name} block {block_index}")
            })?;

            fixtures.push(EestStatelessFixture {
                name: eest_fixture_name(&source_path, &test_name, block_index),
                original_test_name: test_name.clone(),
                source_path: source_path.clone(),
                block_index,
                network: case.network.clone(),
                chain_id,
                block_number,
                block_used_gas,
                stateless_input_bytes,
                stateless_output_bytes,
            });
        }
    }

    Ok(fixtures)
}

fn decode_hex_bytes(field_name: &str, value: &str) -> Result<Vec<u8>> {
    let hex = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);

    if !hex.len().is_multiple_of(2) {
        bail!("{field_name} must contain an even number of hex digits");
    }

    (0..hex.len())
        .step_by(2)
        .map(|index| {
            u8::from_str_radix(&hex[index..index + 2], 16)
                .with_context(|| format!("{field_name} contains invalid hex at byte {index}"))
        })
        .collect()
}

fn parse_optional_json_u64(value: Option<&str>) -> Result<Option<u64>> {
    value.map(parse_json_u64).transpose()
}

fn parse_json_u64(value: &str) -> Result<u64> {
    let value = value.trim();
    if let Some(hex) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
    {
        return u64::from_str_radix(hex, 16)
            .with_context(|| format!("failed to parse hex u64 value {value}"));
    }

    value
        .parse()
        .with_context(|| format!("failed to parse decimal u64 value {value}"))
}

fn relative_source_path(path: &Path, input_root: &Path) -> String {
    let relative = path
        .strip_prefix(input_root)
        .ok()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or(path);

    normalize_path_string(relative)
}

fn normalize_path_string(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn eest_fixture_name(source_path: &str, test_name: &str, block_index: usize) -> String {
    let sanitized = sanitize_fixture_name(test_name);
    let hash_input = format!("{source_path}\0{test_name}\0{block_index}");
    let digest = Sha256::digest(hash_input.as_bytes());
    let hash = hex_lower(&digest[..6]);

    format!("eest__{sanitized}__block{block_index}__{hash}")
}

fn sanitize_fixture_name(value: &str) -> String {
    let mut sanitized = String::new();
    let mut last_was_separator = false;

    for ch in value.chars() {
        let next = if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
            last_was_separator = false;
            ch
        } else if last_was_separator {
            continue;
        } else {
            last_was_separator = true;
            '_'
        };

        if sanitized.len() < EEST_SAFE_NAME_MAX_LEN {
            sanitized.push(next);
        }
    }

    let sanitized = sanitized.trim_matches('_');
    if sanitized.is_empty() {
        return "fixture".to_string();
    }

    sanitized.to_string()
}

fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn load_eest_fixture_flattens_blocks_and_preserves_raw_guest_io() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let fixture_path = dir
            .path()
            .join("blockchain_tests/for_amsterdam/compute/mcopy.json");
        fs::create_dir_all(fixture_path.parent().unwrap())?;
        fs::write(&fixture_path, sample_eest_fixture())?;

        let fixtures = load_eest_benchmark_fixtures(
            serde_json::from_str(sample_eest_fixture())?,
            &fixture_path,
            dir.path(),
        )?;
        assert_eq!(fixtures.len(), 2);

        let names: Vec<_> = fixtures
            .iter()
            .map(|fixture| fixture.name.clone())
            .collect();
        assert_ne!(names[0], names[1]);
        assert!(names.iter().all(|name| name.starts_with("eest__")));
        assert!(names.iter().all(|name| !name.contains('/')));
        assert!(names.iter().all(|name| !name.contains(':')));
        assert!(names.iter().all(|name| !name.contains('[')));

        let fixture = fixtures
            .iter()
            .find(|fixture| fixture.original_test_name == "tests/foo.py::test_same[name/a]")
            .unwrap();
        assert_eq!(fixture.stateless_input_bytes, [0x00, 0x01, 0x02]);
        assert_eq!(fixture.stateless_output_bytes, [0xaa, 0xbb]);
        assert_eq!(
            fixture.source_path,
            "blockchain_tests/for_amsterdam/compute/mcopy.json"
        );
        assert_eq!(fixture.block_index, 0);
        assert_eq!(fixture.chain_id, 1);
        assert_eq!(fixture.block_number, Some(1));
        assert_eq!(fixture.block_used_gas, Some(16));

        Ok(())
    }

    #[test]
    fn eest_block_with_input_requires_output() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let fixture_path = dir.path().join("missing-output.json");
        fs::write(
            &fixture_path,
            r#"{
                "tests/foo.py::test_missing_output": {
                    "network": "Amsterdam",
                    "config": {"chainid": "0x01"},
                    "blocks": [{"statelessInputBytes": "0x0102"}]
                }
            }"#,
        )?;

        let err = load_eest_benchmark_fixtures(
            serde_json::from_str(&fs::read_to_string(&fixture_path)?)?,
            &fixture_path,
            dir.path(),
        )
        .unwrap_err();
        assert!(err
            .to_string()
            .contains("has statelessInputBytes but no statelessOutputBytes"));

        Ok(())
    }

    pub(crate) fn sample_eest_fixture() -> &'static str {
        r#"{
            "tests/foo.py::test_same[name/a]": {
                "network": "Amsterdam",
                "config": {"chainid": "0x01"},
                "blocks": [
                    {
                        "statelessInputBytes": "0x000102",
                        "statelessOutputBytes": "0xaabb",
                        "blockHeader": {"number": "0x01", "gasUsed": "0x10"}
                    },
                    {
                        "blockHeader": {"number": "0x02", "gasUsed": "0x20"}
                    },
                    {
                        "statelessInputBytes": "0x",
                        "statelessOutputBytes": "0xcc"
                    }
                ]
            },
            "tests/foo.py::test_same[name?a]": {
                "network": "Amsterdam",
                "config": {"chainid": "0x01"},
                "blocks": [
                    {
                        "statelessInputBytes": "0x0f",
                        "statelessOutputBytes": "0xdead",
                        "blocknumber": "0x03",
                        "blockHeader": {"gasUsed": "0x30"}
                    }
                ]
            }
        }"#
    }
}
