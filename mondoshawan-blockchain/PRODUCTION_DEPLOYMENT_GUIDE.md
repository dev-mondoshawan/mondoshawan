# Mondoshawan Production Deployment Guide

**Version**: 1.0  
**Last Updated**: 2026-01-08  
**Target Audience**: DevOps Engineers, Node Operators  
**Prerequisites**: Linux system administration, basic blockchain knowledge

---

## Table of Contents

1. [Overview](#overview)
2. [Infrastructure Requirements](#infrastructure-requirements)
3. [Pre-Deployment Checklist](#pre-deployment-checklist)
4. [Node Setup (Single Node)](#node-setup-single-node)
5. [Multi-Node Network Deployment](#multi-node-network-deployment)
6. [Security Hardening](#security-hardening)
7. [Monitoring & Observability](#monitoring--observability)
8. [Operational Runbooks](#operational-runbooks)
9. [Troubleshooting](#troubleshooting)
10. [Appendix](#appendix)

---

## Overview

This guide covers production deployment of Mondoshawan blockchain nodes for:
- **Testnet**: Public test network for validation
- **Mainnet**: Production network (future)

### Deployment Options

| Option | Use Case | Nodes | Complexity |
|--------|----------|-------|------------|
| **Single Node** | Development, testing | 1 | Low |
| **Small Testnet** | Validation, demos | 3-10 | Medium |
| **Production Network** | Public deployment | 10+ | High |

---

## Infrastructure Requirements

### Minimum Specifications (Per Node)

| Component | Minimum | Recommended | Notes |
|-----------|---------|-------------|-------|
| **CPU** | 4 cores | 8+ cores | Mining benefits from more cores |
| **RAM** | 8 GB | 16 GB | More for high transaction volumes |
| **Storage** | 100 GB SSD | 500 GB NVMe | Blockchain data grows over time |
| **Network** | 100 Mbps | 1 Gbps symmetric | P2P and RPC traffic |
| **OS** | Ubuntu 20.04+ | Ubuntu 22.04 LTS | Also tested on Debian, CentOS |

### Cloud Provider Recommendations

#### **AWS EC2**
- **Type**: `t3.xlarge` or `c5.2xlarge` (CPU-optimized)
- **Storage**: EBS GP3 (500 GB, 3000 IOPS)
- **Region**: Multi-region for distributed testnet
- **Cost**: ~$150-200/month per node

#### **DigitalOcean**
- **Type**: CPU-Optimized Droplet (8 vCPU, 16 GB RAM)
- **Storage**: Block Storage (500 GB SSD)
- **Datacenter**: Multiple regions for diversity
- **Cost**: ~$96/month per node

#### **Hetzner**
- **Type**: CX41 or CCX23 (Dedicated CPU)
- **Storage**: Local SSD or Volume Storage
- **Location**: EU/US datacenters
- **Cost**: ~€30-40/month per node (most cost-effective)

### Network Topology

**Recommended Setup** (10-node testnet):
```
Region 1 (US-East):   3 nodes  (AWS us-east-1)
Region 2 (US-West):   3 nodes  (AWS us-west-2)
Region 3 (EU):        2 nodes  (Hetzner Germany)
Region 4 (Asia):      2 nodes  (AWS ap-southeast-1)

Topology: Full mesh (current) or Gossip (future)
Latency: 10-100ms inter-region
```

---

## Pre-Deployment Checklist

### Build Environment Setup

#### 1. Install Rust Toolchain
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup update stable
rustc --version  # Should be 1.75+
```

#### 2. Install System Dependencies
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev git

# CentOS/RHEL
sudo yum groupinstall -y "Development Tools"
sudo yum install -y openssl-devel git

# Verify
gcc --version
git --version
```

#### 3. Clone Repository
```bash
cd /opt
sudo git clone https://github.com/mondoshawan/blockchain.git
cd blockchain
```

#### 4. Build Release Binary
```bash
cargo build --release --bin node

# Verify binary
ls -lh target/release/node
# Should be ~40-60 MB

# Test binary
./target/release/node --version
```

**Expected Build Time**: 5-10 minutes (first build)  
**Binary Location**: `target/release/node`

---

## Node Setup (Single Node)

### Step 1: Create Service User

```bash
# Create dedicated user (no login)
sudo useradd -r -s /bin/false -m -d /var/lib/mondoshawan mondoshawan

# Create directories
sudo mkdir -p /var/lib/mondoshawan/{data,logs,config}
sudo mkdir -p /etc/mondoshawan

# Set permissions
sudo chown -R mondoshawan:mondoshawan /var/lib/mondoshawan
```

### Step 2: Install Binary

```bash
# Copy binary to system location
sudo cp target/release/node /usr/local/bin/mondoshawan-node
sudo chmod +x /usr/local/bin/mondoshawan-node
sudo chown mondoshawan:mondoshawan /usr/local/bin/mondoshawan-node

# Verify
/usr/local/bin/mondoshawan-node --version
```

### Step 3: Create Configuration

Create `/etc/mondoshawan/node.toml`:

```toml
# Mondoshawan Node Configuration

[node]
# Network ports
p2p_port = 8080      # P2P network
rpc_port = 8545      # JSON-RPC API

# Data directory
data_dir = "/var/lib/mondoshawan/data"

# Mining configuration
enable_mining = true
miner_address = "0x0000000000000000000000000000000000000001"

[network]
# Bootstrap nodes (replace with actual testnet nodes)
bootstrap_nodes = [
    "127.0.0.1:8080",
    "127.0.0.1:8081",
    "127.0.0.1:8082"
]

# External IP (for P2P advertising)
external_ip = "AUTO"  # Auto-detect or set manually

# Max peers
max_peers = 50

[features]
# Feature flags
enable_sharding = false
enable_verkle = false
enable_pqc = false

# Sharding config (if enabled)
shard_count = 10

[rpc]
# RPC settings
enable_rpc = true
rpc_host = "0.0.0.0"  # Listen on all interfaces
rpc_port = 8545

# Enable CORS for web access
enable_cors = true
cors_origins = ["*"]

# Rate limiting
rate_limit_enabled = true
rate_limit_requests_per_second = 100

[mining]
# Mining settings
mining_threads = 4  # CPU cores to use

[logging]
# Log level: trace, debug, info, warn, error
level = "info"
log_file = "/var/lib/mondoshawan/logs/node.log"
```

**Set Permissions**:
```bash
sudo chown mondoshawan:mondoshawan /etc/mondoshawan/node.toml
sudo chmod 600 /etc/mondoshawan/node.toml
```

### Step 4: Create Systemd Service

Create `/etc/systemd/system/mondoshawan.service`:

```ini
[Unit]
Description=Mondoshawan Blockchain Node
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=mondoshawan
Group=mondoshawan

# Environment
Environment="RUST_LOG=info"

# Working directory
WorkingDirectory=/var/lib/mondoshawan

# Start command
ExecStart=/usr/local/bin/mondoshawan-node \
    --config /etc/mondoshawan/node.toml \
    --data-dir /var/lib/mondoshawan/data

# Restart policy
Restart=always
RestartSec=10

# Resource limits
LimitNOFILE=65536
LimitNPROC=4096

# Security
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/mondoshawan

# Logging
StandardOutput=journal
StandardError=journal
SyslogIdentifier=mondoshawan

[Install]
WantedBy=multi-user.target
```

### Step 5: Start Node

```bash
# Reload systemd
sudo systemctl daemon-reload

# Enable service (start on boot)
sudo systemctl enable mondoshawan

# Start service
sudo systemctl start mondoshawan

# Check status
sudo systemctl status mondoshawan

# View logs
sudo journalctl -u mondoshawan -f
```

### Step 6: Verify Node is Running

```bash
# Check if process is running
ps aux | grep mondoshawan-node

# Check if ports are listening
sudo netstat -tlnp | grep -E '8080|8545'

# Test RPC
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'

# Expected output: {"jsonrpc":"2.0","result":"0x1","id":1}
```

---

## Multi-Node Network Deployment

### Deployment Strategy

**For 10-Node Testnet**:

1. **Sequential Deployment**: Deploy nodes one by one
2. **Parallel Deployment**: Use automation (Ansible/Terraform)
3. **Staged Rollout**: Deploy in batches (3 → 6 → 10 nodes)

### Bootstrap Node Selection

**First 3 Nodes** = Bootstrap nodes (publicly advertised)

Example bootstrap configuration:
```toml
bootstrap_nodes = [
    "node1.testnet.mondoshawan.io:8080",
    "node2.testnet.mondoshawan.io:8080",
    "node3.testnet.mondoshawan.io:8080"
]
```

### Automated Deployment with Ansible

#### Ansible Inventory (`hosts.ini`)

```ini
[bootstrap]
node1 ansible_host=1.2.3.4 ansible_user=ubuntu p2p_port=8080 rpc_port=8545
node2 ansible_host=5.6.7.8 ansible_user=ubuntu p2p_port=8080 rpc_port=8545
node3 ansible_host=9.10.11.12 ansible_user=ubuntu p2p_port=8080 rpc_port=8545

[peers]
node4 ansible_host=13.14.15.16 ansible_user=ubuntu p2p_port=8080 rpc_port=8545
node5 ansible_host=17.18.19.20 ansible_user=ubuntu p2p_port=8080 rpc_port=8545
# ... nodes 6-10

[all:vars]
ansible_python_interpreter=/usr/bin/python3
```

#### Ansible Playbook (`deploy-node.yml`)

```yaml
---
- name: Deploy Mondoshawan Node
  hosts: all
  become: yes
  vars:
    mondoshawan_version: "0.1.0"
    data_dir: "/var/lib/mondoshawan/data"
    config_dir: "/etc/mondoshawan"

  tasks:
    - name: Install dependencies
      apt:
        name:
          - build-essential
          - pkg-config
          - libssl-dev
          - git
        state: present
        update_cache: yes

    - name: Create mondoshawan user
      user:
        name: mondoshawan
        system: yes
        shell: /bin/false
        home: /var/lib/mondoshawan
        create_home: yes

    - name: Create directories
      file:
        path: "{{ item }}"
        state: directory
        owner: mondoshawan
        group: mondoshawan
        mode: '0755'
      loop:
        - "{{ data_dir }}"
        - /var/lib/mondoshawan/logs
        - "{{ config_dir }}"

    - name: Copy binary
      copy:
        src: ../target/release/node
        dest: /usr/local/bin/mondoshawan-node
        owner: mondoshawan
        group: mondoshawan
        mode: '0755'

    - name: Copy configuration
      template:
        src: node.toml.j2
        dest: "{{ config_dir }}/node.toml"
        owner: mondoshawan
        group: mondoshawan
        mode: '0600'

    - name: Copy systemd service
      template:
        src: mondoshawan.service.j2
        dest: /etc/systemd/system/mondoshawan.service
        mode: '0644'
      notify: Reload systemd

    - name: Enable and start service
      systemd:
        name: mondoshawan
        enabled: yes
        state: started

  handlers:
    - name: Reload systemd
      systemd:
        daemon_reload: yes
```

#### Run Deployment

```bash
# Deploy to all nodes
ansible-playbook -i hosts.ini deploy-node.yml

# Deploy to specific group
ansible-playbook -i hosts.ini deploy-node.yml --limit bootstrap

# Check status across all nodes
ansible all -i hosts.ini -m shell -a "systemctl status mondoshawan" -b
```

### Docker Deployment

#### Dockerfile

```dockerfile
FROM rust:1.75-slim as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source
WORKDIR /build
COPY . .

# Build release binary
RUN cargo build --release --bin node

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create user
RUN useradd -r -s /bin/false -m -d /var/lib/mondoshawan mondoshawan

# Copy binary
COPY --from=builder /build/target/release/node /usr/local/bin/mondoshawan-node

# Set permissions
RUN chown mondoshawan:mondoshawan /usr/local/bin/mondoshawan-node && \
    chmod +x /usr/local/bin/mondoshawan-node

# Data directory
VOLUME ["/var/lib/mondoshawan/data"]

# Expose ports
EXPOSE 8080 8545

# Run as non-root
USER mondoshawan

# Start node
ENTRYPOINT ["/usr/local/bin/mondoshawan-node"]
CMD ["--config", "/etc/mondoshawan/node.toml"]
```

#### Docker Compose (`docker-compose.yml`)

```yaml
version: '3.8'

services:
  node:
    build: .
    image: mondoshawan-node:latest
    container_name: mondoshawan-node
    restart: unless-stopped
    ports:
      - "8080:8080"  # P2P
      - "8545:8545"  # RPC
    volumes:
      - ./data:/var/lib/mondoshawan/data
      - ./config/node.toml:/etc/mondoshawan/node.toml:ro
    environment:
      - RUST_LOG=info
    logging:
      driver: "json-file"
      options:
        max-size: "100m"
        max-file: "5"
```

#### Run with Docker

```bash
# Build image
docker-compose build

# Start node
docker-compose up -d

# View logs
docker-compose logs -f

# Stop node
docker-compose down
```

---

## Security Hardening

### Firewall Configuration (ufw)

```bash
# Install ufw
sudo apt install -y ufw

# Default policies
sudo ufw default deny incoming
sudo ufw default allow outgoing

# SSH access (replace 22 with your SSH port)
sudo ufw allow 22/tcp

# P2P port (required for blockchain operation)
sudo ufw allow 8080/tcp

# RPC port (restrict to trusted IPs or VPN)
# Option 1: Public RPC (NOT recommended for production)
# sudo ufw allow 8545/tcp

# Option 2: Restrict to specific IPs (recommended)
sudo ufw allow from 10.0.0.0/8 to any port 8545 proto tcp

# Option 3: Only localhost (most secure, use reverse proxy)
# RPC will only be accessible locally

# Enable firewall
sudo ufw enable

# Check status
sudo ufw status verbose
```

### SSH Hardening

```bash
# Edit SSH config
sudo nano /etc/ssh/sshd_config

# Recommended settings:
# PermitRootLogin no
# PasswordAuthentication no
# PubkeyAuthentication yes
# Port 2222  # Change from default 22

# Restart SSH
sudo systemctl restart sshd
```

### Nginx Reverse Proxy (for RPC)

```bash
# Install Nginx
sudo apt install -y nginx certbot python3-certbot-nginx

# Create config
sudo nano /etc/nginx/sites-available/mondoshawan-rpc

```

**Nginx Configuration**:

```nginx
server {
    listen 80;
    server_name rpc.testnet.mondoshawan.io;

    # Redirect HTTP to HTTPS
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name rpc.testnet.mondoshawan.io;

    # SSL certificates (managed by certbot)
    ssl_certificate /etc/letsencrypt/live/rpc.testnet.mondoshawan.io/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/rpc.testnet.mondoshawan.io/privkey.pem;

    # SSL security
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;

    # Rate limiting
    limit_req_zone $binary_remote_addr zone=rpc_limit:10m rate=10r/s;
    limit_req zone=rpc_limit burst=20 nodelay;

    # Proxy to local RPC
    location / {
        proxy_pass http://127.0.0.1:8545;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Timeouts
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;

        # CORS headers
        add_header Access-Control-Allow-Origin "*";
        add_header Access-Control-Allow-Methods "GET, POST, OPTIONS";
        add_header Access-Control-Allow-Headers "Content-Type";

        # Handle OPTIONS for CORS preflight
        if ($request_method = 'OPTIONS') {
            return 204;
        }
    }

    # Health check endpoint
    location /health {
        access_log off;
        return 200 "OK\n";
        add_header Content-Type text/plain;
    }
}
```

**Enable and Get SSL Certificate**:

```bash
# Enable site
sudo ln -s /etc/nginx/sites-available/mondoshawan-rpc /etc/nginx/sites-enabled/

# Test config
sudo nginx -t

# Get SSL certificate
sudo certbot --nginx -d rpc.testnet.mondoshawan.io

# Reload Nginx
sudo systemctl reload nginx
```

### Automated Security Updates

```bash
# Install unattended-upgrades
sudo apt install -y unattended-upgrades

# Enable automatic security updates
sudo dpkg-reconfigure --priority=low unattended-upgrades

# Configure
sudo nano /etc/apt/apt.conf.d/50unattended-upgrades

# Set to automatically reboot if needed
# Unattended-Upgrade::Automatic-Reboot "true";
# Unattended-Upgrade::Automatic-Reboot-Time "03:00";
```

---

## Monitoring & Observability

### Prometheus + Grafana Setup

#### Install Prometheus

```bash
# Create prometheus user
sudo useradd -r -s /bin/false prometheus

# Download Prometheus
cd /tmp
wget https://github.com/prometheus/prometheus/releases/download/v2.45.0/prometheus-2.45.0.linux-amd64.tar.gz
tar xvf prometheus-2.45.0.linux-amd64.tar.gz

# Install
sudo mv prometheus-2.45.0.linux-amd64/prometheus /usr/local/bin/
sudo mv prometheus-2.45.0.linux-amd64/promtool /usr/local/bin/
sudo mkdir -p /etc/prometheus /var/lib/prometheus
sudo mv prometheus-2.45.0.linux-amd64/consoles /etc/prometheus/
sudo mv prometheus-2.45.0.linux-amd64/console_libraries /etc/prometheus/

# Set permissions
sudo chown -R prometheus:prometheus /etc/prometheus /var/lib/prometheus
```

#### Configure Prometheus

Create `/etc/prometheus/prometheus.yml`:

```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'mondoshawan-node'
    static_configs:
      - targets: ['localhost:8545']
        labels:
          instance: 'node1'
          region: 'us-east-1'

  # Add more nodes
  - job_name: 'mondoshawan-network'
    static_configs:
      - targets:
          - 'node2.testnet.mondoshawan.io:8545'
          - 'node3.testnet.mondoshawan.io:8545'
          # ... more nodes
```

#### Prometheus Systemd Service

Create `/etc/systemd/system/prometheus.service`:

```ini
[Unit]
Description=Prometheus
After=network.target

[Service]
User=prometheus
Group=prometheus
Type=simple
ExecStart=/usr/local/bin/prometheus \
    --config.file /etc/prometheus/prometheus.yml \
    --storage.tsdb.path /var/lib/prometheus/ \
    --web.console.templates=/etc/prometheus/consoles \
    --web.console.libraries=/etc/prometheus/console_libraries

[Install]
WantedBy=multi-user.target
```

```bash
# Start Prometheus
sudo systemctl daemon-reload
sudo systemctl enable prometheus
sudo systemctl start prometheus

# Check status
sudo systemctl status prometheus

# Access UI: http://localhost:9090
```

#### Install Grafana

```bash
# Add Grafana repository
sudo apt-get install -y software-properties-common
sudo add-apt-repository "deb https://packages.grafana.com/oss/deb stable main"
wget -q -O - https://packages.grafana.com/gpg.key | sudo apt-key add -

# Install
sudo apt-get update
sudo apt-get install -y grafana

# Start service
sudo systemctl enable grafana-server
sudo systemctl start grafana-server

# Access UI: http://localhost:3000 (admin/admin)
```

#### Import Grafana Dashboards

1. Login to Grafana (http://localhost:3000)
2. Go to Configuration → Data Sources
3. Add Prometheus data source (http://localhost:9090)
4. Go to Dashboards → Import
5. Upload dashboard JSON from `grafana/dashboards/` directory

**Available Dashboards**:
- `mondoshawan-overview.json` - Network overview
- `mondoshawan-mining.json` - Mining metrics
- `mondoshawan-sharding.json` - Shard statistics
- `mondoshawan-network.json` - P2P metrics

### Logging with Loki (Optional)

```bash
# Install Loki
cd /tmp
wget https://github.com/grafana/loki/releases/download/v2.8.0/loki-linux-amd64.zip
unzip loki-linux-amd64.zip
sudo mv loki-linux-amd64 /usr/local/bin/loki

# Install Promtail (log shipper)
wget https://github.com/grafana/loki/releases/download/v2.8.0/promtail-linux-amd64.zip
unzip promtail-linux-amd64.zip
sudo mv promtail-linux-amd64 /usr/local/bin/promtail

# Configure (see Loki documentation)
# Add Loki as data source in Grafana
```

---

## Operational Runbooks

### Starting a Node

```bash
# Start service
sudo systemctl start mondoshawan

# Watch logs for startup
sudo journalctl -u mondoshawan -f

# Verify node is syncing
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"net_peerCount","params":[],"id":1}'

# Expected: {"jsonrpc":"2.0","result":"0x3","id":1} (3 peers)
```

### Stopping a Node

```bash
# Graceful stop
sudo systemctl stop mondoshawan

# Force stop (if hung)
sudo systemctl kill -s SIGKILL mondoshawan

# Verify stopped
sudo systemctl status mondoshawan
```

### Restarting a Node

```bash
# Restart service
sudo systemctl restart mondoshawan

# Watch for successful restart
sudo journalctl -u mondoshawan -f

# Verify sync resumed
# (Check block height increases)
```

### Upgrading a Node

```bash
# Stop node
sudo systemctl stop mondoshawan

# Backup data (optional but recommended)
sudo cp -r /var/lib/mondoshawan/data /var/lib/mondoshawan/data.backup.$(date +%Y%m%d)

# Download new binary
cd /opt/blockchain
sudo git pull
cargo build --release --bin node

# Replace binary
sudo cp target/release/node /usr/local/bin/mondoshawan-node

# Start node
sudo systemctl start mondoshawan

# Verify upgrade
/usr/local/bin/mondoshawan-node --version
sudo journalctl -u mondoshawan -f
```

### Backing Up Node Data

```bash
# Stop node (recommended)
sudo systemctl stop mondoshawan

# Backup blockchain data
sudo tar -czf mondoshawan-backup-$(date +%Y%m%d).tar.gz \
  -C /var/lib/mondoshawan data

# Backup configuration
sudo tar -czf mondoshawan-config-$(date +%Y%m%d).tar.gz \
  -C /etc mondoshawan

# Restart node
sudo systemctl start mondoshawan

# Store backups offsite (AWS S3, etc.)
aws s3 cp mondoshawan-backup-$(date +%Y%m%d).tar.gz \
  s3://mondoshawan-backups/
```

### Restoring from Backup

```bash
# Stop node
sudo systemctl stop mondoshawan

# Clear existing data
sudo rm -rf /var/lib/mondoshawan/data/*

# Restore from backup
sudo tar -xzf mondoshawan-backup-20260108.tar.gz \
  -C /var/lib/mondoshawan

# Fix permissions
sudo chown -R mondoshawan:mondoshawan /var/lib/mondoshawan/data

# Start node
sudo systemctl start mondoshawan
```

### Rolling Back a Failed Upgrade

```bash
# Stop node
sudo systemctl stop mondoshawan

# Restore previous binary
sudo cp /usr/local/bin/mondoshawan-node.old \
  /usr/local/bin/mondoshawan-node

# Restore data (if corrupted)
sudo rm -rf /var/lib/mondoshawan/data
sudo cp -r /var/lib/mondoshawan/data.backup.20260108 \
  /var/lib/mondoshawan/data
sudo chown -R mondoshawan:mondoshawan /var/lib/mondoshawan/data

# Start node
sudo systemctl start mondoshawan
```

---

## Troubleshooting

### Node Won't Start

**Check logs**:
```bash
sudo journalctl -u mondoshawan -n 100 --no-pager
```

**Common issues**:

1. **Port already in use**:
   ```bash
   # Check what's using the port
   sudo netstat -tlnp | grep 8080
   
   # Kill process or change port in config
   ```

2. **Permission denied**:
   ```bash
   # Fix permissions
   sudo chown -R mondoshawan:mondoshawan /var/lib/mondoshawan
   sudo chmod 755 /usr/local/bin/mondoshawan-node
   ```

3. **Config file error**:
   ```bash
   # Validate TOML syntax
   cat /etc/mondoshawan/node.toml
   
   # Check for typos, missing quotes, etc.
   ```

### Node Not Syncing

**Check peer count**:
```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"net_peerCount","params":[],"id":1}'
```

**If 0 peers**:
1. Check firewall allows port 8080
2. Verify bootstrap nodes are correct
3. Check external_ip is set correctly

**If peers but not syncing**:
1. Check block height:
   ```bash
   curl -X POST http://localhost:8545 \
     -H "Content-Type: application/json" \
     -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
   ```
2. Compare with other nodes
3. Check logs for sync errors

### High CPU Usage

```bash
# Check CPU usage
top -u mondoshawan

# If mining is enabled, reduce threads
# Edit /etc/mondoshawan/node.toml:
# [mining]
# mining_threads = 2  # Reduce from 4

# Restart node
sudo systemctl restart mondoshawan
```

### High Memory Usage

```bash
# Check memory
ps aux | grep mondoshawan-node

# If excessive (>4GB), check for memory leak
sudo journalctl -u mondoshawan | grep -i "memory\|oom"

# Restart node to clear memory
sudo systemctl restart mondoshawan
```

### Disk Space Full

```bash
# Check disk usage
df -h /var/lib/mondoshawan

# If full, prune old data (after backup)
# TODO: Implement pruning strategy
```

### Connection Refused (RPC)

```bash
# Check if RPC is enabled
cat /etc/mondoshawan/node.toml | grep enable_rpc

# Check if listening
sudo netstat -tlnp | grep 8545

# If not listening, check logs
sudo journalctl -u mondoshawan -n 50
```

---

## Appendix

### A. Configuration Reference

**Complete `node.toml` with all options**:

```toml
[node]
p2p_port = 8080
rpc_port = 8545
data_dir = "/var/lib/mondoshawan/data"
enable_mining = true
miner_address = "0x0000000000000000000000000000000000000001"

[network]
bootstrap_nodes = []
external_ip = "AUTO"
max_peers = 50
peer_discovery_interval = 60  # seconds
connection_timeout = 30  # seconds

[features]
enable_sharding = false
shard_count = 10
enable_verkle = false
enable_pqc = false

[rpc]
enable_rpc = true
rpc_host = "0.0.0.0"
rpc_port = 8545
enable_cors = true
cors_origins = ["*"]
rate_limit_enabled = true
rate_limit_requests_per_second = 100

[mining]
mining_threads = 4
stream_a_enabled = true
stream_b_enabled = true
stream_c_enabled = true

[logging]
level = "info"  # trace, debug, info, warn, error
log_file = "/var/lib/mondoshawan/logs/node.log"
log_rotation_size = "100MB"
log_rotation_count = 5
```

### B. RPC API Reference

**Key Methods for Monitoring**:

```bash
# Get block number
curl -X POST http://localhost:8545 -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'

# Get peer count
curl -X POST http://localhost:8545 -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"net_peerCount","params":[],"id":1}'

# Get mining status
curl -X POST http://localhost:8545 -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_mining","params":[],"id":1}'

# Get DAG stats
curl -X POST http://localhost:8545 -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"mds_getDagStats","params":[],"id":1}'

# Get shard stats (if sharding enabled)
curl -X POST http://localhost:8545 -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"mds_getShardStats","params":[],"id":1}'
```

### C. Performance Tuning

**Linux Kernel Parameters** (`/etc/sysctl.conf`):

```bash
# Network tuning
net.core.somaxconn = 1024
net.core.netdev_max_backlog = 5000
net.core.rmem_max = 134217728
net.core.wmem_max = 134217728
net.ipv4.tcp_rmem = 4096 87380 134217728
net.ipv4.tcp_wmem = 4096 65536 134217728

# File descriptors
fs.file-max = 2097152

# Apply changes
sudo sysctl -p
```

**Systemd Resource Limits**:

```ini
# In /etc/systemd/system/mondoshawan.service
[Service]
LimitNOFILE=65536
LimitNPROC=4096
CPUQuota=400%  # 4 cores max
MemoryLimit=8G
```

### D. Cost Estimation

**10-Node Testnet (Monthly)**:

| Provider | Instance Type | Storage | Cost/Node | Total |
|----------|--------------|---------|-----------|-------|
| **AWS** | t3.xlarge | 500GB EBS | $180 | $1,800 |
| **DigitalOcean** | CPU-Optimized | 500GB Block | $96 | $960 |
| **Hetzner** | CX41 | 500GB Volume | €35 | €350 (~$380) |

**Recommended**: Mix providers for diversity (3 AWS + 3 DO + 4 Hetzner = ~$1,200/month)

### E. Security Checklist

- [ ] SSH key-based authentication only (no passwords)
- [ ] Firewall configured (ufw or iptables)
- [ ] Non-root user for node process
- [ ] RPC behind Nginx reverse proxy with SSL
- [ ] Rate limiting enabled on RPC
- [ ] Automated security updates enabled
- [ ] Monitoring and alerting configured
- [ ] Regular backups scheduled
- [ ] Backup restoration tested
- [ ] Incident response plan documented

### F. Maintenance Schedule

**Daily**:
- [ ] Check node status (`systemctl status mondoshawan`)
- [ ] Verify sync status (compare block height across nodes)
- [ ] Review critical alerts

**Weekly**:
- [ ] Review logs for errors
- [ ] Check disk space
- [ ] Verify backup completion
- [ ] Update security patches

**Monthly**:
- [ ] Review performance metrics
- [ ] Test backup restoration
- [ ] Update node software (if new release)
- [ ] Audit security configuration

**Quarterly**:
- [ ] Full security audit
- [ ] Load testing
- [ ] Disaster recovery drill
- [ ] Review and update documentation

---

**END OF GUIDE**

For questions or issues, consult the [PROJECT_STATUS.md](PROJECT_STATUS.md) or [TROUBLESHOOTING.md](TROUBLESHOOTING.md) documents.
