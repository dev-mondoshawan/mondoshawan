# Mondoshawan Finality UX Guide

**For**: Wallet Developers, Explorer Developers, End Users  
**Last Updated**: January 2026

---

## 🎯 Finality Overview

Mondoshawan uses **three-tier finality** based on which stream processes the transaction:

| Stream | Block Time | Finality Time | Use Case |
|-------|-----------|---------------|----------|
| **Stream C** | 100ms | **1 second** | Low-value, real-time |
| **Stream B** | 1s | **10 seconds** | Standard transactions |
| **Stream A** | 10s | **60 seconds** | High-value, critical |

---

## 🎨 Visual Design Standards

### Color Coding

**Pending** (Yellow/Orange):
- Hex: `#f59e0b` (amber-500)
- Status: "Pending"
- Icon: ⏳ Clock
- Message: "Awaiting confirmation"

**Confirmed** (Blue):
- Hex: `#3b82f6` (blue-500)
- Status: "Confirmed"
- Icon: ✓ Checkmark
- Message: "Confirmed (1s/10s finality)"

**Finalized** (Green):
- Hex: `#10b981` (green-500)
- Status: "Finalized"
- Icon: ✅ Double checkmark
- Message: "Finalized (60s finality)"

### Status Icons

```
Pending:    ⏳ (clock icon)
Confirmed: ✓  (single checkmark)
Finalized: ✅ (double checkmark with shield)
```

---

## 📱 Wallet Display Examples

### Example 1: Simple Display

```
┌─────────────────────────────────┐
│ Transaction Status              │
├─────────────────────────────────┤
│ ✅ Finalized                     │
│                                 │
│ Stream: Stream A                │
│ Finality: 60 seconds            │
│ Confirmations: 6/6              │
│                                 │
│ [████████████████] 100%         │
│                                 │
│ Safe for all transactions       │
└─────────────────────────────────┘
```

### Example 2: Detailed Display

```
┌─────────────────────────────────┐
│ Transaction Details             │
├─────────────────────────────────┤
│ Hash: 0x1234...5678             │
│ Status: ✅ Finalized             │
│                                 │
│ Finality Information:           │
│ • Stream: Stream A              │
│ • Block Time: 10 seconds        │
│ • Finality: 60 seconds           │
│ • Confirmations: 6/6            │
│                                 │
│ [████████████████] 100%         │
│                                 │
│ ⚠️ Recommendation:              │
│ Fully secure for all            │
│ transaction types               │
└─────────────────────────────────┘
```

### Example 3: Progress Indicator

```
┌─────────────────────────────────┐
│ Transaction Status              │
├─────────────────────────────────┤
│ ⏳ Pending → ✓ Confirmed → ✅ Finalized
│                                 │
│ Current: ✓ Confirmed            │
│ Progress: [████████░░░░] 66%     │
│                                 │
│ Time remaining: ~40 seconds      │
│                                 │
│ Safe for: Standard transactions │
└─────────────────────────────────┘
```

---

## 🌐 Explorer Display

### Transaction Page

```html
<div class="finality-status finalized">
    <div class="status-icon">✅</div>
    <div class="status-text">
        <h3>Finalized</h3>
        <p>Stream A - 60 seconds finality</p>
        <p class="safe-indicator">Safe for all transactions</p>
    </div>
    <div class="finality-details">
        <div class="detail">
            <span class="label">Stream:</span>
            <span class="value">Stream A</span>
        </div>
        <div class="detail">
            <span class="label">Confirmations:</span>
            <span class="value">6/6</span>
        </div>
        <div class="detail">
            <span class="label">Finality Time:</span>
            <span class="value">60 seconds</span>
        </div>
    </div>
</div>
```

### Transaction List

```html
<tr class="transaction-row">
    <td>0x1234...</td>
    <td>
        <span class="finality-badge confirmed">✓ Confirmed</span>
    </td>
    <td>Stream B</td>
    <td>10s</td>
    <td>Safe for standard</td>
</tr>
```

---

## 🔌 API Integration

### RPC Response Format

```json
{
    "jsonrpc": "2.0",
    "result": {
        "hash": "0x...",
        "status": "finalized",
        "finality": {
            "level": 3,
            "stream": "StreamA",
            "time": 60,
            "confirmations": 6,
            "required_confirmations": 6
        },
        "safe_for": "all_transactions",
        "recommendation": "Fully secure"
    }
}
```

### Finality Levels

```rust
enum FinalityLevel {
    Pending = 1,    // 0-1s (Stream C), 0-10s (Stream B), 0-60s (Stream A)
    Confirmed = 2,  // 1s+ (Stream C), 10s+ (Stream B)
    Finalized = 3,  // 60s+ (Stream A only)
}
```

---

## 📋 User Messaging

### Pending Transactions

**Message**: "Transaction submitted, awaiting confirmation"
**Action**: Wait for confirmation
**Time**: Varies by stream

### Confirmed Transactions (Stream C/B)

**Message**: "Confirmed (1s/10s finality) - Safe for low-value/standard transactions"
**Action**: Can proceed for appropriate transaction types
**Warning**: "For high-value transactions, wait for Stream A finalization (60s)"

### Finalized Transactions (Stream A)

**Message**: "Finalized (60s finality) - Safe for all transactions"
**Action**: Fully secure, no further action needed
**Confidence**: Highest security level

---

## 🎯 Best Practices

### For Wallet Developers

1. **Always Show Finality Status**
   - Don't just show "confirmed"
   - Show which stream and finality time

2. **Color Code Clearly**
   - Yellow = Pending
   - Blue = Confirmed
   - Green = Finalized

3. **Provide Recommendations**
   - "Safe for low-value" (Stream C/B)
   - "Safe for all" (Stream A)

4. **Show Progress**
   - Progress bar for finality
   - Time remaining indicator

### For End Users

1. **Low-Value Transactions** (< $100)
   - Stream C/B confirmation (1-10s) is sufficient

2. **Standard Transactions** ($100-$10,000)
   - Stream B confirmation (10s) recommended
   - Stream A finalization (60s) for extra security

3. **High-Value Transactions** (> $10,000)
   - Always wait for Stream A finalization (60s)
   - Don't proceed until "Finalized" status

---

## 🔧 Implementation Checklist

### Explorer
- [ ] Add finality status display
- [ ] Color code by finality level
- [ ] Show stream information
- [ ] Display confirmation count
- [ ] Add recommendations

### Wallet
- [ ] Integrate finality API
- [ ] Display three-tier status
- [ ] Color code appropriately
- [ ] Show progress indicators
- [ ] Provide user guidance

### RPC
- [ ] Add finality information to responses
- [ ] Include stream type
- [ ] Include confirmation count
- [ ] Include finality time
- [ ] Include safety recommendations

---

## 📝 Summary

**Three-Tier Finality**:
1. **Pending** (Yellow) - Awaiting confirmation
2. **Confirmed** (Blue) - 1s/10s finality (Stream C/B)
3. **Finalized** (Green) - 60s finality (Stream A)

**Key Principle**: Clear, color-coded, user-friendly finality display

---

**Ready for UX implementation!** 🎨
