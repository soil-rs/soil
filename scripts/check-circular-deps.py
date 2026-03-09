#!/usr/bin/env python3
"""Check for circular dependencies among main/ and runtime/ crates.

Checks normal, build, and dev dependencies separately and reports any
cycles found. Exit code 0 means no cycles; 1 means cycles were found.
"""

import json
import subprocess
import sys
from collections import defaultdict
from pathlib import Path


DEP_DIRS = {"main", "runtime"}


def main() -> int:
    repo_root = Path(__file__).resolve().parent.parent
    metadata = subprocess.run(
        ["cargo", "metadata", "--format-version", "1"],
        cwd=repo_root,
        check=True,
        capture_output=True,
        text=True,
    )
    data = json.loads(metadata.stdout)

    manifest_by_id = {
        pkg["id"]: Path(pkg["manifest_path"]).resolve()
        for pkg in data["packages"]
    }
    name_by_id = {pkg["id"]: pkg["name"] for pkg in data["packages"]}
    nodes_by_id = {node["id"]: node for node in data["resolve"]["nodes"]}

    # Identify packages that live under main/ or runtime/.
    target_ids = set()
    for pkg_id, manifest in manifest_by_id.items():
        try:
            rel = manifest.relative_to(repo_root)
        except ValueError:
            continue
        if rel.parts and rel.parts[0] in DEP_DIRS:
            target_ids.add(pkg_id)

    # Build adjacency lists per dependency kind.
    # "normal" includes build deps (both block compilation), "dev" is separate.
    graphs: dict[str, dict[str, set[str]]] = {
        "normal": defaultdict(set),
        "dev": defaultdict(set),
    }

    for pkg_id in target_ids:
        node = nodes_by_id.get(pkg_id)
        if node is None:
            continue
        for dep in node.get("deps", []):
            dep_id = dep.get("pkg")
            if dep_id not in target_ids:
                continue
            kinds = {
                (dk.get("kind") or "normal") for dk in dep.get("dep_kinds", [])
            }
            for kind in kinds:
                bucket = "dev" if kind == "dev" else "normal"
                graphs[bucket][pkg_id].add(dep_id)

    # Find cycles using DFS with coloring (white/gray/black).
    found_cycles: list[tuple[str, list[str]]] = []

    def find_cycles(graph: dict[str, set[str]]) -> list[list[str]]:
        WHITE, GRAY, BLACK = 0, 1, 2
        color: dict[str, int] = defaultdict(int)
        path: list[str] = []
        cycles: list[list[str]] = []

        def dfs(u: str) -> None:
            color[u] = GRAY
            path.append(u)
            for v in graph.get(u, ()):
                if color[v] == GRAY:
                    # Found a cycle: extract the cycle from path.
                    idx = path.index(v)
                    cycle = path[idx:] + [v]
                    cycles.append(cycle)
                elif color[v] == WHITE:
                    dfs(v)
            path.pop()
            color[u] = BLACK

        for node_id in sorted(target_ids):
            if color[node_id] == WHITE:
                dfs(node_id)

        return cycles

    has_error = False
    for label, graph in graphs.items():
        cycles = find_cycles(graph)
        if not cycles:
            continue
        has_error = True

        # Deduplicate cycles (same set of nodes, different starting points).
        seen: set[frozenset[str]] = set()
        for cycle in cycles:
            key = frozenset(cycle)
            if key in seen:
                continue
            seen.add(key)
            names = [name_by_id[pid] for pid in cycle]
            arrow = " -> ".join(names)
            print(f"ERROR: circular {label} dependency: {arrow}", file=sys.stderr)

    if not has_error:
        print("OK: no circular dependencies found among main/ and runtime/ crates")
        return 0

    return 1


if __name__ == "__main__":
    raise SystemExit(main())
