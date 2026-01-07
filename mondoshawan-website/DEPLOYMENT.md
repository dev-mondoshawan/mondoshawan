# Hostinger Deployment Guide

## Quick Start

1. **Upload all files** from `mondoshawan-website/` to your Hostinger `public_html` directory
2. **Update API URLs** in `explorer/app.js` (see below)
3. **Test** all pages work correctly

## File Structure After Upload

```
public_html/
├── index.html
├── why-mondoshawan.html
├── comparison.html
├── explorer/
│   ├── index.html
│   ├── app.js
│   └── styles.css
└── README.md
```

## Required Updates for Production

### 1. Update Explorer API URLs

Edit `explorer/app.js` and change:

```javascript
// Current (localhost):
const API_BASE = 'http://localhost:8081/api';
const RPC_BASE = 'http://localhost:8545';

// Production (update to your node's public URL):
const API_BASE = 'https://your-node-domain.com:8081/api';
const RPC_BASE = 'https://your-node-domain.com:8545';
```

### 2. Update GitHub Links

In `index.html`, update:
```html
<a href="https://github.com" target="_blank">GitHub</a>
```
to your actual repository URL.

### 3. Update Social Media (Optional)

Add social media links in footer if available.

## Deployment Methods

### Method 1: File Manager (Easiest)
1. Log into Hostinger hPanel
2. Go to **File Manager**
3. Navigate to `public_html`
4. Click **Upload**
5. Select all files from `mondoshawan-website/`
6. Wait for upload to complete

### Method 2: FTP
1. Get FTP credentials from Hostinger
2. Connect using FileZilla or similar
3. Navigate to `public_html`
4. Upload all files maintaining directory structure

### Method 3: Git (If Available)
```bash
cd mondoshawan-website
git init
git add .
git commit -m "Initial website deployment"
# Push to Hostinger git repo if enabled
```

## Testing Checklist

After deployment:
- [ ] Main page (index.html) loads
- [ ] Why Mondoshawan page works
- [ ] Comparison page works
- [ ] Explorer loads (may need API URL update)
- [ ] All internal links work
- [ ] Mobile responsive (test on phone)
- [ ] Images/assets load correctly

## Troubleshooting

### Explorer Not Connecting
- Check API URLs in `explorer/app.js`
- Verify your node is publicly accessible
- Check CORS settings on your node
- Test API endpoints directly in browser

### 404 Errors
- Ensure all files uploaded correctly
- Check file names match exactly (case-sensitive on Linux)
- Verify directory structure is correct

### Styling Issues
- Clear browser cache
- Check CSS is loading (view page source)
- Verify all files uploaded completely

## Production Considerations

1. **SSL Certificate** - Enable HTTPS on Hostinger
2. **Domain** - Point your domain to Hostinger
3. **Node Access** - Ensure your blockchain node is publicly accessible
4. **CORS** - Configure CORS on your node to allow your website domain
5. **CDN** - Consider using CDN for faster loading (optional)

## Support

For issues:
1. Check Hostinger documentation
2. Verify file permissions (should be 644 for files, 755 for directories)
3. Check error logs in Hostinger control panel
