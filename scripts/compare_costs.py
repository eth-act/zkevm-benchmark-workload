#!/usr/bin/env python3
"""Compare compatible Ere cost estimates: compare_costs.py BASELINE CANDIDATE."""

import argparse
import json
from dataclasses import dataclass
from pathlib import Path


@dataclass
class Estimate:
    """One successful estimate with enough context for comparison."""
    path: Path
    name: str
    context: dict
    cost: dict
    heap: int | None
    fixture: tuple

    @property
    def group(self):
        return (self.context['execution_client'], self.context['zkvm'],
                self.context['sdk_version'], self.context['ere_revision'],
                json.dumps(self.context['estimator_settings'], sort_keys=True))

    @property
    def key(self):
        return self.group + self.fixture + (self.context['input_sha256'],)


def uint(value):
    return type(value) is int and 0 <= value <= 2**64 - 1


def load_estimates(root):
    """Read cost results recursively; return eligible records and exclusions."""
    root = Path(root)
    if not root.is_dir():
        raise ValueError(f'Not a directory: {root}')
    records, excluded = {}, []
    for path in sorted(root.rglob('*.json')):
        if path.name == 'hardware.json':
            continue
        try:
            data = json.loads(path.read_text())
            if not isinstance(data, dict) or not data.get('cost_estimation'):
                continue
            outcome = data['cost_estimation']
            if 'crashed' in outcome:
                excluded.append(f"{path}: estimator crashed: {outcome['crashed']['reason']}")
                continue
            success = outcome['success']
            if success['output_matched'] is not True:
                excluded.append(f'{path}: public output did not match')
                continue
            context = success['context']
            for field in ('execution_client', 'execution_client_version', 'zkvm', 'sdk_version',
                          'ere_revision', 'elf_sha256', 'input_sha256'):
                if not isinstance(context[field], str) or not context[field]:
                    raise ValueError(f'Missing context field: {field}')
            settings = context['estimator_settings']
            if not isinstance(settings, dict) or not all(
                isinstance(k, str) and isinstance(v, str) for k, v in settings.items()
            ):
                raise ValueError('Invalid estimator_settings')
            cost, heap = success['cost'], success['peak_heap_bytes']
            if not isinstance(cost, dict) or not all(
                isinstance(k, str) and uint(v) for k, v in cost.items()
            ):
                raise ValueError('Invalid component costs')
            if heap is not None and not uint(heap):
                raise ValueError('Invalid peak_heap_bytes')
            metadata, name = data.get('metadata') or {}, data['name']
            fixture = (metadata.get('original_test_name', name), metadata.get('block_index', 0))
            if not isinstance(name, str) or not isinstance(fixture[0], str) or not uint(fixture[1]):
                raise ValueError('Invalid fixture identity')
            record = Estimate(path, name, context, cost, heap, fixture)
            key = record.key
        except (OSError, ValueError, KeyError, TypeError, AttributeError) as error:
            raise ValueError(f'Invalid metrics in {path}: {error}') from error
        if key in records:
            raise ValueError(f'Ambiguous duplicate estimates: {records[key].path} and {path}')
        records[key] = record
    return records, excluded


def number(value):
    return 'N/A' if value is None else f'{value:,}'


def change(base, candidate):
    """Positive percentages mean an increase in cost or heap use."""
    if base in (None, 0) or candidate is None:
        return 'N/A'
    return f'{(candidate - base) / base * 100:+.2f}%'


def cell(value):
    return str(value).replace('|', '\\|').replace('\n', ' ')


def component_total(records, component):
    # An absent component is unavailable, not zero.
    if any(component not in record.cost for record in records):
        return None
    return sum(record.cost[component] for record in records)


def render_comparison(baseline, candidate):
    """Return a Markdown report and the number of compatible fixture pairs."""
    base, base_excluded = load_estimates(baseline)
    head, head_excluded = load_estimates(candidate)
    common = base.keys() & head.keys()
    lines = ['# Estimated proving cost comparison', '',
             'Positive changes mean higher cost or heap use. Cost units differ by zkVM.',
             f'Matched fixtures: {len(common)}. Excluded results: {len(base_excluded) + len(head_excluded)}.', '']
    groups = {}
    for key in sorted(common):
        groups.setdefault(base[key].group, []).append((base[key], head[key]))
    for group, pairs in groups.items():
        client, zkvm, sdk, ere, settings = group
        lines += [f'## {cell(client)} / {cell(zkvm)} {cell(sdk)} / Ere {cell(ere)}', '',
                  f'Estimator settings: `{cell(settings)}`', '',
                  '| Fixture | Base cost | Candidate cost | Change | Base heap (bytes) | Candidate heap (bytes) | Heap change |',
                  '| --- | ---: | ---: | ---: | ---: | ---: | ---: |']
        for before, after in pairs:
            btotal, htotal = sum(before.cost.values()), sum(after.cost.values())
            lines.append(f'| {cell(before.name)} | {number(btotal)} | {number(htotal)} | {change(btotal, htotal)} '
                         f'| {number(before.heap)} | {number(after.heap)} | {change(before.heap, after.heap)} |')
        before, after = [p[0] for p in pairs], [p[1] for p in pairs]
        lines += ['', '| Component | Base | Candidate | Change |', '| --- | ---: | ---: | ---: |']
        components = sorted({c for r in before + after for c in r.cost})
        for component in components:
            bvalue, hvalue = component_total(before, component), component_total(after, component)
            lines.append(f'| {cell(component)} | {number(bvalue)} | {number(hvalue)} | {change(bvalue, hvalue)} |')
        btotal = sum(sum(r.cost.values()) for r in before)
        htotal = sum(sum(r.cost.values()) for r in after)
        lines.append(f'| **Total cost** | {number(btotal)} | {number(htotal)} | {change(btotal, htotal)} |')
        bheap = [r.heap for r in before if r.heap is not None]
        hheap = [r.heap for r in after if r.heap is not None]
        bpeak, hpeak = max(bheap, default=None), max(hheap, default=None)
        lines += ['', f'Maximum available heap: {number(bpeak)} → {number(hpeak)} bytes ({change(bpeak, hpeak)}).',
                  f'Heap coverage: {len(bheap)}/{len(pairs)} base, {len(hheap)}/{len(pairs)} candidate.', '']
        for label, records in [('Base', before), ('Candidate', after)]:
            artifacts = sorted({(r.context['execution_client_version'], r.context['elf_sha256']) for r in records})
            lines += [f'{label} guest artifacts: ' + '; '.join(f'{cell(v)} / `{cell(sha)}`' for v, sha in artifacts), '']
    for label, records, others in [('Base', base, head), ('Candidate', head, base)]:
        unmatched = records.keys() - others.keys()
        if unmatched:
            lines += [f'## Unmatched {label.lower()} results', '',
                      'No counterpart has the same client, fixture, input hash, zkVM, SDK, Ere revision, and estimator settings.', '']
            lines += [f'- {cell(records[key].path)}' for key in sorted(unmatched)] + ['']
    if base_excluded or head_excluded:
        lines += ['## Excluded results', '']
        lines += [f'- {cell(message)}' for message in base_excluded + head_excluded] + ['']
    if not common:
        lines += ['No compatible successful estimates to compare.', '']
    return '\n'.join(lines), len(common)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('baseline', type=Path)
    parser.add_argument('candidate', type=Path)
    args = parser.parse_args()
    try:
        report, count = render_comparison(args.baseline, args.candidate)
    except ValueError as error:
        parser.error(str(error))
    print(report)
    return 0 if count else 1


if __name__ == '__main__':
    raise SystemExit(main())
