# Security Policies Implementation

**Status:** ✅ **Complete**  
**Date:** December 2024

## Overview

Mondoshawan now includes a comprehensive security policy system that allows wallets, nodes, and contracts to opt-in to behavior gating based on risk scores and other security criteria. This enables users to enforce security policies like "reject transactions above risk score X" or "require risk summary before executing contract Y".

## What Was Implemented

### 1. Security Policy Module (`src/security/policies.rs`)

#### Core Structures

- **`SecurityPolicy`** - Defines a security policy with:
  - `id`: Unique policy identifier
  - `name`: Human-readable policy name
  - `owner`: Address that created the policy (zero address for global policies)
  - `policy_type`: Type of policy (see below)
  - `action`: Action to take when triggered (Reject, Warn, RequireConfirmation, Log)
  - `enabled`: Whether the policy is currently active
  - `metadata`: Extensible metadata for future features

- **`PolicyType`** - Types of policies:
  - `MaxRiskScore { threshold }` - Reject transactions above risk score threshold
  - `RequireRiskSummary { contract_address }` - Require risk summary before executing contract
  - `BlockAddress { addresses }` - Block specific addresses
  - `BlockRiskLabels { labels }` - Block addresses with specific risk labels
  - `MinConfidence { threshold }` - Require minimum confidence for risk scores
  - `Custom { policy_id }` - Custom policies (for future extensibility)

- **`PolicyAction`** - Actions when policy is triggered:
  - `Reject { reason }` - Reject the transaction/operation
  - `Warn { message }` - Warn but allow (with logging)
  - `RequireConfirmation { message }` - Require additional confirmation
  - `Log { message }` - Log and continue

- **`PolicyEvaluation`** - Result of policy evaluation:
  - `triggered`: Whether any policy was triggered
  - `policy`: The policy that was triggered (if any)
  - `action`: The action to take
  - `message`: Evaluation message

- **`SecurityPolicyManager`** - Manages all security policies:
  - Stores policies by owner (address-specific and global)
  - Validates policies before adding
  - Evaluates transactions against policies
  - Enables/disables policies

#### Key Methods

- `add_policy(policy)` - Add a new security policy
- `remove_policy(owner, policy_id)` - Remove a policy
- `set_policy_enabled(owner, policy_id, enabled)` - Enable/disable a policy
- `get_policies(owner)` - Get all policies for an owner (includes global policies)
- `evaluate_transaction(tx, risk_score, owner)` - Evaluate a transaction against policies

### 2. RPC Endpoints

#### `Mondoshawan_addSecurityPolicy`

Add a new security policy.

**Parameters:**
- `policy` (object): Security policy object

**Policy Object Format:**
```json
{
  "id": "policy_1",
  "name": "Reject High Risk Transactions",
  "owner": "0x...",
  "policy_type": {
    "MaxRiskScore": { "threshold": 0.7 }
  },
  "action": {
    "Reject": { "reason": "Risk score too high" }
  },
  "enabled": true,
  "metadata": {}
}
```

**Response:**
```json
{
  "policy_id": "policy_1",
  "message": "Policy added successfully",
  "policy": { ... }
}
```

#### `Mondoshawan_removeSecurityPolicy`

Remove a security policy.

**Parameters:**
- `owner` (string): Owner address
- `policy_id` (string): Policy ID to remove

**Response:**
```json
{
  "message": "Policy removed successfully"
}
```

#### `Mondoshawan_getSecurityPolicies`

Get all security policies for an owner.

**Parameters:**
- `owner` (string, optional): Owner address (default: global policies)

**Response:**
```json
{
  "owner": "0x...",
  "policy_count": 2,
  "policies": [ ... ]
}
```

#### `Mondoshawan_setPolicyEnabled`

Enable or disable a policy.

**Parameters:**
- `owner` (string): Owner address
- `policy_id` (string): Policy ID
- `enabled` (boolean): Enable or disable

**Response:**
```json
{
  "message": "Policy policy_1 enabled"
}
```

#### `Mondoshawan_evaluateTransactionPolicy`

Evaluate a transaction against all applicable policies.

**Parameters:**
- `tx_hash` (string): Transaction hash
- `owner` (string, optional): Owner address for policy lookup

**Response:**
```json
{
  "triggered": true,
  "message": "Policy 'Reject High Risk Transactions' triggered",
  "action": {
    "Reject": { "reason": "Risk score too high" }
  },
  "policy": { ... },
  "risk_score": {
    "score": 0.85,
    "confidence": 0.9,
    "labels": ["high_risk", "suspicious_activity"]
  }
}
```

### 3. Integration

- **Node Integration:** Policy manager initialized in `Node::new()` and passed to RPC server
- **RPC Server:** Policy manager added as optional field, with `set_policy_manager()` method
- **Security Module:** Policies module exported from `src/security/mod.rs`

## Usage Examples

### Example 1: Reject High-Risk Transactions

```json
{
  "jsonrpc": "2.0",
  "method": "Mondoshawan_addSecurityPolicy",
  "params": [{
    "id": "reject_high_risk",
    "name": "Reject High Risk Transactions",
    "owner": "0x1234...",
    "policy_type": {
      "MaxRiskScore": { "threshold": 0.7 }
    },
    "action": {
      "Reject": { "reason": "Transaction risk score exceeds threshold" }
    },
    "enabled": true,
    "metadata": {}
  }],
  "id": 1
}
```

### Example 2: Block Specific Addresses

```json
{
  "jsonrpc": "2.0",
  "method": "Mondoshawan_addSecurityPolicy",
  "params": [{
    "id": "block_scammer",
    "name": "Block Known Scammer Addresses",
    "owner": "0x1234...",
    "policy_type": {
      "BlockAddress": {
        "addresses": ["0xabcd...", "0xef01..."]
      }
    },
    "action": {
      "Reject": { "reason": "Address is on blocklist" }
    },
    "enabled": true,
    "metadata": {}
  }],
  "id": 2
}
```

### Example 3: Require Risk Summary for Contract

```json
{
  "jsonrpc": "2.0",
  "method": "Mondoshawan_addSecurityPolicy",
  "params": [{
    "id": "require_risk_summary",
    "name": "Require Risk Summary for Contract",
    "owner": "0x1234...",
    "policy_type": {
      "RequireRiskSummary": {
        "contract_address": "0xcontract..."
      }
    },
    "action": {
      "RequireConfirmation": {
        "message": "Risk summary required before executing contract"
      }
    },
    "enabled": true,
    "metadata": {}
  }],
  "id": 3
}
```

### Example 4: Evaluate Transaction

```json
{
  "jsonrpc": "2.0",
  "method": "Mondoshawan_evaluateTransactionPolicy",
  "params": [
    "0xtx_hash...",
    "0xowner..."
  ],
  "id": 4
}
```

## Policy Evaluation Flow

1. **Transaction Submission:** When a transaction is submitted, it can be evaluated against policies
2. **Risk Scoring:** Transaction is scored using the `RiskScorer`
3. **Policy Matching:** All applicable policies (global + owner-specific) are checked
4. **Action Execution:** If a policy matches, the specified action is taken:
   - `Reject`: Transaction is rejected
   - `Warn`: Warning is logged, transaction proceeds
   - `RequireConfirmation`: User confirmation required
   - `Log`: Event is logged, transaction proceeds

## Security Considerations

1. **Opt-In:** Policies are opt-in - users must explicitly create and enable them
2. **Owner-Based:** Policies are scoped to owners, allowing per-wallet/per-node policies
3. **Global Policies:** Zero address (`0x0000...`) can be used for global policies
4. **Validation:** Policies are validated before being added (threshold ranges, non-empty lists, etc.)
5. **Non-Blocking:** Policy evaluation does not block protocol-level validation - it's an additional layer

## Future Enhancements

1. **On-Chain Policies:** Store policies on-chain for decentralized policy management
2. **Policy Templates:** Pre-defined policy templates for common use cases
3. **Policy Marketplace:** Share and discover security policies
4. **Advanced Actions:** More sophisticated actions (e.g., rate limiting, multi-factor confirmation)
5. **Policy Analytics:** Track policy effectiveness and false positive rates
6. **Contract Integration:** Allow smart contracts to define and enforce policies

## Summary

Security policies are now fully integrated into Mondoshawan, providing:
- ✅ Flexible policy definition system
- ✅ Multiple policy types (risk score, address blocking, label blocking, etc.)
- ✅ Multiple action types (reject, warn, require confirmation, log)
- ✅ Owner-based policy scoping
- ✅ Global policies support
- ✅ Complete RPC API for policy management
- ✅ Transaction evaluation against policies
- ✅ Integration with risk scoring system

This implementation enables Mondoshawan to be the "security-native L1" by allowing users to enforce their own security policies while maintaining protocol-level flexibility.
