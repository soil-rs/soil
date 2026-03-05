#!/usr/bin/env python3
"""Generate dependency graph DOT files for the Soil workspace."""

import argparse
from pathlib import Path

from dep_graph.extract import extract
from dep_graph.dot_group import generate as gen_group
from dep_graph.dot_crates import generate as gen_crates
from dep_graph.render import render_all


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Generate dependency graph visualizations for the Soil workspace."
    )
    parser.add_argument(
        "--workspace-root",
        type=Path,
        default=None,
        help="Path to workspace root (default: auto-detect from script location)",
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=None,
        help="Output directory (default: output/ next to this script)",
    )
    parser.add_argument(
        "--variant",
        choices=["build", "all", "both"],
        default="both",
        help="Which dependency variant to generate (default: both)",
    )
    parser.add_argument(
        "--render",
        action="store_true",
        help="Render DOT files to SVG (requires graphviz)",
    )
    parser.add_argument(
        "--layout",
        choices=["dot", "fdp", "neato", "sfdp"],
        default="dot",
        help="Graphviz layout engine (default: dot)",
    )
    parser.add_argument(
        "--json",
        type=Path,
        default=None,
        dest="json_path",
        help="Use cached cargo metadata JSON instead of running cargo",
    )
    args = parser.parse_args()

    script_dir = Path(__file__).resolve().parent
    workspace_root = args.workspace_root or script_dir.parent.parent
    output_dir = args.output_dir or script_dir / "output"
    output_dir.mkdir(parents=True, exist_ok=True)

    print("Extracting workspace dependency graph...")
    graph = extract(workspace_root, args.json_path)
    print(
        f"  {len(graph.crate_groups)} crates, "
        f"{len(graph.build_edges())} build edges, "
        f"{len(graph.all_edges())} total edges"
    )

    variants: list[str] = []
    if args.variant in ("build", "both"):
        variants.append("build")
    if args.variant in ("all", "both"):
        variants.append("all")

    for variant in variants:
        edges = graph.build_edges() if variant == "build" else graph.all_edges()

        group_dot = gen_group(graph, edges)
        group_path = output_dir / f"group-{variant}.dot"
        group_path.write_text(group_dot)
        print(f"  wrote {group_path}")

        crates_dot = gen_crates(graph, edges)
        crates_path = output_dir / f"crates-{variant}.dot"
        crates_path.write_text(crates_dot)
        print(f"  wrote {crates_path}")

    if args.render:
        print("Rendering SVGs...")
        render_all(output_dir, layout=args.layout)

    print("Done.")


if __name__ == "__main__":
    main()
