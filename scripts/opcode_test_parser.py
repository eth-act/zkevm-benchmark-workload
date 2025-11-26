#!/usr/bin/env python3
"""
Opcode Test Parser

This script parses pytest test names and determines which EVM opcodes are being tested.

Usage:
    python opcode_test_parser.py "worst_compute::worst_binop_simple[Prague-benchmark-gas-value_10M-opcode_EXP]"
    python opcode_test_parser.py --file test_worst_compute.py::test_worst_push
    python opcode_test_parser.py --batch tests.txt
"""

import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List


@dataclass
class TestInfo:
    """Information about a parsed test."""

    file: str
    module: str
    function: str
    parameters: Dict[str, str]
    raw_params: str
    opcodes: List[str]


# Mapping of test functions to opcodes they test
TEST_FUNCTION_OPCODES = {
    # Stack operations
    "test_worst_push": [
        "PUSH0",
        "PUSH1",
        "PUSH2",
        "PUSH3",
        "PUSH4",
        "PUSH5",
        "PUSH6",
        "PUSH7",
        "PUSH8",
        "PUSH9",
        "PUSH10",
        "PUSH11",
        "PUSH12",
        "PUSH13",
        "PUSH14",
        "PUSH15",
        "PUSH16",
        "PUSH17",
        "PUSH18",
        "PUSH19",
        "PUSH20",
        "PUSH21",
        "PUSH22",
        "PUSH23",
        "PUSH24",
        "PUSH25",
        "PUSH26",
        "PUSH27",
        "PUSH28",
        "PUSH29",
        "PUSH30",
        "PUSH31",
        "PUSH32",
    ],
    "test_worst_dup": [
        "DUP1",
        "DUP2",
        "DUP3",
        "DUP4",
        "DUP5",
        "DUP6",
        "DUP7",
        "DUP8",
        "DUP9",
        "DUP10",
        "DUP11",
        "DUP12",
        "DUP13",
        "DUP14",
        "DUP15",
        "DUP16",
    ],
    "test_worst_swap": [
        "SWAP1",
        "SWAP2",
        "SWAP3",
        "SWAP4",
        "SWAP5",
        "SWAP6",
        "SWAP7",
        "SWAP8",
        "SWAP9",
        "SWAP10",
        "SWAP11",
        "SWAP12",
        "SWAP13",
        "SWAP14",
        "SWAP15",
        "SWAP16",
    ],
    # Arithmetic
    "test_worst_binop_simple": [
        "ADD",
        "MUL",
        "SUB",
        "DIV",
        "SDIV",
        "MOD",
        "SMOD",
        "EXP",
        "SIGNEXTEND",
        "LT",
        "GT",
        "SLT",
        "SGT",
        "EQ",
        "AND",
        "OR",
        "XOR",
        "BYTE",
        "SHL",
        "SHR",
        "SAR",
    ],
    "test_worst_unop": ["ISZERO", "NOT"],
    "test_worst_mod": ["MOD", "SMOD"],
    "test_worst_modarith": ["ADDMOD", "MULMOD"],
    "test_worst_shifts": ["SHL", "SHR", "SAR"],
    # Memory operations
    "test_worst_memory_access": ["MLOAD", "MSTORE", "MSTORE8"],
    "test_worst_msize": ["MSIZE"],
    "test_worst_calldatacopy": ["CALLDATACOPY"],
    "test_worst_codecopy": ["CODECOPY"],
    "test_worst_returndatacopy": ["RETURNDATACOPY"],
    "test_worst_mcopy": ["MCOPY"],
    # Storage
    "test_worst_storage_access_cold": ["SLOAD", "SSTORE"],
    "test_worst_storage_access_warm": ["SLOAD", "SSTORE"],
    "test_worst_tload": ["TLOAD"],
    "test_worst_tstore": ["TSTORE"],
    # Control flow
    "test_worst_jumps": ["JUMP"],
    "test_worst_jumpi_fallthrough": ["JUMPI"],
    "test_worst_jumpis": ["JUMPI"],
    "test_worst_jumpdests": ["JUMPDEST"],
    "test_worst_keccak": ["SHA3", "KECCAK256"],
    # Environmental
    "test_worst_zero_param": [
        "ADDRESS",
        "ORIGIN",
        "CALLER",
        "CODESIZE",
        "GASPRICE",
        "COINBASE",
        "TIMESTAMP",
        "NUMBER",
        "PREVRANDAO",
        "GASLIMIT",
        "CHAINID",
        "BASEFEE",
        "BLOBBASEFEE",
        "GAS",
    ],
    "test_worst_calldatasize": ["CALLDATASIZE"],
    "test_worst_calldataload": ["CALLDATALOAD"],
    "test_worst_callvalue": ["CALLVALUE"],
    "test_worst_returndatasize_nonzero": ["RETURNDATASIZE"],
    "test_worst_returndatasize_zero": ["RETURNDATASIZE"],
    "test_worst_blobhash": ["BLOBHASH"],
    # Stateful operations
    "test_worst_address_state_cold": ["BALANCE", "EXTCODESIZE", "EXTCODEHASH"],
    "test_worst_address_state_warm": [
        "BALANCE",
        "EXTCODESIZE",
        "EXTCODEHASH",
        "CALL",
        "CALLCODE",
        "DELEGATECALL",
        "STATICCALL",
    ],
    "test_worst_blockhash": ["BLOCKHASH"],
    "test_worst_selfbalance": ["SELFBALANCE"],
    "test_worst_extcodecopy_warm": ["EXTCODECOPY"],
    "test_worst_selfdestruct_existing": ["SELFDESTRUCT"],
    "test_worst_selfdestruct_created": ["SELFDESTRUCT"],
    "test_worst_selfdestruct_initcode": ["SELFDESTRUCT"],
    # Logging
    "test_worst_log_opcodes": ["LOG0", "LOG1", "LOG2", "LOG3", "LOG4"],
    # Bytecode operations
    "test_worst_bytecode_single_opcode": [
        "EXTCODESIZE",
        "EXTCODEHASH",
        "EXTCODECOPY",
        "CALL",
        "CALLCODE",
        "DELEGATECALL",
        "STATICCALL",
    ],
    "test_worst_create": ["CREATE", "CREATE2"],
    "test_worst_creates_collisions": ["CREATE", "CREATE2"],
    "test_worst_initcode_jumpdest_analysis": ["JUMPDEST", "JUMP"],
    # Return operations
    "test_worst_return_revert": ["RETURN", "REVERT"],
    # Precompiles
    "test_worst_precompile_only_data_input": ["CALL"],  # To precompiles 0x02, 0x03, 0x04
    "test_worst_modexp": ["CALL"],  # To precompile 0x05
    "test_worst_precompile_fixed_cost": ["CALL", "STATICCALL"],  # To various precompiles
    "test_amortized_bn128_pairings": ["CALL"],  # To precompile 0x08
    # Osaka opcodes
    "test_worst_clz_same_input": ["CLZ"],
    "test_worst_clz_diff_input": ["CLZ"],
    # BloatNet
    "test_bloatnet_balance_extcodesize": ["BALANCE", "EXTCODESIZE", "SHA3"],
    "test_bloatnet_balance_extcodecopy": ["BALANCE", "EXTCODECOPY", "SHA3"],
    "test_bloatnet_balance_extcodehash": ["BALANCE", "EXTCODEHASH", "SHA3"],
    # Block tests
    "test_block_full_access_list_and_data": [
        "SLOAD",
        "SSTORE",
        "BALANCE",
        "EXTCODESIZE",
        "EXTCODEHASH",
        "CALLDATALOAD",
        "CALLDATACOPY",
        "CALLDATASIZE",
    ],
    "test_block_full_data": ["CALLDATALOAD", "CALLDATACOPY", "CALLDATASIZE"],
    "test_block_full_of_ether_transfers": ["CALL"],
}

# Precompile address to name mapping
PRECOMPILES = {
    "0x01": "ECRECOVER",
    "0x02": "SHA2-256",
    "0x03": "RIPEMD-160",
    "0x04": "IDENTITY",
    "0x05": "MODEXP",
    "0x06": "EC_ADD",
    "0x07": "EC_MUL",
    "0x08": "EC_PAIRING",
    "0x09": "BLAKE2F",
    "0x0a": "POINT_EVALUATION",
    "0x0A": "POINT_EVALUATION",
    "0x0b": "BLS12_G1ADD",
    "0x0B": "BLS12_G1ADD",
    "0x0c": "BLS12_G1MSM",
    "0x0C": "BLS12_G1MSM",
    "0x0d": "BLS12_G2ADD",
    "0x0D": "BLS12_G2ADD",
    "0x0e": "BLS12_G2MSM",
    "0x0E": "BLS12_G2MSM",
    "0x0f": "BLS12_PAIRING",
    "0x0F": "BLS12_PAIRING",
    "0x10": "BLS12_MAP_FP_TO_G1",
    "0x11": "BLS12_MAP_FP2_TO_G2",
    "0x14": "P256VERIFY",
}

# Precompile name variations (for parameter matching)
PRECOMPILE_NAMES = {
    "ecrecover": "ECRECOVER",
    "sha2-256": "SHA2-256",
    "sha256": "SHA2-256",
    "ripemd-160": "RIPEMD-160",
    "ripemd160": "RIPEMD-160",
    "identity": "IDENTITY",
    "modexp": "MODEXP",
    "bn128_add": "EC_ADD",
    "bn128add": "EC_ADD",
    "ec_add": "EC_ADD",
    "ecadd": "EC_ADD",
    "bn128_mul": "EC_MUL",
    "bn128mul": "EC_MUL",
    "ec_mul": "EC_MUL",
    "ecmul": "EC_MUL",
    "bn128_pairing": "EC_PAIRING",
    "bn128pairing": "EC_PAIRING",
    "ec_pairing": "EC_PAIRING",
    "ecpairing": "EC_PAIRING",
    "blake2f": "BLAKE2F",
    "point_evaluation": "POINT_EVALUATION",
    "pointevaluation": "POINT_EVALUATION",
    "bls12_g1add": "BLS12_G1ADD",
    "bls12g1add": "BLS12_G1ADD",
    "bls12_g1msm": "BLS12_G1MSM",
    "bls12g1msm": "BLS12_G1MSM",
    "bls12_g2add": "BLS12_G2ADD",
    "bls12g2add": "BLS12_G2ADD",
    "bls12_g2msm": "BLS12_G2MSM",
    "bls12g2msm": "BLS12_G2MSM",
    "bls12_pairing": "BLS12_PAIRING",
    "bls12pairing": "BLS12_PAIRING",
    "bls12_pairing_check": "BLS12_PAIRING",  # Alternate name
    "bls12pairingcheck": "BLS12_PAIRING",
    "bls12_map_fp_to_g1": "BLS12_MAP_FP_TO_G1",
    "bls12mapfptog1": "BLS12_MAP_FP_TO_G1",
    "bls12_fp_to_g1": "BLS12_MAP_FP_TO_G1",  # Short form
    "bls12fptog1": "BLS12_MAP_FP_TO_G1",
    "bls12_map_fp2_to_g2": "BLS12_MAP_FP2_TO_G2",
    "bls12mapfp2tog2": "BLS12_MAP_FP2_TO_G2",
    "bls12_fp2_to_g2": "BLS12_MAP_FP2_TO_G2",  # Short form
    "bls12_fp_to_g2": "BLS12_MAP_FP2_TO_G2",  # Alternate short form
    "bls12fp2tog2": "BLS12_MAP_FP2_TO_G2",
    "bls12fptog2": "BLS12_MAP_FP2_TO_G2",
    "p256verify": "P256VERIFY",
}

# Common parameter name patterns
PARAM_PATTERNS = {
    "opcode": r"opcode[_-]([A-Z0-9]+)",
    "precompile_address": r"precompile_address[_-](0x[0-9A-Fa-f]+)",
    "address": r"address[_-](0x[0-9A-Fa-f]+)",
}


class OpcodeTestParser:
    """Parser for extracting opcodes from pytest test names."""

    def __init__(self):
        self.test_function_opcodes = TEST_FUNCTION_OPCODES
        self.precompiles = PRECOMPILES
        self.precompile_names = PRECOMPILE_NAMES

    def parse_test_name(self, test_name: str) -> TestInfo:
        """
        Parse a pytest test name and extract opcode information.

        Args:
            test_name: Full pytest test name (e.g., "test_worst_compute.py::test_worst_push[...]")

        Returns:
            TestInfo object with parsed information

        """
        # Split into file::function[parameters]
        parts = test_name.split("::")

        if len(parts) == 1:
            # Just a test function name
            file_part = "unknown"
            rest = parts[0]
        else:
            file_part = parts[0]
            rest = "::".join(parts[1:])

        # Extract function name and parameters
        if "[" in rest:
            function_part, params_part = rest.split("[", 1)
            params_part = params_part.rstrip("]")
        else:
            function_part = rest
            params_part = ""

        # Parse parameters
        parsed_params = self._parse_parameters(params_part)

        # Extract module name from file
        module = self._extract_module(file_part)

        # Determine opcodes
        opcodes = self._determine_opcodes(function_part, parsed_params, params_part)

        return TestInfo(
            file=file_part,
            module=module,
            function=function_part,
            parameters=parsed_params,
            raw_params=params_part,
            opcodes=opcodes,
        )

    def _extract_module(self, file_path: str) -> str:
        """Extract module name from file path."""
        if not file_path or file_path == "unknown":
            return "unknown"

        # Remove .py extension and path
        module = Path(file_path).stem
        return module

    def _parse_parameters(self, params_str: str) -> Dict[str, str]:
        """Parse parameter string into key-value pairs."""
        if not params_str:
            return {}

        params = {}

        # Try to parse as key-value pairs
        parts = params_str.split("-")

        for part in parts:
            if "_" in part and not part.startswith("_"):
                # Try to split on first underscore
                key_value = part.split("_", 1)
                if len(key_value) == 2:
                    params[key_value[0]] = key_value[1]

        # Also store the raw string
        params["_raw"] = params_str

        return params

    def _determine_opcodes(
        self, function_name: str, params: Dict[str, str], raw_params: str
    ) -> List[str]:
        """Determine which opcodes are being tested."""
        opcodes = []

        # 1. Check for explicit opcode in parameters
        if "opcode" in params:
            opcode = params["opcode"].upper()
            opcodes.append(opcode)
            return self._deduplicate_opcodes(opcodes)

        # 2. Check for precompile address
        if "precompile" in params and "address" in params:
            addr = params["address"]
            if addr in self.precompiles:
                opcodes.append(f"PRECOMPILE_{self.precompiles[addr]}")
                return self._deduplicate_opcodes(opcodes)

        # 2.5. Check for precompile name in parameters
        precompile_found = self._find_precompile_in_text(raw_params)
        if precompile_found:
            opcodes.extend(precompile_found)
            # If this is a precompile test, also include the calling opcodes
            precompile_test_functions = [
                "test_worst_precompile_only_data_input",
                "test_worst_precompile_fixed_cost",
                "test_worst_modexp",
                "test_amortized_bn128_pairings",
                # Without test_ prefix
                "worst_precompile_only_data_input",
                "worst_precompile_fixed_cost",
                "worst_modexp",
                "amortized_bn128_pairings",
            ]
            if function_name in precompile_test_functions:
                # Add the calling opcodes from function mapping
                lookup_name = (
                    function_name
                    if function_name in self.test_function_opcodes
                    else f"test_{function_name}"
                )
                if lookup_name in self.test_function_opcodes:
                    opcodes.extend(self.test_function_opcodes[lookup_name])
            return self._deduplicate_opcodes(opcodes)

        # 3. Try to extract from raw parameters using patterns
        for pattern_name, pattern in PARAM_PATTERNS.items():
            matches = re.findall(pattern, raw_params, re.IGNORECASE)
            if matches:
                if pattern_name == "opcode":
                    opcodes.extend([m.upper() for m in matches])
                elif pattern_name in ["precompile_address", "address"]:
                    for addr in matches:
                        if addr.lower() in self.precompiles or addr in self.precompiles:
                            precomp = self.precompiles.get(
                                addr.lower(), self.precompiles.get(addr)
                            )
                            opcodes.append(f"PRECOMPILE_{precomp}")

        if opcodes:
            return self._deduplicate_opcodes(opcodes)

        # 4. Look up function name in mapping
        # Try with and without 'test_' prefix
        if function_name in self.test_function_opcodes:
            opcodes.extend(self.test_function_opcodes[function_name])
            # Special case: detect specific precompiles from function name
            if "modexp" in function_name.lower():
                opcodes.insert(0, "PRECOMPILE_MODEXP")
            elif "bn128" in function_name.lower() and "pairing" in function_name.lower():
                opcodes.insert(0, "PRECOMPILE_EC_PAIRING")
            return self._deduplicate_opcodes(opcodes)

        # Try adding 'test_' prefix if not present
        if not function_name.startswith("test_"):
            prefixed_name = f"test_{function_name}"
            if prefixed_name in self.test_function_opcodes:
                opcodes.extend(self.test_function_opcodes[prefixed_name])
                # Special case: detect specific precompiles from function name
                if "modexp" in function_name.lower():
                    opcodes.insert(0, "PRECOMPILE_MODEXP")
                elif "bn128" in function_name.lower() and "pairing" in function_name.lower():
                    opcodes.insert(0, "PRECOMPILE_EC_PAIRING")
                return self._deduplicate_opcodes(opcodes)

        # 5. Try to infer from function name patterns
        opcodes.extend(self._infer_from_function_name(function_name))

        if opcodes:
            return self._deduplicate_opcodes(opcodes)

        # 6. Check raw parameters for opcode names
        opcodes.extend(self._find_opcodes_in_text(raw_params))

        return self._deduplicate_opcodes(opcodes) if opcodes else ["UNKNOWN"]

    def _deduplicate_opcodes(self, opcodes: List[str]) -> List[str]:
        """Remove duplicates while preserving order."""
        seen = set()
        result = []
        for opcode in opcodes:
            if opcode not in seen:
                seen.add(opcode)
                result.append(opcode)
        return result

    def _infer_from_function_name(self, function_name: str) -> List[str]:
        """Infer opcodes from function name patterns."""
        opcodes = []

        # Pattern: test_worst_<opcode>
        if function_name.startswith("test_worst_"):
            suffix = function_name.replace("test_worst_", "").upper()

            # Check if it's a known opcode
            known_opcodes = [
                "ADD",
                "MUL",
                "SUB",
                "DIV",
                "SDIV",
                "MOD",
                "SMOD",
                "EXP",
                "ADDMOD",
                "MULMOD",
                "SIGNEXTEND",
                "LT",
                "GT",
                "SLT",
                "SGT",
                "EQ",
                "ISZERO",
                "AND",
                "OR",
                "XOR",
                "NOT",
                "BYTE",
                "SHL",
                "SHR",
                "SAR",
                "SHA3",
                "KECCAK256",
                "ADDRESS",
                "BALANCE",
                "ORIGIN",
                "CALLER",
                "CALLVALUE",
                "CALLDATALOAD",
                "CALLDATASIZE",
                "CALLDATACOPY",
                "CODESIZE",
                "CODECOPY",
                "GASPRICE",
                "EXTCODESIZE",
                "EXTCODECOPY",
                "EXTCODEHASH",
                "RETURNDATASIZE",
                "RETURNDATACOPY",
                "BLOCKHASH",
                "COINBASE",
                "TIMESTAMP",
                "NUMBER",
                "PREVRANDAO",
                "GASLIMIT",
                "CHAINID",
                "SELFBALANCE",
                "BASEFEE",
                "POP",
                "MLOAD",
                "MSTORE",
                "MSTORE8",
                "SLOAD",
                "SSTORE",
                "JUMP",
                "JUMPI",
                "PC",
                "MSIZE",
                "GAS",
                "JUMPDEST",
                "TLOAD",
                "TSTORE",
                "MCOPY",
                "LOG0",
                "LOG1",
                "LOG2",
                "LOG3",
                "LOG4",
                "CREATE",
                "CREATE2",
                "CALL",
                "CALLCODE",
                "RETURN",
                "DELEGATECALL",
                "STATICCALL",
                "REVERT",
                "SELFDESTRUCT",
                "BLOBHASH",
                "BLOBBASEFEE",
                "CLZ",
            ]

            if suffix in known_opcodes:
                opcodes.append(suffix)

        return opcodes

    def _find_opcodes_in_text(self, text: str) -> List[str]:
        """Find opcode names in text string using word boundaries."""
        opcodes = []
        text_upper = text.upper()

        # List of all opcodes - sorted by length (longest first) to avoid partial matches
        all_opcodes = [
            "ADD",
            "MUL",
            "SUB",
            "DIV",
            "SDIV",
            "MOD",
            "SMOD",
            "EXP",
            "ADDMOD",
            "MULMOD",
            "SIGNEXTEND",
            "LT",
            "GT",
            "SLT",
            "SGT",
            "EQ",
            "ISZERO",
            "AND",
            "OR",
            "XOR",
            "NOT",
            "BYTE",
            "SHL",
            "SHR",
            "SAR",
            "SHA3",
            "KECCAK256",
            "LOG0",
            "LOG1",
            "LOG2",
            "LOG3",
            "LOG4",
            "SLOAD",
            "SSTORE",
            "TLOAD",
            "TSTORE",
            "MLOAD",
            "MSTORE",
            "MSTORE8",
            "MCOPY",
            "MSIZE",
            "CALLDATALOAD",
            "CALLDATASIZE",
            "CALLDATACOPY",
            "CODECOPY",
            "CODESIZE",
            "EXTCODESIZE",
            "EXTCODECOPY",
            "EXTCODEHASH",
            "RETURNDATASIZE",
            "RETURNDATACOPY",
            "BALANCE",
            "SELFBALANCE",
            "CALL",
            "CALLCODE",
            "DELEGATECALL",
            "STATICCALL",
            "CREATE",
            "CREATE2",
            "JUMP",
            "JUMPI",
            "JUMPDEST",
            "RETURN",
            "REVERT",
            "SELFDESTRUCT",
            "PUSH0",
            "PUSH1",
            "DUP1",
            "SWAP1",  # Representatives
            "BLOBHASH",
            "BLOBBASEFEE",
            "CLZ",
        ]

        # Use word boundaries to avoid false positives like "BYTE" in "bytes"
        # Check each opcode with word boundary regex
        for opcode in all_opcodes:
            # Use word boundary pattern: must not be preceded or followed by alphanumeric
            pattern = r"\b" + re.escape(opcode) + r"\b"
            if re.search(pattern, text_upper):
                opcodes.append(opcode)

        return opcodes

    def _find_precompile_in_text(self, text: str) -> List[str]:
        """Find precompile names in text string."""
        precompiles = []
        text_lower = text.lower()

        # Check for precompile names (case-insensitive)
        for precompile_pattern, precompile_name in self.precompile_names.items():
            # Use word boundaries to avoid false positives
            pattern = r"\b" + re.escape(precompile_pattern) + r"\b"
            if re.search(pattern, text_lower):
                precompile_label = f"PRECOMPILE_{precompile_name}"
                if precompile_label not in precompiles:
                    precompiles.append(precompile_label)

        return precompiles

    def format_output(self, test_info: TestInfo, verbose: bool = False) -> str:
        """Format the test information as a string."""
        if verbose:
            lines = [
                f"Test: {test_info.file}::{test_info.function}",
                f"Module: {test_info.module}",
                f"Parameters: {test_info.raw_params}",
                f"Opcodes: {', '.join(test_info.opcodes)}",
            ]
            return "\n".join(lines)
        else:
            # Simple format: just the opcodes
            return ", ".join(test_info.opcodes)


def main():
    """Main entry point for the script."""
    import argparse

    parser = argparse.ArgumentParser(
        description="Parse pytest test names and determine which opcodes are being tested",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Parse a single test name
  %(prog)s "test_worst_compute.py::test_worst_binop_simple[Prague-benchmark-gas-value_10M-opcode_EXP]"
  
  # Parse with verbose output
  %(prog)s -v "test_worst_compute.py::test_worst_push[opcode_PUSH32]"
  
  # Parse multiple tests from a file
  %(prog)s --batch tests.txt
  
  # Parse test names from stdin
  pytest --collect-only -q | %(prog)s --stdin
        """,
    )

    parser.add_argument("test_name", nargs="?", help="Test name to parse (pytest node ID format)")

    parser.add_argument("-v", "--verbose", action="store_true", help="Show detailed information")

    parser.add_argument(
        "-b", "--batch", metavar="FILE", help="Read test names from file (one per line)"
    )

    parser.add_argument("--stdin", action="store_true", help="Read test names from stdin")

    parser.add_argument("--json", action="store_true", help="Output results as JSON")

    args = parser.parse_args()

    opcode_parser = OpcodeTestParser()

    # Collect test names to parse
    test_names = []

    if args.batch:
        with open(args.batch, "r") as f:
            test_names = [line.strip() for line in f if line.strip()]
    elif args.stdin:
        test_names = [line.strip() for line in sys.stdin if line.strip()]
    elif args.test_name:
        test_names = [args.test_name]
    else:
        parser.print_help()
        sys.exit(1)

    # Parse and output
    results = []

    for test_name in test_names:
        try:
            test_info = opcode_parser.parse_test_name(test_name)
            results.append(test_info)

            if not args.json:
                if args.verbose:
                    print(opcode_parser.format_output(test_info, verbose=True))
                    print()  # Blank line between entries
                else:
                    print(f"{test_name} -> {opcode_parser.format_output(test_info)}")
        except Exception as e:
            print(f"Error parsing '{test_name}': {e}", file=sys.stderr)

    if args.json:
        import json

        json_results = [
            {
                "test": f"{r.file}::{r.function}",
                "module": r.module,
                "function": r.function,
                "parameters": r.parameters,
                "opcodes": r.opcodes,
            }
            for r in results
        ]
        print(json.dumps(json_results, indent=2))


if __name__ == "__main__":
    main()
