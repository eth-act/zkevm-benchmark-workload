#!/usr/bin/env python3
"""
Analyze Zisk profiling outputs and generate an aggregate summary report.

Parses .prof files from ziskemu and produces a Markdown report with:
- MARK_ID custom scope statistics (primary focus)
- Cost distribution breakdown (MAIN, OPCODES, PRECOMPILES, MEMORY)
- Top opcodes by cost

Usage:
    python3 scripts/analyze_zisk_profiles.py zisk-profiles/reth/
    python3 scripts/analyze_zisk_profiles.py zisk-profiles/reth/ --output report.md
    python3 scripts/analyze_zisk_profiles.py zisk-profiles/reth/ --verbose

Options:
    --output, -o FILE   Write report to FILE instead of stdout
    --verbose, -v       Show parsing progress (file count, each file being parsed,
                        success count).
"""

import argparse
import re
import statistics
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Optional


@dataclass
class ProfileData:
    """Data extracted from a single .prof file."""

    filename: str
    block_id: str
    total_cost: int = 0
    cost_distribution: Dict[str, int] = field(default_factory=dict)
    opcodes: Dict[str, Dict[str, int]] = field(default_factory=dict)
    mark_ids: Dict[str, Dict[str, int]] = field(default_factory=dict)


def parse_number(s: str) -> int:
    """Parse a number string, removing commas."""
    return int(s.replace(",", ""))


def format_number(n: float) -> str:
    """Format a number in human-readable form (e.g., 5.7B, 959M, 12.8K)."""
    abs_n = abs(n)
    if abs_n >= 1_000_000_000:
        return f"{n / 1_000_000_000:.1f}B"
    elif abs_n >= 1_000_000:
        return f"{n / 1_000_000:.1f}M"
    elif abs_n >= 1_000:
        return f"{n / 1_000:.1f}K"
    else:
        return f"{n:.0f}"


def format_percent(p: float) -> str:
    """Format a percentage value."""
    return f"{p:.1f}%"


def extract_block_id(filename: str) -> str:
    """Extract block ID from filename like 'zisk_profile_rpc_block_23326233.prof'."""
    match = re.search(r"block_(\d+)", filename)
    return match.group(1) if match else filename


def parse_profile(filepath: Path, verbose: bool = False) -> Optional[ProfileData]:
    """Parse a single .prof file and extract relevant data."""
    try:
        content = filepath.read_text()
    except Exception as e:
        if verbose:
            print(f"Warning: Could not read {filepath}: {e}", file=sys.stderr)
        return None

    profile = ProfileData(
        filename=filepath.name, block_id=extract_block_id(filepath.name)
    )

    lines = content.split("\n")
    i = 0

    while i < len(lines):
        line = lines[i].strip()

        # Parse COST DISTRIBUTION section
        if line.startswith("COST DISTRIBUTION"):
            i += 2  # Skip header line
            while i < len(lines):
                line = lines[i].strip()
                if not line or line.startswith("TOTAL") or line.startswith("FROPS"):
                    break
                # Match lines like: BASE                         293,601,280   1.10%
                match = re.match(r"^(\w+)\s+([\d,]+)\s+[\d.]+%$", line)
                if match:
                    category = match.group(1)
                    cost = parse_number(match.group(2))
                    profile.cost_distribution[category] = cost
                i += 1

            # Get total cost from TOTAL line
            while i < len(lines):
                line = lines[i].strip()
                if line.startswith("TOTAL"):
                    match = re.match(r"^TOTAL\s+([\d,]+)", line)
                    if match:
                        profile.total_cost = parse_number(match.group(1))
                    break
                if line.startswith("FROPS"):
                    break
                i += 1
            continue

        # Parse COST BY OPCODE section
        if line.startswith("COST BY OPCODE"):
            i += 2  # Skip header line
            while i < len(lines):
                line = lines[i].strip()
                if not line or line.startswith("FROPS"):
                    break
                # Match lines like: OP keccak                         44,347   5,681,205,476  21.20% #1
                match = re.match(
                    r"^OP\s+(\w+)\s+([\d,]+)\s+([\d,]+)\s+[\d.]+%", line
                )
                if match:
                    opcode = match.group(1)
                    count = parse_number(match.group(2))
                    cost = parse_number(match.group(3))
                    profile.opcodes[opcode] = {"count": count, "cost": cost}
                i += 1
            continue

        # Parse MARK_ID section
        if line.startswith("MARK_ID"):
            i += 2  # Skip header line
            while i < len(lines):
                line = lines[i].strip()
                if not line or re.match(r"^[a-f0-9]{8}$", line):
                    break
                # Match MARK_ID lines
                # RECOVER_BLOCK                     0          1       2,376,088   1.06%     384,171,246   1.43%     161,573,984      38,085,509     150,738,364      33,773,389
                parts = line.split()
                if len(parts) >= 11 and not parts[0].startswith("-"):
                    try:
                        name = parts[0]
                        # parts[1] = INDEX, parts[2] = COUNT, parts[3] = STEPS, parts[4] = STEPS%
                        # parts[5] = TOTAL COST, parts[6] = %, parts[7] = MAIN COST, parts[8] = OPCODE COST
                        # parts[9] = PRECOMPILE COST, parts[10] = MEMORY COST
                        total_cost = parse_number(parts[5])
                        main_cost = parse_number(parts[7])
                        opcode_cost = parse_number(parts[8])
                        precompile_cost = parse_number(parts[9])
                        memory_cost = parse_number(parts[10])

                        profile.mark_ids[name] = {
                            "total_cost": total_cost,
                            "main_cost": main_cost,
                            "opcode_cost": opcode_cost,
                            "precompile_cost": precompile_cost,
                            "memory_cost": memory_cost,
                        }
                    except (IndexError, ValueError):
                        pass
                i += 1
            continue

        i += 1

    return profile


def compute_statistics(values: List[float]) -> Dict[str, float]:
    """Compute statistics for a list of values."""
    if not values:
        return {
            "count": 0,
            "sum": 0,
            "mean": 0,
            "median": 0,
            "min": 0,
            "max": 0,
            "std_dev": 0,
        }

    result = {
        "count": len(values),
        "sum": sum(values),
        "mean": statistics.mean(values),
        "median": statistics.median(values),
        "min": min(values),
        "max": max(values),
        "std_dev": statistics.stdev(values) if len(values) > 1 else 0,
    }
    return result


def aggregate_profiles(profiles: List[ProfileData]) -> Dict:
    """Aggregate data across all profiles."""
    result = {
        "count": len(profiles),
        "block_ids": [p.block_id for p in profiles],
        "total_costs": [p.total_cost for p in profiles],
        "cost_distribution": {},
        "opcodes": {},
        "mark_ids": {},
    }

    # Aggregate cost distribution
    all_categories = set()
    for p in profiles:
        all_categories.update(p.cost_distribution.keys())

    for category in all_categories:
        values = [p.cost_distribution.get(category, 0) for p in profiles]
        percentages = [
            (p.cost_distribution.get(category, 0) / p.total_cost * 100)
            if p.total_cost > 0
            else 0
            for p in profiles
        ]
        stats = compute_statistics(values)
        stats["avg_pct"] = statistics.mean(percentages) if percentages else 0
        result["cost_distribution"][category] = stats

    # Aggregate opcodes
    all_opcodes = set()
    for p in profiles:
        all_opcodes.update(p.opcodes.keys())

    for opcode in all_opcodes:
        costs = [p.opcodes.get(opcode, {}).get("cost", 0) for p in profiles]
        counts = [p.opcodes.get(opcode, {}).get("count", 0) for p in profiles]
        percentages = [
            (p.opcodes.get(opcode, {}).get("cost", 0) / p.total_cost * 100)
            if p.total_cost > 0
            else 0
            for p in profiles
        ]
        stats = compute_statistics(costs)
        stats["avg_pct"] = statistics.mean(percentages) if percentages else 0
        stats["avg_count"] = statistics.mean(counts) if counts else 0
        result["opcodes"][opcode] = stats

    # Aggregate MARK_IDs
    all_mark_ids = set()
    for p in profiles:
        all_mark_ids.update(p.mark_ids.keys())

    for mark_id in all_mark_ids:
        total_costs = [
            p.mark_ids.get(mark_id, {}).get("total_cost", 0) for p in profiles
        ]
        main_costs = [p.mark_ids.get(mark_id, {}).get("main_cost", 0) for p in profiles]
        opcode_costs = [
            p.mark_ids.get(mark_id, {}).get("opcode_cost", 0) for p in profiles
        ]
        precompile_costs = [
            p.mark_ids.get(mark_id, {}).get("precompile_cost", 0) for p in profiles
        ]
        memory_costs = [
            p.mark_ids.get(mark_id, {}).get("memory_cost", 0) for p in profiles
        ]
        percentages = [
            (p.mark_ids.get(mark_id, {}).get("total_cost", 0) / p.total_cost * 100)
            if p.total_cost > 0
            else 0
            for p in profiles
        ]

        result["mark_ids"][mark_id] = {
            "total_cost": compute_statistics(total_costs),
            "main_cost": compute_statistics(main_costs),
            "opcode_cost": compute_statistics(opcode_costs),
            "precompile_cost": compute_statistics(precompile_costs),
            "memory_cost": compute_statistics(memory_costs),
            "avg_pct": statistics.mean(percentages) if percentages else 0,
        }

    return result


def generate_markdown_report(aggregated: Dict, directory: str) -> str:
    """Generate a Markdown report from aggregated data."""
    lines = []

    # Header
    lines.append("# Zisk Profile Summary\n")
    lines.append(f"- **Profiles analyzed:** {aggregated['count']}")
    lines.append(f"- **Directory:** `{directory}`")

    block_ids = sorted(aggregated["block_ids"], key=lambda x: int(x) if x.isdigit() else 0)
    if block_ids:
        lines.append(f"- **Block range:** {block_ids[0]} - {block_ids[-1]}")

    total_cost_stats = compute_statistics(aggregated["total_costs"])
    lines.append(f"- **Total cost (sum):** {format_number(total_cost_stats['sum'])}")
    lines.append(f"- **Avg cost per profile:** {format_number(total_cost_stats['mean'])}")
    lines.append("")

    # MARK_ID Summary (primary focus)
    lines.append("## Custom Scopes (MARK_ID)\n")
    lines.append(
        "| Scope | Avg Cost | Avg % | Median | Min | Max | Std Dev |"
    )
    lines.append("|-------|----------|-------|--------|-----|-----|---------|")

    # Sort by average cost descending
    sorted_mark_ids = sorted(
        aggregated["mark_ids"].items(),
        key=lambda x: x[1]["total_cost"]["mean"],
        reverse=True,
    )

    for name, data in sorted_mark_ids:
        stats = data["total_cost"]
        lines.append(
            f"| {name} | {format_number(stats['mean'])} | {format_percent(data['avg_pct'])} | "
            f"{format_number(stats['median'])} | {format_number(stats['min'])} | "
            f"{format_number(stats['max'])} | {format_number(stats['std_dev'])} |"
        )

    lines.append("")

    # MARK_ID Cost Breakdown
    lines.append("### Cost Breakdown by Scope\n")
    lines.append("| Scope | Main | Opcodes | Precompiles | Memory |")
    lines.append("|-------|------|---------|-------------|--------|")

    for name, data in sorted_mark_ids:
        main_pct = (
            data["main_cost"]["mean"] / data["total_cost"]["mean"] * 100
            if data["total_cost"]["mean"] > 0
            else 0
        )
        opcode_pct = (
            data["opcode_cost"]["mean"] / data["total_cost"]["mean"] * 100
            if data["total_cost"]["mean"] > 0
            else 0
        )
        precompile_pct = (
            data["precompile_cost"]["mean"] / data["total_cost"]["mean"] * 100
            if data["total_cost"]["mean"] > 0
            else 0
        )
        memory_pct = (
            data["memory_cost"]["mean"] / data["total_cost"]["mean"] * 100
            if data["total_cost"]["mean"] > 0
            else 0
        )
        lines.append(
            f"| {name} | {format_percent(main_pct)} | {format_percent(opcode_pct)} | "
            f"{format_percent(precompile_pct)} | {format_percent(memory_pct)} |"
        )

    lines.append("")

    # Cost Distribution Summary
    lines.append("## Cost Distribution\n")
    lines.append("| Category | Avg Cost | Avg % | Median | Min | Max |")
    lines.append("|----------|----------|-------|--------|-----|-----|")

    # Sort by average cost descending
    sorted_categories = sorted(
        aggregated["cost_distribution"].items(),
        key=lambda x: x[1]["mean"],
        reverse=True,
    )

    for category, stats in sorted_categories:
        lines.append(
            f"| {category} | {format_number(stats['mean'])} | {format_percent(stats['avg_pct'])} | "
            f"{format_number(stats['median'])} | {format_number(stats['min'])} | "
            f"{format_number(stats['max'])} |"
        )

    lines.append("")

    # Top 10 Opcodes by Cost
    lines.append("## Top 10 Opcodes by Cost\n")
    lines.append("| Opcode | Avg Cost | Avg % | Avg Count | Median Cost |")
    lines.append("|--------|----------|-------|-----------|-------------|")

    sorted_opcodes = sorted(
        aggregated["opcodes"].items(), key=lambda x: x[1]["mean"], reverse=True
    )[:10]

    for opcode, stats in sorted_opcodes:
        lines.append(
            f"| {opcode} | {format_number(stats['mean'])} | {format_percent(stats['avg_pct'])} | "
            f"{format_number(stats['avg_count'])} | {format_number(stats['median'])} |"
        )

    lines.append("")

    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(
        description="Analyze Zisk profiling outputs and generate an aggregate summary.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__,
    )
    parser.add_argument(
        "directory",
        type=str,
        help="Directory containing .prof files to analyze",
    )
    parser.add_argument(
        "--output",
        "-o",
        type=str,
        help="Output file path (default: stdout)",
    )
    parser.add_argument(
        "--verbose",
        "-v",
        action="store_true",
        help="Show verbose output during parsing",
    )

    args = parser.parse_args()

    directory = Path(args.directory)
    if not directory.exists():
        print(f"Error: Directory does not exist: {directory}", file=sys.stderr)
        sys.exit(1)

    if not directory.is_dir():
        print(f"Error: Not a directory: {directory}", file=sys.stderr)
        sys.exit(1)

    prof_files = list(directory.glob("*.prof"))
    if not prof_files:
        print(f"Error: No .prof files found in {directory}", file=sys.stderr)
        sys.exit(1)

    if args.verbose:
        print(f"Found {len(prof_files)} profile files", file=sys.stderr)

    # Parse all profiles
    profiles = []
    for filepath in sorted(prof_files):
        if args.verbose:
            print(f"Parsing {filepath.name}...", file=sys.stderr)
        profile = parse_profile(filepath, args.verbose)
        if profile:
            profiles.append(profile)

    if not profiles:
        print("Error: All profiles failed to parse", file=sys.stderr)
        sys.exit(2)

    if args.verbose:
        print(f"Successfully parsed {len(profiles)} profiles", file=sys.stderr)

    # Aggregate and generate report
    aggregated = aggregate_profiles(profiles)
    report = generate_markdown_report(aggregated, str(directory))

    # Output
    if args.output:
        output_path = Path(args.output)
        output_path.write_text(report)
        print(f"Report written to {output_path}", file=sys.stderr)
    else:
        print(report)


if __name__ == "__main__":
    main()
