#!/usr/bin/env bash
# Update all source code references: sc_* → soil_* (use statements, module paths)
# and sc-* → soil-* (in string literals, doc comments, etc.)
set -euo pipefail

cd "$(dirname "$0")/.."

# Underscore form renames (for use statements, module paths)
# Ordered from most specific to least specific
RENAMES_UNDERSCORE=(
  "sc_consensus_babe_rpc|soil_consensus_babe_rpc"
  "sc_consensus_beefy_rpc|soil_consensus_beefy_rpc"
  "sc_consensus_grandpa_rpc|soil_consensus_grandpa_rpc"
  "sc_consensus_manual_seal|soil_consensus_manual_seal"
  "sc_consensus_epochs|soil_consensus_epochs"
  "sc_transaction_pool_api|soil_transaction_pool_api"
  "sc_tracing_proc_macro|soil_tracing_proc_macro"
  "sc_chain_spec_derive|soil_chain_spec_derive"
  "sc_executor_wasmtime|soil_executor_wasmtime"
  "sc_executor_polkavm|soil_executor_polkavm"
  "sc_executor_common|soil_executor_common"
  "sc_network_transactions|soil_network_transactions"
  "sc_network_statement|soil_network_statement"
  "sc_network_common|soil_network_common"
  "sc_network_gossip|soil_network_gossip"
  "sc_network_light|soil_network_light"
  "sc_network_sync|soil_network_sync"
  "sc_network_test|soil_network_test"
  "sc_network_types|soil_network_types"
  "sc_basic_authorship|soil_basic_authorship"
  "sc_runtime_utilities|soil_runtime_utilities"
  "sc_runtime_test|soil_runtime_test"
  "sc_storage_monitor|soil_storage_monitor"
  "sc_sync_state_rpc|soil_sync_state_rpc"
  "sc_proposer_metrics|soil_proposer_metrics"
  "sc_rpc_spec_v2|soil_rpc_spec_v2"
  "sc_rpc_server|soil_rpc_server"
  "sc_service_test|soil_service_test"
  "sc_chain_spec|soil_chain_spec"
  "sc_client_api|soil_client_api"
  "sc_client_db|soil_client_db"
  "sc_rpc_api|soil_rpc_api"
  "sc_state_db|soil_state_db"
  "sc_allocator|soil_allocator"
  "sc_executor|soil_executor"
  "sc_informant|soil_informant"
  "sc_network|soil_network"
  "sc_service|soil_service"
  "sc_sysinfo|soil_sysinfo"
  "sc_telemetry|soil_telemetry"
  "sc_utils|soil_utils"
  "sc_cli|soil_cli"
  "mmr_gadget|soil_mmr_gadget"
  "mmr_rpc|soil_mmr_rpc"
)

# Hyphen form renames (for string literals, doc comments, log targets, etc.)
RENAMES_HYPHEN=(
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

echo "Updating source code references (underscore form: sc_* → soil_*)..."

# Find all Rust source files
find . -type f \( -name "*.rs" \) -not -path "./target/*" | while read -r f; do
  changed=false
  for entry in "${RENAMES_UNDERSCORE[@]}"; do
    IFS='|' read -r old_name new_name <<< "$entry"
    if grep -q "$old_name" "$f"; then
      sed -i "s/\b${old_name}\b/${new_name}/g" "$f"
      changed=true
    fi
  done
  if $changed; then
    echo "  Updated (underscore): $f"
  fi
done

echo ""
echo "Updating source code references (hyphen form in strings: sc-* → soil-*)..."

# Update hyphen forms in .rs files (doc comments, string literals, log targets)
find . -type f \( -name "*.rs" \) -not -path "./target/*" | while read -r f; do
  changed=false
  for entry in "${RENAMES_HYPHEN[@]}"; do
    IFS='|' read -r old_name new_name <<< "$entry"
    if grep -q "$old_name" "$f"; then
      sed -i "s/${old_name}/${new_name}/g" "$f"
      changed=true
    fi
  done
  if $changed; then
    echo "  Updated (hyphen): $f"
  fi
done

echo ""
echo "Done updating source code references"
