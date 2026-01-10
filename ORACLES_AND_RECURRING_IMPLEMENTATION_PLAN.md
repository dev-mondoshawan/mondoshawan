# Oracles, Recurring Transactions & Stop-Loss Implementation Plan

**Date**: January 2026  
**Priority**: ⭐⭐⭐ **HIGH**  
**Status**: 📋 **PLANNED**

---

## 🎯 Overview

Implementing three high-value features:
1. **Built-In Oracle Network** - Native price feeds and randomness
2. **Recurring Transactions** - Scheduled/recurring payments
3. **Stop-Loss Feature** - Automatic transaction execution on price thresholds

---

## 📋 Feature 1: Built-In Oracle Network

### **Architecture**

```
OracleNode
├── Data Sources (APIs, on-chain data)
├── Aggregation Logic (median, weighted average)
├── Staking Mechanism (security)
└── Slashing (penalize false data)
```

### **Core Components**

1. **OracleRegistry**
   - Register/unregister oracle nodes
   - Track oracle reputation
   - Manage oracle sets per feed

2. **PriceFeedManager**
   - Multiple price feeds (crypto, stocks, commodities)
   - Aggregation (median of N oracles)
   - Update frequency management
   - Historical price data

3. **VRF (Verifiable Random Function)**
   - Cryptographically secure randomness
   - Request/fulfill pattern
   - Proof generation

4. **OracleStaking**
   - Stake MSHW tokens to become oracle
   - Slashing for false data
   - Rewards for accurate data

### **RPC Methods**

```rust
// Oracle Management
mds_registerOracle(address, feed_types, stake_amount)
mds_unregisterOracle(address)
mds_getOracleInfo(address)
mds_getOracleList(feed_type)

// Price Feeds
mds_getPrice(feed_id, asset_pair) // e.g., "BTC/USD", "ETH/MSHW"
mds_getPriceHistory(feed_id, asset_pair, timeframe)
mds_getPriceFeeds() // List all available feeds
mds_subscribePriceFeed(feed_id, callback_url) // Webhook support

// Randomness
mds_requestRandomness(seed, callback)
mds_getRandomness(request_id)
mds_verifyRandomness(proof, seed)

// Oracle Staking
mds_stakeOracle(address, amount)
mds_unstakeOracle(address, amount)
mds_getOracleStake(address)
mds_reportOracle(address, is_accurate) // For slashing
```

### **Data Structures**

```rust
pub struct OracleNode {
    pub address: Address,
    pub feed_types: Vec<FeedType>, // Price, Randomness, Custom
    pub stake_amount: u128,
    pub reputation_score: f64,
    pub last_update: u64,
    pub accuracy_rate: f64,
}

pub struct PriceFeed {
    pub feed_id: String, // e.g., "BTC/USD"
    pub asset_pair: (String, String), // (base, quote)
    pub current_price: u128, // Scaled (e.g., 1e18)
    pub last_update: u64,
    pub oracle_count: usize,
    pub update_frequency: u64, // seconds
}

pub struct RandomnessRequest {
    pub request_id: Hash,
    pub requester: Address,
    pub seed: Hash,
    pub fulfilled: bool,
    pub randomness: Option<Hash>,
    pub proof: Option<Vec<u8>>,
}
```

### **Implementation Files**

- `mondoshawan-blockchain/src/oracles/mod.rs` - Main module
- `mondoshawan-blockchain/src/oracles/registry.rs` - Oracle registration
- `mondoshawan-blockchain/src/oracles/price_feed.rs` - Price feed management
- `mondoshawan-blockchain/src/oracles/vrf.rs` - Randomness generation
- `mondoshawan-blockchain/src/oracles/staking.rs` - Staking mechanism
- `mondoshawan-blockchain/src/oracles/tests.rs` - Unit tests

---

## 📋 Feature 2: Recurring Transactions

### **Architecture**

```
RecurringTransaction
├── Schedule (daily, weekly, monthly, custom)
├── Next Execution Time
├── Remaining Executions (or infinite)
└── Transaction Template
```

### **Core Components**

1. **RecurringTransactionManager**
   - Create/cancel recurring transactions
   - Schedule execution
   - Track execution history

2. **Scheduler**
   - Check for due transactions
   - Execute recurring transactions
   - Handle failures

### **RPC Methods**

```rust
// Recurring Transaction Management
mds_createRecurringTransaction(
    from: Address,
    to: Address,
    value: u128,
    schedule: Schedule, // Daily, Weekly, Monthly, Custom
    start_date: u64,
    end_date: Option<u64>, // None = infinite
    max_executions: Option<u64> // None = infinite
) -> recurring_tx_id

mds_cancelRecurringTransaction(recurring_tx_id: Hash)
mds_getRecurringTransaction(recurring_tx_id: Hash)
mds_getRecurringTransactions(address: Address) // All recurring txs for address
mds_getRecurringTransactionHistory(recurring_tx_id: Hash) // Execution history
mds_pauseRecurringTransaction(recurring_tx_id: Hash)
mds_resumeRecurringTransaction(recurring_tx_id: Hash)
```

### **Data Structures**

```rust
pub enum Schedule {
    Daily { hour: u8, minute: u8 }, // 0-23, 0-59
    Weekly { day_of_week: u8, hour: u8, minute: u8 }, // 0-6 (Sun-Sat)
    Monthly { day_of_month: u8, hour: u8, minute: u8 }, // 1-31
    Custom { interval_seconds: u64 }, // Every N seconds
}

pub struct RecurringTransaction {
    pub recurring_tx_id: Hash,
    pub from: Address,
    pub to: Address,
    pub value: u128,
    pub schedule: Schedule,
    pub created_at: u64,
    pub start_date: u64,
    pub end_date: Option<u64>,
    pub next_execution: u64,
    pub max_executions: Option<u64>,
    pub execution_count: u64,
    pub status: RecurringTxStatus,
    pub last_execution: Option<u64>,
    pub last_execution_tx_hash: Option<Hash>,
}

pub enum RecurringTxStatus {
    Active,
    Paused,
    Cancelled,
    Completed, // Reached max_executions or end_date
    Failed, // Too many failures
}
```

### **Implementation Files**

- `mondoshawan-blockchain/src/recurring/mod.rs` - Main module
- `mondoshawan-blockchain/src/recurring/manager.rs` - Transaction management
- `mondoshawan-blockchain/src/recurring/scheduler.rs` - Execution scheduler
- `mondoshawan-blockchain/src/recurring/tests.rs` - Unit tests

---

## 📋 Feature 3: Stop-Loss Feature

### **Architecture**

```
StopLossOrder
├── Trigger Condition (price threshold)
├── Oracle Feed (price source)
├── Transaction to Execute (when triggered)
└── Status (active, triggered, cancelled)
```

### **Core Components**

1. **StopLossManager**
   - Create/cancel stop-loss orders
   - Monitor price feeds
   - Execute when threshold hit

2. **PriceMonitor**
   - Subscribe to price feeds
   - Check stop-loss conditions
   - Trigger execution

### **RPC Methods**

```rust
// Stop-Loss Management
mds_createStopLoss(
    wallet_address: Address,
    asset_pair: String, // e.g., "BTC/USD"
    trigger_price: u128, // Scaled price
    trigger_type: StopLossType, // Above, Below, PercentChange
    transaction: Transaction, // Transaction to execute when triggered
    oracle_feed_id: Option<String> // Optional: specific oracle feed
) -> stop_loss_id

mds_cancelStopLoss(stop_loss_id: Hash)
mds_getStopLoss(stop_loss_id: Hash)
mds_getStopLossOrders(address: Address) // All stop-loss orders for address
mds_updateStopLossPrice(stop_loss_id: Hash, new_price: u128)
mds_pauseStopLoss(stop_loss_id: Hash)
mds_resumeStopLoss(stop_loss_id: Hash)
```

### **Data Structures**

```rust
pub enum StopLossType {
    PriceAbove(u128), // Execute when price >= threshold
    PriceBelow(u128), // Execute when price <= threshold
    PercentChangeUp(f64), // Execute when price increases by X%
    PercentChangeDown(f64), // Execute when price decreases by X%
    PriceRange { min: u128, max: u128 }, // Execute when price leaves range
}

pub struct StopLossOrder {
    pub stop_loss_id: Hash,
    pub wallet_address: Address,
    pub asset_pair: String, // e.g., "BTC/USD"
    pub trigger_type: StopLossType,
    pub transaction: Transaction, // Transaction to execute
    pub oracle_feed_id: Option<String>,
    pub created_at: u64,
    pub status: StopLossStatus,
    pub triggered_at: Option<u64>,
    pub triggered_price: Option<u128>,
    pub execution_tx_hash: Option<Hash>,
}

pub enum StopLossStatus {
    Active,
    Paused,
    Triggered,
    Cancelled,
    Expired, // If expiration date set
}
```

### **Integration with Account Abstraction**

Stop-loss can be integrated with smart contract wallets:
- Wallet-level stop-loss orders
- Multi-sig approval for stop-loss creation
- Spending limits can interact with stop-loss

### **Implementation Files**

- `mondoshawan-blockchain/src/stop_loss/mod.rs` - Main module
- `mondoshawan-blockchain/src/stop_loss/manager.rs` - Order management
- `mondoshawan-blockchain/src/stop_loss/monitor.rs` - Price monitoring
- `mondoshawan-blockchain/src/stop_loss/tests.rs` - Unit tests

---

## 🔄 Integration Points

### **Oracles + Recurring Transactions**
- Recurring transactions can use oracle prices for dynamic amounts
- Example: "Send 0.1 BTC worth of MSHW every week" (uses BTC/USD oracle)

### **Oracles + Stop-Loss**
- Stop-loss orders require oracle price feeds
- Real-time price monitoring triggers execution

### **All Three Together**
- Recurring stop-loss orders: "Check stop-loss every hour"
- Conditional recurring: "Send if price > X"
- Dynamic amounts: "Send X% of balance when price hits Y"

---

## 📊 Implementation Phases

### **Phase 1: Oracles (Week 1-4)**
1. Oracle registry and staking
2. Basic price feed (single source)
3. Price feed aggregation (multiple sources)
4. VRF implementation
5. RPC methods
6. Tests

### **Phase 2: Recurring Transactions (Week 5-7)**
1. Recurring transaction structure
2. Scheduler implementation
3. Integration with transaction pool
4. RPC methods
5. Tests

### **Phase 3: Stop-Loss (Week 8-10)**
1. Stop-loss order structure
2. Price monitoring integration
3. Conditional execution
4. Integration with Account Abstraction
5. RPC methods
6. Tests

### **Phase 4: Integration & Testing (Week 11-12)**
1. End-to-end testing
2. Desktop app integration
3. Explorer integration
4. Documentation
5. Performance optimization

---

## 🎯 Success Criteria

### **Oracles**
- ✅ 3+ oracle nodes can register
- ✅ Price feeds update every 60 seconds
- ✅ Aggregation works (median of N sources)
- ✅ VRF generates verifiable randomness
- ✅ Slashing works for false data

### **Recurring Transactions**
- ✅ Daily/weekly/monthly schedules work
- ✅ Transactions execute automatically
- ✅ Cancellation works
- ✅ Execution history tracked

### **Stop-Loss**
- ✅ Orders trigger at correct price
- ✅ Transactions execute automatically
- ✅ Multiple orders per wallet
- ✅ Integration with Account Abstraction

---

## 📝 Next Steps

1. **Start with Oracles** (foundation for stop-loss)
2. **Implement Recurring Transactions** (independent feature)
3. **Build Stop-Loss** (uses oracles)
4. **Integrate all three** (desktop app, explorer, RPC)

---

**Ready to start implementation!** 🚀
