#!/usr/bin/env python3
"""Compare execution durations: compare_executions.py BASELINE CANDIDATE."""

import argparse
import json
from pathlib import Path


def load_metrics(root):
    """Read execution times in nanoseconds, keyed by client, zkVM, and fixture."""
    if not root.is_dir():
        raise ValueError(f'Not a directory: {root}')
    records, excluded = {}, []
    for path in sorted(root.rglob('*.json')):
        if path.name == 'hardware.json':
            continue
        try:
            data = json.loads(path.read_text())
            execution = data.get('execution')
            if not execution:
                continue
            success = execution.get('success')
            if not success or success.get('output_matched') is False:
                excluded.append(str(path))
                continue
            duration = success['execution_duration']
            secs, nanos = duration['secs'], duration['nanos']
            if type(secs) is not int or secs < 0 or type(nanos) is not int or not 0 <= nanos < 10**9:
                raise ValueError('Invalid execution duration')
            context = (data.get('cost_estimation') or {}).get('success', {}).get('context', {})
            client = context.get('execution_client', path.parent.parent.name.split('-', 1)[0])
            zkvm = path.parent.name
            metadata = data.get('metadata') or {}
            key = (client, zkvm, metadata.get('original_test_name', data['name']), metadata.get('block_index', 0))
        except (ValueError, KeyError, TypeError, AttributeError) as error:
            raise ValueError(f'Invalid metrics in {path}: {error}') from error
        if key in records:
            raise ValueError(f'Ambiguous duplicate execution: {key}')
        records[key] = secs * 10**9 + nanos
    return records, excluded


def render_comparison(baseline, candidate):
    base, excluded_base = load_metrics(Path(baseline))
    head, excluded_head = load_metrics(Path(candidate))
    common = sorted(base.keys() & head.keys())
    lines = ['Execution duration comparison', '',
             'Client / zkVM / fixture | Base (s) | Candidate (s) | Speedup']
    for key in common:
        ratio = f'{base[key] / head[key]:.3f}x' if head[key] else 'N/A'
        lines.append(f"{' / '.join(map(str, key))} | {base[key] / 1e9:.6f} | {head[key] / 1e9:.6f} | {ratio}")
    total_base, total_head = sum(base[k] for k in common), sum(head[k] for k in common)
    ratio = f'{total_base / total_head:.3f}x' if total_head else 'N/A'
    lines += ['', f'Matched: {len(common)}; total speedup: {ratio}',
              f'Total seconds: {total_base / 1e9:.6f} → {total_head / 1e9:.6f}',
              f'Unmatched: {len(base.keys() - head.keys())} base, {len(head.keys() - base.keys())} candidate',
              f'Excluded crashes or output mismatches: {len(excluded_base) + len(excluded_head)}']
    lines += [f'Excluded: {path}' for path in excluded_base + excluded_head]
    return '\n'.join(lines), len(common)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('baseline', type=Path)
    parser.add_argument('candidate', type=Path)
    args = parser.parse_args()
    try:
        report, count = render_comparison(args.baseline, args.candidate)
    except (ValueError, OSError) as error:
        parser.error(str(error))
    print(report)
    return 0 if count else 1


if __name__ == '__main__':
    raise SystemExit(main())
