#!/usr/bin/env python3
"""
Script to process zkevm fixture JSON files by removing existing chain_config
and adding a new fixed chain_config inside the block_and_witness field.

This is used to migrate previously generated fixture files to the new format.
"""

import argparse
import json
import os
import shutil
from pathlib import Path


def get_fixed_chain_config():
    """Returns the fixed chain configuration to be added to all files."""
    return {
        "chain_id": 1,
        "homestead_block": 1150000,
        "dao_fork_block": 1920000,
        "dao_fork_support": True,
        "eip150_block": 2463000,
        "eip155_block": 2675000,
        "eip158_block": 2463000,
        "byzantium_block": 4370000,
        "constantinople_block": 7280000,
        "petersburg_block": 7280000,
        "istanbul_block": 9069000,
        "muir_glacier_block": 9200000,
        "berlin_block": 12244000,
        "london_block": 12965000,
        "arrow_glacier_block": 13773000,
        "gray_glacier_block": 15050000,
        "merge_netsplit_block": None,
        "shanghai_time": 1681338455,
        "cancun_time": 1710338135,
        "prague_time": 1746612311,
        "osaka_time": None,
        "bpo1_time": None,
        "bpo2_time": None,
        "bpo3_time": None,
        "bpo4_time": None,
        "bpo5_time": None,
        "terminal_total_difficulty": "0xc70d808a128d7380000",
        "terminal_total_difficulty_passed": True,
        "ethash": None,
        "clique": None,
        "parlia": None,
        "deposit_contract_address": "0x00000000219ab540356cbb839cbe05303d7705fa",
        "blob_schedule": {},
        "extra_fields": {}
    }


def process_json_file(input_file_path, output_file_path):
    """
    Process a single JSON file by removing existing chain_config
    and adding the fixed chain_config inside block_and_witness.
    """
    try:
        # Read the input JSON file
        with open(input_file_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        # Remove existing chain_config if it exists at the top level
        if 'chain_config' in data:
            del data['chain_config']
        
        # Ensure block_and_witness exists
        if 'block_and_witness' not in data:
            raise ValueError("Missing 'block_and_witness' field in JSON")
        
        # Add the new fixed chain_config inside block_and_witness
        data['block_and_witness']['chain_config'] = get_fixed_chain_config()
        
        # Write the modified data to the output file
        with open(output_file_path, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=2)
        
        print(f"Processed: {input_file_path.name} -> {output_file_path.name}")
        return True
        
    except Exception as e:
        print(f"Error processing {input_file_path}: {str(e)}")
        return False


def main():
    parser = argparse.ArgumentParser(
        description="Process zkevm fixture JSON files by updating chain_config"
    )
    parser.add_argument(
        "input_folder",
        help="Path to the folder containing JSON files to process"
    )
    parser.add_argument(
        "-o", "--output",
        help="Output folder path (default: <input_folder>_processed)",
        default=None
    )
    
    args = parser.parse_args()
    
    # Convert to Path objects and resolve
    input_folder = Path(args.input_folder).resolve()
    
    if not input_folder.exists():
        print(f"Error: Input folder '{input_folder}' does not exist")
        return 1
    
    if not input_folder.is_dir():
        print(f"Error: '{input_folder}' is not a directory")
        return 1
    
    # Determine output folder
    if args.output:
        output_folder = Path(args.output).resolve()
    else:
        output_folder = input_folder.parent / f"{input_folder.name}_processed"
    
    # Create output folder
    output_folder.mkdir(exist_ok=True)
    print(f"Processing files from: {input_folder}")
    print(f"Output folder: {output_folder}")
    
    # Find all JSON files in the input folder
    json_files = list(input_folder.glob("*.json"))
    
    if not json_files:
        print("No JSON files found in the input folder")
        return 1
    
    print(f"Found {len(json_files)} JSON files to process")
    
    # Process each JSON file
    successful = 0
    failed = 0
    
    for json_file in json_files:
        output_file = output_folder / json_file.name
        if process_json_file(json_file, output_file):
            successful += 1
        else:
            failed += 1
    
    print(f"\nProcessing complete:")
    print(f"  Successfully processed: {successful} files")
    print(f"  Failed: {failed} files")
    print(f"  Output folder: {output_folder}")
    
    return 0 if failed == 0 else 1


if __name__ == "__main__":
    exit(main())
