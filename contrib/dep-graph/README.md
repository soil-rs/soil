# Dependency Graph

Generates dependency graph visualizations for all crates in the Soil workspace.

## Output

Two levels of abstraction, each with two variants:

| File | Description |
|------|-------------|
| `group-build` | Group-level summary (4 nodes) — build deps only |
| `group-all` | Group-level summary — build + dev deps |
| `crates-build` | Full crate graph (~286 nodes, clustered) — build deps only |
| `crates-all` | Full crate graph — build + dev deps |

With `--render`, crate graphs are rendered twice:
- **`dot` layout** (hierarchical) — large, detailed, best for zooming/scrolling in an SVG viewer
- **`sfdp` layout** (force-directed) — compact, fits on one screen, good for the big picture

Dev-only edges are rendered with dashed lines. Crates are clustered and
color-coded by group (`soil`, `topsoil`, `substrate`, `substrate-client`).
Edge thickness in the group graph is proportional to the number of dependencies.

## Usage

```sh
# From the workspace root (soil/):
python3 contrib/dep-graph/generate.py

# Also render SVGs (requires graphviz):
python3 contrib/dep-graph/generate.py --render

# Only one variant:
python3 contrib/dep-graph/generate.py --variant build

# Use a cached cargo metadata JSON:
cargo metadata --format-version 1 > /tmp/meta.json
python3 contrib/dep-graph/generate.py --json /tmp/meta.json

# Try a different layout engine:
python3 contrib/dep-graph/generate.py --render --layout sfdp
```

Output is written to `contrib/dep-graph/output/`.

## Requirements

- Python 3.12+
- `cargo` (for `cargo metadata`)
- `graphviz` (optional, for SVG rendering): `sudo apt-get install graphviz`

No external Python dependencies are required.
