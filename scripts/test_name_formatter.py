#!/usr/bin/env python3
"""
Test Name Formatter for Benchmark Tests.

Converts pytest test names to human-readable display names with the format:
OPCODE/PRECOMPILE (parameters)

Example:
    Input:  worst_compute::worst_binop_simple[Prague-benchmark-gas-value_10M-opcode_EXP]
    Output: EXP (10M gas, Prague)

    Input:  worst_compute::worst_precompile_fixed_cost[
                Prague-benchmark-gas-value_120M-bls12_pairing_check]
    Output: PRECOMPILE_BLS12_PAIRING (120M gas, Prague)

"""

import argparse
import json
import re
import sys
from dataclasses import dataclass
from typing import List

from opcode_test_parser import OpcodeTestParser

# Import the original display name function as fallback
try:
    from test_name_parser import get_display_name as get_fallback_display_name
except ImportError:
    def get_fallback_display_name(filename: str) -> str:
        return filename


@dataclass
class DisplayName:
    """Structured display name for a test."""
    
    opcodes: List[str]
    parameters: List[str]
    
    def __str__(self) -> str:
        """Format as 'OPCODE (param1, param2, ...)'."""
        opcode_str = ", ".join(self.opcodes)
        if self.parameters:
            param_str = ", ".join(self.parameters)
            return f"{opcode_str} ({param_str})"
        return opcode_str


class TestNameFormatter:
    """Formats pytest test names into human-readable display names."""
    
    def __init__(self) -> None:
        """Initialize the formatter with an opcode parser."""
        self.opcode_parser = OpcodeTestParser()
    
    def format_test_name(self, test_name: str) -> str:
        """
        Convert a pytest test name to a human-readable display name.
        
        Args:
            test_name: The pytest test name (e.g., "test_file::test_func[params]")
        
        Returns:
            A formatted display name with opcodes first, then parameters
        
        """
        # Parse opcodes using the existing parser
        test_info = self.opcode_parser.parse_test_name(test_name)
        
        # If we can't determine the opcode, fall back to the original display name
        if test_info.opcodes == ["UNKNOWN"]:
            return get_fallback_display_name(test_name)
        
        # Extract parameters from the test name
        parameters = self._extract_parameters(test_name)
        
        # Create display name
        display = DisplayName(
            opcodes=test_info.opcodes if test_info.opcodes else ["UNKNOWN"],
            parameters=parameters
        )
        
        return str(display)
    
    def _extract_parameters(self, test_name: str) -> List[str]:
        """
        Extract relevant parameters from the test name.
        
        Prioritizes:
        1. Gas value (e.g., "10M gas")
        2. Fork name (e.g., "Prague", "Cancun")
        3. Other meaningful parameters (sizes, counts, modes)
        """
        params: List[str] = []
        
        # Extract the parameter string (everything in brackets)
        param_match = re.search(r"\[(.*?)\]$", test_name)
        if not param_match:
            return params
        
        param_str = param_match.group(1)
        param_parts = param_str.split("-")
        
        # Track what we've found to avoid duplicates
        found_gas = False
        found_fork = False
        
        for part in param_parts:
            # Gas value (e.g., "value_10M" -> "10M gas")
            if not found_gas and "value_" in part:
                gas_match = re.search(r"value_(\d+[KMG]?)", part)
                if gas_match:
                    params.append(f"{gas_match.group(1)} gas")
                    found_gas = True
                    continue
            
            # Fork names (handle both "Prague" and "fork_Prague")
            fork_names = [
                "Frontier", "Homestead", "Byzantium", "Constantinople", "Istanbul",
                "Berlin", "London", "Paris", "Shanghai", "Cancun", "Prague",
                "Osaka", "Amsterdam",
            ]
            if not found_fork:
                # Check if part is directly a fork name
                if part in fork_names:
                    params.append(part)
                    found_fork = True
                    continue
                # Check if part starts with "fork_"
                if part.startswith("fork_"):
                    fork_name = part.replace("fork_", "")
                    if fork_name in fork_names:
                        params.append(fork_name)
                        found_fork = True
                        continue
            
            # Skip common noise parameters
            if part in [
                "benchmark", "gas", "blockchain_test", "state_test",
                "blockchain_test_from_state_test", "test",
            ]:
                continue
            
            # Skip fork parameters (already processed above)
            if part.startswith("fork_"):
                continue
            
            # Skip opcode parameters (already captured)
            if part.startswith("opcode_"):
                continue
            
            # Skip precompile addresses (already captured)
            if part.startswith("precompile_address_"):
                continue
            
            # Skip precompile names if they match known ones (already captured)
            if self._is_precompile_name(part):
                continue
            
            # Skip opcode names that are already captured (log0-4, dup1-16, swap1-16, push1-32)
            if part.upper() in ["LOG0", "LOG1", "LOG2", "LOG3", "LOG4",
                               "DUP1", "DUP2", "DUP3", "DUP4", "DUP5", "DUP6", "DUP7", "DUP8",
                               "DUP9", "DUP10", "DUP11", "DUP12", "DUP13", "DUP14", "DUP15", "DUP16",
                               "SWAP1", "SWAP2", "SWAP3", "SWAP4", "SWAP5", "SWAP6", "SWAP7", "SWAP8",
                               "SWAP9", "SWAP10", "SWAP11", "SWAP12", "SWAP13", "SWAP14", "SWAP15", "SWAP16",
                               "PUSH1", "PUSH2", "PUSH3", "PUSH4", "PUSH5", "PUSH6", "PUSH7", "PUSH8",
                               "PUSH9", "PUSH10", "PUSH11", "PUSH12", "PUSH13", "PUSH14", "PUSH15", "PUSH16",
                               "PUSH17", "PUSH18", "PUSH19", "PUSH20", "PUSH21", "PUSH22", "PUSH23", "PUSH24",
                               "PUSH25", "PUSH26", "PUSH27", "PUSH28", "PUSH29", "PUSH30", "PUSH31", "PUSH32"]:
                continue
            
            # Memory/storage sizes
            size_match = re.search(r"(\d+)\s*(bytes?|words?|KB|MB)", part, re.IGNORECASE)
            if size_match:
                params.append(f"{size_match.group(1)} {size_match.group(2)}")
                continue
            
            # Counts (e.g., "pairs_1", "logs_3")
            count_match = re.search(r"([a-z_]+)_(\d+)", part)
            if count_match:
                name = count_match.group(1).replace("_", " ")
                count = count_match.group(2)
                params.append(f"{count} {name}")
                continue
            
            # Boolean flags (e.g., "fixed_src_dst_True")
            if part.endswith("_True") or part.endswith("_False"):
                flag_name = part.rsplit("_", 1)[0].replace("_", " ")
                flag_value = part.split("_")[-1]
                params.append(f"{flag_name}: {flag_value}")
                continue
            
            # Gas cost types for modexp
            if "mod_" in part or "exp_" in part or "base_" in part:
                params.append(part.replace("_", " "))
                continue
            
            # Other meaningful short parameters (avoid very long ones)
            if len(part) <= 20 and not part.startswith("test_"):
                # Clean up underscores for readability
                clean_part = part.replace("_", " ")
                if clean_part not in params:
                    params.append(clean_part)
        
        return params
    
    def _is_precompile_name(self, name: str) -> bool:
        """Check if a parameter looks like a precompile name."""
        precompile_patterns = [
            "ecrecover", "sha256", "ripemd", "identity", "modexp", "bn128",
            "ec_add", "ecadd", "ec_mul", "ecmul", "ec_pairing", "ecpairing",
            "blake2", "bls12", "p256verify", "secp256r1verify",
            "g1add", "g1mul", "g2add", "g2mul", "pairing",
        ]
        name_lower = name.lower()
        return any(pattern in name_lower for pattern in precompile_patterns)


def main() -> None:
    """CLI interface for the test name formatter."""
    parser = argparse.ArgumentParser(
        description="Format pytest test names into human-readable display names",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Format single test name
  python test_name_formatter.py "worst_compute::worst_binop_simple[Prague-benchmark-gas-value_10M-opcode_EXP]"
  
  # Format multiple test names
  python test_name_formatter.py "test1" "test2" "test3"
  
  # Read from stdin
  echo "worst_memory::worst_mcopy[Cancun-benchmark-gas-value_10M-0 bytes]" | python test_name_formatter.py
  
  # JSON output
  python test_name_formatter.py --json "worst_compute::worst_binop_simple[Prague-benchmark-gas-value_10M-opcode_EXP]"
  
  # Show original test name alongside display name
  python test_name_formatter.py --show-original "worst_compute::worst_binop_simple[Prague-benchmark-gas-value_10M-opcode_EXP]"
        """,
    )
    
    parser.add_argument(
        "test_names",
        nargs="*",
        help="One or more pytest test names to format. If omitted, reads from stdin.",
    )
    parser.add_argument("--json", action="store_true", help="Output results as JSON")
    parser.add_argument(
        "--show-original",
        action="store_true",
        help="Show original test name alongside display name",
    )
    
    args = parser.parse_args()
    
    formatter = TestNameFormatter()
    
    # Get test names from arguments or stdin
    if args.test_names:
        test_names = args.test_names
    else:
        # Read from stdin
        test_names = [line.strip() for line in sys.stdin if line.strip()]
    
    if not test_names:
        parser.print_help()
        sys.exit(1)
    
    # Format test names
    results = []
    for test_name in test_names:
        display_name = formatter.format_test_name(test_name)
        results.append({"original": test_name, "display": display_name})
    
    # Output results
    if args.json:
        print(json.dumps(results, indent=2))
    else:
        for result in results:
            if args.show_original:
                print(f"{result['original']}")
                print(f"  → {result['display']}")
            else:
                print(result["display"])


if __name__ == "__main__":
    main()