"""Behavior tests for the ZisK profile parser."""
from pathlib import Path
import sys
import tempfile
import unittest

SCRIPTS = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(SCRIPTS))
import analyze_zisk_profiles as profiles

# Trimmed ziskemu v1.3.2-alpha `-X -S -D` output.
PROFILE = """\
COST DISTRIBUTION                   COST       %
------------------------------------------------
MAIN                          17,167,824   5.36%
                        ------------------------
BASE                         287,309,824  89.74%
                        ------------------------
TOTAL                        320,170,841 100.00%

COST BY BASE OPCODE                COUNT       %            COST       %
------------------------------------------------------------------------
OP and                             7,897   3.13%         473,820   1.44%
OP add                            22,805   9.03%         359,315   1.09%

COST BY PRECOMPILED OPCODE           COUNT       %            COST       %
--------------------------------------------------------------------------
OP keccak                            119   0.05%       4,576,026  13.93%
"""


class ProfileTests(unittest.TestCase):
    def test_parses_base_and_precompiled_opcodes(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / 'zisk_profile_block_1.prof'
            path.write_text(PROFILE)
            profile = profiles.parse_profile(path)
        self.assertEqual(profile.total_cost, 320_170_841)
        self.assertEqual(profile.opcodes, {
            'and': {'count': 7_897, 'cost': 473_820},
            'add': {'count': 22_805, 'cost': 359_315},
            'keccak': {'count': 119, 'cost': 4_576_026},
        })


if __name__ == '__main__':
    unittest.main()
