"""Render DOT files to SVG using graphviz."""

import shutil
import subprocess
from pathlib import Path


def find_dot() -> str | None:
    return shutil.which("dot")


def render_svg(dot_path: Path, svg_path: Path, layout: str = "dot") -> None:
    subprocess.run(
        [layout, "-Tsvg", "-o", str(svg_path), str(dot_path)],
        check=True,
    )


def render_all(output_dir: Path, layout: str = "dot") -> bool:
    dot_bin = find_dot()
    if dot_bin is None:
        print("graphviz not found. Install with:")
        print("  sudo apt-get install graphviz")
        print("Then re-run with --render")
        return False

    for dot_path in sorted(output_dir.glob("*.dot")):
        svg_path = dot_path.with_suffix(".svg")
        engine = layout
        # For crate-level graphs, also produce an sfdp variant (compact
        # force-directed layout) alongside the primary hierarchical one.
        print(f"  {dot_path.name} -> {svg_path.name} ({engine})")
        render_svg(dot_path, svg_path, layout=engine)

        if dot_path.name.startswith("crates-") and engine == "dot":
            sfdp_path = dot_path.with_stem(dot_path.stem + "-sfdp").with_suffix(".svg")
            print(f"  {dot_path.name} -> {sfdp_path.name} (sfdp)")
            render_svg(dot_path, sfdp_path, layout="sfdp")

    return True
