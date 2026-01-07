# Mondoshawan Website

Complete website package ready for Hostinger deployment.

## Directory Structure

```
mondoshawan-website/
├── index.html              # Main landing page
├── why-mondoshawan.html    # Technical advantages page
├── comparison.html          # Feature comparison page
├── explorer/               # Blockchain explorer
│   ├── index.html
│   ├── app.js
│   └── styles.css
└── README.md               # This file
```

## Deployment to Hostinger

### Option 1: File Manager
1. Log into Hostinger control panel
2. Go to File Manager
3. Navigate to `public_html` (or your domain's root)
4. Upload all files from `mondoshawan-website/` directory
5. Ensure `index.html` is in the root

### Option 2: FTP
1. Connect via FTP to your Hostinger account
2. Upload all files to `public_html` directory
3. Maintain the directory structure (especially `explorer/` folder)

### Option 3: Git (if available)
1. Initialize git in `mondoshawan-website/`
2. Push to Hostinger's git repository (if enabled)
3. Pull on server

## Pages

- **index.html** - Main landing page with features and stats
- **why-mondoshawan.html** - Technical advantages and comparisons
- **comparison.html** - Detailed feature comparison table
- **explorer/index.html** - Live blockchain explorer

## Notes

- All pages are self-contained (CSS in `<style>` tags)
- Explorer connects to `http://localhost:8081` (update for production)
- Update API URLs in `explorer/app.js` for production deployment
- All links are relative paths (should work after upload)

## Production Updates Needed

Before going live, update:
1. **explorer/app.js** - Change `localhost:8081` to your node's public URL
2. **API endpoints** - Update RPC and HTTP API URLs
3. **GitHub links** - Add actual repository URL
4. **Social media** - Add social media links if available

## Testing

After deployment:
1. Test all pages load correctly
2. Verify explorer connects to your node
3. Check all internal links work
4. Test on mobile devices (responsive design)
