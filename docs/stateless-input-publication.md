# Stateless Input Publication

This guide describes how to publish canonical stateless input batches as a public R2 dataset.

## Public Dataset Shape

R2 public buckets do not provide directory listing, so share the generated HTML page rather than the bucket or prefix root:

```text
https://<public-host>/devnets/<network>/index.html
```

The public dataset is batch-first. Users should download complete `.tar.zst` batch archives from:

```text
exports/batches/<start>-<end>.tar.zst
```

Individual block artifacts remain described inside each batch archive's `manifest.json`, but they are not published as standalone public objects.

## Generated Catalog Files

Running `witness-generator-spec-cli export` rebuilds these files at the network root:

- `index.html`: human-readable landing page with download examples and a batch table.
- `manifest.json`: dataset summary and paths to all public metadata files.
- `batches.jsonl`: one completed batch archive per line.
- `SHA256SUMS`: checksums for completed batch archives.

The generated links are relative, so the same catalog works with an `r2.dev` development URL or a custom domain.

## Operator Flow

Collect live stateless inputs:

```bash
cargo run -p witness-generator-spec-cli --release -- collect \
    --config /etc/witness-generator-spec-cli/glamsterdam-devnet-5.toml
```

Export complete local block ranges and rebuild the public catalog:

```bash
cargo run -p witness-generator-spec-cli --release -- export \
    --config /etc/witness-generator-spec-cli/glamsterdam-devnet-5.toml
```

Publish batch archives and catalog files to R2:

```bash
cargo run -p witness-generator-spec-cli --release -- publish-r2 \
    --config /etc/witness-generator-spec-cli/glamsterdam-devnet-5.toml
```

If `publish-r2` reports a missing public catalog file, run `export` first.

## User Download Examples

Download a batch archive from the generated HTML page:

```bash
curl -LO https://<public-host>/devnets/<network>/exports/batches/32500-32999.tar.zst
tar --zstd -xf 32500-32999.tar.zst
```

Verify checksums:

```bash
curl -LO https://<public-host>/devnets/<network>/SHA256SUMS
sha256sum -c SHA256SUMS
```

Inspect machine-readable metadata:

```bash
curl -fsSL https://<public-host>/devnets/<network>/manifest.json | jq
curl -fsSL https://<public-host>/devnets/<network>/batches.jsonl | head
```
