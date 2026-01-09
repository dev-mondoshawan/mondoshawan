# Mondoshawan Setup Guide

## Installation Status

### ✅ Installed on D Drive

1. **Rust** - `D:\Rust\`
   - rustc 1.92.0
   - cargo 1.92.0
   - Toolchain: stable-x86_64-pc-windows-msvc

2. **Python** - `D:\Python\`
   - Python 3.12.0
   - pip 23.2.1

3. **Node.js** - System installation
   - v22.19.0

### ⚠️ Required: Visual Studio Build Tools

The Rust project uses the MSVC toolchain which requires Visual Studio Build Tools.

#### Option 1: Install Visual Studio Build Tools (Recommended)

1. Download from: https://visualstudio.microsoft.com/downloads/
2. Select "Build Tools for Visual Studio 2022"
3. During installation, select:
   - ✅ C++ build tools
   - ✅ Windows 10/11 SDK (latest)
   - ✅ MSVC v143 compiler toolset
4. Install location: Can be on D drive if preferred

#### Option 2: Switch to GNU Toolchain

If you prefer not to install Visual Studio Build Tools, you can switch to the GNU toolchain:

```powershell
$env:CARGO_HOME = "D:\Rust\.cargo"
$env:RUSTUP_HOME = "D:\Rust\.rustup"
$env:PATH = "D:\Rust\.cargo\bin;$env:PATH"

# Install GNU toolchain
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

Note: You'll need to install MinGW-w64 for the GNU toolchain.

## Environment Setup

### PowerShell Profile Setup

Add to your PowerShell profile (`$PROFILE`):

```powershell
# Rust
$env:CARGO_HOME = "D:\Rust\.cargo"
$env:RUSTUP_HOME = "D:\Rust\.rustup"
$env:PATH = "D:\Rust\.cargo\bin;$env:PATH"

# Python
$env:PATH = "D:\Python;D:\Python\Scripts;$env:PATH"
```

### Verify Installation

```powershell
# Check Rust
rustc --version
cargo --version

# Check Python
python --version
pip --version

# Check Node.js
node --version
```

## Building the Project

### Rust Project

```powershell
cd Mondoshawan-blockchain
cargo build
cargo test
```

### Python POC

```powershell
cd Mondoshawan_poc
python -m asyncio  # Test imports
```

## Troubleshooting

### Rust: "linker `link.exe` not found"

**Solution**: Install Visual Studio Build Tools (see above) or switch to GNU toolchain.

### Python: "python not recognized"

**Solution**: 
1. Verify Python is installed at `D:\Python\`
2. Add to PATH: `$env:PATH = "D:\Python;D:\Python\Scripts;$env:PATH"`
3. Restart terminal

### Cargo: "could not compile"

**Solution**: 
1. Ensure Visual Studio Build Tools are installed
2. Or switch to GNU toolchain
3. Run `cargo clean` and try again

## Next Steps

1. ✅ Install Visual Studio Build Tools (or switch to GNU)
2. ✅ Build Rust project: `cargo build`
3. ✅ Test Python POC: `python -m Mondoshawan_poc.testnet`
4. ✅ Run frontend: Open `Mondoshawan-explorer-frontend/index.html`

