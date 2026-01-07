# Mondoshawan Quick Start Guide

## ✅ Everything Should Be Installed Now

### Quick Build Options

#### Option 1: Use the Build Script (Easiest)
```powershell
cd D:\Mondoshawan
.\build.ps1
```

#### Option 2: Use Developer Command Prompt (Recommended)
1. Open **"Developer Command Prompt for VS 2022"** from Start Menu
2. Run:
   ```powershell
   cd D:\Mondoshawan\Mondoshawan-blockchain
   cargo build
   ```

#### Option 3: Manual Setup
```powershell
# Set Rust paths
$env:CARGO_HOME = "D:\Rust\.cargo"
$env:RUSTUP_HOME = "D:\Rust\.rustup"
$env:PATH = "D:\Rust\.cargo\bin;$env:PATH"

# Set Visual Studio paths
$vsPath = "C:\Program Files\Microsoft Visual Studio\18\Community"
$msvcVersion = "14.50.35717"
$linkerDir = "$vsPath\VC\Tools\MSVC\$msvcVersion\bin\Hostx64\x64"
$vcLibPath = "$vsPath\VC\Tools\MSVC\$msvcVersion\lib\x64"

# Find Windows SDK
$sdkPaths = @("C:\Program Files (x86)\Windows Kits\10", "C:\Program Files\Windows Kits\10")
$sdkLib = $null
foreach ($sdkBase in $sdkPaths) {
    if (Test-Path "$sdkBase\Lib") {
        $sdkVersions = Get-ChildItem "$sdkBase\Lib" -Directory | Where-Object { $_.Name -match "10\." } | Sort-Object Name -Descending
        if ($sdkVersions) {
            $sdkLib = "$($sdkVersions[0].FullName)\um\x64"
            if (Test-Path $sdkLib) { break }
        }
    }
}

# Update environment
$env:PATH = "$linkerDir;$env:PATH"
if ($sdkLib) {
    $env:LIB = "$vcLibPath;$sdkLib;$env:LIB"
}

# Build
cd D:\Mondoshawan\Mondoshawan-blockchain
cargo build
```

## 🧪 Testing

### Test Rust
```powershell
cd D:\Mondoshawan\Mondoshawan-blockchain
cargo test
```

### Test Python POC
```powershell
cd D:\Mondoshawan\Mondoshawan_poc
python -c "import asyncio; print('Python OK')"
```

## 🔧 Troubleshooting

### If you get "link.exe not found"
- Use Developer Command Prompt for VS 2022
- Or run `.\build.ps1` script

### If you get "kernel32.lib not found"
- Install Windows 10/11 SDK via Visual Studio Installer
- Or use Developer Command Prompt (it sets this up automatically)

### If build is slow
- First build downloads and compiles dependencies (can take 10-20 minutes)
- Subsequent builds are much faster

## 📝 What's Installed

- ✅ Rust: D:\Rust\ (rustc 1.92.0)
- ✅ Python: D:\Python\ (Python 3.12.0)
- ✅ Node.js: v22.19.0
- ✅ Visual Studio 18 Community with C++ tools
- ✅ Windows SDK (should be installed)

## 🚀 Next Steps

1. Build the project: `.\build.ps1` or use Developer Command Prompt
2. Run tests: `cargo test`
3. Explore the code: Check `Mondoshawan-blockchain/src/` for Rust code
4. Run Python POC: `cd Mondoshawan_poc && python -m asyncio`

