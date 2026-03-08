#!/usr/bin/env python3

import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent
GPL = "GPL-3.0-or-later WITH Classpath-exception-2.0"

OLD_BODY_MARKERS = (
    "This program is free software: you can redistribute it and/or modify",
    "Licensed under the Apache License, Version 2.0",
    "Permission is hereby granted, free of charge, to any person obtaining a copy of",
)


def is_rust_file(path: Path) -> bool:
    return path.suffix == ".rs" and not any(part in {".git", "target", "node_modules"} for part in path.parts)


def mapped_spdx(previous: str | None) -> str:
    if not previous:
        return f"Apache-2.0 OR {GPL}"
    if previous == GPL or f" OR {GPL}" in previous:
        return previous
    return f"{previous} OR {GPL}"


def parse_header_block(lines: list[str]) -> tuple[list[str], int]:
    start = 0
    while start < len(lines) and lines[start] == "":
        start += 1

    if start >= len(lines):
        return [], 0

    first = lines[start]
    if not (
        first.startswith("// This file is part of ")
        or first.startswith("// Copyright (C) ")
        or first.startswith("// SPDX-License-Identifier: ")
        or any(marker in first for marker in OLD_BODY_MARKERS)
    ):
        return [], 0

    idx = 0
    while idx < len(lines):
        line = lines[idx]
        if line == "":
            idx += 1
            continue
        stripped = line.strip()
        if stripped == "//":
            idx += 1
            continue
        if stripped.startswith("//!") or stripped.startswith("///"):
            break
        if line.startswith("//") and not stripped.startswith("//!") and not stripped.startswith("///"):
            idx += 1
            continue
        break

    return lines[:idx], idx


def parse_existing_metadata(header_lines: list[str]) -> tuple[list[str], str | None]:
    owners: list[str] = []
    seen = set()
    spdx = None

    for line in header_lines:
        if "SPDX-License-Identifier:" in line:
            spdx = line.split("SPDX-License-Identifier:", 1)[1].strip()
        if "// Copyright (C) " in line:
            owner = "// Copyright (C) " + line.split("// Copyright (C) ", 1)[1]
            owner = owner.split("SPDX-License-Identifier:", 1)[0].rstrip()
            if owner != "// Copyright (C) Soil contributors." and owner not in seen:
                owners.append(owner)
                seen.add(owner)

    return owners, spdx


def validate_file(path: Path) -> list[str]:
    text = path.read_text()
    lines = text.splitlines()
    header, idx = parse_header_block(lines)
    owners, prior_spdx = parse_existing_metadata(header)
    errors: list[str] = []

    rel = path.relative_to(REPO_ROOT)

    if text.startswith("// This file is part of Substrate."):
        errors.append(f"{rel}: stale Substrate first line")

    if any(marker in text for marker in OLD_BODY_MARKERS):
        errors.append(f"{rel}: old long license body text remains")

    if sum("This file is part of " in line for line in lines[:40]) > 1:
        errors.append(f"{rel}: duplicate header block remains")

    if idx == 0:
        errors.append(f"{rel}: missing normalized header")
        return errors

    if len(lines) < 5:
        errors.append(f"{rel}: normalized header does not match expected shape")
        return errors

    if lines[0] != "// This file is part of Soil." or lines[1] != "" or lines[2] != "// Copyright (C) Soil contributors.":
        errors.append(f"{rel}: normalized header does not match expected shape")
        return errors

    idx = 3
    while idx < len(lines) and lines[idx].startswith("// Copyright (C) "):
        idx += 1

    if idx >= len(lines) or not lines[idx].startswith("// SPDX-License-Identifier: "):
        errors.append(f"{rel}: normalized header does not match expected shape")
        return errors

    spdx = lines[idx].split("SPDX-License-Identifier:", 1)[1].strip()
    if spdx != mapped_spdx(prior_spdx):
        errors.append(f"{rel}: SPDX line does not match expected mapping")

    if prior_spdx == GPL and "Apache-2.0" in spdx:
        errors.append(f"{rel}: GPL-only file incorrectly gained Apache")

    if idx + 1 >= len(lines) or lines[idx + 1] != "":
        errors.append(f"{rel}: normalized header does not end with a blank separator line")

    return errors


def main() -> int:
    violations: list[str] = []
    for path in sorted(REPO_ROOT.rglob("*.rs")):
        if not is_rust_file(path):
            continue
        violations.extend(validate_file(path))

    if not violations:
        print("OK: Rust license headers are normalized")
        return 0

    print("ERROR: license header violations found", file=sys.stderr)
    for violation in violations:
        print(violation, file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
