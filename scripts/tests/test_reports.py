"""Behavior tests for cost and timing reports."""
import json
from pathlib import Path
import sys
import tempfile
import unittest

SCRIPTS = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(SCRIPTS))
import compare_costs as costs
import compare_executions as times


def estimate(name='case', cost=None, heap=None):
    return {
        'name': name, 'metadata': {'original_test_name': name, 'block_index': 0},
        'cost_estimation': {'success': {
            'output_matched': True, 'cost': {'opcode': 100, 'system': 20} if cost is None else cost,
            'peak_heap_bytes': heap,
            'context': {
                'execution_client': 'reth', 'execution_client_version': '0.1.0-rc.3',
                'zkvm': 'sp1', 'sdk_version': 'v6.4.0', 'ere_revision': '5023513',
                'elf_sha256': 'a' * 64, 'input_sha256': name,
                'estimator_settings': {'heap_start': '_end'},
            },
        }},
    }


def timing(data, secs=1, nanos=0):
    data['execution'] = {'success': {
        'output_matched': True, 'execution_duration': {'secs': secs, 'nanos': nanos},
    }}
    return data


class ReportTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.base, self.head = self.root / 'base', self.root / 'head'
        self.base.mkdir()
        self.head.mkdir()

    def write(self, root, data, filename=None):
        path = root / 'reth-version' / 'sp1-v6.4.0' / (filename or data['name'] + '.json')
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(data))
        return path

    def test_matches_different_guest_versions_and_paths(self):
        before = estimate(heap=1000)
        after = estimate(cost={'opcode': 50, 'system': 10}, heap=700)
        after['cost_estimation']['success']['context'].update(
            execution_client_version='new', elf_sha256='b' * 64)
        self.write(self.base, before)
        path = self.write(self.head, after)
        report, count = costs.render_comparison(self.base, path.parent)
        self.assertEqual(count, 1)
        self.assertIn('| **Total cost** | 120 | 60 | -50.00% |', report)
        self.assertIn('1,000 → 700 bytes (-30.00%)', report)

    def test_incompatible_context_and_inputs_never_compare(self):
        self.write(self.base, estimate())
        for field, value in [('zkvm', 'zisk'), ('sdk_version', 'different'),
                             ('ere_revision', 'different'), ('input_sha256', 'different'),
                             ('estimator_settings', {'heap_start': 'other'})]:
            with self.subTest(field=field):
                after = estimate()
                after['cost_estimation']['success']['context'][field] = value
                self.write(self.head, after)
                report, count = costs.render_comparison(self.base, self.head)
                self.assertEqual(count, 0)
                self.assertIn('Unmatched', report)

    def test_duplicates_are_errors(self):
        self.write(self.base, estimate())
        self.write(self.base, estimate(), 'duplicate.json')
        with self.assertRaisesRegex(ValueError, 'Ambiguous duplicate'):
            costs.load_estimates(self.base)

    def test_matched_set_aggregation_and_missing_components(self):
        for root in [self.base, self.head]:
            self.write(root, estimate('a', {'opcode': 0}, 100))
            self.write(root, estimate('b', {'opcode': 20}, 200))
        self.write(self.base, estimate('unmatched', {'opcode': 999}, 999))
        self.write(self.head, estimate('a', {'opcode': 0, 'new': 10}, None))
        report, count = costs.render_comparison(self.base, self.head)
        self.assertEqual(count, 2)
        self.assertIn('| **Total cost** | 20 | 30 | +50.00% |', report)
        self.assertIn('| new | N/A | N/A | N/A |', report)
        self.assertIn('Maximum available heap: 200 → 200 bytes', report)
        self.assertIn('Heap coverage: 2/2 base, 1/2 candidate', report)
        self.assertEqual(costs.change(0, 10), 'N/A')
        self.assertEqual(costs.change(None, 10), 'N/A')

    def test_crashes_mismatches_and_missing_heap(self):
        for root in [self.base, self.head]:
            self.write(root, estimate())
            mismatch = estimate('mismatch')
            mismatch['cost_estimation']['success']['output_matched'] = False
            self.write(root, mismatch)
            self.write(root, {'name': 'crash', 'cost_estimation': {'crashed': {'reason': 'boom'}}})
        report, count = costs.render_comparison(self.base, self.head)
        self.assertEqual(count, 1)
        self.assertIn('Excluded results: 4', report)
        self.assertIn('Maximum available heap: N/A → N/A', report)
        self.assertIn('boom', report)

    def test_empty_component_map_keeps_components_unavailable(self):
        self.write(self.base, estimate(cost={}))
        self.write(self.head, estimate(cost={'opcode': 10}))
        report, count = costs.render_comparison(self.base, self.head)
        self.assertEqual(count, 1)
        self.assertIn('| opcode | N/A | 10 | N/A |', report)
        self.assertIn('| **Total cost** | 0 | 10 | N/A |', report)

    def test_backends_keep_separate_totals(self):
        for root in [self.base, self.head]:
            self.write(root, estimate())
            zisk = estimate('zisk-case', {'main': 90}, 0)
            zisk['cost_estimation']['success']['context'].update(zkvm='zisk', sdk_version='v1.1.0-alpha')
            self.write(root, zisk)
        report, count = costs.render_comparison(self.base, self.head)
        self.assertEqual(count, 2)
        self.assertEqual(report.count('**Total cost**'), 2)
        self.assertIn('| **Total cost** | 90 | 90 |', report)
        self.assertIn('| **Total cost** | 120 | 120 |', report)

    def test_invalid_metrics_are_identified(self):
        data = estimate()
        data['cost_estimation']['success']['cost']['opcode'] = -1
        self.write(self.base, data)
        with self.assertRaisesRegex(ValueError, 'Invalid component costs'):
            costs.load_estimates(self.base)

    def test_timing_uses_fractional_seconds_and_ignores_cost_only(self):
        self.write(self.base, timing(estimate(), 1, 500_000_000))
        self.write(self.head, timing(estimate(), 0, 750_000_000))
        self.write(self.base, estimate('cost-only'))
        report, count = times.render_comparison(self.base, self.head)
        self.assertEqual(count, 1)
        self.assertIn('1.500000 | 0.750000 | 2.000x', report)
        data = timing(estimate(), 0)
        self.write(self.head, data)
        report, _ = times.render_comparison(self.base, self.head)
        self.assertIn('total speedup: N/A', report)


if __name__ == '__main__':
    unittest.main()
