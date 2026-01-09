# Public Repository Structure Recommendation

## ✅ What to Include (Public)

### Core Code
- `mondoshawan-blockchain/` - Full blockchain implementation
  - `src/` - All source code
  - `Cargo.toml` - Dependencies
  - `README.md` - Setup instructions
  - Exclude: `target/`, `Cargo.lock` (in .gitignore)

### Explorer Frontend
- `mondoshawan-explorer-frontend/` - Block explorer
  - All HTML, CSS, JS files
  - Public-facing only

### Website
- `mondoshawan-website/` - Marketing website
  - All public pages
  - Exclude: Internal deployment notes

### Documentation (Public)
- `README.md` - Main project README
- `Mondoshawan_WHITEPAPER.md` - Whitepaper (markdown)
- `Mondoshawan_WHITEPAPER.html` - Whitepaper (HTML)
- `BRANDING.md` - Branding guidelines
- `CURRENT_STATUS_SUMMARY.md` - Current status
- `DEVELOPER_GUIDE.md` - Developer documentation
- `BUILD_INSTRUCTIONS.md` - Build instructions
- `QUICK_START.md` - Quick start guide

### Monitoring
- `grafana/` - Grafana dashboards
  - `docker-compose.yml`
  - Dashboard JSON files
  - Exclude: Any local configs

### Configuration
- `.gitignore` - Updated (no Pyrax references)
- `LICENSE` - Add license file
- `.github/` - GitHub workflows (if any)

## ❌ What to EXCLUDE (Private/Internal)

### Internal Development Docs
- `*_INTERNAL*.md`
- `*_PRIVATE*.md`
- `*_DEV*.md`
- `*_NOTES*.md`
- `PYRAX_*.md` (old name docs)
- `RENAME_*.md`
- `RENAME_*.md`
- Internal status/roadmap docs with sensitive info

### Build Artifacts
- `target/` directories
- `node_modules/`
- `dist/` directories
- `*.exe` (compiled binaries)
- `*.dll` (Windows libraries)

### Sensitive Data
- `*.key`, `*.pem` files
- `.env` files
- `config.local.*`
- `keystore/` directories
- `wallets/` directories
- Database files (`*.db`, `*.sqlite`)

### IDE/Editor Files
- `.vscode/`
- `.idea/`
- `*.swp`, `*.swo`

### Scripts (Development Only)
- `*.ps1` (PowerShell scripts)
- `*.sh` (if internal)
- Test scripts with hardcoded paths

### Old/Deprecated
- `pyrax-*` directories
- `mondoshawan_poc/`
- `mondoshawan_real/` (if contains internal stuff)

## 📋 Recommended Repository Structure

```
mondoshawan/
├── README.md
├── LICENSE
├── .gitignore
├── Mondoshawan_WHITEPAPER.md
├── Mondoshawan_WHITEPAPER.html
├── BRANDING.md
├── DEVELOPER_GUIDE.md
├── BUILD_INSTRUCTIONS.md
├── QUICK_START.md
├── mondoshawan-blockchain/
│   ├── README.md
│   ├── Cargo.toml
│   └── src/
├── mondoshawan-explorer-frontend/
│   ├── index.html
│   ├── app.js
│   └── styles.css
├── mondoshawan-website/
│   ├── index.html
│   ├── why-mondoshawan.html
│   ├── comparison.html
│   └── explorer/
└── grafana/
    ├── docker-compose.yml
    └── dashboards/
```

## 🔒 Security Checklist

- [x] No API keys in code
- [x] No passwords in code
- [x] No private keys
- [x] No hardcoded credentials
- [x] No internal IPs/domains
- [x] No Pyrax references
- [x] .gitignore properly configured
- [ ] LICENSE file added
- [ ] README updated for public

## 📝 Pre-Push Checklist

1. ✅ Fix PYRAX_WHITEPAPER.md reference in website
2. ✅ Update .gitignore (remove pyrax references)
3. ✅ Remove all internal development docs
4. ✅ Remove build artifacts
5. ✅ Remove test scripts with hardcoded paths
6. ✅ Add LICENSE file
7. ✅ Update README for public audience
8. ✅ Verify no sensitive data in code
9. ✅ Test that repo can be cloned and built
