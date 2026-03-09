#!/usr/bin/env python3

import json
import subprocess
import sys
from pathlib import Path


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

    protected_dirs = {"main", "runtime", "library", "harness"}
    crate_ids = set()
    contrib_ids = set()
    for pkg_id, manifest in manifest_by_id.items():
        try:
            rel = manifest.relative_to(repo_root)
        except ValueError:
            continue
        if rel.parts and rel.parts[0] in protected_dirs:
            crate_ids.add(pkg_id)
        elif rel.parts and rel.parts[0] == "contrib":
            contrib_ids.add(pkg_id)

    violations = []
    for pkg_id in sorted(crate_ids, key=lambda item: str(manifest_by_id[item])):
        node = nodes_by_id.get(pkg_id)
        if node is None:
            continue

        deps = []
        for dep in node.get("deps", []):
            dep_pkg_id = dep.get("pkg")
            if dep_pkg_id not in contrib_ids:
                continue

            kinds = sorted(
                {(dep_kind.get("kind") or "normal") for dep_kind in dep.get("dep_kinds", [])}
                or {"normal"}
            )
            deps.append(
                {
                    "dep_name": dep["name"],
                    "dep_pkg": name_by_id[dep_pkg_id],
                    "dep_manifest": manifest_by_id[dep_pkg_id],
                    "kinds": kinds,
                }
            )

        if deps:
            violations.append(
                {
                    "pkg": name_by_id[pkg_id],
                    "manifest": manifest_by_id[pkg_id],
                    "deps": sorted(deps, key=lambda item: (str(item["dep_manifest"]), item["dep_name"])),
                }
            )

    if not violations:
        dirs = ", ".join(sorted(protected_dirs))
        print(f"OK: no {dirs} package directly depends on a contrib/* package")
        return 0

    dirs = ", ".join(sorted(protected_dirs))
    print(f"ERROR: found {dirs} packages with direct deps on contrib/* packages", file=sys.stderr)
    for violation in violations:
        rel_manifest = violation["manifest"].relative_to(repo_root)
        print(f"{violation['pkg']}\t{rel_manifest}", file=sys.stderr)
        for dep in violation["deps"]:
            rel_dep_manifest = dep["dep_manifest"].relative_to(repo_root)
            dep_kinds = ",".join(dep["kinds"])
            print(
                f"  -> {dep['dep_name']}\t{dep['dep_pkg']}\t{rel_dep_manifest}\t{dep_kinds}",
                file=sys.stderr,
            )
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
