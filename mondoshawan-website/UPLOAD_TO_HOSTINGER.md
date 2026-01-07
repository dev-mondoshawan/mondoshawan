# Upload to Hostinger - Complete Guide

## ✅ All Files Ready

All website files are consolidated in the `mondoshawan-website/` directory.

## 📁 Directory Structure

```
mondoshawan-website/
├── index.html                    # Main landing page
├── why-mondoshawan.html          # Technical advantages
├── comparison.html               # Feature comparison
├── README.md                     # Documentation
├── DEPLOYMENT.md                 # Deployment guide
├── HOSTINGER_UPLOAD.txt          # Quick reference
├── FILES_TO_UPLOAD.txt           # File list
├── .htaccess                     # Server configuration
└── explorer/                     # Blockchain explorer
    ├── index.html
    ├── app.js
    └── styles.css
```

## 🚀 Upload Instructions

### Step 1: Access Hostinger
1. Log into your Hostinger account
2. Go to **hPanel**
3. Click **File Manager**

### Step 2: Navigate to public_html
1. In File Manager, navigate to `public_html` directory
2. This is your website's root directory

### Step 3: Upload Files
**Option A: Upload Folder (Recommended)**
1. Click **Upload** button
2. Select entire `mondoshawan-website` folder
3. Wait for upload to complete
4. Extract if uploaded as ZIP

**Option B: Upload Individual Files**
1. Upload all root files (index.html, etc.)
2. Create `explorer` folder
3. Upload explorer files into `explorer/` folder

### Step 4: Verify Structure
After upload, your `public_html` should have:
- ✅ index.html
- ✅ why-mondoshawan.html
- ✅ comparison.html
- ✅ explorer/ folder with 3 files

## ⚙️ Post-Upload Configuration

### 1. Update Explorer API URLs

Edit `explorer/app.js` and change:

```javascript
// FROM:
const API_BASE = 'http://localhost:8081/api';
const RPC_BASE = 'http://localhost:8545';

// TO (your node's public URL):
const API_BASE = 'https://your-node-domain.com:8081/api';
const RPC_BASE = 'https://your-node-domain.com:8545';
```

### 2. Test Your Website
1. Visit your domain (e.g., `mondoshawan.network`)
2. Test all pages load correctly
3. Test explorer connects to your node
4. Check mobile responsiveness

### 3. Enable HTTPS (Recommended)
1. In Hostinger hPanel, go to **SSL**
2. Enable free SSL certificate
3. Update `.htaccess` to redirect HTTP to HTTPS (uncomment lines)

## 📋 Checklist

- [ ] All files uploaded to public_html
- [ ] Directory structure maintained (explorer/ folder)
- [ ] Explorer API URLs updated for production
- [ ] All pages load correctly
- [ ] Explorer connects to node
- [ ] Mobile responsive (test on phone)
- [ ] HTTPS enabled (optional but recommended)

## 🎉 Done!

Your Mondoshawan website is now live on Hostinger!

## Troubleshooting

**404 Errors:**
- Check file names match exactly
- Verify directory structure
- Check file permissions (should be 644)

**Explorer Not Working:**
- Verify API URLs in app.js
- Check node is publicly accessible
- Test API endpoints directly in browser
- Check CORS settings on node

**Styling Issues:**
- Clear browser cache
- Check CSS files uploaded
- Verify file paths are correct
