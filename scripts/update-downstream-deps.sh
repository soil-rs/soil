#!/usr/bin/env bash
# Update all downstream Cargo.toml files: rename sc-* dependencies to soil-*
set -euo pipefail

cd "$(dirname "$0")/.."

# Ordered from most specific to least specific to avoid partial matches
# e.g., sc-consensus-manual-seal before sc-consensus, sc-network-gossip before sc-network
RENAMES=(
  # Long/specific names first
  "sc-consensus-babe-rpc|soil-consensus-babe-rpc"
  "sc-consensus-beefy-rpc|soil-consensus-beefy-rpc"
  "sc-consensus-grandpa-rpc|soil-consensus-grandpa-rpc"
  "sc-consensus-manual-seal|soil-consensus-manual-seal"
  "sc-consensus-epochs|soil-consensus-epochs"
  "sc-transaction-pool-api|soil-transaction-pool-api"
  "sc-tracing-proc-macro|soil-tracing-proc-macro"
  "sc-chain-spec-derive|soil-chain-spec-derive"
  "sc-executor-wasmtime|soil-executor-wasmtime"
  "sc-executor-polkavm|soil-executor-polkavm"
  "sc-executor-common|soil-executor-common"
  "sc-network-transactions|soil-network-transactions"
  "sc-network-statement|soil-network-statement"
  "sc-network-common|soil-network-common"
  "sc-network-gossip|soil-network-gossip"
  "sc-network-light|soil-network-light"
  "sc-network-sync|soil-network-sync"
  "sc-network-test|soil-network-test"
  "sc-network-types|soil-network-types"
  "sc-basic-authorship|soil-basic-authorship"
  "sc-runtime-utilities|soil-runtime-utilities"
  "sc-runtime-test|soil-runtime-test"
  "sc-storage-monitor|soil-storage-monitor"
  "sc-sync-state-rpc|soil-sync-state-rpc"
  "sc-proposer-metrics|soil-proposer-metrics"
  "sc-rpc-spec-v2|soil-rpc-spec-v2"
  "sc-rpc-server|soil-rpc-server"
  "sc-service-test|soil-service-test"
  "sc-chain-spec|soil-chain-spec"
  "sc-client-api|soil-client-api"
  "sc-client-db|soil-client-db"
  "sc-rpc-api|soil-rpc-api"
  "sc-state-db|soil-state-db"
  "sc-allocator|soil-allocator"
  "sc-executor|soil-executor"
  "sc-informant|soil-informant"
  "sc-network|soil-network"
  "sc-service|soil-service"
  "sc-sysinfo|soil-sysinfo"
  "sc-telemetry|soil-telemetry"
  "sc-utils|soil-utils"
  "sc-cli|soil-cli"
  "mmr-gadget|soil-mmr-gadget"
  "mmr-rpc|soil-mmr-rpc"
)

echo "Updating downstream Cargo.toml references..."

# Find all Cargo.toml files (exclude workspace root which is already done)
find . -name "Cargo.toml" -not -path "./Cargo.toml" -not -path "./target/*" | while read -r toml; do
  changed=false
  for entry in "${RENAMES[@]}"; do
    IFS='|' read -r old_name new_name <<< "$entry"
    if grep -q "$old_name" "$toml"; then
      # Replace dependency key names (start of line or after whitespace)
      # Replace in dependency declarations: sc-foo = { ... } → soil-foo = { ... }
      sed -i "s/\b${old_name}\b/${new_name}/g" "$toml"
      changed=true
    fi
  done
  if $changed; then
    echo "  Updated: $toml"
  fi
done

echo "Done updating downstream Cargo.toml files"
