#!/usr/bin/env python3
"""Download and organize raw input files for the witness generator.

Reads a text file containing URLs (one per line) pointing to eth_block.json
and debug_executionWitness.json files, downloads them, and organizes them
into the directory structure expected by the raw input fixture generator:

    output_folder/
    ├── chain_config.json
    ├── <test_name_1>/
    │   ├── eth_block.json
    │   └── debug_executionWitness.json
    └── <test_name_2>/
        ├── eth_block.json
        └── debug_executionWitness.json

Usage:
    python scripts/download_raw_inputs.py \\
        --url-list raw_input_parts.txt \\
        --chain-config /path/to/chain_config.json \\
        --output ./raw_inputs \\
        --workers 16
"""

import argparse
import shutil
import sys
import urllib.request
from collections import defaultdict
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from urllib.parse import urlparse


EXPECTED_FILENAMES = {"eth_block.json", "debug_executionWitness.json"}


def parse_url(url: str) -> tuple[str, str]:
    """Extract (test_name, filename) from a raw input URL.

    Expected URL path structure:
        .../runs/<run_id>/<fixture_num>/<test_name>/post_test_rpc_calls/<filename>
    """
    path = urlparse(url).path
    parts = path.strip("/").split("/")

    filename = parts[-1]
    if filename not in EXPECTED_FILENAMES:
        raise ValueError(
            f"Unexpected filename '{filename}' in URL: {url}\n"
            f"Expected one of: {EXPECTED_FILENAMES}"
        )

    # Walk backwards to find 'post_test_rpc_calls' marker
    try:
        rpc_idx = parts.index("post_test_rpc_calls")
    except ValueError:
        raise ValueError(
            f"Could not find 'post_test_rpc_calls' in URL path: {url}"
        )

    test_name = parts[rpc_idx - 1]
    return test_name, filename


def download_file(url: str, dest: Path) -> Path:
    """Download a single file from url to dest."""
    dest.parent.mkdir(parents=True, exist_ok=True)
    urllib.request.urlretrieve(url, dest)
    return dest


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Download and organize raw input files for the witness generator."
    )
    parser.add_argument(
        "--url-list",
        required=True,
        type=Path,
        help="Path to a text file with one URL per line.",
    )
    parser.add_argument(
        "--chain-config",
        required=True,
        type=Path,
        help="Path to a local chain_config.json to copy into the output folder.",
    )
    parser.add_argument(
        "--output",
        required=True,
        type=Path,
        help="Output directory to create the organized structure in.",
    )
    parser.add_argument(
        "--workers",
        type=int,
        default=8,
        help="Number of parallel download workers (default: 8).",
    )
    args = parser.parse_args()

    if not args.url_list.is_file():
        print(f"Error: URL list file not found: {args.url_list}", file=sys.stderr)
        sys.exit(1)

    if not args.chain_config.is_file():
        print(
            f"Error: chain_config.json not found: {args.chain_config}", file=sys.stderr
        )
        sys.exit(1)

    # Parse URLs and group by test name
    urls = [
        line.strip()
        for line in args.url_list.read_text().splitlines()
        if line.strip()
    ]

    fixtures: dict[str, dict[str, str]] = defaultdict(dict)
    for url in urls:
        test_name, filename = parse_url(url)
        if filename in fixtures[test_name]:
            print(
                f"Warning: duplicate {filename} for fixture '{test_name}', "
                f"keeping last occurrence.",
                file=sys.stderr,
            )
        fixtures[test_name][filename] = url

    # Validate that each fixture has both files
    incomplete = []
    for test_name, files in sorted(fixtures.items()):
        missing = EXPECTED_FILENAMES - files.keys()
        if missing:
            incomplete.append((test_name, missing))

    if incomplete:
        print("Warning: some fixtures are missing files:", file=sys.stderr)
        for test_name, missing in incomplete:
            print(f"  {test_name}: missing {missing}", file=sys.stderr)

    # Prepare output directory
    args.output.mkdir(parents=True, exist_ok=True)

    # Copy chain_config.json
    chain_config_dest = args.output / "chain_config.json"
    shutil.copy2(args.chain_config, chain_config_dest)
    print(f"Copied chain_config.json -> {chain_config_dest}")

    # Build download tasks: list of (url, destination_path)
    tasks: list[tuple[str, Path]] = []
    for test_name, files in sorted(fixtures.items()):
        fixture_dir = args.output / test_name
        for filename, url in files.items():
            tasks.append((url, fixture_dir / filename))

    print(
        f"Downloading {len(tasks)} files for {len(fixtures)} fixtures "
        f"({args.workers} workers)..."
    )

    # Download in parallel
    total = len(tasks)
    failed: list[tuple[str, str]] = []
    completed = 0
    last_pct = -1
    with ThreadPoolExecutor(max_workers=args.workers) as pool:
        future_to_info = {
            pool.submit(download_file, url, dest): (url, dest)
            for url, dest in tasks
        }
        for future in as_completed(future_to_info):
            url, dest = future_to_info[future]
            try:
                future.result()
                completed += 1
                pct = completed * 100 // total
                if pct != last_pct:
                    last_pct = pct
                    print(
                        f"\r  [{completed}/{total}] {pct}%",
                        end="",
                        flush=True,
                    )
            except Exception as exc:
                failed.append((url, str(exc)))
                print(f"\n  FAILED: {dest.name} - {exc}", file=sys.stderr)
    print()  # newline after progress

    if failed:
        print(f"\n{len(failed)} downloads failed:", file=sys.stderr)
        for url, err in failed:
            print(f"  {url}\n    {err}", file=sys.stderr)
        sys.exit(1)

    print(f"\nDone. Output directory: {args.output}")
    print(f"  {len(fixtures)} fixtures ready")


if __name__ == "__main__":
    main()
