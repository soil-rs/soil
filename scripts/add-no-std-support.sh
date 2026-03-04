#!/usr/bin/env bash
# Add no_std support to all migrated standalone sc-* → soil-* crates.
# Under no_std, each crate compiles as an empty library.
# Proc-macro crates and runtime-test are exempt.
set -euo pipefail

cd "$(dirname "$0")/.."

# All migrated crate directories (excluding proc-macros and runtime-test)
CRATES=(
  soil/allocator
  soil/basic-authorship
  soil/chain-spec
  soil/cli
  soil/client-api
  soil/client-db
  soil/consensus-babe-rpc
  soil/consensus-beefy-rpc
  soil/consensus-epochs
  soil/consensus-grandpa-rpc
  soil/consensus-manual-seal
  soil/executor
  soil/executor-common
  soil/executor-polkavm
  soil/executor-wasmtime
  soil/informant
  soil/mmr-gadget
  soil/mmr-rpc
  soil/network
  soil/network-common
  soil/network-gossip
  soil/network-light
  soil/network-statement
  soil/network-sync
  soil/network-test
  soil/network-transactions
  soil/network-types
  soil/proposer-metrics
  soil/rpc-api
  soil/rpc-server
  soil/rpc-spec-v2
  soil/runtime-utilities
  soil/service
  soil/service-test
  soil/state-db
  soil/storage-monitor
  soil/sync-state-rpc
  soil/sysinfo
  soil/telemetry
  soil/transaction-pool-api
  soil/utils
)

for crate_dir in "${CRATES[@]}"; do
  cargo_toml="$crate_dir/Cargo.toml"
  lib_rs="$crate_dir/src/lib.rs"

  echo "Processing $crate_dir..."

  # ── Step 1: Add std feature to Cargo.toml ──
  if grep -q '^\[features\]' "$cargo_toml"; then
    # [features] section exists - add std = [] if not present
    if ! grep -q '^std = ' "$cargo_toml"; then
      sed -i '/^\[features\]/a std = []' "$cargo_toml"
    fi
  else
    # No [features] section - add one before [dev-dependencies] or at end
    if grep -q '^\[dev-dependencies\]' "$cargo_toml"; then
      sed -i '/^\[dev-dependencies\]/i [features]\ndefault = ["std"]\nstd = []\n' "$cargo_toml"
    else
      printf '\n[features]\ndefault = ["std"]\nstd = []\n' >> "$cargo_toml"
    fi
  fi

  # ── Step 2: Add #![cfg_attr(not(feature = "std"), no_std)] to lib.rs ──
  if [ ! -f "$lib_rs" ]; then
    echo "  SKIP: no lib.rs"
    continue
  fi

  if grep -q 'cfg_attr.*no_std' "$lib_rs"; then
    echo "  SKIP: already has no_std"
    continue
  fi

  # Add no_std attribute after the license header comment block
  # Find the line after the last // comment at the top or the first non-comment line
  python3 - "$lib_rs" <<'PYEOF'
import sys, re

filepath = sys.argv[1]
with open(filepath, 'r') as f:
    lines = f.readlines()

# Find insertion point: after license header comments and any existing #! attributes
insert_idx = 0
in_header = True
for i, line in enumerate(lines):
    stripped = line.strip()
    if in_header:
        if stripped.startswith('//') or stripped == '' or stripped.startswith('#!['):
            insert_idx = i + 1
            if stripped.startswith('#!['):
                # After the last #! attribute, keep going
                pass
        else:
            in_header = False
            break

# Insert the no_std cfg_attr
no_std_line = '#![cfg_attr(not(feature = "std"), no_std)]\n'

# Check if there's already a blank line before insertion point
if insert_idx > 0 and lines[insert_idx - 1].strip() == '':
    lines.insert(insert_idx, no_std_line + '\n')
else:
    lines.insert(insert_idx, '\n' + no_std_line + '\n')

with open(filepath, 'w') as f:
    f.writelines(lines)
PYEOF

  # ── Step 3: Add #[cfg(feature = "std")] before mod and pub use declarations ──
  # This gates all module content so under no_std the crate is empty
  python3 - "$lib_rs" <<'PYEOF'
import sys, re

filepath = sys.argv[1]
with open(filepath, 'r') as f:
    content = f.read()

lines = content.split('\n')
result = []
i = 0
while i < len(lines):
    line = lines[i]
    stripped = line.strip()

    # Skip lines that are already cfg-gated
    if stripped.startswith('#[cfg('):
        result.append(line)
        i += 1
        continue

    # Skip the no_std cfg_attr we just added
    if 'cfg_attr' in stripped and 'no_std' in stripped:
        result.append(line)
        i += 1
        continue

    # Skip comments and blank lines
    if stripped.startswith('//') or stripped == '' or stripped.startswith('#!'):
        result.append(line)
        i += 1
        continue

    # Gate mod declarations
    if re.match(r'^(pub(\(crate\))?\s+)?mod\s+\w+', stripped):
        # Check if previous line is already a cfg gate
        if result and result[-1].strip().startswith('#[cfg(feature = "std")]'):
            result.append(line)
        else:
            result.append('#[cfg(feature = "std")]')
            result.append(line)
        i += 1
        continue

    # Gate pub use declarations
    if re.match(r'^pub\s+use\s+', stripped):
        if result and result[-1].strip().startswith('#[cfg(feature = "std")]'):
            result.append(line)
        else:
            result.append('#[cfg(feature = "std")]')
            result.append(line)
        i += 1
        continue

    # Gate use declarations (non-pub, typically importing from own modules)
    if re.match(r'^use\s+', stripped) and not stripped.startswith('use std::') and not stripped.startswith('use core::'):
        if result and result[-1].strip().startswith('#[cfg(feature = "std")]'):
            result.append(line)
        else:
            result.append('#[cfg(feature = "std")]')
            result.append(line)
        i += 1
        continue

    # Gate any other top-level items (structs, functions, traits, impls, type aliases, consts, statics, extern)
    # but NOT inner attributes (#![...])
    if re.match(r'^(pub\s+)?(fn|struct|enum|trait|impl|type|const|static|extern|macro_rules!|lazy_static)', stripped):
        if result and result[-1].strip().startswith('#[cfg(feature = "std")]'):
            result.append(line)
        else:
            result.append('#[cfg(feature = "std")]')
            result.append(line)
        i += 1
        continue

    # Gate derive macros and attribute macros on items
    if stripped.startswith('#[') and not stripped.startswith('#![') and not stripped.startswith('#[cfg('):
        result.append(line)
        i += 1
        continue

    result.append(line)
    i += 1

with open(filepath, 'w') as f:
    f.write('\n'.join(result))
PYEOF

  echo "  Done"
done

echo ""
echo "=== no_std support added ==="
