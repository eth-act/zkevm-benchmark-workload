#!/usr/bin/env python3
"""
Test name parser utility for zkEVM benchmark results.

This module provides functions to parse complex test file names and extract
meaningful information that can be used for display purposes in markdown tables.
"""

from typing import List, NamedTuple, Tuple


class TestInfo(NamedTuple):
    """Container for parsed test information."""
    category: str
    function: str
    parameters: List[str]
    simplified_name: str
    display_name: str


class TestNameParser:
    """Parser for complex test file names."""
    
    def __init__(self):
        # Mapping of test categories to their prefixes
        self.category_prefixes = {
            'test_worst_blocks.py': 'blocks',
            'test_worst_bytecode.py': 'bytecode', 
            'test_worst_compute.py': 'compute',
            'test_worst_memory.py': 'memory',
            'test_worst_opcode.py': 'opcode',
            'test_worst_stateful_opcodes.py': 'stateful'
        }
        
        # Common parameter patterns to simplify
        self.param_simplifications = {
            'fork_Prague': '',
            'benchmark-gas-value_1M': '',
            'blockchain_test': '',
            'blockchain_test_from_state_test': '',
            'big_memory_expansion_True': 'big_mem',
            'big_memory_expansion_False': 'small_mem',
            'offset_initialized_True': 'init_offset',
            'offset_initialized_False': 'uninit_offset',
            'non_zero_data_True': 'non_zero',
            'non_zero_data_False': 'zero_data',
            'fixed_offset_True': 'fixed',
            'fixed_offset_False': 'dynamic',
            'zeros_topic': 'zero_topic',
            'non_zero_topic': 'non_zero_topic',
            'value_bearing_True': 'with_value',
            'value_bearing_False': 'no_value',
            'absent_target_True': 'absent_target',
            'absent_target_False': 'present_target',
            'fixed_src_dst_True': 'fixed_src_dst',
            'fixed_src_dst_False': 'dynamic_src_dst',
            'zero_byte_True': 'zero_byte',
            'zero_byte_False': 'non_zero_byte'
        }
    
    def extract_parameters(self, filename: str) -> Tuple[str, str, List[str]]:
        """
        Extract test category, function, and parameters from filename.
        
        Args:
            filename: The test filename (with or without .json extension)
            
        Returns:
            (category, function, parameters)
        """
        # Remove .json extension
        name = filename.replace('.json', '')
        
        # Split on :: to get test_file and test_function[params]
        if '::' not in name:
            return '', '', []
            
        test_file, rest = name.split('::', 1)
        
        # Extract function name and parameters
        if '[' in rest and ']' in rest:
            function_part = rest[:rest.index('[')]
            params_part = rest[rest.index('[')+1:rest.rindex(']')]
            # Split on '-' but be careful with benchmark-gas-value which contains hyphens
            parameters = []
            current_param = ""
            i = 0
            while i < len(params_part):
                if params_part[i] == '-':
                    # Check if this is part of "benchmark-gas-value"
                    if params_part[i:i+8] == '-gas-value':
                        current_param += params_part[i]
                    else:
                        # This is a parameter separator
                        if current_param.strip():
                            parameters.append(current_param.strip())
                        current_param = ""
                else:
                    current_param += params_part[i]
                i += 1
            if current_param.strip():
                parameters.append(current_param.strip())
        else:
            function_part = rest
            parameters = []
        
        return test_file, function_part, parameters
    
    def simplify_parameters(self, parameters: List[str]) -> List[str]:
        """
        Simplify parameter names by removing common prefixes and applying mappings.
        
        Args:
            parameters: List of parameter strings
            
        Returns:
            List of simplified parameter strings
        """
        simplified = []
        
        for param in parameters:
            # Skip common parameters that don't add value
            if param in ['fork_Prague', 'benchmark-gas-value_1M', 'blockchain_test', 'blockchain_test_from_state_test']:
                continue
            # Also skip any parameter that starts with benchmark-gas-value
            if param.startswith('benchmark-gas-value'):
                continue
            # Skip individual parts of benchmark-gas-value_1M
            if param in ['benchmark', 'gas', 'value_1M', '1M']:
                continue
                
            # Apply simplifications
            if param in self.param_simplifications:
                simplified_param = self.param_simplifications[param]
                if simplified_param:  # Only add if not empty
                    simplified.append(simplified_param)
            else:
                # For parameters not in our mapping, try to extract meaningful parts
                if param.startswith('opcode_'):
                    simplified.append(param.replace('opcode_', ''))
                elif param.startswith('case_id_'):
                    simplified.append(param.replace('case_id_', ''))
                elif param.startswith('offset_'):
                    simplified.append(param.replace('offset_', 'off_'))
                elif param.startswith('size_'):
                    simplified.append(param.replace('size_', ''))
                elif param.startswith('data_'):
                    simplified.append(param.replace('data_', ''))
                elif param.startswith('value_'):
                    simplified.append(param.replace('value_', ''))
                elif param.startswith('0 bytes'):
                    simplified.append('0bytes')
                elif param.startswith('100 bytes'):
                    simplified.append('100bytes')
                elif param.startswith('1 MiB'):
                    simplified.append('1MiB')
                elif param.startswith('0.25x max code size'):
                    simplified.append('0.25x_max_code')
                elif param.startswith('max code size'):
                    simplified.append('max_code')
                elif param.startswith('with value'):
                    simplified.append('with_value')
                elif param.startswith('without value'):
                    simplified.append('without_value')
                elif param.startswith('with non-zero data'):
                    simplified.append('non_zero_data')
                elif param.startswith('with zero data'):
                    simplified.append('zero_data')
                else:
                    # Keep the parameter as is, but make it shorter if possible
                    simplified.append(param)
        
        return simplified
    
    def parse_test_name(self, filename: str) -> TestInfo:
        """
        Parse a test filename and return structured information.
        
        Args:
            filename: The test filename (with or without .json extension)
            
        Returns:
            TestInfo object containing parsed information
        """
        test_file, function, parameters = self.extract_parameters(filename)
        
        # Get category prefix
        category = self.category_prefixes.get(test_file, 'unknown')
        
        # Simplify parameters
        simplified_params = self.simplify_parameters(parameters)
        
        # Create simplified name
        if simplified_params:
            param_str = '_'.join(simplified_params)
            simplified_name = f"{category}_{param_str}"
        else:
            # Fallback to function name if no meaningful parameters
            function_clean = function.replace('test_worst_', '').replace('test_', '')
            simplified_name = f"{category}_{function_clean}"
        
        # Create display name (more human-readable)
        display_name = self._create_display_name(category, function, simplified_params)
        
        return TestInfo(
            category=category,
            function=function,
            parameters=simplified_params,
            simplified_name=simplified_name,
            display_name=display_name
        )
    
    def _create_display_name(self, category: str, function: str, parameters: List[str]) -> str:
        """
        Create a human-readable display name.
        
        Args:
            category: Test category
            function: Test function name
            parameters: Simplified parameters
            
        Returns:
            Human-readable display name
        """
        # Clean up function name
        function_clean = function.replace('test_worst_', '').replace('test_', '')
        function_clean = function_clean.replace('_', ' ').title()
        
        # Create readable parameter string
        if parameters:
            param_parts = []
            for param in parameters:
                # Convert underscores to spaces and capitalize
                readable_param = param.replace('_', ' ').title()
                param_parts.append(readable_param)
            
            param_str = ', '.join(param_parts)
            return f"{function_clean} ({param_str})"
        else:
            return function_clean
    
    def get_display_name(self, filename: str) -> str:
        """
        Get a human-readable display name for a test file.
        
        Args:
            filename: The test filename
            
        Returns:
            Human-readable display name
        """
        test_info = self.parse_test_name(filename)
        return test_info.display_name
    
    def get_simplified_name(self, filename: str) -> str:
        """
        Get a simplified name for a test file.
        
        Args:
            filename: The test filename
            
        Returns:
            Simplified name
        """
        test_info = self.parse_test_name(filename)
        return test_info.simplified_name
    
    def get_category(self, filename: str) -> str:
        """
        Get the category of a test file.
        
        Args:
            filename: The test filename
            
        Returns:
            Test category
        """
        test_info = self.parse_test_name(filename)
        return test_info.category


# Global parser instance
_parser = TestNameParser()


def parse_test_name(filename: str) -> TestInfo:
    """
    Parse a test filename and return structured information.
    
    Args:
        filename: The test filename (with or without .json extension)
        
    Returns:
        TestInfo object containing parsed information
    """
    return _parser.parse_test_name(filename)


def get_display_name(filename: str) -> str:
    """
    Get a human-readable display name for a test file.
    
    Args:
        filename: The test filename
        
    Returns:
        Human-readable display name
    """
    return _parser.get_display_name(filename)


def get_simplified_name(filename: str) -> str:
    """
    Get a simplified name for a test file.
    
    Args:
        filename: The test filename
        
    Returns:
        Simplified name
    """
    return _parser.get_simplified_name(filename)


def get_category(filename: str) -> str:
    """
    Get the category of a test file.
    
    Args:
        filename: The test filename
        
    Returns:
        Test category
    """
    return _parser.get_category(filename)


def main():
    """Test the parser with example filenames."""
    test_files = [
        "test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_True-offset_initialized_False-offset_0-opcode_MSTORE8].json",
        "test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_False-offset_initialized_False-offset_31-opcode_MSTORE].json",
        "test_worst_stateful_opcodes.py::test_worst_selfdestruct_created[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-value_bearing_False].json",
        "test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-1_MiB_non_zero_data-log3].json"
    ]
    
    print("Test Name Parser Results:")
    print("=" * 80)
    
    for filename in test_files:
        test_info = parse_test_name(filename)
        print(f"Original: {filename}")
        print(f"Category: {test_info.category}")
        print(f"Function: {test_info.function}")
        print(f"Parameters: {test_info.parameters}")
        print(f"Simplified: {test_info.simplified_name}")
        print(f"Display: {test_info.display_name}")
        print("-" * 80)


if __name__ == "__main__":
    main()