# Starting Grafana for Mondoshawan Monitoring

## Prerequisites

1. **Docker Desktop** must be installed and running
2. **Mondoshawan node** should be running and exposing metrics at `http://localhost:8545/metrics`

## Quick Start

### Step 1: Start Docker Desktop

1. Open Docker Desktop application
2. Wait for it to fully start (whale icon in system tray should be steady)
3. Verify Docker is running:
   ```powershell
   docker ps
   ```

### Step 2: Start Grafana and Prometheus

```powershell
cd D:\Mondoshawan\grafana
docker-compose up -d
```

### Step 3: Verify Services

```powershell
docker-compose ps
```

You should see:
- `Mondoshawan-prometheus` - Running
- `Mondoshawan-grafana` - Running

### Step 4: Access Services

- **Grafana:** http://localhost:3000
  - Username: `admin`
  - Password: `admin`
  - Dashboards will be automatically loaded!

- **Prometheus:** http://localhost:9090
  - Check targets: http://localhost:9090/targets
  - Verify `Mondoshawan-node` target is UP

## Troubleshooting

### Docker Desktop Not Running

**Error:** `The system cannot find the file specified`

**Solution:**
1. Open Docker Desktop
2. Wait for it to fully start
3. Try the command again

### Prometheus Can't Scrape Metrics

**Check 1:** Verify Mondoshawan node is running
```powershell
# Check if node is running
curl http://localhost:8545/metrics
```

**Check 2:** Update Prometheus config
- Edit `grafana/prometheus/prometheus.yml`
- Change `host.docker.internal:8545` to `localhost:8545` if needed
- Restart Prometheus: `docker-compose restart prometheus`

**Check 3:** Check Prometheus targets
- Go to http://localhost:9090/targets
- Click on `Mondoshawan-node` target
- Check for errors

### Grafana Shows "No Data"

1. **Verify data source:**
   - Go to Configuration → Data Sources
   - Click on Prometheus
   - Click "Save & Test"
   - Should show "Data source is working"

2. **Check time range:**
   - Ensure you're viewing a time range where data exists
   - Try "Last 5 minutes"

3. **Verify metrics are being collected:**
   - Go to Prometheus → Graph
   - Query: `Mondoshawan_blocks_mined_total`
   - Should return a value

### Windows Network Issues

If `host.docker.internal` doesn't work:

1. **Option 1:** Use `localhost` in `prometheus.yml`
   ```yaml
   - targets: ['localhost:8545']
   ```

2. **Option 2:** Use host network mode (Linux only)
   - Add to docker-compose.yml:
   ```yaml
   prometheus:
     network_mode: host
   ```

## Manual Setup (Without Docker)

If you prefer not to use Docker:

### Install Prometheus

1. Download from https://prometheus.io/download/
2. Extract and run:
   ```powershell
   prometheus.exe --config.file=prometheus.yml
   ```

### Install Grafana

1. Download from https://grafana.com/grafana/download
2. Install and start Grafana service
3. Open http://localhost:3000
4. Add Prometheus as data source: `http://localhost:9090`
5. Import dashboards from `grafana/dashboards/` directory

## Useful Commands

### View Logs
```powershell
docker-compose logs -f grafana
docker-compose logs -f prometheus
```

### Stop Services
```powershell
docker-compose down
```

### Restart Services
```powershell
docker-compose restart
```

### Update and Restart
```powershell
docker-compose pull
docker-compose up -d
```

## Next Steps

Once Grafana is running:

1. **Explore Dashboards:**
   - Mondoshawan Blockchain Overview
   - Mondoshawan Mining Metrics
   - Mondoshawan Sharding Metrics
   - Mondoshawan Network Metrics
   - Mondoshawan Transaction Metrics

2. **Set Up Alerts:**
   - Configure notification channels
   - Create alert rules based on metrics

3. **Customize:**
   - Add custom panels
   - Modify thresholds
   - Create new dashboards

## Support

For issues:
1. Check Docker Desktop is running
2. Verify Mondoshawan node is running and exposing metrics
3. Check Prometheus targets status
4. Review logs: `docker-compose logs`
