# Cloud Platform Comparison for Mondoshawan Testnet Node

**Date**: January 2026  
**Purpose**: Choose the best platform for hosting testnet node

---

## 🎯 Quick Recommendation

**For Free/Cheap Testnet:**
1. **Railway.app** - Best free tier, easy deployment
2. **Fly.io** - Generous free tier, global edge
3. **Render.com** - Good free tier, but sleeps after inactivity

**For Production Testnet:**
1. **DigitalOcean App Platform** - $5/month, reliable
2. **Render.com Starter** - $7/month, always on
3. **AWS Lightsail** - $5/month, full control

---

## 📊 Detailed Comparison

| Platform | Free Tier | Paid (Min) | Docker | Always On | Storage | Best For |
|----------|-----------|------------|--------|-----------|---------|----------|
| **Railway.app** | ✅ $5 credit/mo | $5/mo | ✅ | ✅ | ✅ | **Best overall** |
| **Fly.io** | ✅ Generous | $0-5/mo | ✅ | ✅ | ✅ | Global edge |
| **Render.com** | ⚠️ Sleeps | $7/mo | ✅ | ⚠️ | ⚠️ | Easy setup |
| **DigitalOcean** | ❌ | $5/mo | ✅ | ✅ | ✅ | Reliability |
| **Heroku** | ❌ | $7/mo | ✅ | ✅ | ✅ | Legacy |
| **AWS EC2** | ❌ | $10/mo | ✅ | ✅ | ✅ | Full control |
| **AWS Lightsail** | ❌ | $5/mo | ✅ | ✅ | ✅ | Budget AWS |

---

## 🚂 Railway.app (Recommended)

### Pros
- ✅ **$5 free credit/month** (enough for testnet)
- ✅ **Always on** (no sleep)
- ✅ **Persistent storage** included
- ✅ **Docker support** (native)
- ✅ **Easy GitHub deployment**
- ✅ **Automatic HTTPS**
- ✅ **Great free tier**

### Cons
- ⚠️ Credit expires monthly (need to top up)
- ⚠️ Less well-known than Render

### Cost
- **Free**: $5 credit/month
- **Paid**: $5-10/month for testnet node

### Deployment
```bash
# Install Railway CLI
npm i -g @railway/cli

# Login
railway login

# Deploy
railway init
railway up
```

**URL**: `https://your-app.railway.app`

---

## ✈️ Fly.io

### Pros
- ✅ **Generous free tier**
- ✅ **Global edge network** (fast worldwide)
- ✅ **Always on**
- ✅ **Persistent volumes** available
- ✅ **Docker support**
- ✅ **Great for global testnet**

### Cons
- ⚠️ Slightly more complex setup
- ⚠️ CLI-based deployment

### Cost
- **Free**: Generous (3 shared VMs)
- **Paid**: $1.94/month per VM (if needed)

### Deployment
```bash
# Install Fly CLI
curl -L https://fly.io/install.sh | sh

# Login
fly auth login

# Deploy
fly launch
fly deploy
```

**URL**: `https://your-app.fly.dev`

---

## 🎨 Render.com

### Pros
- ✅ **Free tier available**
- ✅ **Easy web dashboard**
- ✅ **GitHub integration**
- ✅ **Docker support**
- ✅ **Automatic HTTPS**

### Cons
- ⚠️ **Free tier sleeps** after 15 min inactivity
- ⚠️ **No persistent storage** on free tier
- ⚠️ Need paid plan ($7/mo) for always-on

### Cost
- **Free**: Sleeps after inactivity
- **Starter**: $7/month (always on, persistent storage)

### Deployment
- Web dashboard → New Web Service → Connect GitHub
- Or use `render.yaml` (Infrastructure as Code)

**URL**: `https://your-app.onrender.com`

---

## 💧 DigitalOcean App Platform

### Pros
- ✅ **Simple deployment**
- ✅ **Always on**
- ✅ **Persistent storage**
- ✅ **Docker support**
- ✅ **Reliable infrastructure**
- ✅ **Good documentation**

### Cons
- ❌ **No free tier**
- ⚠️ Minimum $5/month

### Cost
- **Basic**: $5/month minimum
- **Professional**: $12/month (better resources)

### Deployment
- Web dashboard → Create App → Connect GitHub
- Or use `app.yaml`

**URL**: `https://your-app.ondigitalocean.app`

---

## ☁️ AWS Options

### AWS Lightsail
- **Cost**: $5/month (1GB RAM, 1 vCPU, 40GB SSD)
- **Pros**: Full VPS control, persistent storage
- **Cons**: Manual setup, more complex

### AWS EC2
- **Cost**: $10-20/month (t3.micro or t3.small)
- **Pros**: Full control, scalable
- **Cons**: More expensive, complex setup

---

## 🎯 Recommendation by Use Case

### **Free Testnet (Development)**
→ **Railway.app** or **Fly.io**
- Both have good free tiers
- Always on
- Persistent storage

### **Public Testnet (Always On)**
→ **Railway.app Starter** ($5/mo) or **DigitalOcean** ($5/mo)
- Reliable
- Always on
- Good performance

### **Production Testnet (High Performance)**
→ **DigitalOcean** ($12/mo) or **AWS EC2** ($20/mo)
- Better resources
- More reliable
- Better support

---

## 📝 Quick Setup Guides

### Railway.app (Recommended)

1. **Sign up**: https://railway.app
2. **New Project** → Deploy from GitHub
3. **Select repo**: `dev-mondoshawan/mondoshawan`
4. **Configure**:
   - Service type: `Web Service`
   - Dockerfile: `./Dockerfile`
   - Port: `8545`
5. **Deploy** → Get URL: `https://your-app.railway.app`

### Fly.io

1. **Sign up**: https://fly.io
2. **Install CLI**: `curl -L https://fly.io/install.sh | sh`
3. **Deploy**:
   ```bash
   fly launch
   fly deploy
   ```
4. **Get URL**: `https://your-app.fly.dev`

### Render.com

1. **Sign up**: https://render.com
2. **New Web Service** → Connect GitHub
3. **Configure**:
   - Environment: `Docker`
   - Dockerfile: `./Dockerfile`
   - Plan: `Starter` ($7/mo) for always-on
4. **Deploy** → Get URL: `https://your-app.onrender.com`

---

## 🔧 Configuration for All Platforms

### Environment Variables (Set in Platform Dashboard)

```
RUST_LOG=info
RPC_PORT=8545
P2P_PORT=8080
```

### CORS Configuration

In your RPC server, allow CORS:
```rust
headers.insert("Access-Control-Allow-Origin", "*");
headers.insert("Access-Control-Allow-Methods", "POST, GET, OPTIONS");
```

### Explorer Configuration

Update explorer to use deployed URL:
```javascript
const RPC_BASE = 'https://your-app.railway.app';  // or .fly.dev, .onrender.com
```

---

## 💡 Pro Tips

1. **Start with Railway.app** - Best free tier, easiest setup
2. **Use Docker** - Works on all platforms
3. **Set up monitoring** - Use platform's built-in monitoring
4. **Backup data** - Use persistent volumes for important data
5. **Test locally first** - Ensure Dockerfile works before deploying

---

## ✅ Final Recommendation

**For Your Testnet Node:**

1. **Try Railway.app first** (free $5 credit/month)
   - Easy setup
   - Always on
   - Persistent storage
   - Great for testnet

2. **If Railway doesn't work**, try **Fly.io**
   - Also great free tier
   - Global edge network

3. **For production testnet**, use **DigitalOcean** ($5/mo)
   - Reliable
   - Always on
   - Good performance

---

**Choose Railway.app for the best free tier experience!** 🚂
