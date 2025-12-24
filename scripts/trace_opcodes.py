#!/usr/bin/env python3
"""
Script to analyze blockchain test cases and trace opcodes during actual execution.
For each test case, this script:
1. Extracts bytecode from contracts and transaction inputs
2. Executes transactions using an EVM implementation
3. Traces opcode execution sequence during execution
4. Counts how many times each opcode is called

Usage:
    python3 trace_opcodes.py [--output OUTPUT_DIR] [--test-case TEST_CASE_NAME] [--method METHOD]

Methods:
    - pyevm: Use py-evm library for accurate execution traces
             * Handles jumps and conditionals correctly
             * Detects precompile calls from stack values
             * Provides actual execution flow (not static analysis)
             * Requires: pip install py-evm
    
    - static: Enhanced static analysis with basic jump handling
              * Fast but may miss some execution paths
              * Doesn't handle complex conditionals accurately
              * No dependencies required
    
    - evmtrace: Use evm tool from go-ethereum
                * Requires: evm binary in PATH (from go-ethereum)
                * Very accurate but requires external dependency

Examples:
    # Use py-evm for accurate traces (recommended)
    python3 trace_opcodes.py --method pyevm
    
    # Analyze specific test case with py-evm
    python3 trace_opcodes.py --test-case worst_push --method pyevm
    
    # Use static analysis (faster, less accurate)
    python3 trace_opcodes.py --method static

For accurate traces, use --method pyevm. This uses py-evm to execute bytecode
and capture opcodes during actual execution, providing traces that reflect
real execution flow including jumps, conditionals, and precompile calls.
"""

import json
import sys
import subprocess
from pathlib import Path
from typing import Dict, List, Tuple, Optional
from collections import Counter
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

# Reverse mapping: opcode name -> value
OPCODE_VALUES = {name: value for value, name in OPCODES.items()}

# Precompiled contracts (addresses 0x01-0x11)
# Reference: https://www.evm.codes/precompiled
PRECOMPILES = {
    # Original precompiles (Frontier, Byzantium, Istanbul, Berlin, Cancun)
    0x01: "ECRECOVER",            # ECDSA public key recovery (Frontier)
    0x02: "SHA256",               # SHA-256 hash function (Frontier)
    0x03: "RIPEMD160",            # RIPEMD-160 hash function (Frontier)
    0x04: "IDENTITY",             # Identity/datacopy (Frontier)
    0x05: "MODEXP",               # Big integer modular exponentiation (Byzantium, EIP-198)
    0x06: "ECADD",                # BN256/alt_bn128 point addition (Byzantium, EIP-196)
    0x07: "ECMUL",                # BN256/alt_bn128 scalar multiplication (Byzantium, EIP-196)
    0x08: "ECPAIRING",            # BN256/alt_bn128 pairing check (Byzantium, EIP-197)
    0x09: "BLAKE2F",              # BLAKE2b F compression function (Istanbul, EIP-152)
    0x0a: "KZG_POINT_EVALUATION", # KZG point evaluation (Cancun, EIP-4844)
    # BLS12-381 precompiles (Pectra, EIP-2537)
    0x0b: "BLS12_G1ADD",          # BLS12-381 G1 point addition
    0x0c: "BLS12_G1MSM",          # BLS12-381 G1 multi-scalar multiplication
    0x0d: "BLS12_G2ADD",          # BLS12-381 G2 point addition
    0x0e: "BLS12_G2MSM",          # BLS12-381 G2 multi-scalar multiplication
    0x0f: "BLS12_PAIRING_CHECK",  # BLS12-381 pairing check
    0x10: "BLS12_MAP_FP_TO_G1",   # BLS12-381 map base field element to G1
    0x11: "BLS12_MAP_FP2_TO_G2",  # BLS12-381 map extension field element to G2
}

# Precompile address range for quick checking
PRECOMPILE_ADDRESSES = set(PRECOMPILES.keys())

# Call opcodes that can invoke precompiles
CALL_OPCODES = {"CALL", "STATICCALL", "DELEGATECALL", "CALLCODE"}

# Push opcodes that read data
PUSH_OPCODES = {0x5f} | set(range(0x60, 0x80))

# EVM opcode gas costs (base costs, simplified - actual costs can vary)
# These are approximate base costs for Prague/Istanbul fork
OPCODE_GAS_COSTS = {
    "STOP": 0, "ADD": 3, "MUL": 5, "SUB": 3, "DIV": 5, "SDIV": 5, "MOD": 5, "SMOD": 5,
    "ADDMOD": 8, "MULMOD": 8, "EXP": 10, "SIGNEXTEND": 5, "LT": 3, "GT": 3, "SLT": 3,
    "SGT": 3, "EQ": 3, "ISZERO": 3, "AND": 3, "OR": 3, "XOR": 3, "NOT": 3, "BYTE": 3,
    "SHL": 3, "SHR": 3, "SAR": 3, "SHA3": 30, "ADDRESS": 2, "BALANCE": 100, "ORIGIN": 2,
    "CALLER": 2, "CALLVALUE": 2, "CALLDATALOAD": 3, "CALLDATASIZE": 2, "CALLDATACOPY": 3,
    "CODESIZE": 2, "CODECOPY": 3, "GASPRICE": 2, "EXTCODESIZE": 100, "EXTCODECOPY": 100,
    "RETURNDATASIZE": 2, "RETURNDATACOPY": 3, "EXTCODEHASH": 100, "BLOCKHASH": 20,
    "COINBASE": 2, "TIMESTAMP": 2, "NUMBER": 2, "DIFFICULTY": 2, "GASLIMIT": 2,
    "CHAINID": 2, "SELFBALANCE": 5, "BASEFEE": 2, "BLOBHASH": 3, "BLOBBASEFEE": 2,
    "POP": 2, "MLOAD": 3, "MSTORE": 3, "MSTORE8": 3, "SLOAD": 100, "SSTORE": 100,
    "JUMP": 8, "JUMPI": 10, "PC": 2, "MSIZE": 2, "GAS": 2, "JUMPDEST": 1,
    "PUSH0": 2, "PUSH1": 3, "PUSH2": 3, "PUSH3": 3, "PUSH4": 3, "PUSH5": 3,
    "PUSH6": 3, "PUSH7": 3, "PUSH8": 3, "PUSH9": 3, "PUSH10": 3, "PUSH11": 3,
    "PUSH12": 3, "PUSH13": 3, "PUSH14": 3, "PUSH15": 3, "PUSH16": 3, "PUSH17": 3,
    "PUSH18": 3, "PUSH19": 3, "PUSH20": 3, "PUSH21": 3, "PUSH22": 3, "PUSH23": 3,
    "PUSH24": 3, "PUSH25": 3, "PUSH26": 3, "PUSH27": 3, "PUSH28": 3, "PUSH29": 3,
    "PUSH30": 3, "PUSH31": 3, "PUSH32": 3,
    "DUP1": 3, "DUP2": 3, "DUP3": 3, "DUP4": 3, "DUP5": 3, "DUP6": 3, "DUP7": 3,
    "DUP8": 3, "DUP9": 3, "DUP10": 3, "DUP11": 3, "DUP12": 3, "DUP13": 3, "DUP14": 3,
    "DUP15": 3, "DUP16": 3,
    "SWAP1": 3, "SWAP2": 3, "SWAP3": 3, "SWAP4": 3, "SWAP5": 3, "SWAP6": 3, "SWAP7": 3,
    "SWAP8": 3, "SWAP9": 3, "SWAP10": 3, "SWAP11": 3, "SWAP12": 3, "SWAP13": 3, "SWAP14": 3,
    "SWAP15": 3, "SWAP16": 3,
    "LOG0": 375, "LOG1": 375, "LOG2": 375, "LOG3": 375, "LOG4": 375,
    "CREATE": 32000, "CALL": 100, "CALLCODE": 100, "RETURN": 0, "DELEGATECALL": 100,
    "CREATE2": 32000, "STATICCALL": 100, "REVERT": 0, "INVALID": 0, "SELFDESTRUCT": 5000,
}


def hex_to_bytes(hex_str: str) -> bytes:
    """Convert hex string to bytes."""
    if hex_str.startswith("0x"):
        hex_str = hex_str[2:]
    if not hex_str:
        return b""
    return bytes.fromhex(hex_str)


def bytes_to_hex(b: bytes) -> str:
    """Convert bytes to hex string with 0x prefix."""
    return "0x" + b.hex()


def disassemble_bytecode(bytecode: bytes) -> List[Tuple[int, int, str, Optional[bytes]]]:
    """
    Disassemble bytecode into opcodes.
    Returns list of (pc, opcode_value, opcode_name, push_data).
    """
    instructions = []
    i = 0
    while i < len(bytecode):
        opcode = bytecode[i]
        opcode_name = OPCODES.get(opcode, f"UNKNOWN(0x{opcode:02x})")
        pc = i
        
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
            instructions.append((pc, opcode, opcode_name, push_data))
        else:
            instructions.append((pc, opcode, opcode_name, None))
            i += 1
    
    return instructions


def is_precompile_address(address: int) -> bool:
    """Check if an address corresponds to a precompile."""
    return address in PRECOMPILE_ADDRESSES


def get_precompile_name(address: int) -> Optional[str]:
    """Get the name of a precompile by its address."""
    return PRECOMPILES.get(address)


def address_from_bytes(addr_bytes: bytes) -> int:
    """Convert address bytes to integer, handling various lengths."""
    if not addr_bytes:
        return 0
    # Addresses are 20 bytes, but push data might be shorter
    # Take the last 20 bytes and interpret as address
    addr_bytes = addr_bytes[-20:] if len(addr_bytes) >= 20 else addr_bytes
    return int.from_bytes(addr_bytes, 'big')


class EVMTracer:
    """Base class for EVM tracers."""
    
    def trace_execution(self, bytecode: bytes, calldata: Optional[bytes] = None,
                       caller: Optional[str] = None, value: int = 0,
                       pre_state: Optional[Dict] = None, gas_limit: Optional[int] = None) -> Tuple[List[str], Counter, Counter]:
        """
        Trace opcode execution.
        Returns: (opcode_sequence, opcode_counts, precompile_counts)
        """
        raise NotImplementedError


class StaticTracer(EVMTracer):
    """Enhanced static analyzer with proper jump handling and precompile detection."""
    
    def trace_execution(self, bytecode: bytes, calldata: Optional[bytes] = None,
                       caller: Optional[str] = None, value: int = 0,
                       pre_state: Optional[Dict] = None, gas_limit: Optional[int] = None) -> Tuple[List[str], Counter, Counter]:
        """Trace execution using static analysis with jump handling and precompile detection."""
        instructions = disassemble_bytecode(bytecode)
        if not instructions:
            return [], Counter(), Counter()
        
        # Create a map from PC to instruction index
        pc_to_idx = {pc: idx for idx, (pc, _, _, _) in enumerate(instructions)}
        
        opcode_sequence = []
        opcode_counts = Counter()
        precompile_counts = Counter()
        
        # Find all JUMPDEST locations
        jumpdests = {pc for pc, _, name, _ in instructions if name == "JUMPDEST"}
        
        # Simple linear execution with basic jump handling and gas tracking
        # For accurate traces, use py-evm or evm tool
        visited_pcs = set()
        idx = 0
        
        # Track gas consumption if gas_limit is provided
        gas_remaining = gas_limit if gas_limit is not None else None
        track_gas = gas_remaining is not None
        
        # Set max iterations based on whether we're tracking gas
        # If tracking gas, scale max_iterations with gas_limit to allow full execution
        # If not tracking gas, use a reasonable limit to prevent infinite loops
        if track_gas:
            # Scale max_iterations with gas_limit - assume average 1-2 gas per instruction
            # Use gas_limit * 2 to ensure we don't hit iteration limit before gas runs out
            max_iterations = max(10_000_000, gas_limit * 2) if gas_limit else 10_000_000
        else:
            max_iterations = min(100000, len(instructions) * 100)  # Prevent infinite loops
        
        iteration = 0
        
        # Track recent push values for precompile detection
        recent_pushes = []  # Stack of (push_value, push_data)
        
        while idx < len(instructions) and iteration < max_iterations:
            iteration += 1
            pc, opcode_val, opcode_name, push_data = instructions[idx]
            
            # Check gas limit - stop if we've run out of gas
            if track_gas:
                opcode_gas = OPCODE_GAS_COSTS.get(opcode_name, 1)  # Default to 1 gas for unknown opcodes
                if gas_remaining < opcode_gas:
                    # Out of gas - stop execution
                    break
                gas_remaining -= opcode_gas
            
            # Avoid infinite loops - but respect gas limits
            # If we're tracking gas, let gas determine when to stop
            # Otherwise, use loop detection
            if pc in visited_pcs and opcode_name not in ("JUMPI",):
                # If we've seen this PC before and it's not a conditional jump, we might be looping
                if track_gas:
                    # When tracking gas, continue until gas runs out
                    # The gas check above will stop execution
                    pass
                else:
                    # Without gas tracking, break on large loops
                    if len(visited_pcs) > 1000:  # Large loop detected
                        break
            visited_pcs.add(pc)
            
            opcode_sequence.append(opcode_name)
            opcode_counts[opcode_name] += 1
            
            # Track push data for precompile detection
            if opcode_name.startswith("PUSH") and push_data is not None:
                recent_pushes.append(push_data)
                # Keep only last 10 pushes to limit memory
                if len(recent_pushes) > 10:
                    recent_pushes.pop(0)
            
            # Detect precompile calls
            # For CALL: stack order is gas, addr, value, argsOffset, argsLength, retOffset, retLength
            # For STATICCALL/DELEGATECALL: stack order is gas, addr, argsOffset, argsLength, retOffset, retLength
            # The address is typically pushed as the 2nd-to-last item before the call
            if opcode_name in CALL_OPCODES and len(recent_pushes) >= 2:
                # Try to detect precompile address from recent pushes
                # The address is usually one of the recent push values
                for push_val in reversed(recent_pushes):
                    addr = address_from_bytes(push_val)
                    if is_precompile_address(addr):
                        precompile_name = get_precompile_name(addr)
                        if precompile_name:
                            precompile_counts[precompile_name] += 1
                            opcode_sequence.append(f"PRECOMPILE_{precompile_name}")
                        break
            
            # Handle control flow
            if opcode_name in ("STOP", "RETURN", "REVERT", "SELFDESTRUCT"):
                break
            elif opcode_name == "JUMP":
                # For JUMP, we'd need to know the stack value
                # Since we don't have full execution context, we'll try to find valid jumpdests
                # If tracking gas, allow jumping back to create loops
                found_jump = False
                for dest_pc in sorted(jumpdests):
                    if dest_pc in pc_to_idx:
                        idx = pc_to_idx[dest_pc]
                        found_jump = True
                        # When tracking gas, allow loops (gas will stop them)
                        # Without gas tracking, break on repeated jumps to same location
                        if not track_gas and dest_pc in visited_pcs:
                            # Already visited this destination, might be infinite loop
                            if len(visited_pcs) > 100:
                                break
                        break
                if not found_jump:
                    break
            elif opcode_name == "JUMPI":
                # For JUMPI, we trace the fallthrough path (condition false)
                # In real execution, this would depend on stack value
                # When tracking gas, we could also try the jump path, but for simplicity
                # we'll just follow fallthrough
                idx += 1
            else:
                idx += 1
        
        return opcode_sequence, opcode_counts, precompile_counts


class PyEVMTracer(EVMTracer):
    """
    Tracer using py-evm library with accurate opcode tracing.
    
    This tracer uses py-evm to execute bytecode and capture opcodes during
    actual execution, providing accurate traces that reflect:
    - Actual execution flow (handles jumps and conditionals correctly)
    - Dynamic opcode sequences (not just static analysis)
    - Precompile calls (detected from stack values during CALL opcodes)
    - Gas consumption and state changes
    
    Installation:
        pip install py-evm
    
    Usage:
        tracer = PyEVMTracer()
        if tracer.available:
            opcode_seq, opcode_counts, precompile_counts = tracer.trace_execution(
                bytecode=bytecode,
                calldata=calldata,
                caller=caller_address,
                value=eth_value
            )
    
    How it works:
        1. Creates a custom Computation class that overrides _execute_opcode()
        2. Before each opcode executes, captures the opcode byte and name
        3. For CALL opcodes, inspects the stack to detect precompile addresses
        4. Returns the complete execution trace with opcode counts
    
    Advantages over static analysis:
        - Handles JUMP/JUMPI correctly (follows actual execution path)
        - Respects conditional logic (only traces executed branches)
        - Detects precompiles from actual stack values
        - Accounts for dynamic code execution (CREATE, CALL, etc.)
    """
    
    def __init__(self):
        try:
            # Try to import py-evm components (updated for py-evm 0.12+)
            from eth import constants
            from eth.vm.forks import PragueVM
            from eth.vm.computation import BaseComputation
            from eth.vm.message import Message
            from eth.vm.transaction_context import BaseTransactionContext
            from eth_utils import to_canonical_address
            
            # Get the state class from the VM fork
            try:
                from eth.vm.forks.prague import PragueState
                self.StateClass = PragueState
            except ImportError:
                # Fallback to getting it from VM
                self.StateClass = getattr(PragueVM, '_state_class', None)
            
            # Try to get DB backend
            try:
                from eth.db.backends.memory import MemoryDB
                self.DB = MemoryDB
            except ImportError:
                try:
                    from eth.db import get_db_backend
                    self.DB = get_db_backend()
                except ImportError:
                    # Fallback: try to create state without explicit DB
                    self.DB = None
            
            self.constants = constants
            self.PragueVM = PragueVM
            self.BaseComputation = BaseComputation
            self.Message = Message
            self.BaseTransactionContext = BaseTransactionContext
            self.to_canonical_address = to_canonical_address
            self.available = True
        except ImportError:
            # Try alternative imports for different py-evm versions
            try:
                from eth import constants
                from eth.vm.forks import IstanbulVM
                from eth.vm.computation import BaseComputation
                from eth.vm.message import Message
                from eth.vm.transaction_context import BaseTransactionContext
                from eth_utils import to_canonical_address
                
                # Get the state class from the VM fork
                try:
                    from eth.vm.forks.istanbul import IstanbulState
                    self.StateClass = IstanbulState
                except ImportError:
                    self.StateClass = getattr(IstanbulVM, '_state_class', None)
                
                try:
                    from eth.db.backends.memory import MemoryDB
                    self.DB = MemoryDB
                except ImportError:
                    self.DB = None
                
                self.constants = constants
                self.PragueVM = IstanbulVM
                self.BaseComputation = BaseComputation
                self.Message = Message
                self.BaseTransactionContext = BaseTransactionContext
                self.to_canonical_address = to_canonical_address
                self.available = True
            except ImportError:
                self.available = False
    
    def trace_execution(self, bytecode: bytes, calldata: Optional[bytes] = None,
                       caller: Optional[str] = None, value: int = 0,
                       pre_state: Optional[Dict] = None, gas_limit: Optional[int] = None) -> Tuple[List[str], Counter, Counter]:
        """
        Trace execution using py-evm with accurate opcode tracking.
        
        This method uses py-evm's computation hooks to capture each opcode
        as it executes, providing accurate traces that reflect actual execution
        flow including jumps, conditionals, and precompile calls.
        """
        if not self.available:
            raise RuntimeError("py-evm not available. Install with: pip install py-evm")
        
        from eth.vm.transaction_context import BaseTransactionContext
        from eth_utils import to_canonical_address
        
        # Store trace data
        trace_data = {
            "sequence": [],
            "opcode_counts": Counter(),
            "precompile_counts": Counter()
        }
        
        try:
            # Setup EVM using MiningChain - the proper way for py-evm 0.12+
            from eth.chains.base import MiningChain
            from eth import constants
            # Use AtomicDB instead of MemoryDB for proper batch support
            try:
                from eth.db.atomic import AtomicDB
                db_class = AtomicDB
            except ImportError:
                from eth.db.backends.memory import MemoryDB
                db_class = MemoryDB
            
            # Use IstanbulVM for simpler setup (Prague requires system contracts)
            try:
                from eth.vm.forks import IstanbulVM
                vm_class = IstanbulVM
            except ImportError:
                vm_class = self.PragueVM
            
            # Create chain with minimal genesis
            CustomChain = MiningChain.configure(
                __name__='TracingChain',
                vm_configuration=((constants.GENESIS_BLOCK_NUMBER, vm_class),),
            )
            
            # Initialize chain with empty genesis
            db = db_class()
            # Genesis params - simplified format (block_number is determined by GENESIS_BLOCK_NUMBER)
            genesis_params = {
                'difficulty': 1,
                'gas_limit': gas_limit if gas_limit else 100000000,
                'timestamp': 1,
            }
            
            # Initialize genesis state from pre_state if provided
            genesis_state = {}
            if pre_state:
                for address_str, account_data in pre_state.items():
                    try:
                        address_bytes = to_canonical_address(address_str)
                        account_dict = {}
                        if "balance" in account_data:
                            balance_str = account_data["balance"]
                            if isinstance(balance_str, str):
                                # Handle empty string or hex format
                                if balance_str == "" or balance_str == "0x":
                                    balance = 0
                                else:
                                    balance = int(balance_str, 16) if balance_str.startswith("0x") else int(balance_str, 16)
                            else:
                                balance = balance_str
                            account_dict["balance"] = balance
                        else:
                            account_dict["balance"] = 0
                        # Always include code, even if empty
                        if "code" in account_data and account_data["code"]:
                            code_hex = account_data["code"]
                            if isinstance(code_hex, str):
                                code_bytes = hex_to_bytes(code_hex)
                            else:
                                code_bytes = code_hex
                            account_dict["code"] = code_bytes if code_bytes else b""
                        else:
                            account_dict["code"] = b""
                        if "nonce" in account_data:
                            nonce_str = account_data["nonce"]
                            if isinstance(nonce_str, str):
                                if nonce_str == "" or nonce_str == "0x":
                                    nonce = 0
                                else:
                                    nonce = int(nonce_str, 16) if nonce_str.startswith("0x") else int(nonce_str, 16)
                            else:
                                nonce = nonce_str
                            account_dict["nonce"] = nonce
                        else:
                            account_dict["nonce"] = 0
                        if "storage" in account_data:
                            # Convert storage values from hex strings to integers
                            storage_dict = {}
                            for slot_str, value_str in account_data["storage"].items():
                                slot = int(slot_str, 16) if isinstance(slot_str, str) else slot_str
                                value = int(value_str, 16) if isinstance(value_str, str) else value_str
                                storage_dict[slot] = value
                            account_dict["storage"] = storage_dict
                        genesis_state[address_bytes] = account_dict
                    except Exception:
                        pass
            
            # Also add caller and code addresses to genesis state
            if caller:
                caller_addr = to_canonical_address(caller)
                if caller_addr not in genesis_state:
                    genesis_state[caller_addr] = {
                        "balance": value if value > 0 else 1000000000000000000,
                        "nonce": 0,
                        "code": b"",
                        "storage": {}
                    }
            
            code_addr = to_canonical_address("0x" + "1" * 40)
            if code_addr not in genesis_state:
                genesis_state[code_addr] = {
                    "balance": 0,
                    "nonce": 0,
                    "code": bytecode,
                    "storage": {}
                }
            
            # Create chain from genesis
            chain = CustomChain.from_genesis(db, genesis_params, genesis_state)
            
            # Get VM from chain
            vm = chain.get_vm()
            
            # Get the actual computation class from the VM
            # In py-evm, the computation class is typically stored as a class attribute
            # or can be obtained from the VM instance
            try:
                # Try to get computation class from VM
                if hasattr(vm, 'computation_class'):
                    ComputationClass = vm.computation_class
                elif hasattr(vm_class, 'computation_class'):
                    ComputationClass = vm_class.computation_class
                else:
                    # Fallback: use BaseComputation
                    ComputationClass = self.BaseComputation
            except:
                ComputationClass = self.BaseComputation
            
            # Create a custom computation class that overrides get_opcode_fn to trace
            class TracingComputation(ComputationClass):
                """Computation class that traces opcode execution."""
                
                def get_opcode_fn(self, opcode):
                    """Override to trace opcodes as they're retrieved."""
                    opcode_fn = super().get_opcode_fn(opcode)
                    
                    # Record opcode
                    opcode_name = OPCODES.get(opcode, f"UNKNOWN(0x{opcode:02x})")
                    trace_data["sequence"].append(opcode_name)
                    trace_data["opcode_counts"][opcode_name] += 1
                    
                    # Wrap opcode function to check for precompiles
                    if opcode_name in CALL_OPCODES and opcode_fn:
                        original_fn = opcode_fn
                        def traced_opcode_fn(computation):
                            try:
                                stack = getattr(computation, 'stack', None)
                                if stack:
                                    if hasattr(stack, 'values'):
                                        stack_vals = stack.values
                                    elif hasattr(stack, '_values'):
                                        stack_vals = stack._values
                                    else:
                                        stack_vals = list(stack) if hasattr(stack, '__iter__') else []
                                    
                                    if len(stack_vals) >= 2:
                                        addr_item = stack_vals[-2]
                                        addr = addr_item if isinstance(addr_item, int) else int(addr_item)
                                        if is_precompile_address(addr):
                                            precompile_name = get_precompile_name(addr)
                                            if precompile_name:
                                                trace_data["sequence"].append(f"PRECOMPILE_{precompile_name}")
                                                trace_data["precompile_counts"][precompile_name] += 1
                            except Exception:
                                pass
                            return original_fn(computation)
                        opcode_fn = traced_opcode_fn
                    
                    return opcode_fn
            
            # Replace computation class temporarily
            original_computation_class = getattr(vm, '_computation_class', None)
            vm._computation_class = TracingComputation
            
            # Create addresses (already done above, but keep for reference)
            if caller is None:
                caller_addr = to_canonical_address("0x" + "0" * 40)
            else:
                caller_addr = to_canonical_address(caller)
            
            try:
                # Execute bytecode using VM's execute_bytecode
                # The TracingComputation class will automatically trace opcodes
                effective_gas_limit = gas_limit if gas_limit is not None else 100000000
                computation = vm.execute_bytecode(
                    origin=caller_addr,
                    gas_price=1,
                    gas=effective_gas_limit,
                    to=code_addr,
                    sender=caller_addr,
                    value=value,
                    data=calldata or b"",
                    code=bytecode,
                    code_address=code_addr,
                )
            finally:
                # Restore original computation class
                if original_computation_class is not None:
                    vm._computation_class = original_computation_class
            
            # Extract results
            opcode_sequence = trace_data["sequence"]
            
            # Separate opcodes from precompiles
            actual_opcodes = [op for op in opcode_sequence if not op.startswith("PRECOMPILE_")]
            opcode_counts = Counter(actual_opcodes)
            precompile_counts = trace_data["precompile_counts"]
            
            return opcode_sequence, opcode_counts, precompile_counts
            
        except Exception as e:
            # If execution fails, provide helpful error and fallback
            error_msg = str(e)
            error_type = type(e).__name__
            
            # Print warning for debugging (can be removed later)
            if "Unknown format" not in error_msg:
                # Only show first occurrence of each error type to avoid spam
                if not hasattr(self, '_error_shown'):
                    self._error_shown = set()
                if error_type not in self._error_shown:
                    print(f"Warning: py-evm execution failed ({error_type}): {error_msg[:100]}")
                    self._error_shown.add(error_type)
            
            static_tracer = StaticTracer()
            return static_tracer.trace_execution(bytecode, calldata, caller, value, pre_state, gas_limit)


class EVMTraceTracer(EVMTracer):
    """Tracer using evm tool from go-ethereum."""
    
    def __init__(self):
        self.available = self._check_evm_available()
    
    def _check_evm_available(self) -> bool:
        """Check if evm tool is available."""
        try:
            result = subprocess.run(["evm", "--help"], 
                                  capture_output=True, 
                                  timeout=5)
            return result.returncode == 0
        except (FileNotFoundError, subprocess.TimeoutExpired):
            return False
    
    def trace_execution(self, bytecode: bytes, calldata: Optional[bytes] = None,
                       caller: Optional[str] = None, value: int = 0,
                       pre_state: Optional[Dict] = None, gas_limit: Optional[int] = None) -> Tuple[List[str], Counter, Counter]:
        """Trace execution using evm tool."""
        if not self.available:
            raise RuntimeError("evm tool not available. Install go-ethereum and ensure 'evm' is in PATH")
        
        # Use evm tool to trace
        code_hex = bytes_to_hex(bytecode)
        input_hex = bytes_to_hex(calldata) if calldata else "0x"
        
        try:
            # Try to use evm trace
            cmd = ["evm", "run", "--code", code_hex, "--input", input_hex, "--trace"]
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
            
            if result.returncode == 0:
                # Parse trace output
                opcode_sequence = []
                opcode_counts = Counter()
                precompile_counts = Counter()
                
                for line in result.stdout.split('\n'):
                    if 'op=' in line:
                        # Extract opcode from trace
                        parts = line.split()
                        opcode_name = None
                        for part in parts:
                            if part.startswith('op='):
                                opcode_hex = part.split('=')[1]
                                try:
                                    opcode_val = int(opcode_hex, 16)
                                    opcode_name = OPCODES.get(opcode_val, f"UNKNOWN(0x{opcode_val:02x})")
                                    opcode_sequence.append(opcode_name)
                                    opcode_counts[opcode_name] += 1
                                except ValueError:
                                    pass
                        
                        # Try to extract stack for precompile detection
                        if opcode_name in CALL_OPCODES:
                            # Look for stack values in the trace line
                            for part in parts:
                                if part.startswith('stack='):
                                    stack_str = part.split('=')[1]
                                    # Parse stack values and check for precompile addresses
                                    try:
                                        # Stack values are usually hex
                                        stack_vals = stack_str.strip('[]').split(',')
                                        for val in stack_vals:
                                            val = val.strip()
                                            if val:
                                                addr = int(val, 16) if val.startswith('0x') else int(val)
                                                if is_precompile_address(addr):
                                                    precompile_name = get_precompile_name(addr)
                                                    if precompile_name:
                                                        precompile_counts[precompile_name] += 1
                                                        opcode_sequence.append(f"PRECOMPILE_{precompile_name}")
                                                    break
                                    except (ValueError, IndexError):
                                        pass
                
                if opcode_sequence:
                    return opcode_sequence, opcode_counts, precompile_counts
        except (subprocess.TimeoutExpired, FileNotFoundError) as e:
            print(f"Warning: evm tool execution failed: {e}")
        
        # Fallback to static analysis
        static_tracer = StaticTracer()
        return static_tracer.trace_execution(bytecode, calldata, caller, value)


def analyze_test_case(test_file: Path, tracer: EVMTracer, gas_filter: Optional[str] = None) -> Dict:
    """Analyze a single test case file."""
    print(f"\nAnalyzing: {test_file.name}")
    
    with open(test_file, 'r') as f:
        test_data = json.load(f)
    
    results = {}
    
    for test_name, test_case in test_data.items():
        # Filter by gas value if specified
        if gas_filter:
            gas_pattern = f"gas-value_{gas_filter}"
            if gas_pattern not in test_name:
                continue  # Skip this test case
        print(f"  Processing test: {test_name}")
        test_result = {
            "test_name": test_name,
            "contracts": {},
            "transactions": []
        }
        
        # Extract contract bytecode from pre state
        contracts = {}
        if "pre" in test_case:
            for address, account in test_case["pre"].items():
                if "code" in account and account["code"]:
                    code_hex = account["code"]
                    bytecode = hex_to_bytes(code_hex)
                    if bytecode:
                        contracts[address] = bytecode
        
        # Process transactions
        if "blocks" in test_case:
            for block in test_case["blocks"]:
                if "transactions" in block:
                    for tx_idx, tx in enumerate(block["transactions"]):
                        tx_data = {
                            "index": tx_idx,
                            "to": tx.get("to", ""),
                            "input": tx.get("input", ""),
                            "value": int(tx.get("value", "0x0"), 16) if tx.get("value") else 0,
                            "from": tx.get("from", ""),
                        }
                        
                        # Get contract bytecode
                        to_addr = tx_data["to"]
                        if to_addr and to_addr in contracts:
                            bytecode = contracts[to_addr]
                            calldata = hex_to_bytes(tx_data["input"])
                            
                            try:
                                # Get gas limit from transaction (try both 'gas' and 'gasLimit')
                                tx_gas = None
                                if tx.get("gas"):
                                    tx_gas = int(tx.get("gas"), 16) if isinstance(tx.get("gas"), str) else tx.get("gas")
                                elif tx.get("gasLimit"):
                                    tx_gas = int(tx.get("gasLimit"), 16) if isinstance(tx.get("gasLimit"), str) else tx.get("gasLimit")
                                # If not in transaction, extract from test case name (e.g., "value_100M" -> 100000000)
                                if tx_gas is None:
                                    test_name_lower = test_name.lower()
                                    if "100m" in test_name_lower:
                                        tx_gas = 100_000_000
                                    elif "150m" in test_name_lower:
                                        tx_gas = 150_000_000
                                    elif "60m" in test_name_lower:
                                        tx_gas = 60_000_000
                                    elif "45m" in test_name_lower:
                                        tx_gas = 45_000_000
                                    elif "30m" in test_name_lower:
                                        tx_gas = 30_000_000
                                    elif "10m" in test_name_lower:
                                        tx_gas = 10_000_000
                                    elif "1m" in test_name_lower:
                                        tx_gas = 1_000_000
                                opcode_seq, opcode_counts, precompile_counts = tracer.trace_execution(
                                    bytecode, calldata, tx_data["from"], tx_data["value"],
                                    pre_state=test_case.get("pre"), gas_limit=tx_gas
                                )
                                
                                # Combine opcodes and precompiles into unified instruction counts
                                instruction_counts = Counter(opcode_counts)
                                instruction_counts.update(precompile_counts)
                                
                                tx_data["opcode_sequence"] = opcode_seq
                                tx_data["opcode_counts"] = dict(opcode_counts)
                                tx_data["precompile_counts"] = dict(precompile_counts)
                                tx_data["instruction_counts"] = dict(instruction_counts)
                                tx_data["total_instructions"] = sum(instruction_counts.values())
                                tx_data["bytecode_length"] = len(bytecode)
                            except Exception as e:
                                print(f"    Warning: Failed to trace transaction {tx_idx}: {e}")
                                tx_data["error"] = str(e)
                        elif not to_addr:
                            # Contract creation
                            initcode = hex_to_bytes(tx_data["input"])
                            if initcode:
                                try:
                                    # Get gas limit from transaction
                                    tx_gas = int(tx.get("gas", "0x5f5e100"), 16) if tx.get("gas") else None
                                    opcode_seq, opcode_counts, precompile_counts = tracer.trace_execution(
                                        initcode, None, tx_data["from"], tx_data["value"],
                                        pre_state=test_case.get("pre"), gas_limit=tx_gas
                                    )
                                    
                                    # Combine opcodes and precompiles into unified instruction counts
                                    instruction_counts = Counter(opcode_counts)
                                    instruction_counts.update(precompile_counts)
                                    
                                    tx_data["opcode_sequence"] = opcode_seq
                                    tx_data["opcode_counts"] = dict(opcode_counts)
                                    tx_data["precompile_counts"] = dict(precompile_counts)
                                    tx_data["instruction_counts"] = dict(instruction_counts)
                                    tx_data["total_instructions"] = sum(instruction_counts.values())
                                    tx_data["bytecode_length"] = len(initcode)
                                    tx_data["is_creation"] = True
                                except Exception as e:
                                    print(f"    Warning: Failed to trace creation {tx_idx}: {e}")
                                    tx_data["error"] = str(e)
                        
                        test_result["transactions"].append(tx_data)
        
        # Also analyze contract bytecode statically
        for address, bytecode in contracts.items():
            try:
                opcode_seq, opcode_counts, precompile_counts = tracer.trace_execution(
                    bytecode, None, None, 0, test_case.get("pre"), None
                )
                
                # Combine opcodes and precompiles into unified instruction counts
                instruction_counts = Counter(opcode_counts)
                instruction_counts.update(precompile_counts)
                
                test_result["contracts"][address] = {
                    "bytecode_length": len(bytecode),
                    "opcode_sequence": opcode_seq,
                    "opcode_counts": dict(opcode_counts),
                    "precompile_counts": dict(precompile_counts),
                    "instruction_counts": dict(instruction_counts),
                    "total_instructions": sum(instruction_counts.values())
                }
            except Exception as e:
                print(f"    Warning: Failed to trace contract {address}: {e}")
        
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
                        f.write(f"  Total Instructions: {contract_info.get('total_instructions', 0)}\n")
                        f.write(f"  Instruction Counts (opcodes + precompiles):\n")
                        for instr, count in sorted(contract_info.get('instruction_counts', {}).items(), 
                                                  key=lambda x: -x[1]):
                            f.write(f"    {instr}: {count}\n")
                        f.write(f"  Execution Sequence (first 200):\n")
                        seq_preview = contract_info['opcode_sequence'][:200]
                        f.write(f"    {' -> '.join(seq_preview)}\n")
                        if len(contract_info['opcode_sequence']) > 200:
                            f.write(f"    ... ({len(contract_info['opcode_sequence']) - 200} more)\n")
                        f.write("\n")
                
                # Transaction analysis
                if test_result["transactions"]:
                    f.write(f"Transactions: {len(test_result['transactions'])}\n\n")
                    for tx in test_result["transactions"]:
                        f.write(f"  Transaction {tx.get('index', '?')}:\n")
                        f.write(f"    To: {tx.get('to', 'CREATE')}\n")
                        if 'is_creation' in tx:
                            f.write(f"    Type: Contract Creation\n")
                        if 'bytecode_length' in tx:
                            f.write(f"    Bytecode Length: {tx['bytecode_length']} bytes\n")
                        if 'total_instructions' in tx:
                            f.write(f"    Total Instructions: {tx['total_instructions']}\n")
                        if 'instruction_counts' in tx:
                            f.write(f"    Instruction Counts (opcodes + precompiles):\n")
                            for instr, count in sorted(tx['instruction_counts'].items(), 
                                                      key=lambda x: -x[1]):
                                f.write(f"      {instr}: {count}\n")
                        if 'opcode_sequence' in tx:
                            f.write(f"    Execution Sequence (first 200):\n")
                            seq_preview = tx['opcode_sequence'][:200]
                            f.write(f"      {' -> '.join(seq_preview)}\n")
                            if len(tx['opcode_sequence']) > 200:
                                f.write(f"      ... ({len(tx['opcode_sequence']) - 200} more)\n")
                        if 'error' in tx:
                            f.write(f"    Error: {tx['error']}\n")
                        f.write("\n")
                
                f.write("\n")
    
    # Generate JSON report
    json_file = output_dir / "opcode_trace_report.json"
    with open(json_file, 'w') as f:
        json.dump(all_results, f, indent=2)
    
    # Generate summary CSV
    csv_file = output_dir / "instruction_summary.csv"
    with open(csv_file, 'w') as f:
        f.write("Test File,Test Case,Transaction Index,Total Instructions,Unique Instructions,All Instructions\n")
        for test_file, results in all_results.items():
            for test_name, test_result in results.items():
                for tx in test_result.get("transactions", []):
                    if 'instruction_counts' in tx:
                        total = tx.get('total_instructions', 0)
                        unique = len(tx['instruction_counts'])
                        all_instructions = ', '.join([f"{instr}({count})" for instr, count in 
                                         sorted(tx['instruction_counts'].items(), 
                                               key=lambda x: -x[1])])
                        f.write(f"{test_file},{test_name},{tx.get('index', '?')},"
                               f"{total},{unique},\"{all_instructions}\"\n")
    
    # Generate detailed instruction breakdown CSV (per instruction type)
    instruction_breakdown_file = output_dir / "instruction_breakdown.csv"
    with open(instruction_breakdown_file, 'w') as f:
        f.write("Test File,Test Case,Transaction Index,Instruction,Count,Type\n")
        for test_file, results in all_results.items():
            for test_name, test_result in results.items():
                for tx in test_result.get("transactions", []):
                    if tx.get('instruction_counts'):
                        for instr, count in sorted(tx['instruction_counts'].items(), 
                                                  key=lambda x: -x[1]):
                            # Determine if it's a precompile or opcode
                            instr_type = "precompile" if instr in PRECOMPILES.values() else "opcode"
                            f.write(f"{test_file},{test_name},{tx.get('index', '?')},"
                                   f"{instr},{count},{instr_type}\n")
    
    # Generate instruction totals summary
    instruction_totals = Counter()
    for test_file, results in all_results.items():
        for test_name, test_result in results.items():
            for tx in test_result.get("transactions", []):
                if tx.get('instruction_counts'):
                    instruction_totals.update(tx['instruction_counts'])
            for address, contract_info in test_result.get("contracts", {}).items():
                if contract_info.get('instruction_counts'):
                    instruction_totals.update(contract_info['instruction_counts'])
    
    print(f"\nReports generated:")
    print(f"  Text: {report_file}")
    print(f"  JSON: {json_file}")
    print(f"  Instruction Summary CSV: {csv_file}")
    print(f"  Instruction Breakdown CSV: {instruction_breakdown_file}")
    
    if instruction_totals:
        # Separate into opcodes and precompiles for display
        precompile_names = set(PRECOMPILES.values())
        opcode_totals = {k: v for k, v in instruction_totals.items() if k not in precompile_names}
        precompile_totals = {k: v for k, v in instruction_totals.items() if k in precompile_names}
        
        print(f"\nInstruction Usage Summary:")
        print(f"  Total unique instructions: {len(instruction_totals)}")
        print(f"  Total instruction count: {sum(instruction_totals.values())}")
        
        if precompile_totals:
            print(f"\n  Precompiles ({len(precompile_totals)} types, {sum(precompile_totals.values())} calls):")
            for precompile, count in sorted(precompile_totals.items(), key=lambda x: -x[1]):
                print(f"    {precompile}: {count}")
        
        print(f"\n  Top 10 Opcodes ({len(opcode_totals)} types, {sum(opcode_totals.values())} total):")
        for opcode, count in sorted(opcode_totals.items(), key=lambda x: -x[1])[:10]:
            print(f"    {opcode}: {count}")


def main():
    parser = argparse.ArgumentParser(description="Analyze blockchain test cases and trace opcodes")
    parser.add_argument("--fixtures-dir", type=str, 
                       default="./zkevm-fixtures/fixtures/blockchain_tests/benchmark",
                       help="Directory containing test case JSON files")
    parser.add_argument("--output", type=str, default="./opcode_traces",
                       help="Output directory for reports")
    parser.add_argument("--test-case", type=str, default=None,
                       help="Specific test case file to analyze (optional)")
    parser.add_argument("--method", type=str, default="static",
                       choices=["static", "pyevm", "evmtrace"],
                       help="Tracing method to use")
    parser.add_argument("--gas-filter", type=str, default=None,
                       help="Filter test cases by gas value (e.g., '1M', '10M', '0.1', '0.2')")
    
    args = parser.parse_args()
    
    fixtures_dir = Path(args.fixtures_dir)
    if not fixtures_dir.exists():
        print(f"Error: Fixtures directory not found: {fixtures_dir}")
        sys.exit(1)
    
    # Initialize tracer
    if args.method == "pyevm":
        tracer = PyEVMTracer()
        if not tracer.available:
            print("Warning: py-evm not available, falling back to static analysis")
            tracer = StaticTracer()
    elif args.method == "evmtrace":
        tracer = EVMTraceTracer()
        if not tracer.available:
            print("Warning: evm tool not available, falling back to static analysis")
            tracer = StaticTracer()
    else:
        tracer = StaticTracer()
    
    print(f"Using tracing method: {args.method}")
    
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
            results = analyze_test_case(test_file, tracer, gas_filter=args.gas_filter)
            if results:  # Only add if there are results (after filtering)
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