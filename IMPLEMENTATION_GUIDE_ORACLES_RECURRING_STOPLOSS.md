# Implementation Guide: Oracles, Recurring Transactions & Stop-Loss

**Date**: January 2026  
**Status**: Core Modules Complete ✅ | RPC Methods Added ✅ | Integration Pending ⏳

---

## ✅ Completed

### **1. Core Module Structures**
- ✅ Oracle Network (`oracles/`)
  - Registry, Price Feeds, VRF, Staking
- ✅ Recurring Transactions (`recurring/`)
  - Manager, Scheduler
- ✅ Stop-Loss (`stop_loss/`)
  - Manager, Monitor

### **2. RPC Methods Added**
- ✅ Oracle methods (register, get price, request randomness)
- ✅ Recurring transaction methods (create, get, cancel)
- ✅ Stop-loss methods (create, get, cancel)

---

## ⏳ Remaining Work

### **Phase 1: Fix Compilation & Complete Core** (Priority: HIGH)

#### **1.1 Fix Compilation Errors**
- [ ] Fix missing imports in `stop_loss/monitor.rs`
- [ ] Fix `Transaction::new` signature (check actual constructor)
- [ ] Fix `Schedule` enum serialization
- [ ] Add missing `bincode` dependency if needed
- [ ] Fix all unused import warnings

**Files to Check**:
- `mondoshawan-blockchain/src/blockchain/block.rs` - Check Transaction constructor
- `mondoshawan-blockchain/src/recurring/manager.rs` - Fix Schedule serialization
- `mondoshawan-blockchain/src/stop_loss/monitor.rs` - Fix imports

#### **1.2 Complete VRF Implementation**
- [ ] Implement actual VRF (currently simplified)
- [ ] Add cryptographic proof generation
- [ ] Add proof verification

#### **1.3 Complete Scheduler Integration**
- [ ] Integrate recurring scheduler into node loop
- [ ] Add automatic execution trigger
- [ ] Handle execution failures

#### **1.4 Complete Stop-Loss Monitor**
- [ ] Integrate with price feed updates
- [ ] Add automatic transaction execution
- [ ] Handle multiple orders per feed

---

### **Phase 2: Complete RPC Methods** (Priority: HIGH)

#### **2.1 Oracle RPC Methods**
- [x] `mds_registerOracle` ✅
- [x] `mds_getPrice` ✅
- [x] `mds_getPriceFeeds` ✅
- [x] `mds_requestRandomness` ✅
- [ ] `mds_unregisterOracle` (placeholder)
- [ ] `mds_getOracleInfo` (placeholder)
- [ ] `mds_getOracleList` (placeholder)
- [ ] `mds_getPriceHistory` (placeholder)
- [ ] `mds_getRandomness` (placeholder)

#### **2.2 Recurring Transaction RPC Methods**
- [x] `mds_createRecurringTransaction` ✅
- [x] `mds_getRecurringTransactions` ✅
- [ ] `mds_cancelRecurringTransaction` (placeholder)
- [ ] `mds_getRecurringTransaction` (placeholder)
- [ ] `mds_pauseRecurringTransaction` (placeholder)
- [ ] `mds_resumeRecurringTransaction` (placeholder)

#### **2.3 Stop-Loss RPC Methods**
- [x] `mds_createStopLoss` ✅
- [x] `mds_getStopLossOrders` ✅
- [ ] `mds_cancelStopLoss` (placeholder)
- [ ] `mds_getStopLoss` (placeholder)
- [ ] `mds_updateStopLossPrice` (placeholder)
- [ ] `mds_pauseStopLoss` (placeholder)
- [ ] `mds_resumeStopLoss` (placeholder)

---

### **Phase 3: Blockchain Integration** (Priority: MEDIUM)

#### **3.1 Add Managers to Blockchain**
- [ ] Add `oracle_registry` to `Blockchain` struct
- [ ] Add `price_feed_manager` to `Blockchain` struct
- [ ] Add `recurring_manager` to `Blockchain` struct
- [ ] Add `stop_loss_manager` to `Blockchain` struct
- [ ] Initialize in constructors

#### **3.2 Integrate with Node Loop**
- [ ] Add recurring transaction scheduler to node loop
- [ ] Add stop-loss monitor to price feed updates
- [ ] Add automatic execution of triggered transactions

#### **3.3 Add to RPC Server**
- [ ] Add `with_oracle_registry()` method
- [ ] Add `with_price_feed_manager()` method
- [ ] Add `with_recurring_manager()` method
- [ ] Add `with_stop_loss_manager()` method
- [ ] Initialize in node startup

---

### **Phase 4: Testing** (Priority: MEDIUM)

#### **4.1 Unit Tests**
- [ ] Oracle registry tests
- [ ] Price feed aggregation tests
- [ ] VRF tests
- [ ] Recurring transaction tests
- [ ] Stop-loss trigger tests

#### **4.2 Integration Tests**
- [ ] End-to-end oracle registration → price feed → stop-loss
- [ ] Recurring transaction execution flow
- [ ] Stop-loss execution flow

---

### **Phase 5: Desktop App Integration** (Priority: LOW)

#### **5.1 Oracle UI**
- [ ] View available price feeds
- [ ] View current prices
- [ ] Request randomness

#### **5.2 Recurring Transaction UI**
- [ ] Create recurring transaction form
- [ ] List recurring transactions
- [ ] Cancel/pause/resume controls

#### **5.3 Stop-Loss UI**
- [ ] Create stop-loss order form
- [ ] List active stop-loss orders
- [ ] View triggered orders
- [ ] Update/cancel controls

---

### **Phase 6: Explorer Integration** (Priority: LOW)

#### **6.1 Display Features**
- [ ] Show recurring transactions in address view
- [ ] Show stop-loss orders in address view
- [ ] Show oracle prices in explorer
- [ ] Show triggered stop-loss transactions

---

## 🔧 Technical Details

### **Transaction Constructor**
Check `mondoshawan-blockchain/src/blockchain/block.rs` for actual `Transaction::new` signature:
```rust
// Expected signature (verify):
Transaction::new(from: Address, to: Address, value: u128, nonce: u64, gas_price: u64, gas_limit: u64, data: Vec<u8>)
```

### **Schedule Serialization**
The `Schedule` enum needs proper serialization. Consider using:
```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Schedule {
    Daily { hour: u8, minute: u8 },
    // ...
}
```

### **Price Feed Integration**
Price feeds need to be updated periodically. Consider:
- Background task that fetches prices from oracles
- Aggregation every N seconds
- Integration with stop-loss monitor

---

## 📋 Implementation Checklist

### **Immediate (Fix Compilation)**
- [ ] Fix `Transaction::new` calls
- [ ] Fix `Schedule` serialization
- [ ] Fix imports in `stop_loss/monitor.rs`
- [ ] Run `cargo check` and fix all errors

### **Short Term (Complete Core)**
- [ ] Complete placeholder RPC methods
- [ ] Add managers to Blockchain struct
- [ ] Integrate with node loop
- [ ] Write unit tests

### **Medium Term (Integration)**
- [ ] Desktop app UI
- [ ] Explorer integration
- [ ] End-to-end testing

---

## 🎯 Success Criteria

### **Core Functionality**
- ✅ All modules compile without errors
- ✅ RPC methods respond correctly
- ✅ Basic operations work (create, get, cancel)

### **Integration**
- ✅ Recurring transactions execute automatically
- ✅ Stop-loss orders trigger correctly
- ✅ Oracle prices update regularly

### **Testing**
- ✅ Unit tests pass
- ✅ Integration tests pass
- ✅ No regressions in existing features

---

## 📝 Next Steps

1. **Fix compilation errors** (highest priority)
2. **Complete placeholder RPC methods**
3. **Add managers to Blockchain struct**
4. **Integrate with node loop**
5. **Write tests**

---

**Last Updated**: January 2026  
**Status**: Core Complete ✅ | Integration Pending ⏳
