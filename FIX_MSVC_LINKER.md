# Fix MSVC Linker Issue - Complete Solution

## Problem
`msvcrt.lib` cannot be found. The lib directory only contains Clang libraries, not MSVC runtime libraries.

## Root Cause
The **C++ build tools** component is not fully installed. The installation has Clang tools but is missing the standard MSVC runtime libraries.

## Solution: Install Missing C++ Components

### Step-by-Step:

1. **Open Visual Studio Installer**
   - Search "Visual Studio Installer" in Start Menu
   - Or go to: `C:\Program Files (x86)\Microsoft Visual Studio\Installer\vs_installer.exe`

2. **Find Your Installation**
   - Look for "Visual Studio 2022" (or VS 18)
   - Click **Modify**

3. **Install Required Components**
   - Under **Workloads**, check:
     - ✅ **Desktop development with C++**
   
   - Under **Individual components**, ensure:
     - ✅ **MSVC v143 - VS 2022 C++ x64/x86 build tools (Latest)**
     - ✅ **Windows 10 SDK** (or Windows 11 SDK) - latest version
     - ✅ **C++ CMake tools for Windows**
     - ✅ **C++ core features**

4. **Click Modify** and wait for installation

5. **Restart your terminal/PowerShell**

6. **Use Developer Command Prompt**
   - Open "Developer Command Prompt for VS 2022"
   - Run: `cd D:\Mondoshawan\Mondoshawan-blockchain && cargo build`

## Alternative: Use Developer Command Prompt Now

Even without the full installation, try the Developer Command Prompt:

1. Open Start Menu
2. Search: **"Developer Command Prompt for VS 2022"**
3. If it exists, use it - it may have access to libraries we can't find
4. Run:
   ```cmd
   cd D:\Mondoshawan\Mondoshawan-blockchain
   cargo build
   ```

## Alternative: Switch to GNU Toolchain

If MSVC continues to be problematic:

```powershell
# Install GNU toolchain
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu

# Install MinGW-w64 (required)
# Download from: https://www.mingw-w64.org/downloads/
# Or use: winget install mingw-w64
```

Then rebuild:
```cmd
cd D:\Mondoshawan\Mondoshawan-blockchain
cargo clean
cargo build
```

## Verification

After installing components, verify:

```cmd
dir "C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\14.50.35717\lib\x64\msvcrt.lib"
```

If this file exists, the installation is complete.

## Why This Happens

Visual Studio can be installed with different configurations:
- **Clang/LLVM tools** - What you currently have
- **MSVC tools** - What Rust needs (includes msvcrt.lib)

You need **both** or just **MSVC tools** for Rust compilation.

## Quick Fix Summary

**Best**: Install "Desktop development with C++" workload in Visual Studio Installer  
**Quick**: Use Developer Command Prompt (if available)  
**Alternative**: Switch to GNU toolchain

## After Fixing

Once `msvcrt.lib` is available:

```cmd
cd D:\Mondoshawan\Mondoshawan-blockchain
cargo build
cargo test blockchain::tests
```

All tests should pass!

