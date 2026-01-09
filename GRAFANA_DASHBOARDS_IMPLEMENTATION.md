# Grafana Dashboards Implementation

**Status:** ✅ **Complete**  
**Date:** December 2024

## Overview

Comprehensive Grafana dashboards have been created for monitoring Mondoshawan blockchain nodes. These dashboards provide real-time visualization of all key metrics including blocks, transactions, network, mining, and sharding.

## What Was Created

### 1. Dashboard Files

#### `grafana/dashboards/Mondoshawan-overview.json`
**Main Overview Dashboard**

Comprehensive overview showing:
- **Key Stats:**
  - Blocks mined (total)
  - Transactions processed
  - Peers connected
  - Transactions per second
- **Graphs:**
  - Blocks mined over time (mined vs received)
  - Transaction pool size
  - Block size distribution (p50, p95, p99)
  - Network message rates

#### `grafana/dashboards/Mondoshawan-mining.json`
**TriStream Mining Dashboard**

Mining-specific metrics:
- **Key Stats:**
  - Blocks mined per stream (A, B, C)
  - Total mining rewards (in tokens)
- **Graphs:**
  - Blocks mined by stream (rate)
  - Mining rewards over time
  - Stream distribution (pie chart)
  - Block mining rate (blocks/min)

#### `grafana/dashboards/Mondoshawan-sharding.json`
**Sharding Dashboard**

Sharding and scalability metrics:
- **Key Stats:**
  - Cross-shard transactions (total)
  - Cross-shard transaction rate
  - Total shards
  - Total shard transactions
- **Graphs:**
  - Transactions per shard
  - Cross-shard transaction rate
  - Shard load distribution (bar gauge)
  - Shard utilization percentage

#### `grafana/dashboards/Mondoshawan-network.json`
**Network Dashboard**

P2P network metrics:
- **Key Stats:**
  - Connected peers
  - Messages sent (total)
  - Messages received (total)
  - Message rate
- **Graphs:**
  - Network message rate (sent vs received)
  - Peer connection status
  - Network throughput
  - Message send/receive ratio

#### `grafana/dashboards/Mondoshawan-transactions.json`
**Transaction Dashboard**

Transaction processing metrics:
- **Key Stats:**
  - Transactions processed (total)
  - Transaction pool size
  - Transactions per second
  - Transaction processing rate
- **Graphs:**
  - Transaction processing rate
  - Transaction pool size over time (with alert)
  - Transactions per block
  - Throughput efficiency

### 2. Docker Compose Setup

#### `grafana/docker-compose.yml`
Complete Docker Compose configuration for:
- **Prometheus** (port 9090)
  - Scrapes metrics from Mondoshawan node
  - 30-day retention
  - Persistent storage
- **Grafana** (port 3000)
  - Pre-configured dashboards
  - Auto-provisioning
  - Persistent storage

### 3. Prometheus Configuration

#### `grafana/prometheus/prometheus.yml`
Prometheus scrape configuration:
- Scrapes from `host.docker.internal:8545` (Mondoshawan node)
- 10-second scrape interval
- 5-second timeout
- Labels for instance and environment

### 4. Grafana Provisioning

#### `grafana/provisioning/datasources/prometheus.yml`
Auto-configures Prometheus as default data source

#### `grafana/provisioning/dashboards/dashboard.yml`
Auto-loads dashboards from `/var/lib/grafana/dashboards`

### 5. Documentation

#### `grafana/README.md`
Comprehensive guide covering:
- Quick start (Docker and manual)
- Dashboard descriptions
- Metrics reference
- Troubleshooting
- Customization guide
- Alerting examples

## Dashboard Features

### Visualizations

Each dashboard includes:
- **Stat Panels:** Key metrics with color-coded thresholds
- **Time Series Graphs:** Historical trends
- **Bar Gauges:** Load distribution
- **Pie Charts:** Stream distribution

### Color Coding

- **Green:** Normal/healthy state
- **Yellow:** Warning threshold
- **Orange/Red:** Critical threshold

### Refresh Intervals

- Default: 10 seconds
- Configurable: 5s, 10s, 30s, 1m, 5m, 15m, 30m, 1h, 2h, 1d

### Time Ranges

- Default: Last 1 hour
- Quick ranges: 5m, 15m, 30m, 1h, 3h, 6h, 12h, 24h, 7d, 30d

## Setup Instructions

### Quick Start (Docker)

```bash
cd grafana
docker-compose up -d
```

Then:
1. Open http://localhost:3000
2. Login: `admin` / `admin`
3. Dashboards are automatically loaded!

### Manual Setup

1. **Install Prometheus and Grafana**
2. **Configure Prometheus** to scrape `http://localhost:8545/metrics`
3. **Import dashboards** from `grafana/dashboards/` directory
4. **Configure Grafana** data source to point to Prometheus

## Metrics Coverage

### Block Metrics ✅
- Blocks mined (total, by stream)
- Blocks received
- Block size distribution

### Transaction Metrics ✅
- Transactions processed
- Transaction pool size
- Transactions per second
- Processing rate

### Network Metrics ✅
- Peers connected
- Messages sent/received
- Message rates
- Network throughput

### Mining Metrics ✅
- Blocks per stream (A, B, C)
- Mining rewards
- Mining rates
- Stream distribution

### Sharding Metrics ✅
- Shard transaction counts
- Cross-shard transactions
- Shard utilization
- Load distribution

## Alerting Examples

The dashboards support Grafana alerts. Example alert rules:

1. **Transaction Pool Size Alert:**
   - Condition: Pool size > 100,000
   - Duration: 5 minutes
   - Severity: Warning

2. **No Peers Alert:**
   - Condition: Peers connected = 0
   - Duration: 1 minute
   - Severity: Critical

3. **Low TPS Alert:**
   - Condition: TPS < 1
   - Duration: 5 minutes
   - Severity: Warning

4. **No Blocks Alert:**
   - Condition: Block mining rate = 0
   - Duration: 10 minutes
   - Severity: Critical

## Customization

### Adding Custom Panels

1. Open dashboard in Grafana
2. Click "Add Panel"
3. Use Prometheus queries:
   ```promql
   # Example queries
   Mondoshawan_blocks_mined_total
   rate(Mondoshawan_transactions_processed_total[5m])
   histogram_quantile(0.95, rate(Mondoshawan_block_size_bytes_bucket[5m]))
   ```

### Modifying Thresholds

Edit the `thresholds` section in panel JSON:
```json
"thresholds": {
  "mode": "absolute",
  "steps": [
    {"value": 0, "color": "green"},
    {"value": 1000, "color": "yellow"},
    {"value": 10000, "color": "red"}
  ]
}
```

### Changing Refresh Intervals

Edit dashboard JSON:
```json
"refresh": "30s"  // Change to desired interval
```

## Integration with Mondoshawan Node

The dashboards automatically work with Mondoshawan nodes that have metrics enabled:

1. **Metrics are exposed** at `http://localhost:8545/metrics`
2. **Prometheus scrapes** from this endpoint
3. **Grafana queries** Prometheus for visualization

## File Structure

```
grafana/
├── dashboards/
│   ├── Mondoshawan-overview.json
│   ├── Mondoshawan-mining.json
│   ├── Mondoshawan-sharding.json
│   ├── Mondoshawan-network.json
│   └── Mondoshawan-transactions.json
├── prometheus/
│   └── prometheus.yml
├── provisioning/
│   ├── datasources/
│   │   └── prometheus.yml
│   └── dashboards/
│       └── dashboard.yml
├── docker-compose.yml
└── README.md
```

## Usage Examples

### Viewing Mining Performance

1. Open "Mondoshawan Mining Metrics" dashboard
2. Check Stream A/B/C block counts
3. Monitor mining rewards over time
4. View stream distribution

### Monitoring Sharding

1. Open "Mondoshawan Sharding Metrics" dashboard
2. Check cross-shard transaction rate
3. Monitor shard load distribution
4. View shard utilization

### Network Health

1. Open "Mondoshawan Network Metrics" dashboard
2. Check peer count
3. Monitor message rates
4. View network throughput

## Troubleshooting

### Common Issues

1. **No data in Grafana:**
   - Verify Prometheus is scraping: http://localhost:9090/targets
   - Check metrics endpoint: `curl http://localhost:8545/metrics`
   - Verify time range in Grafana

2. **Docker network issues:**
   - Use `host.docker.internal:8545` for Windows/Mac
   - For Linux, use `host.docker.internal` or `network_mode: host`

3. **Dashboards not loading:**
   - Check Grafana logs: `docker-compose logs grafana`
   - Verify dashboard files are in correct directory
   - Check provisioning configuration

## Future Enhancements

1. **Additional Dashboards:**
   - Security metrics (risk scores, anomalies)
   - Fairness/MEV metrics
   - RPC performance metrics
   - EVM execution metrics

2. **Alerting Rules:**
   - Pre-configured alert rules
   - Notification channels (email, Slack, PagerDuty)

3. **Custom Queries:**
   - Advanced Prometheus queries
   - Derived metrics
   - Composite visualizations

4. **Export/Import:**
   - Dashboard templates
   - Snapshot sharing
   - Version control

## Summary

Grafana dashboards are now fully configured and ready to use:
- ✅ 5 comprehensive dashboards
- ✅ Docker Compose setup
- ✅ Auto-provisioning
- ✅ Complete documentation
- ✅ Metrics coverage for all key areas
- ✅ Color-coded thresholds
- ✅ Real-time updates
- ✅ Customizable and extensible

This provides production-grade monitoring capabilities for Mondoshawan blockchain nodes, enabling operators to track performance, identify issues, and optimize operations.
