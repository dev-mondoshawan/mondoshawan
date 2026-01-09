# Deploying Mondoshawan Testnet Node on Render.com

**Status**: ✅ Ready for Deployment  
**Platform**: Render.com (Docker-based)  
**Date**: January 2026

---

## 🎯 Overview

Yes, you can deploy your Mondoshawan testnet node on Render.com! Render supports Docker containers, which is perfect since we already have a `Dockerfile`.

**Render.com Advantages:**
- ✅ Free tier available (with limitations)
- ✅ Docker support
- ✅ Automatic HTTPS
- ✅ Public URL provided
- ✅ Easy deployment from GitHub
- ✅ Auto-restart on failure

**Limitations:**
- ⚠️ Free tier: Services sleep after 15 minutes of inactivity
- ⚠️ Free tier: Limited CPU/RAM
- ⚠️ Storage: Ephemeral (data lost on restart) - need paid plan for persistence

---

## 📋 Prerequisites

1. **GitHub Repository**: Your code must be on GitHub
2. **Render.com Account**: Sign up at https://render.com
3. **Dockerfile**: Already exists in your repo ✅

---

## 🚀 Step-by-Step Deployment

### Step 1: Prepare Your Repository

Ensure your `Dockerfile` is in the root directory and properly configured:

```dockerfile
# Dockerfile (already exists)
FROM rust:1.75 as builder
WORKDIR /app
COPY mondoshawan-blockchain/Cargo.toml mondoshawan-blockchain/Cargo.toml
COPY mondoshawan-blockchain/src mondoshawan-blockchain/src
WORKDIR /app/mondoshawan-blockchain
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/mondoshawan-blockchain/target/release/node /app/node
RUN mkdir -p /data
EXPOSE 8080 8545 9090
CMD ["./node"]
```

### Step 2: Create `render.yaml` (Optional but Recommended)

Create `render.yaml` in your repository root:

```yaml
services:
  - type: web
    name: mondoshawan-testnet-node
    runtime: docker
    plan: free  # or 'starter' for $7/month (no sleep, persistent storage)
    dockerfilePath: ./Dockerfile
    dockerContext: .
    envVars:
      - key: RUST_LOG
        value: info
      - key: RPC_PORT
        value: 8545
      - key: P2P_PORT
        value: 8080
    healthCheckPath: /
    healthCheckGracePeriod: 300
```

### Step 3: Deploy on Render.com

#### Option A: Using Render Dashboard

1. **Log in to Render.com**
   - Go to https://dashboard.render.com
   - Sign up or log in

2. **Create New Web Service**
   - Click "New +" → "Web Service"
   - Connect your GitHub repository
   - Select the repository: `dev-mondoshawan/mondoshawan`

3. **Configure Service**
   - **Name**: `mondoshawan-testnet-node`
   - **Environment**: `Docker`
   - **Region**: Choose closest to you (e.g., `Oregon (US West)`)
   - **Branch**: `master` (or your main branch)
   - **Root Directory**: Leave empty (or `./` if needed)
   - **Dockerfile Path**: `./Dockerfile`
   - **Docker Context**: `.` (root)

4. **Environment Variables**
   Add these in the "Environment" section:
   ```
   RUST_LOG=info
   RPC_PORT=8545
   P2P_PORT=8080
   ```

5. **Advanced Settings**
   - **Health Check Path**: `/` (or remove if not needed)
   - **Auto-Deploy**: `Yes` (deploys on git push)
   - **Plan**: 
     - **Free**: Sleeps after inactivity (not ideal for testnet)
     - **Starter ($7/month)**: Always on, persistent storage

6. **Deploy**
   - Click "Create Web Service"
   - Render will build and deploy your Docker container
   - Wait 5-10 minutes for build to complete

#### Option B: Using `render.yaml` (Infrastructure as Code)

1. **Push `render.yaml` to GitHub**
2. **In Render Dashboard**:
   - Click "New +" → "Blueprint"
   - Connect repository
   - Render will detect `render.yaml` and create services automatically

### Step 4: Configure Ports

**Important**: Render.com assigns a random port, but your app should listen on the port specified in the `PORT` environment variable.

**Update your node code** (if needed) to read from environment:

```rust
// In node.rs or config.rs
let rpc_port = std::env::var("PORT")
    .unwrap_or_else(|_| "8545".to_string())
    .parse()
    .unwrap_or(8545);
```

**Or update Dockerfile CMD**:

```dockerfile
# Use PORT env var or default to 8545
CMD ["sh", "-c", "./node ${PORT:-8545}"]
```

**Render.com Port Mapping:**
- Render exposes your service on port `10000` internally
- Render provides a public URL (e.g., `https://mondoshawan-testnet-node.onrender.com`)
- Your app should listen on `PORT` env var (Render sets this automatically)

### Step 5: Configure CORS

Your explorer needs to connect to the RPC endpoint. Update your node's CORS settings:

**In your RPC server code** (`rpc.rs`), ensure CORS headers are set:

```rust
// Allow CORS from your explorer domain
headers.insert("Access-Control-Allow-Origin", "*");  // For testnet, allow all
headers.insert("Access-Control-Allow-Methods", "POST, GET, OPTIONS");
headers.insert("Access-Control-Allow-Headers", "Content-Type");
```

### Step 6: Get Your Public URL

After deployment, Render provides a URL like:
```
https://mondoshawan-testnet-node.onrender.com
```

**RPC Endpoint**: 
```
https://mondoshawan-testnet-node.onrender.com:8545
```

**Note**: Render.com free tier uses HTTP (not HTTPS) for custom ports. For HTTPS on port 8545, you may need:
- Use Render's built-in HTTPS (port 443)
- Or upgrade to paid plan
- Or use a reverse proxy

---

## 🔧 Configuration for Render.com

### Update Explorer to Use Render URL

In your explorer (`mondoshawan-explorer-frontend/app.js`):

```javascript
// Option 1: URL parameter
// http://localhost:3000/explorer?rpc=https://mondoshawan-testnet-node.onrender.com

// Option 2: Update default
const RPC_BASE = 'https://mondoshawan-testnet-node.onrender.com';
```

### Environment Variables for Render

Set these in Render dashboard → Environment:

| Variable | Value | Description |
|----------|-------|-------------|
| `RUST_LOG` | `info` | Logging level |
| `RPC_PORT` | `8545` | RPC server port |
| `P2P_PORT` | `8080` | P2P network port |
| `PORT` | `10000` | Render's internal port (auto-set) |

---

## 💰 Pricing Options

### Free Tier
- ✅ **Cost**: $0/month
- ⚠️ **Limitation**: Service sleeps after 15 minutes of inactivity
- ⚠️ **Storage**: Ephemeral (data lost on restart)
- ✅ **Good for**: Testing, demos, development

### Starter Plan ($7/month)
- ✅ **Always On**: No sleep
- ✅ **Persistent Storage**: 10GB included
- ✅ **Better Performance**: More CPU/RAM
- ✅ **Recommended for**: Public testnet

### Professional Plan ($25/month)
- ✅ **Even Better Performance**
- ✅ **More Storage**: 100GB
- ✅ **Dedicated Resources**

---

## 🌐 Alternative Cloud Platforms

### 1. **Railway.app** (Recommended Alternative)
- ✅ **Free tier**: $5 credit/month
- ✅ **Docker support**: Yes
- ✅ **Always on**: Yes (on paid)
- ✅ **Easy deployment**: GitHub integration
- **Cost**: ~$5-10/month for testnet node

### 2. **Fly.io**
- ✅ **Free tier**: Generous
- ✅ **Docker support**: Yes
- ✅ **Global edge**: Fast worldwide
- **Cost**: Free tier available, ~$5/month for persistent storage

### 3. **DigitalOcean App Platform**
- ✅ **Docker support**: Yes
- ✅ **Simple deployment**: GitHub integration
- **Cost**: $5/month minimum

### 4. **Heroku** (Legacy)
- ⚠️ **Free tier**: Discontinued
- ✅ **Docker support**: Yes
- **Cost**: $7/month minimum

### 5. **AWS EC2 / Lightsail**
- ✅ **Full control**: VPS
- ✅ **Persistent storage**: Yes
- **Cost**: $5-10/month (Lightsail), $10-20/month (EC2)

---

## 📝 Render.com Deployment Checklist

- [ ] GitHub repository is public or Render has access
- [ ] `Dockerfile` is in repository root
- [ ] Environment variables are set in Render dashboard
- [ ] Service is deployed and running
- [ ] Public URL is accessible
- [ ] RPC endpoint is responding (test with curl)
- [ ] Explorer is configured to use Render URL
- [ ] CORS is configured correctly

---

## 🧪 Testing Your Deployment

### 1. Test RPC Endpoint

```bash
curl -X POST https://mondoshawan-testnet-node.onrender.com \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "eth_blockNumber",
    "params": [],
    "id": 1
  }'
```

**Expected Response:**
```json
{
  "jsonrpc": "2.0",
  "result": "0x1",
  "id": 1
}
```

### 2. Test from Explorer

Open your explorer with:
```
http://localhost:3000/explorer?rpc=https://mondoshawan-testnet-node.onrender.com
```

### 3. Check Logs

In Render dashboard → Logs, you should see:
```
✅ Node started on port 8545
✅ RPC server listening
✅ Mining started
```

---

## 🔒 Security Considerations

### For Testnet (OK to be lenient):
- ✅ CORS: Allow all origins (`*`)
- ✅ RPC: Public access is fine
- ⚠️ No authentication needed (testnet)

### For Mainnet (Future):
- ❌ CORS: Restrict to specific domains
- ❌ RPC: Add authentication
- ❌ Rate limiting: Implement strict limits
- ❌ Firewall: Restrict access

---

## 🐛 Troubleshooting

### Service Won't Start

**Check Logs**:
- Render Dashboard → Your Service → Logs
- Look for error messages

**Common Issues**:
1. **Port mismatch**: Ensure app listens on `PORT` env var
2. **Build failure**: Check Dockerfile syntax
3. **Missing dependencies**: Verify all files are in repo

### RPC Not Accessible

**Check**:
1. Service is running (not sleeping)
2. CORS headers are set
3. Port is correct (8545 or Render's assigned port)
4. Firewall allows connections

### Data Lost on Restart

**Solution**: Upgrade to Starter plan ($7/month) for persistent storage

---

## 📊 Monitoring

### Render.com Built-in Monitoring
- **Logs**: Real-time logs in dashboard
- **Metrics**: CPU, RAM, network usage
- **Alerts**: Email notifications on failures

### Add Prometheus/Grafana (Optional)
- Deploy as separate service on Render
- Or use external monitoring service

---

## 🚀 Quick Start Commands

### Deploy to Render (One-Time Setup)

1. **Push to GitHub**:
   ```bash
   git add .
   git commit -m "Add Render deployment config"
   git push origin master
   ```

2. **Deploy on Render**:
   - Go to Render dashboard
   - Create new Web Service
   - Connect GitHub repo
   - Deploy

3. **Get URL**:
   - Copy public URL from Render dashboard
   - Use in explorer: `?rpc=https://your-service.onrender.com`

---

## 📞 Support

- **Render.com Docs**: https://render.com/docs
- **Render.com Support**: support@render.com
- **Mondoshawan Issues**: https://github.com/dev-mondoshawan/mondoshawan/issues

---

## ✅ Summary

**Yes, you can deploy on Render.com!**

**Best Approach**:
1. Use **Starter plan ($7/month)** for always-on testnet
2. Deploy via **Docker** (already have Dockerfile)
3. Configure **CORS** for explorer access
4. Use **public URL** in explorer

**Alternative**: Railway.app or Fly.io for similar pricing with better free tier.

---

**Ready to deploy your testnet node!** 🚀
