"""Generate group-level summary DOT graph (4 nodes)."""

from .extract import GROUP_COLORS, Edge, WorkspaceGraph


def _aggregate_edges(
    edges: list[Edge], crate_groups: dict[str, str]
) -> dict[tuple[str, str], tuple[int, int]]:
    """Count (total, dev_only) edges between groups."""
    counts: dict[tuple[str, str], tuple[int, int]] = {}
    for e in edges:
        src_group = crate_groups.get(e.source, "unknown")
        tgt_group = crate_groups.get(e.target, "unknown")
        key = (src_group, tgt_group)
        total, dev_only = counts.get(key, (0, 0))
        counts[key] = (total + 1, dev_only + (1 if e.is_dev_only else 0))
    return counts


def _penwidth(count: int, max_count: int) -> float:
    """Scale edge width between 1.0 and 6.0 based on count."""
    if max_count <= 0:
        return 1.0
    return 1.0 + 5.0 * (count / max_count)


def generate(graph: WorkspaceGraph, edges: list[Edge]) -> str:
    lines = [
        "digraph groups {",
        "  rankdir=LR;",
        "  dpi=150;",
        "  pad=0.5;",
        "  nodesep=1.0;",
        "  ranksep=1.5;",
        "  splines=true;",
        '  node [shape=box, style="filled,rounded", fontname="Helvetica", fontsize=16,',
        "        margin=\"0.3,0.2\", penwidth=1.5];",
        '  edge [fontname="Helvetica", fontsize=11];',
        "",
    ]

    counts = graph.group_crate_counts
    for group in sorted(counts):
        color = GROUP_COLORS.get(group, "#ffffff")
        label = f"{group}\\n({counts[group]} crates)"
        node_id = group.replace("-", "_")
        lines.append(f'  {node_id} [label="{label}", fillcolor="{color}"];')

    lines.append("")

    agg = _aggregate_edges(edges, graph.crate_groups)
    # Find max count for penwidth scaling
    max_count = max((total for _, (total, _) in agg.items()), default=1)

    for (src, tgt), (total, dev_only) in sorted(agg.items()):
        src_id = src.replace("-", "_")
        tgt_id = tgt.replace("-", "_")
        build_count = total - dev_only
        if build_count > 0 and dev_only > 0:
            label = f"{build_count} + {dev_only} dev"
        elif dev_only > 0:
            label = f"{dev_only} dev"
        else:
            label = str(build_count)

        pw = _penwidth(total, max_count)
        attrs = [f'label="{label}"', f"penwidth={pw:.1f}"]
        if build_count == 0:
            attrs.append("style=dashed")
            attrs.append('color="#999999"')
        else:
            attrs.append('color="#555555"')
        lines.append(f"  {src_id} -> {tgt_id} [{', '.join(attrs)}];")

    lines.append("}")
    return "\n".join(lines) + "\n"
