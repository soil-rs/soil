#!/usr/bin/env bash
# Update workspace Cargo.toml: member paths and dependency paths for migrated sc-* crates
set -euo pipefail

cd "$(dirname "$0")/.."

CARGO="Cargo.toml"

# All path mappings: old_member_path|new_member_path|old_dep_path|new_dep_path|old_pkg|new_pkg
# (member path and dep path are usually the same, but listed separately for clarity)
MAPPINGS=(
  # Group 1: Simple standalone
  "client/allocator|soil/allocator|sc-allocator|soil-allocator"
  "client/basic-authorship|soil/basic-authorship|sc-basic-authorship|soil-basic-authorship"
  "client/cli|soil/cli|sc-cli|soil-cli"
  "client/api|soil/client-api|sc-client-api|soil-client-api"
  "client/db|soil/client-db|sc-client-db|soil-client-db"
  "client/informant|soil/informant|sc-informant|soil-informant"
  "client/network-gossip|soil/network-gossip|sc-network-gossip|soil-network-gossip"
  "client/proposer-metrics|soil/proposer-metrics|sc-proposer-metrics|soil-proposer-metrics"
  "client/rpc-api|soil/rpc-api|sc-rpc-api|soil-rpc-api"
  "client/rpc-servers|soil/rpc-server|sc-rpc-server|soil-rpc-server"
  "client/rpc-spec-v2|soil/rpc-spec-v2|sc-rpc-spec-v2|soil-rpc-spec-v2"
  "client/runtime-utilities|soil/runtime-utilities|sc-runtime-utilities|soil-runtime-utilities"
  "client/state-db|soil/state-db|sc-state-db|soil-state-db"
  "client/storage-monitor|soil/storage-monitor|sc-storage-monitor|soil-storage-monitor"
  "client/sync-state-rpc|soil/sync-state-rpc|sc-sync-state-rpc|soil-sync-state-rpc"
  "client/sysinfo|soil/sysinfo|sc-sysinfo|soil-sysinfo"
  "client/telemetry|soil/telemetry|sc-telemetry|soil-telemetry"
  "client/utils|soil/utils|sc-utils|soil-utils"
  # Group 2: Sub-crates of Phase 4 parents
  "client/consensus/babe/rpc|soil/consensus-babe-rpc|sc-consensus-babe-rpc|soil-consensus-babe-rpc"
  "client/consensus/beefy/rpc|soil/consensus-beefy-rpc|sc-consensus-beefy-rpc|soil-consensus-beefy-rpc"
  "client/consensus/epochs|soil/consensus-epochs|sc-consensus-epochs|soil-consensus-epochs"
  "client/consensus/grandpa/rpc|soil/consensus-grandpa-rpc|sc-consensus-grandpa-rpc|soil-consensus-grandpa-rpc"
  "client/consensus/manual-seal|soil/consensus-manual-seal|sc-consensus-manual-seal|soil-consensus-manual-seal"
  "client/tracing/proc-macro|soil/tracing-proc-macro|sc-tracing-proc-macro|soil-tracing-proc-macro"
  "client/transaction-pool/api|soil/transaction-pool-api|sc-transaction-pool-api|soil-transaction-pool-api"
  # Group 3 subs
  "client/chain-spec/derive|soil/chain-spec-derive|sc-chain-spec-derive|soil-chain-spec-derive"
  "client/executor/common|soil/executor-common|sc-executor-common|soil-executor-common"
  "client/executor/polkavm|soil/executor-polkavm|sc-executor-polkavm|soil-executor-polkavm"
  "client/executor/wasmtime|soil/executor-wasmtime|sc-executor-wasmtime|soil-executor-wasmtime"
  "client/executor/runtime-test|soil/runtime-test|sc-runtime-test|soil-runtime-test"
  "client/network/common|soil/network-common|sc-network-common|soil-network-common"
  "client/network/light|soil/network-light|sc-network-light|soil-network-light"
  "client/network/statement|soil/network-statement|sc-network-statement|soil-network-statement"
  "client/network/sync|soil/network-sync|sc-network-sync|soil-network-sync"
  "client/network/test|soil/network-test|sc-network-test|soil-network-test"
  "client/network/transactions|soil/network-transactions|sc-network-transactions|soil-network-transactions"
  "client/network/types|soil/network-types|sc-network-types|soil-network-types"
  "client/service/test|soil/service-test|sc-service-test|soil-service-test"
  "client/merkle-mountain-range/rpc|soil/mmr-rpc|mmr-rpc|soil-mmr-rpc"
  # Group 3 parents
  "client/chain-spec|soil/chain-spec|sc-chain-spec|soil-chain-spec"
  "client/executor|soil/executor|sc-executor|soil-executor"
  "client/network|soil/network|sc-network|soil-network"
  "client/service|soil/service|sc-service|soil-service"
  "client/merkle-mountain-range|soil/mmr-gadget|mmr-gadget|soil-mmr-gadget"
)

echo "Updating workspace Cargo.toml member paths and dependency paths..."

for entry in "${MAPPINGS[@]}"; do
  IFS='|' read -r old_path new_path old_pkg new_pkg <<< "$entry"

  # Update member path (in [workspace] members)
  sed -i "s|\"${old_path}\"|\"${new_path}\"|g" "$CARGO"

  # Update dependency path: path = "client/..." → path = "soil/..."
  sed -i "s|path = \"${old_path}\"|path = \"${new_path}\"|g" "$CARGO"

  # Update dependency key name (e.g., sc-allocator = ... → soil-allocator = ...)
  # Only for the workspace.dependencies section - match lines like:
  # sc-allocator = { path = ...
  sed -i "s/^${old_pkg} = /${new_pkg} = /" "$CARGO"
done

echo "Done updating workspace Cargo.toml"
