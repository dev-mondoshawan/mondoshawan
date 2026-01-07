# Mondoshawan Grafana Dashboards

This directory contains Grafana dashboard configurations for monitoring the Mondoshawan blockchain node.

## Quick Start

### Option 1: Docker Compose (Recommended)

1. **Start the monitoring stack:**
   ```bash
   cd grafana
   docker-compose up -d
   ```

2. **Access Grafana:**
   - Open http://localhost:3001 (Note: Port 3001 is used if 3000 is already in use)
   - Login with: `admin` / `admin`
   - Dashboards will be automatically loaded

3. **Access Prometheus:**
   - Open http://localhost:9090
   - Verify that `Mondoshawan-node` target is UP

### Option 2: Manual Setup

#### Prerequisites
- Prometheus installed and running
- Grafana installed and running

#### Setup Prometheus

1. **Copy `prometheus/prometheus.yml` to your Prometheus config directory**

2. **Update the target URL** in `prometheus.yml`:
   ```yaml
   - targets: ['localhost:8545']  # Change to your node's RPC port
   ```

3. **Start Prometheus:**
   ```bash
   prometheus --config.file=prometheus.yml
   ```

#### Setup Grafana

1. **Add Prometheus as a data source:**
   - Go to Configuration → Data Sources
   - Add Prometheus
   - URL: `http://localhost:9090`
   - Save & Test

2. **Import Dashboards:**
   - Go to Dashboards → Import
   - Upload each JSON file from `dashboards/` directory:
     - `Mondoshawan-overview.json`
     - `Mondoshawan-mining.json`
     - `Mondoshawan-sharding.json`
     - `Mondoshawan-network.json`
     - `Mondoshawan-transactions.json`

## Available Dashboards

### 1. Mondoshawan Blockchain Overview
**File:** `dashboards/Mondoshawan-overview.json`

Comprehensive overview dashboard showing:
- Total blocks mined
- Transactions processed
- Peers connected
- Transactions per second
- Block mining rate
- Transaction pool size
- Block size distribution
- Network message rates

### 2. Mondoshawan Mining Metrics
**File:** `dashboards/Mondoshawan-mining.json`

TriStream mining specific metrics:
- Blocks mined per stream (A, B, C)
- Total mining rewards
- Mining rate by stream
- Rewards over time
- Stream distribution (pie chart)
- Block mining rate

### 3. Mondoshawan Sharding Metrics
**File:** `dashboards/Mondoshawan-sharding.json`

Sharding and scalability metrics:
- Cross-shard transaction count
- Cross-shard transaction rate
- Total shards
- Transactions per shard
- Shard load distribution
- Shard utilization

### 4. Mondoshawan Network Metrics
**File:** `dashboards/Mondoshawan-network.json`

P2P network metrics:
- Connected peers
- Messages sent/received totals
- Message rate
- Network throughput
- Peer connection status
- Message send/receive ratio

### 5. Mondoshawan Transaction Metrics
**File:** `dashboards/Mondoshawan-transactions.json`

Transaction processing metrics:
- Total transactions processed
- Transaction pool size
- Transactions per second
- Transaction processing rate
- Transactions per block
- Throughput efficiency

## Metrics Endpoint

The Mondoshawan node exposes metrics at:
```
http://localhost:8545/metrics
```

Make sure your Prometheus configuration points to this endpoint.

## Customization

### Adding New Panels

1. Open Grafana
2. Edit the dashboard
3. Add new panel
4. Use Prometheus queries like:
   ```promql
   Mondoshawan_blocks_mined_total
   rate(Mondoshawan_transactions_processed_total[5m])
   ```

### Modifying Refresh Intervals

Edit the `refresh` field in each dashboard JSON:
```json
"refresh": "10s"  // Change to desired interval
```

### Time Range

Edit the `time` field in each dashboard JSON:
```json
"time": {
  "from": "now-1h",  // Change default time range
  "to": "now"
}
```

## Troubleshooting

### Prometheus Can't Scrape Metrics

1. **Check if metrics endpoint is accessible:**
   ```bash
   curl http://localhost:8545/metrics
   ```

2. **Verify Prometheus target status:**
   - Go to http://localhost:9090/targets
   - Check if `Mondoshawan-node` is UP

3. **Check firewall/network:**
   - Ensure port 8545 is accessible
   - For Docker, use `host.docker.internal` instead of `localhost`

### Grafana Shows "No Data"

1. **Verify data source connection:**
   - Go to Configuration → Data Sources
   - Test the Prometheus connection

2. **Check time range:**
   - Ensure you're viewing a time range where data exists
   - Try "Last 5 minutes" or "Last 1 hour"

3. **Verify metrics are being collected:**
   - Go to Prometheus → Graph
   - Query: `Mondoshawan_blocks_mined_total`
   - Should return a value if metrics are working

### Docker Network Issues

If using Docker and metrics aren't accessible:

1. **For Windows/Mac:**
   - Use `host.docker.internal:8545` in Prometheus config

2. **For Linux:**
   - Use `host.docker.internal:8545` or add `network_mode: host` to docker-compose

## Metrics Reference

### Block Metrics
- `Mondoshawan_blocks_mined_total` - Total blocks mined
- `Mondoshawan_blocks_received_total` - Total blocks received from network
- `Mondoshawan_block_size_bytes` - Block size histogram

### Transaction Metrics
- `Mondoshawan_transactions_processed_total` - Total transactions processed
- `Mondoshawan_transaction_pool_size` - Current transaction pool size
- `Mondoshawan_transactions_per_second` - Current TPS

### Network Metrics
- `Mondoshawan_peers_connected` - Number of connected peers
- `Mondoshawan_messages_sent_total` - Total messages sent
- `Mondoshawan_messages_received_total` - Total messages received

### Mining Metrics
- `Mondoshawan_blocks_mined_stream_a_total` - Stream A blocks
- `Mondoshawan_blocks_mined_stream_b_total` - Stream B blocks
- `Mondoshawan_blocks_mined_stream_c_total` - Stream C blocks
- `Mondoshawan_mining_rewards_total` - Total mining rewards (in smallest unit)

### Sharding Metrics
- `Mondoshawan_shard_transaction_count{shard_id="X"}` - Transactions in shard X
- `Mondoshawan_cross_shard_transactions_total` - Total cross-shard transactions

## Advanced Usage

### Alerting

You can set up alerts in Grafana based on these metrics:

**Example Alert Rules:**
- Transaction pool size > 100,000
- Peers connected = 0
- TPS < 1 for 5 minutes
- Block mining rate = 0 for 10 minutes

### Exporting Dashboards

To export a dashboard:
1. Open the dashboard in Grafana
2. Click the gear icon (Settings)
3. Click "JSON Model"
4. Copy the JSON
5. Save to a file

### Sharing Dashboards

Dashboards can be shared via:
- Grafana Cloud
- Export as JSON
- Grafana snapshot URLs

## Support

For issues or questions:
1. Check the Mondoshawan documentation
2. Review Prometheus logs: `docker-compose logs prometheus`
3. Review Grafana logs: `docker-compose logs grafana`
