"""Generate full crate-level DOT graph with clustering."""

from .extract import GROUP_COLORS, Edge, WorkspaceGraph

# Prefixes to strip within each group for readability.
GROUP_PREFIXES = {
    "soil": "soil-",
    "topsoil": "topsoil-",
    "substrate": "",
    "substrate-client": "sc-",
}

# Darker shades for node fill within clusters.
GROUP_NODE_COLORS = {
    "soil": "#80bfff",
    "topsoil": "#80ff80",
    "substrate": "#ff8080",
    "substrate-client": "#ffff80",
}

# Lighter shades for cluster background.
GROUP_CLUSTER_COLORS = {
    "soil": "#e6f2ff",
    "topsoil": "#e6ffe6",
    "substrate": "#ffe6e6",
    "substrate-client": "#fffff0",
}


def _short_label(name: str, group: str) -> str:
    prefix = GROUP_PREFIXES.get(group, "")
    if prefix and name.startswith(prefix):
        short = name[len(prefix):]
        if short:
            return short
    return name


def _node_id(name: str) -> str:
    return name.replace("-", "_")


def generate(graph: WorkspaceGraph, edges: list[Edge]) -> str:
    lines = [
        "digraph crates {",
        "  rankdir=LR;",
        "  newrank=true;",
        "  compound=true;",
        "  concentrate=true;",
        "  ranksep=0.6;",
        "  nodesep=0.2;",
        "  splines=true;",
        '  node [shape=box, style="filled,rounded", fontname="Helvetica", fontsize=9,',
        '        height=0.3, margin="0.08,0.04"];',
        '  edge [arrowsize=0.4];',
        "",
    ]

    # Group crates by their group
    groups: dict[str, list[str]] = {}
    for crate_name, group in sorted(graph.crate_groups.items()):
        groups.setdefault(group, []).append(crate_name)

    # Emit clustered subgraphs
    for group in sorted(groups):
        crates = sorted(groups[group])
        cluster_color = GROUP_CLUSTER_COLORS.get(group, "#f0f0f0")
        node_color = GROUP_NODE_COLORS.get(group, "#cccccc")
        cluster_id = group.replace("-", "_")

        lines.append(f"  subgraph cluster_{cluster_id} {{")
        lines.append(f'    label="{group}";')
        lines.append(f'    style="filled,rounded";')
        lines.append(f'    color="#aaaaaa";')
        lines.append(f'    fillcolor="{cluster_color}";')
        lines.append(f'    fontname="Helvetica Bold";')
        lines.append(f'    fontsize=16;')
        lines.append(f'    margin=12;')
        lines.append("")
        for crate_name in crates:
            nid = _node_id(crate_name)
            label = _short_label(crate_name, group)
            lines.append(
                f'    {nid} [label="{label}", fillcolor="{node_color}", '
                f'tooltip="{crate_name}"];'
            )
        lines.append("  }")
        lines.append("")

    # Emit edges
    for e in edges:
        src_id = _node_id(e.source)
        tgt_id = _node_id(e.target)
        src_group = graph.crate_groups.get(e.source, "")
        tgt_group = graph.crate_groups.get(e.target, "")
        cross_group = src_group != tgt_group

        attrs: list[str] = []
        if e.is_dev_only:
            attrs.append('style=dashed')
            attrs.append('color="#cccccc44"')
            attrs.append('penwidth=0.3')
        elif cross_group:
            attrs.append('color="#33333366"')
            attrs.append('penwidth=0.8')
        else:
            attrs.append('color="#99999944"')
            attrs.append('penwidth=0.3')

        attr_str = f" [{', '.join(attrs)}]" if attrs else ""
        lines.append(f"  {src_id} -> {tgt_id}{attr_str};")

    lines.append("}")
    return "\n".join(lines) + "\n"
