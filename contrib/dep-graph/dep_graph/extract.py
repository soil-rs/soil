"""Extract workspace dependency graph from cargo metadata."""

import json
import re
import subprocess
from dataclasses import dataclass, field
from pathlib import Path


# Order matters: longer prefixes first to avoid misclassification.
GROUP_PATTERNS = [
    ("substrate-client", re.compile(r"/crates/substrate-client/")),
    ("substrate", re.compile(r"/crates/substrate/")),
    ("topsoil", re.compile(r"/crates/topsoil(?:-|/)")),
    ("soil", re.compile(r"/crates/soil/")),
]

GROUP_COLORS = {
    "soil": "#b3d9ff",
    "topsoil": "#b3ffb3",
    "substrate": "#ffb3b3",
    "substrate-client": "#ffffb3",
}


@dataclass
class Edge:
    source: str
    target: str
    has_normal: bool = False
    has_build: bool = False
    has_dev: bool = False

    @property
    def is_dev_only(self) -> bool:
        return self.has_dev and not self.has_normal and not self.has_build


@dataclass
class WorkspaceGraph:
    crate_groups: dict[str, str] = field(default_factory=dict)
    edges: list[Edge] = field(default_factory=list)

    @property
    def group_crate_counts(self) -> dict[str, int]:
        counts: dict[str, int] = {}
        for group in self.crate_groups.values():
            counts[group] = counts.get(group, 0) + 1
        return counts

    def build_edges(self) -> list[Edge]:
        return [e for e in self.edges if e.has_normal or e.has_build]

    def all_edges(self) -> list[Edge]:
        return list(self.edges)


def classify_group(manifest_path: str) -> str:
    for group_name, pattern in GROUP_PATTERNS:
        if pattern.search(manifest_path):
            return group_name
    return "unknown"


def load_metadata(workspace_root: Path, json_path: Path | None = None) -> dict:
    if json_path is not None:
        return json.loads(json_path.read_text())
    result = subprocess.run(
        ["cargo", "metadata", "--format-version", "1"],
        cwd=workspace_root,
        capture_output=True,
        text=True,
        check=True,
    )
    return json.loads(result.stdout)


def extract(workspace_root: Path, json_path: Path | None = None) -> WorkspaceGraph:
    meta = load_metadata(workspace_root, json_path)

    ws_member_ids = set(meta["workspace_members"])

    # Build lookup: package_id -> (name, group)
    pkg_info: dict[str, tuple[str, str]] = {}
    for pkg in meta["packages"]:
        if pkg["id"] in ws_member_ids:
            group = classify_group(pkg["manifest_path"])
            pkg_info[pkg["id"]] = (pkg["name"], group)

    crate_groups = {name: group for name, group in pkg_info.values()}

    # Extract edges from resolve section
    edges: list[Edge] = []
    for node in meta["resolve"]["nodes"]:
        if node["id"] not in pkg_info:
            continue
        source_name = pkg_info[node["id"]][0]
        for dep in node.get("deps", []):
            dep_pkg = dep["pkg"]
            if dep_pkg not in pkg_info:
                continue
            target_name = pkg_info[dep_pkg][0]
            edge = Edge(source=source_name, target=target_name)
            for dk in dep.get("dep_kinds", []):
                kind = dk.get("kind")
                if kind is None:
                    edge.has_normal = True
                elif kind == "build":
                    edge.has_build = True
                elif kind == "dev":
                    edge.has_dev = True
            edges.append(edge)

    return WorkspaceGraph(crate_groups=crate_groups, edges=edges)
