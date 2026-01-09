# Mondoshawan Flagship Features Roadmap

**Status:** 🚧 **In Progress**  
**Date:** December 2024

## Overview

This document outlines the plan to turn Mondoshawan's advanced features into provable, default behaviors and establish a clear flagship identity. The goal is to make Mondoshawan the obvious choice for specific use cases through deep integration and visibility of unique capabilities.

## A. Turn Advanced Features into Provable, Default Behaviors

### 1. Verkle/Stateless: Close the Loop ✅ (Partially Complete)

**Current Status:**
- ✅ Verkle tree integrated into blockchain
- ✅ State proofs generated
- ✅ RPC endpoints for state root and proofs
- ⚠️ Verkle is NOT yet the canonical source (still uses in-memory cache first)
- ❌ Light-client mode not implemented

**Required Changes:**

1. **Make Verkle Canonical Source** ✅ (In Progress)
   - [x] Update `get_balance()` to check Verkle first when enabled
   - [x] Update `get_nonce()` to check Verkle first when enabled
   - [ ] Remove in-memory cache when Verkle is enabled (or make it read-only)
   - [ ] Ensure all state updates go through Verkle when enabled

2. **Light-Client Mode**
   - [ ] Create `LightClient` struct that only stores state roots
   - [ ] Implement proof verification for balance/nonce queries
   - [ ] Add RPC endpoint `Mondoshawan_lightClientMode` to enable/disable
   - [ ] Add explorer toggle for light-client mode
   - [ ] Document light-client usage

3. **Enhanced RPC Integration**
   - [x] `Mondoshawan_getStateRoot` - Returns current state root
   - [x] `Mondoshawan_getStateProof` - Returns proof for address
   - [x] `Mondoshawan_verifyStateProof` - Verifies a proof
   - [ ] Add `Mondoshawan_getStateRootHistory` - Track state root changes over time
   - [ ] Add `Mondoshawan_getLightClientSyncStatus` - Check if light client is synced

### 2. PQ as First-Class Account Type ⚠️ (Partially Complete)

**Current Status:**
- ✅ PQ accounts implemented (Dilithium3, SPHINCS+)
- ✅ PQ signatures in transactions
- ✅ Kyber key exchange for P2P
- ❌ No tooling for generating PQ keys
- ❌ No RPC helpers for PQ account creation
- ❌ No CLI commands for PQ operations

**Required Changes:**

1. **PQ Account Tooling** 🚧 (In Progress)
   - [x] Create `src/pqc/tooling.rs` module
   - [ ] Fix tooling to match actual PqAccount structure
   - [ ] Add `generate_pq_account()` function
   - [ ] Add `create_pq_transaction()` helper
   - [ ] Add key export/import functions

2. **RPC Endpoints for PQ**
   - [ ] `Mondoshawan_generatePqAccount` - Generate new PQ account
   - [ ] `Mondoshawan_getPqAccountType` - Check if address is PQ account
   - [ ] `Mondoshawan_exportPqKey` - Export PQ private key (with auth)
   - [ ] `Mondoshawan_importPqKey` - Import PQ private key
   - [ ] `Mondoshawan_createPqTransaction` - Create PQ-signed transaction

3. **CLI Commands**
   - [ ] `Mondoshawan-cli account generate --pq dilithium3`
   - [ ] `Mondoshawan-cli account generate --pq sphincsplus`
   - [ ] `Mondoshawan-cli account list --pq-only`
   - [ ] `Mondoshawan-cli tx create --pq-account <address>`

4. **Explorer Integration**
   - [ ] Show PQ account badge in address view
   - [ ] Display PQ signature type in transaction view
   - [ ] Add PQ account generation UI
   - [ ] Show PQ vs Ed25519 statistics

### 3. Security Scoring that Gates Behavior ⚠️ (Not Started)

**Current Status:**
- ✅ Risk scoring implemented
- ✅ Anomaly detection working
- ✅ Forensic analysis available
- ❌ No policy enforcement
- ❌ No opt-in security gates

**Required Changes:**

1. **Security Policies**
   - [ ] Create `SecurityPolicy` struct:
     ```rust
     pub struct SecurityPolicy {
         max_risk_score: f64,  // Reject txs above this
         require_risk_summary: bool,  // Require risk analysis before execution
         whitelist_addresses: Vec<Address>,  // Always allow these
         blacklist_addresses: Vec<Address>,  // Always reject these
         contract_policies: HashMap<Address, ContractPolicy>,  // Per-contract policies
     }
     ```
   - [ ] Add policy storage (on-chain or config file)
   - [ ] Implement policy checking in transaction validation

2. **RPC Endpoints**
   - [ ] `Mondoshawan_setSecurityPolicy` - Set node security policy
   - [ ] `Mondoshawan_getSecurityPolicy` - Get current policy
   - [ ] `Mondoshawan_checkTransactionRisk` - Pre-check transaction risk
   - [ ] `Mondoshawan_getPolicyViolations` - List recent policy violations

3. **Contract Integration**
   - [ ] Add `requireRiskCheck()` modifier for contracts
   - [ ] Add `getRiskScore()` function for contracts
   - [ ] Document contract security patterns

4. **Explorer Integration**
   - [ ] Show security policy status
   - [ ] Display policy violations
   - [ ] Allow policy configuration via UI
   - [ ] Show which addresses/contracts have policies

## B. Make AI/Fairness/Security Story Visible and Actionable

### 1. Explorer Upgrades 🚧 (Partially Complete)

**Current Status:**
- ✅ Fairness metrics displayed
- ✅ Risk scores shown
- ✅ Shard information visible
- ⚠️ Missing "Explain this address/tx" section
- ⚠️ Missing comprehensive dashboards

**Required Changes:**

1. **New Explorer Panels**
   - [ ] **Fairness Metrics Panel**
     - [x] Reordering distance per block
     - [x] MEV pattern counts
     - [x] Fairness score
     - [ ] Historical fairness trends
     - [ ] Stream-specific fairness breakdown

   - [ ] **Risk Overlays**
     - [x] Risk badges on addresses
     - [x] Risk scores displayed
     - [ ] Risk trend graphs
     - [ ] Risk comparison tool

   - [ ] **Shard & TriStream Breakdown**
     - [x] Blocks/txs per shard
     - [x] Cross-shard traffic
     - [ ] Stream-specific statistics
     - [ ] Shard utilization graphs

   - [ ] **"Explain This Address/Tx" Section** ⚠️ (Priority)
     - [ ] Natural language summary of address behavior
     - [ ] Transaction flow explanation
     - [ ] Risk factors breakdown
     - [ ] Anomaly explanations
     - [ ] Recommendations

2. **Enhanced Visualizations**
   - [ ] Interactive flow diagrams
   - [ ] Risk heatmaps
   - [ ] Fairness score charts
   - [ ] MEV pattern visualization

### 2. Grafana Dashboards ❌ (Not Started)

**Required:**
- [ ] Create `grafana/dashboards/` directory
- [ ] **MEV/Fairness Dashboard**
  - MEV metrics over time
  - Fairness scores per stream
  - Ordering policy effectiveness
- [ ] **PQ Usage Dashboard**
  - PQ transaction count
  - PQ node count
  - PQ vs Ed25519 ratio
- [ ] **Shard Utilization Dashboard**
  - Per-shard transaction volume
  - Cross-shard transaction flow
  - Shard balance
- [ ] **Security Dashboard**
  - Risk score distribution
  - Anomaly detection rate
  - Policy violations
- [ ] **Network Health Dashboard**
  - Block production rate
  - Peer connectivity
  - Network latency

### 3. Experiment Scripts ❌ (Not Started)

**Required:**
- [ ] `scripts/experiments/compare_ordering_policies.py`
  - Run two policies side-by-side
  - Compare fairness/MEV metrics
  - Generate report
- [ ] `scripts/experiments/stress_test_pq.py`
  - Test PQ vs non-PQ performance
  - Measure signature verification time
  - Compare transaction throughput
- [ ] `scripts/experiments/stateless_vs_stateful.py`
  - Compare Verkle vs traditional state
  - Measure proof generation time
  - Compare storage requirements

## C. Production-Grade Testnet + Stability Hardening

### 1. Stabilize on Linux + CI ❌ (Not Started)

**Required:**
- [ ] Set up GitHub Actions / GitLab CI
- [ ] Test on Ubuntu 20.04, 22.04
- [ ] Test on Debian 11, 12
- [ ] Fix all unit tests
- [ ] Fix all integration tests
- [ ] Add test coverage reporting
- [ ] Add linting (clippy)
- [ ] Add formatting checks (rustfmt)

### 2. Public Testnet ❌ (Not Started)

**Required:**
- [ ] Stand up persistent testnet nodes
- [ ] Configure with all features enabled:
  - EVM
  - GhostDAG
  - TriStream
  - Verkle
  - Sharding
  - Security/Fairness
- [ ] Connect explorer to testnet
- [ ] Document testnet endpoints
- [ ] Create testnet faucet
- [ ] Monitor testnet health

### 3. Operational Playbooks ❌ (Not Started)

**Required:**
- [ ] **Node Bootstrap Playbook**
  - Step-by-step node setup
  - Key generation (Ed25519 + PQ)
  - Peer connectivity
  - Configuration examples
- [ ] **Metrics & Alerting Setup**
  - Prometheus configuration
  - Alert rules (forks, stalled mining, peer loss)
  - Grafana setup
- [ ] **Backup & Recovery**
  - Database backup procedures
  - State recovery from Verkle proofs
  - Disaster recovery plan

## D. Flagship Identity Selection

### Option 1: "The Security-Native L1" 🎯 (Recommended)

**Focus:**
- Deepen risk scoring and anomaly detection
- On-chain security oracles
- Community-provided risk labels
- Security-first contract patterns

**Target:** Regulated finance, high-risk-sensitive applications

**Implementation:**
- [ ] Baseline, documented risk model
- [ ] On-chain oracle for risk labels
- [ ] Slashing mechanism for false reports
- [ ] Security-first contract templates
- [ ] Compliance tooling

### Option 2: "The Stateless, PQ-Ready L1"

**Focus:**
- Verkle + stateless mode as default
- Light clients with proofs
- PQ accounts and P2P as default
- Long-term survivability

**Target:** Future-proof applications, quantum-resistant use cases

**Implementation:**
- [ ] Make Verkle default (not optional)
- [ ] Light-client SDK
- [ ] PQ account migration tools
- [ ] PQ-first documentation

### Option 3: "The MEV-Aware, Fair-Ordering Lab Chain"

**Focus:**
- Fully expose MEV + fairness metrics
- Configurable ordering policies
- Research-friendly tools
- Fairness-first design

**Target:** Researchers, DEX teams, fairness-obsessed builders

**Implementation:**
- [ ] Advanced MEV detection
- [ ] Custom ordering policy framework
- [ ] Research dataset generation
- [ ] Fairness benchmarking tools

## Recommended Immediate Next Moves

1. **Lock Flagship Identity** → Choose "Security-Native L1"
2. **Wire Features End-to-End:**
   - [ ] Make Verkle canonical source
   - [ ] Add light-client mode
   - [ ] Complete PQ tooling
   - [ ] Implement security policies
   - [ ] Add "Explain this address/tx" to explorer
3. **Stand Up Testnet:**
   - [ ] Configure with all features
   - [ ] Connect explorer
   - [ ] Monitor for stability
4. **Iterate on UX:**
   - [ ] Make security features obvious in first 5 minutes
   - [ ] Add Grafana dashboards
   - [ ] Create experiment scripts

## Success Metrics

- **Verkle:** Light clients can verify state without full node
- **PQ:** Users can generate and use PQ accounts via RPC/CLI
- **Security:** Policies can reject high-risk transactions
- **Visibility:** Explorer shows all advanced features clearly
- **Testnet:** Stable, long-running testnet with all features

---

**Next Steps:** Implement Verkle canonical source, light-client mode, PQ tooling, and security policies in priority order.
