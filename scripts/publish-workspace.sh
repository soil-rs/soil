#!/usr/bin/env bash
set -euo pipefail

# Wrapper around `cargo publish --workspace` that automatically excludes
# already-published crates. Retries with --exclude until all remaining
# crates are published or a real error occurs.

EXCLUDES=()
MAX_RETRIES=200  # more workspace members than we'll ever have

for ((i = 0; i < MAX_RETRIES; i++)); do
    CMD=(cargo publish --workspace)
    for ex in "${EXCLUDES[@]}"; do
        CMD+=(--exclude "$ex")
    done
    CMD+=("$@")

    echo ">> ${CMD[*]}"
    tmpfile=$(mktemp)
    if "${CMD[@]}" 2>&1 | tee "$tmpfile"; then
        rm -f "$tmpfile"
        echo "Published successfully."
        exit 0
    fi

    output=$(cat "$tmpfile")
    rm -f "$tmpfile"

    # Check if the error is "already exists on crates.io index"
    crate=$(echo "$output" | grep -oP 'error: crate \K[^@]+(?=@\S+ already exists on crates\.io index)' || true)

    if [[ -n "$crate" ]]; then
        echo "Skipping $crate (already published)"
        EXCLUDES+=("$crate")
    else
        # Real error — output already shown by tee
        exit 1
    fi
done

echo "ERROR: exceeded $MAX_RETRIES retries" >&2
exit 1
