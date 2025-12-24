#!/usr/bin/env python3
"""
Script to analyze blockchain test cases and trace opcodes.
For each test case, this script:
1. Extracts bytecode from contracts and transaction inputs
2. Traces opcode execution sequence
3. Counts how many times each opcode is called
Usage:
    python3 analyze_opcode_traces.py [--output OUTPUT_DIR] [--test-case TEST_CASE_NAME]
Example:
    python3 analyze_opcode_traces.py
    python3 analyze_opcode_traces.py --test-case worst_push
"""

import json
import os
import sys
from pathlib import Path
from typing import Dict, List, Tuple, Optional
from collections import Counter, defaultdict
import argparse

# EVM opcode mapping (0x00-0xff)
OPCODES = {
    0x00: "STOP", 0x01: "ADD", 0x02: "MUL", 0x03: "SUB", 0x04: "DIV",
    0x05: "SDIV", 0x06: "MOD", 0x07: "SMOD", 0x08: "ADDMOD", 0x09: "MULMOD",
    0x0a: "EXP", 0x0b: "SIGNEXTEND", 0x10: "LT", 0x11: "GT", 0x12: "SLT",
    0x13: "SGT", 0x14: "EQ", 0x15: "ISZERO", 0x16: "AND", 0x17: "OR",
    0x18: "XOR", 0x19: "NOT", 0x1a: "BYTE", 0x1b: "SHL", 0x1c: "SHR",
    0x1d: "SAR", 0x20: "SHA3", 0x30: "ADDRESS", 0x31: "BALANCE", 0x32: "ORIGIN",
    0x33: "CALLER", 0x34: "CALLVALUE", 0x35: "CALLDATALOAD", 0x36: "CALLDATASIZE",
    0x37: "CALLDATACOPY", 0x38: "CODESIZE", 0x39: "CODECOPY", 0x3a: "GASPRICE",
    0x3b: "EXTCODESIZE", 0x3c: "EXTCODECOPY", 0x3d: "RETURNDATASIZE",
    0x3e: "RETURNDATACOPY", 0x3f: "EXTCODEHASH", 0x40: "BLOCKHASH", 0x41: "COINBASE",
    0x42: "TIMESTAMP", 0x43: "NUMBER", 0x44: "DIFFICULTY", 0x45: "GASLIMIT",
    0x46: "CHAINID", 0x47: "SELFBALANCE", 0x48: "BASEFEE", 0x49: "BLOBHASH",
    0x4a: "BLOBBASEFEE", 0x50: "POP", 0x51: "MLOAD", 0x52: "MSTORE", 0x53: "MSTORE8",
    0x54: "SLOAD", 0x55: "SSTORE", 0x56: "JUMP", 0x57: "JUMPI", 0x58: "PC",
    0x59: "MSIZE", 0x5a: "GAS", 0x5b: "JUMPDEST", 0x5f: "PUSH0", 0x60: "PUSH1",
    0x61: "PUSH2", 0x62: "PUSH3", 0x63: "PUSH4", 0x64: "PUSH5", 0x65: "PUSH6",
    0x66: "PUSH7", 0x67: "PUSH8", 0x68: "PUSH9", 0x69: "PUSH10", 0x6a: "PUSH11",
    0x6b: "PUSH12", 0x6c: "PUSH13", 0x6d: "PUSH14", 0x6e: "PUSH15", 0x6f: "PUSH16",
    0x70: "PUSH17", 0x71: "PUSH18", 0x72: "PUSH19", 0x73: "PUSH20", 0x74: "PUSH21",
    0x75: "PUSH22", 0x76: "PUSH23", 0x77: "PUSH24", 0x78: "PUSH25", 0x79: "PUSH26",
    0x7a: "PUSH27", 0x7b: "PUSH28", 0x7c: "PUSH29", 0x7d: "PUSH30", 0x7e: "PUSH31",
    0x7f: "PUSH32", 0x80: "DUP1", 0x81: "DUP2", 0x82: "DUP3", 0x83: "DUP4",
    0x84: "DUP5", 0x85: "DUP6", 0x86: "DUP7", 0x87: "DUP8", 0x88: "DUP9",
    0x89: "DUP10", 0x8a: "DUP11", 0x8b: "DUP12", 0x8c: "DUP13", 0x8d: "DUP14",
    0x8e: "DUP15", 0x8f: "DUP16", 0x90: "SWAP1", 0x91: "SWAP2", 0x92: "SWAP3",
    0x93: "SWAP4", 0x94: "SWAP5", 0x95: "SWAP6", 0x96: "SWAP7", 0x97: "SWAP8",
    0x98: "SWAP9", 0x99: "SWAP10", 0x9a: "SWAP11", 0x9b: "SWAP12", 0x9c: "SWAP13",
    0x9d: "SWAP14", 0x9e: "SWAP15", 0x9f: "SWAP16", 0xa0: "LOG0", 0xa1: "LOG1",
    0xa2: "LOG2", 0xa3: "LOG3", 0xa4: "LOG4", 0xf0: "CREATE", 0xf1: "CALL",
    0xf2: "CALLCODE", 0xf3: "RETURN", 0xf4: "DELEGATECALL", 0xf5: "CREATE2",
    0xfa: "STATICCALL", 0xfd: "REVERT", 0xfe: "INVALID", 0xff: "SELFDESTRUCT",
}

# Push opcodes that read data
PUSH_OPCODES = {0x5f} | set(range(0x60, 0x80))


def hex_to_bytes(hex_str: str) -> bytes:
    """Convert hex string to bytes."""
    if hex_str.startswith("0x"):
        hex_str = hex_str[2:]
    return bytes.fromhex(hex_str)


def disassemble_bytecode(bytecode: bytes) -> List[Tuple[int, str, Optional[bytes]]]:
    """
    Disassemble bytecode into opcodes.
    Returns list of (offset, opcode_name, push_data).
    """
    instructions = []
    i = 0
    while i < len(bytecode):
        opcode = bytecode[i]
        opcode_name = OPCODES.get(opcode, f"UNKNOWN(0x{opcode:02x})")
        
        if opcode in PUSH_OPCODES:
            # PUSH opcodes read 1-32 bytes after the opcode
            if opcode == 0x5f:  # PUSH0
                push_data = None
                i += 1
            else:
                push_size = opcode - 0x5f  # PUSH1 = 1 byte, PUSH2 = 2 bytes, etc.
                if i + 1 + push_size <= len(bytecode):
                    push_data = bytecode[i + 1:i + 1 + push_size]
                    i += 1 + push_size
                else:
                    push_data = bytecode[i + 1:] if i + 1 < len(bytecode) else None
                    i = len(bytecode)
            instructions.append((i - (1 if opcode == 0x5f else 1 + push_size), opcode_name, push_data))
        else:
            instructions.append((i, opcode_name, None))
            i += 1
    
    return instructions


def trace_execution(bytecode: bytes, calldata: Optional[bytes] = None) -> Tuple[List[str], Counter]:
    """
    Trace opcode execution.
    This is a simplified static analysis - for full dynamic tracing,
    you would need an actual EVM implementation.
    
    Returns:
        (opcode_sequence, opcode_counts)
    """
    instructions = disassemble_bytecode(bytecode)
    opcode_sequence = []
    opcode_counts = Counter()
    
    # Simple linear execution trace (doesn't handle jumps)
    # For full tracing, we'd need to simulate the EVM stack and handle JUMP/JUMPI
    pc = 0
    visited = set()
    
    while pc < len(instructions):
        if pc in visited:
            # Avoid infinite loops in static analysis
            break
        visited.add(pc)
        
        offset, opcode_name, push_data = instructions[pc]
        
        opcode_sequence.append(opcode_name)
        opcode_counts[opcode_name] += 1
        
        # Handle control flow (simplified)
        if opcode_name == "STOP" or opcode_name == "RETURN" or opcode_name == "REVERT" or opcode_name == "SELFDESTRUCT":
            break
        elif opcode_name == "JUMP":
            # In real execution, we'd pop the stack to get jump destination
            # For static analysis, we'll just continue linearly
            pc += 1
        elif opcode_name == "JUMPI":
            # Conditional jump - we'll trace both paths (simplified)
            pc += 1
        else:
            pc += 1
    
    return opcode_sequence, opcode_counts


def analyze_test_case(test_file: Path) -> Dict:
    """Analyze a single test case file."""
    print(f"\nAnalyzing: {test_file.name}")
    
    with open(test_file, 'r') as f:
        test_data = json.load(f)
    
    results = {}
    
    for test_name, test_case in test_data.items():
        print(f"  Processing test: {test_name}")
        test_result = {
            "test_name": test_name,
            "contracts": {},
            "transactions": []
        }
        
        # Extract contract bytecode from pre state
        if "pre" in test_case:
            for address, account in test_case["pre"].items():
                if "code" in account and account["code"]:
                    code_hex = account["code"]
                    if code_hex.startswith("0x"):
                        code_hex = code_hex[2:]
                    if code_hex:
                        bytecode = bytes.fromhex(code_hex)
                        opcode_seq, opcode_counts = trace_execution(bytecode)
                        test_result["contracts"][address] = {
                            "bytecode_length": len(bytecode),
                            "opcode_sequence": opcode_seq,
                            "opcode_counts": dict(opcode_counts),
                            "total_opcodes": len(opcode_seq)
                        }
        
        # Extract transaction data
        if "blocks" in test_case:
            for block in test_case["blocks"]:
                if "transactions" in block:
                    for tx in block["transactions"]:
                        tx_data = {
                            "to": tx.get("to", ""),
                            "input": tx.get("input", ""),
                        }
                        
                        # Trace transaction input if it's contract creation or call data
                        if tx_data["input"]:
                            input_hex = tx_data["input"]
                            if input_hex.startswith("0x"):
                                input_hex = input_hex[2:]
                            if input_hex:
                                input_bytes = bytes.fromhex(input_hex)
                                # For call data, we can't directly trace it as opcodes
                                # but we can note it
                                tx_data["input_length"] = len(input_bytes)
                                tx_data["input_preview"] = input_hex[:100] + "..." if len(input_hex) > 100 else input_hex
                        
                        test_result["transactions"].append(tx_data)
        
        results[test_name] = test_result
    
    return results


def generate_report(all_results: Dict[str, Dict], output_dir: Path):
    """Generate HTML and text reports."""
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # Generate text report
    report_file = output_dir / "opcode_trace_report.txt"
    with open(report_file, 'w') as f:
        f.write("=" * 80 + "\n")
        f.write("EVM Opcode Trace Analysis Report\n")
        f.write("=" * 80 + "\n\n")
        
        for test_file, results in all_results.items():
            f.write(f"\n{'=' * 80}\n")
            f.write(f"Test File: {test_file}\n")
            f.write(f"{'=' * 80}\n\n")
            
            for test_name, test_result in results.items():
                f.write(f"Test Case: {test_result['test_name']}\n")
                f.write("-" * 80 + "\n\n")
                
                # Contract analysis
                if test_result["contracts"]:
                    f.write("Contracts:\n")
                    for address, contract_info in test_result["contracts"].items():
                        f.write(f"  Address: {address}\n")
                        f.write(f"  Bytecode Length: {contract_info['bytecode_length']} bytes\n")
                        f.write(f"  Total Opcodes Executed: {contract_info['total_opcodes']}\n")
                        f.write(f"  Opcode Counts:\n")
                        for opcode, count in sorted(contract_info['opcode_counts'].items(), key=lambda x: -x[1]):
                            f.write(f"    {opcode}: {count}\n")
                        f.write(f"  Opcode Sequence (first 100):\n")
                        seq_preview = contract_info['opcode_sequence'][:100]
                        f.write(f"    {' -> '.join(seq_preview)}\n")
                        if len(contract_info['opcode_sequence']) > 100:
                            f.write(f"    ... ({len(contract_info['opcode_sequence']) - 100} more)\n")
                        f.write("\n")
                
                # Transaction analysis
                if test_result["transactions"]:
                    f.write(f"Transactions: {len(test_result['transactions'])}\n")
                    for i, tx in enumerate(test_result["transactions"]):
                        f.write(f"  Transaction {i+1}:\n")
                        f.write(f"    To: {tx.get('to', 'CREATE')}\n")
                        if 'input_length' in tx:
                            f.write(f"    Input Length: {tx['input_length']} bytes\n")
                        f.write("\n")
                
                f.write("\n")
    
    # Generate JSON report
    json_file = output_dir / "opcode_trace_report.json"
    with open(json_file, 'w') as f:
        json.dump(all_results, f, indent=2)
    
    print(f"\nReports generated:")
    print(f"  Text: {report_file}")
    print(f"  JSON: {json_file}")


def main():
    parser = argparse.ArgumentParser(description="Analyze blockchain test cases and trace opcodes")
    parser.add_argument("--fixtures-dir", type=str, 
                       default="./zkevm-fixtures/fixtures/blockchain_tests/benchmark",
                       help="Directory containing test case JSON files")
    parser.add_argument("--output", type=str, default="./opcode_traces",
                       help="Output directory for reports")
    parser.add_argument("--test-case", type=str, default=None,
                       help="Specific test case file to analyze (optional)")
    
    args = parser.parse_args()
    
    fixtures_dir = Path(args.fixtures_dir)
    if not fixtures_dir.exists():
        print(f"Error: Fixtures directory not found: {fixtures_dir}")
        sys.exit(1)
    
    # Find all test case JSON files
    if args.test_case:
        test_files = list(fixtures_dir.rglob(f"*{args.test_case}*.json"))
        if not test_files:
            print(f"Error: Test case '{args.test_case}' not found")
            sys.exit(1)
    else:
        test_files = list(fixtures_dir.rglob("*.json"))
    
    print(f"Found {len(test_files)} test case file(s)")
    
    all_results = {}
    for test_file in test_files:
        try:
            results = analyze_test_case(test_file)
            all_results[test_file.name] = results
        except Exception as e:
            print(f"Error analyzing {test_file}: {e}")
            import traceback
            traceback.print_exc()
    
    # Generate reports
    output_dir = Path(args.output)
    generate_report(all_results, output_dir)
    
    print(f"\nAnalysis complete! Processed {len(test_files)} test case file(s).")


if __name__ == "__main__":
    main()