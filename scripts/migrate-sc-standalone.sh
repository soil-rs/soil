#!/usr/bin/env bash
# Phase 5: Migrate standalone sc-* crates to soil-*
# This script handles directory moves, Cargo.toml renames, and reference updates.
set -euo pipefail

cd "$(dirname "$0")/.."

# ── Mapping: old_pkg|old_dir|new_pkg|new_dir ──
# Group 1: Simple standalone crates (no sub-crates, not nested in Phase 4 parent)
GROUP1=(
  "sc-allocator|client/allocator|soil-allocator|soil/allocator"
  "sc-basic-authorship|client/basic-authorship|soil-basic-authorship|soil/basic-authorship"
  "sc-cli|client/cli|soil-cli|soil/cli"
  "sc-client-api|client/api|soil-client-api|soil/client-api"
  "sc-client-db|client/db|soil-client-db|soil/client-db"
  "sc-informant|client/informant|soil-informant|soil/informant"
  "sc-network-gossip|client/network-gossip|soil-network-gossip|soil/network-gossip"
  "sc-proposer-metrics|client/proposer-metrics|soil-proposer-metrics|soil/proposer-metrics"
  "sc-rpc-api|client/rpc-api|soil-rpc-api|soil/rpc-api"
  "sc-rpc-server|client/rpc-servers|soil-rpc-server|soil/rpc-server"
  "sc-rpc-spec-v2|client/rpc-spec-v2|soil-rpc-spec-v2|soil/rpc-spec-v2"
  "sc-runtime-utilities|client/runtime-utilities|soil-runtime-utilities|soil/runtime-utilities"
  "sc-state-db|client/state-db|soil-state-db|soil/state-db"
  "sc-storage-monitor|client/storage-monitor|soil-storage-monitor|soil/storage-monitor"
  "sc-sync-state-rpc|client/sync-state-rpc|soil-sync-state-rpc|soil/sync-state-rpc"
  "sc-sysinfo|client/sysinfo|soil-sysinfo|soil/sysinfo"
  "sc-telemetry|client/telemetry|soil-telemetry|soil/telemetry"
  "sc-utils|client/utils|soil-utils|soil/utils"
)

# Group 2: Sub-crates nested inside Phase 4 parents
GROUP2=(
  "sc-consensus-babe-rpc|client/consensus/babe/rpc|soil-consensus-babe-rpc|soil/consensus-babe-rpc"
  "sc-consensus-beefy-rpc|client/consensus/beefy/rpc|soil-consensus-beefy-rpc|soil/consensus-beefy-rpc"
  "sc-consensus-epochs|client/consensus/epochs|soil-consensus-epochs|soil/consensus-epochs"
  "sc-consensus-grandpa-rpc|client/consensus/grandpa/rpc|soil-consensus-grandpa-rpc|soil/consensus-grandpa-rpc"
  "sc-consensus-manual-seal|client/consensus/manual-seal|soil-consensus-manual-seal|soil/consensus-manual-seal"
  "sc-tracing-proc-macro|client/tracing/proc-macro|soil-tracing-proc-macro|soil/tracing-proc-macro"
  "sc-transaction-pool-api|client/transaction-pool/api|soil-transaction-pool-api|soil/transaction-pool-api"
)

# Group 3: Parent crates that have sub-crates (sub-crates moved first in Group 2/3sub)
# Sub-crates extracted first, then parent moved
GROUP3_SUBS=(
  "sc-chain-spec-derive|client/chain-spec/derive|soil-chain-spec-derive|soil/chain-spec-derive"
  "sc-executor-common|client/executor/common|soil-executor-common|soil/executor-common"
  "sc-executor-polkavm|client/executor/polkavm|soil-executor-polkavm|soil/executor-polkavm"
  "sc-executor-wasmtime|client/executor/wasmtime|soil-executor-wasmtime|soil/executor-wasmtime"
  "sc-runtime-test|client/executor/runtime-test|soil-runtime-test|soil/runtime-test"
  "sc-network-common|client/network/common|soil-network-common|soil/network-common"
  "sc-network-light|client/network/light|soil-network-light|soil/network-light"
  "sc-network-statement|client/network/statement|soil-network-statement|soil/network-statement"
  "sc-network-sync|client/network/sync|soil-network-sync|soil/network-sync"
  "sc-network-test|client/network/test|soil-network-test|soil/network-test"
  "sc-network-transactions|client/network/transactions|soil-network-transactions|soil/network-transactions"
  "sc-network-types|client/network/types|soil-network-types|soil/network-types"
  "sc-service-test|client/service/test|soil-service-test|soil/service-test"
  "mmr-rpc|client/merkle-mountain-range/rpc|soil-mmr-rpc|soil/mmr-rpc"
)

GROUP3_PARENTS=(
  "sc-chain-spec|client/chain-spec|soil-chain-spec|soil/chain-spec"
  "sc-executor|client/executor|soil-executor|soil/executor"
  "sc-network|client/network|soil-network|soil/network"
  "sc-service|client/service|soil-service|soil/service"
  "mmr-gadget|client/merkle-mountain-range|soil-mmr-gadget|soil/mmr-gadget"
)

move_crate() {
  local old_dir="$1" new_dir="$2" old_pkg="$3" new_pkg="$4"

  if [ ! -d "$old_dir" ]; then
    echo "SKIP (not found): $old_dir"
    return
  fi

  echo "Moving $old_pkg → $new_pkg ($old_dir → $new_dir)"
  mv "$old_dir" "$new_dir"

  # Rename package in Cargo.toml
  sed -i "s/^name = \"${old_pkg}\"/name = \"${new_pkg}\"/" "$new_dir/Cargo.toml"
}

echo "=== Phase 5: Standalone sc-* → soil-* ==="
echo ""

# Step 1: Move Group 2 (sub-crates of Phase 4 parents)
echo "--- Group 2: Sub-crates of Phase 4 parents ---"
for entry in "${GROUP2[@]}"; do
  IFS='|' read -r old_pkg old_dir new_pkg new_dir <<< "$entry"
  move_crate "$old_dir" "$new_dir" "$old_pkg" "$new_pkg"
done

echo ""

# Step 2: Move Group 3 subs (sub-crates of Phase 5 parents)
echo "--- Group 3 subs: Sub-crates of Phase 5 parents ---"
for entry in "${GROUP3_SUBS[@]}"; do
  IFS='|' read -r old_pkg old_dir new_pkg new_dir <<< "$entry"
  move_crate "$old_dir" "$new_dir" "$old_pkg" "$new_pkg"
done

echo ""

# Step 3: Move Group 3 parents (now safe since sub-crates extracted)
echo "--- Group 3 parents ---"
for entry in "${GROUP3_PARENTS[@]}"; do
  IFS='|' read -r old_pkg old_dir new_pkg new_dir <<< "$entry"
  move_crate "$old_dir" "$new_dir" "$old_pkg" "$new_pkg"
done

echo ""

# Step 4: Move Group 1 (simple standalone)
echo "--- Group 1: Simple standalone ---"
for entry in "${GROUP1[@]}"; do
  IFS='|' read -r old_pkg old_dir new_pkg new_dir <<< "$entry"
  move_crate "$old_dir" "$new_dir" "$old_pkg" "$new_pkg"
done

echo ""
echo "=== Directory moves complete ==="
