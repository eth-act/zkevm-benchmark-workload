# Using py-evm for Accurate EVM Opcode Traces

## Overview

The `PyEVMTracer` class in `trace_opcodes.py` uses py-evm to execute EVM bytecode and capture opcodes during actual execution. This provides accurate traces that reflect real execution flow, unlike static analysis which can miss execution paths.

## Installation

```bash
pip install py-evm
```

## Usage

### Command Line

```bash
# Use py-evm for accurate traces
python3 trace_opcodes.py --method pyevm

# Analyze specific test case
python3 trace_opcodes.py --test-case worst_push --method pyevm
```

### Programmatic Usage

```python
from scripts.trace_opcodes import PyEVMTracer

# Create tracer
tracer = PyEVMTracer()

if tracer.available:
    # Trace execution
    opcode_seq, opcode_counts, precompile_counts = tracer.trace_execution(
        bytecode=bytecode_bytes,
        calldata=calldata_bytes,  # Optional
        caller="0x1234...",        # Optional
        value=0                    # Optional, in wei
    )
    
    print(f"Executed {len(opcode_seq)} opcodes")
    print(f"Unique opcodes: {len(opcode_counts)}")
    print(f"Precompile calls: {sum(precompile_counts.values())}")
else:
    print("py-evm not available. Install with: pip install py-evm")
```

## How It Works

1. **Custom Computation Class**: Creates a `TracingComputation` class that extends py-evm's computation class
2. **Opcode Hooking**: Overrides `_execute_opcode()` to capture each opcode before execution
3. **Stack Inspection**: For CALL opcodes, inspects the stack to detect precompile addresses
4. **Execution**: Uses py-evm's VM to execute bytecode with the tracing computation class

## Advantages Over Static Analysis

- ✅ **Handles Jumps Correctly**: Follows actual execution path, not just linear analysis
- ✅ **Respects Conditionals**: Only traces executed branches (JUMPI)
- ✅ **Detects Precompiles**: Inspects stack values during CALL to identify precompiles
- ✅ **Dynamic Execution**: Accounts for CREATE, CALL, and other dynamic code execution
- ✅ **Accurate Counts**: Each opcode is counted as it actually executes

## Example Output

```python
opcode_seq = [
    "PUSH1", "0x60", "PUSH1", "0x40", "MSTORE", 
    "CALLVALUE", "DUP1", "ISZERO", "PUSH2", "0x0010", 
    "JUMPI", "PUSH1", "0x00", "DUP1", "REVERT", 
    "JUMPDEST", "POP", "CALLER", ...
]

opcode_counts = Counter({
    "PUSH1": 15,
    "DUP1": 8,
    "MSTORE": 3,
    "JUMPI": 2,
    ...
})

precompile_counts = Counter({
    "SHA256": 1,
    "ECRECOVER": 0,
    ...
})
```

## Troubleshooting

### Import Errors

If you see import errors, ensure py-evm is installed:
```bash
pip install py-evm
```

### Execution Failures

If execution fails, the tracer falls back to static analysis. Common issues:
- Invalid bytecode
- Unsupported opcodes for the VM fork
- Gas limit exceeded (increase gas in the message)

### VM Fork Compatibility

The tracer tries to use PragueVM first, then falls back to IstanbulVM. If your bytecode uses newer opcodes, ensure you have a compatible py-evm version.

## Comparison with Other Methods

| Method | Accuracy | Speed | Dependencies |
|--------|----------|-------|--------------|
| `pyevm` | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | py-evm |
| `static` | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | None |
| `evmtrace` | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | go-ethereum |

## Key Implementation Details

The tracer works by:

1. **Replacing Computation Class**: Temporarily replaces the VM's computation class with a tracing version
2. **Opcode Capture**: In `_execute_opcode()`, captures the opcode byte from `self.msg.code[pc]`
3. **Stack Access**: For precompile detection, accesses `self.stack.values[-2]` to get the address
4. **Execution**: Calls `vm.execute_bytecode(message)` which uses the tracing computation

## Best Practices

1. **Use pyevm for accurate traces**: Always use `--method pyevm` when you need accurate execution traces
2. **Handle errors gracefully**: The tracer falls back to static analysis if py-evm fails
3. **Check availability**: Always check `tracer.available` before using
4. **Provide calldata**: Include transaction calldata for accurate function call traces

## See Also

- [py-evm Documentation](https://py-evm.readthedocs.io/)
- `trace_opcodes.py` - Main tracing script
- `analyze_opcode_traces.py` - Analysis of trace results
